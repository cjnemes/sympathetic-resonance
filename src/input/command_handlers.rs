//! Command execution handlers
//!
//! This module contains handlers that execute parsed commands

use crate::input::command_parser::ParsedCommand;
use crate::core::{Player, WorldState};
use crate::persistence::DatabaseManager;
use crate::systems::magic::MagicSystem;
use crate::systems::dialogue::DialogueSystem;
use crate::systems::factions::FactionSystem;
use crate::systems::knowledge::{KnowledgeSystem, LearningMethod};
use crate::GameResult;

/// Trait for handling command execution
pub trait CommandHandler {
    fn execute(
        &self,
        command: ParsedCommand,
        player: &mut Player,
        world: &mut WorldState,
        database: &DatabaseManager,
        magic_system: &mut MagicSystem,
        dialogue_system: &mut DialogueSystem,
        faction_system: &FactionSystem,
        knowledge_system: &mut KnowledgeSystem,
    ) -> GameResult<String>;
}

/// Default command handler implementation
pub struct DefaultCommandHandler;

impl CommandHandler for DefaultCommandHandler {
    fn execute(
        &self,
        command: ParsedCommand,
        player: &mut Player,
        world: &mut WorldState,
        database: &DatabaseManager,
        magic_system: &mut MagicSystem,
        dialogue_system: &mut DialogueSystem,
        faction_system: &FactionSystem,
        knowledge_system: &mut KnowledgeSystem,
    ) -> GameResult<String> {
        match command {
            ParsedCommand::Move { direction } => {
                handle_movement(direction, player, world)
            }

            ParsedCommand::Look { target } => {
                handle_look(target, player, world, database)
            }

            ParsedCommand::Examine { target } => {
                handle_examine(target, player, world, database)
            }

            ParsedCommand::CastMagic { spell_type, crystal, target } => {
                handle_magic(spell_type, crystal, target, player, world, magic_system)
            }

            ParsedCommand::Talk { target } => {
                handle_talk(target, player, world, database, dialogue_system, faction_system)
            }

            ParsedCommand::Ask { target, topic } => {
                handle_ask(target, topic, player, world, database, dialogue_system, faction_system)
            }

            ParsedCommand::Inventory => {
                handle_inventory(player)
            }

            ParsedCommand::Status => {
                handle_status(player)
            }

            ParsedCommand::CrystalStatus => {
                handle_crystal_status(player)
            }

            ParsedCommand::FactionStatus => {
                handle_faction_status(player)
            }

            ParsedCommand::Rest => {
                handle_rest(player, world)
            }

            ParsedCommand::Meditate => {
                handle_meditate(player, world)
            }

            ParsedCommand::Study { theory } => {
                handle_study(theory, player, database, knowledge_system, world)
            }

            ParsedCommand::Research { topic } => {
                handle_research(topic, player, knowledge_system, world)
            }

            ParsedCommand::Take { item } => {
                handle_take(item, player, world)
            }

            ParsedCommand::Drop { item } => {
                handle_drop(item, player, world)
            }

            ParsedCommand::Equip { crystal } => {
                handle_equip_crystal(crystal, player)
            }

            ParsedCommand::Save { slot: _ } => {
                Ok("Save functionality not yet implemented.".to_string())
            }

            ParsedCommand::Load { slot: _ } => {
                Ok("Load functionality not yet implemented.".to_string())
            }

            ParsedCommand::Help { topic: _ } => {
                Ok("Help is handled by the parser.".to_string())
            }

            ParsedCommand::Quit => {
                Ok("QUIT_GAME".to_string()) // Special return value for game loop
            }

            ParsedCommand::Unknown { original, suggestions } => {
                handle_unknown_command(original, suggestions)
            }
        }
    }
}

/// Handle movement commands
fn handle_movement(
    direction: crate::core::world_state::Direction,
    player: &mut Player,
    world: &mut WorldState,
) -> GameResult<String> {
    match world.move_to_location(direction.clone()) {
        Ok(destination) => {
            player.current_location = destination.clone();

            // Advance time slightly for movement
            world.advance_time(1);
            player.playtime_minutes += 1;

            let mut response = format!("You head {}.\n\n", direction.display_name());

            let location = world.current_location()
                .ok_or_else(|| crate::GameError::ContentNotFound("Current location not found".to_string()))?;

            response.push_str(&generate_location_description(location, player));

            Ok(response)
        }
        Err(e) => {
            Ok(format!("You can't go that way. {}", e))
        }
    }
}

/// Handle look commands
fn handle_look(
    target: Option<String>,
    player: &Player,
    world: &WorldState,
    _database: &DatabaseManager,
) -> GameResult<String> {
    match target {
        Some(target_str) => {
            // Look at specific target
            Ok(format!("You look closely at the {}. [Detailed examination not yet implemented]", target_str))
        }
        None => {
            // Look around current location
            let location = world.current_location()
                .ok_or_else(|| crate::GameError::ContentNotFound("Current location not found".to_string()))?;

            Ok(generate_location_description(location, player))
        }
    }
}

/// Handle examine commands
fn handle_examine(
    target: String,
    player: &Player,
    world: &WorldState,
    _database: &DatabaseManager,
) -> GameResult<String> {
    // Check if examining own crystals
    if target.contains("crystal") && (target.contains("my") || target.contains("crystals")) {
        return handle_crystal_status(player);
    }

    // Check if target is in current location
    let location = world.current_location()
        .ok_or_else(|| crate::GameError::ContentNotFound("Current location not found".to_string()))?;

    // For now, provide basic examination
    let mut response = format!("You examine the {} carefully.\n\n", target);

    // Add magical analysis if player has resonance sensitivity
    if player.attributes.resonance_sensitivity > 10 {
        response.push_str(&format!(
            "Your magical senses detect: [Magical analysis not yet implemented]\n\
             Resonance Sensitivity: {}/100\n",
            player.attributes.resonance_sensitivity
        ));
    }

    // Add environmental context
    response.push_str(&format!(
        "\nAmbient magical energy: {:.1}\n",
        location.magical_properties.ambient_energy
    ));

    if let Some(freq) = location.magical_properties.dominant_frequency {
        response.push_str(&format!("Dominant resonance frequency: {}\n", freq));
    }

    Ok(response)
}

/// Handle magic casting
fn handle_magic(
    spell_type: String,
    _crystal: Option<String>,
    target: Option<String>,
    player: &mut Player,
    world: &mut WorldState,
    magic_system: &mut MagicSystem,
) -> GameResult<String> {
    // Use the MagicSystem for proper calculation and execution
    match magic_system.attempt_magic(&spell_type, player, world, target.as_deref()) {
        Ok(result) => {
            let mut response = String::new();

            if result.success {
                response.push_str(&format!(
                    "You successfully cast {}{}.\n\n",
                    spell_type,
                    target.as_ref().map(|t| format!(" on {}", t)).unwrap_or_default()
                ));

                response.push_str(&result.explanation);
                response.push_str(&format!(
                    "\n\nPower Level: {:.1}\nEnergy Cost: {}\nTime Taken: {} minutes",
                    result.power_level,
                    result.energy_cost,
                    result.time_cost
                ));
            } else {
                response.push_str(&format!(
                    "Your attempt to cast {} failed.\n\n",
                    spell_type
                ));
                response.push_str(&result.explanation);
            }

            // Show current energy status
            response.push_str(&format!(
                "\n\nMental Energy: {}/{} (Fatigue: {})",
                player.mental_state.current_energy,
                player.mental_state.max_energy,
                player.mental_state.fatigue
            ));

            Ok(response)
        }
        Err(e) => {
            Ok(format!("Unable to cast {}: {}", spell_type, e))
        }
    }
}

/// Handle talking to NPCs with theory-aware responses
fn handle_talk(
    target: String,
    player: &Player,
    world: &WorldState,
    _database: &DatabaseManager,
    dialogue_system: &mut DialogueSystem,
    faction_system: &FactionSystem,
) -> GameResult<String> {
    // For now, try to find an NPC in the current location
    let location = world.current_location()
        .ok_or_else(|| crate::GameError::ContentNotFound("Current location not found".to_string()))?;

    // Check if the target is mentioned in the location description or NPCs
    if location.description.to_lowercase().contains(&target.to_lowercase()) {
        match dialogue_system.talk_to_npc(&target, player, faction_system) {
            Ok(mut response) => {
                // Add theory-aware topics
                let theory_topics = dialogue_system.get_theory_topics(&target, player);
                let theory_only_topics: Vec<String> = theory_topics.iter()
                    .filter(|topic| {
                        matches!(topic.as_str(),
                            "resonance_theory" | "crystal_research" | "mental_techniques" |
                            "light_experiments" | "healing_methods" | "detection_techniques" |
                            "network_theory" | "advanced_amplification" | "theoretical_mastery" |
                            "advanced_theory_discussion" | "research_collaboration" |
                            "theoretical_breakthroughs" | "healing_applications" |
                            "magical_detection" | "long_distance_communication" | "spell_innovation"
                        )
                    })
                    .cloned()
                    .collect();

                if !theory_only_topics.is_empty() {
                    response.push_str("\n\nTheory Discussion Topics: ");
                    response.push_str(&theory_only_topics.join(", "));
                }

                Ok(response)
            },
            Err(_) => {
                // If specific NPC not found, create a basic interaction
                Ok(format!(
                    "You approach the {} to start a conversation.\n\nThe {} acknowledges you but seems to have little to say.\n\n[Full NPC dialogue system loading...]",
                    target, target
                ))
            }
        }
    } else {
        Ok(format!("You don't see {} here to talk to.", target))
    }
}

/// Handle asking NPCs about topics with theory-aware responses
fn handle_ask(
    target: String,
    topic: String,
    player: &Player,
    world: &WorldState,
    _database: &DatabaseManager,
    dialogue_system: &mut DialogueSystem,
    faction_system: &FactionSystem,
) -> GameResult<String> {
    // For now, try to find an NPC in the current location
    let location = world.current_location()
        .ok_or_else(|| crate::GameError::ContentNotFound("Current location not found".to_string()))?;

    // Check if the target is mentioned in the location description or NPCs
    if location.description.to_lowercase().contains(&target.to_lowercase()) {
        // First try theory-aware responses
        if let Some(theory_response) = dialogue_system.get_theory_response(&target, &topic, player) {
            return Ok(format!("You ask {} about {}.\n\n{}", target, topic, theory_response));
        }

        // Fall back to standard dialogue system
        match dialogue_system.ask_about_topic(&target, &topic, player, faction_system) {
            Ok(response) => Ok(response),
            Err(_) => {
                // If specific NPC not found, create a basic interaction
                Ok(format!(
                    "You ask the {} about {}.\n\nThe {} doesn't seem to know much about that topic.\n\n[Topic: {} - Full dialogue system loading...]",
                    target, topic, target, topic
                ))
            }
        }
    } else {
        Ok(format!("You don't see {} here to ask about {}.", target, topic))
    }
}

/// Handle inventory display
fn handle_inventory(player: &Player) -> GameResult<String> {
    let mut response = String::new();
    response.push_str("=== INVENTORY ===\n\n");

    // Crystals
    response.push_str("Crystals:\n");
    if player.inventory.crystals.is_empty() {
        response.push_str("  None\n");
    } else {
        for (i, crystal) in player.inventory.crystals.iter().enumerate() {
            let equipped = if Some(i) == player.inventory.active_crystal { " (equipped)" } else { "" };
            response.push_str(&format!(
                "  {} - {:.0}% integrity, {:.0}% purity{}\n",
                crystal.display_name(),
                crystal.integrity,
                crystal.purity * 100.0,
                equipped
            ));
        }
    }

    // Items
    response.push_str("\nItems:\n");
    if player.inventory.items.is_empty() {
        response.push_str("  None\n");
    } else {
        for item in &player.inventory.items {
            response.push_str(&format!("  {}\n", item.name));
        }
    }

    // Currency
    response.push_str(&format!("\nSilver: {} pieces\n", player.inventory.silver));

    Ok(response)
}

/// Handle status display with theory benefits
fn handle_status(player: &Player) -> GameResult<String> {
    let mut response = String::new();
    response.push_str(&format!("=== {} ===\n\n", player.name));

    // Attributes
    response.push_str("Attributes:\n");
    response.push_str(&format!("  Mental Acuity: {}/100\n", player.attributes.mental_acuity));
    response.push_str(&format!("  Resonance Sensitivity: {}/100\n", player.attributes.resonance_sensitivity));

    // Mental state
    response.push_str("\nMental State:\n");
    response.push_str(&format!("  Energy: {}/{}\n", player.mental_state.current_energy, player.mental_state.max_energy));
    response.push_str(&format!("  Fatigue: {}/100\n", player.mental_state.fatigue));
    response.push_str(&format!("  Effective Energy: {}\n", player.effective_mental_energy()));

    // Active crystal
    response.push_str("\nActive Crystal:\n");
    if let Some(crystal) = player.active_crystal() {
        response.push_str(&format!("  {} (Freq: {}, {:.0}% integrity)\n",
            crystal.display_name(), crystal.frequency, crystal.integrity));
    } else {
        response.push_str("  None equipped\n");
    }

    // Theory Benefits
    response.push_str("\nTheory Benefits:\n");
    let magic_bonus = player.calculate_theory_magic_bonus();
    let energy_reduction = player.calculate_theory_energy_reduction();
    let crystal_protection = player.calculate_theory_crystal_protection();
    let fatigue_resistance = player.calculate_theory_fatigue_resistance();

    if magic_bonus > 0.01 {
        response.push_str(&format!("  Magic Success Bonus: +{:.1}%\n", magic_bonus * 100.0));
    }
    if energy_reduction > 0.01 {
        response.push_str(&format!("  Energy Cost Reduction: -{:.1}%\n", energy_reduction * 100.0));
    }
    if crystal_protection > 0.01 {
        response.push_str(&format!("  Crystal Protection: -{:.1}% degradation\n", crystal_protection * 100.0));
    }
    if fatigue_resistance > 0.01 {
        response.push_str(&format!("  Fatigue Resistance: -{:.1}%\n", fatigue_resistance * 100.0));
    }

    if magic_bonus <= 0.01 && energy_reduction <= 0.01 && crystal_protection <= 0.01 && fatigue_resistance <= 0.01 {
        response.push_str("  None (study theories to gain benefits)\n");
    }

    // Magic Capabilities
    response.push_str("\nMagic Capabilities:\n");
    let mut capabilities = Vec::new();
    if player.has_magic_capability("healing_spells") {
        capabilities.push("Healing Spells");
    }
    if player.has_magic_capability("detection_spells") {
        capabilities.push("Detection Spells");
    }
    if player.has_magic_capability("long_distance_magic") {
        capabilities.push("Long-Distance Magic");
    }
    if player.has_magic_capability("power_amplification") {
        capabilities.push("Power Amplification");
    }
    if player.has_magic_capability("custom_spell_combinations") {
        capabilities.push("Custom Spell Combinations");
    }

    if capabilities.is_empty() {
        response.push_str("  Basic magic only (learn theories to unlock advanced capabilities)\n");
    } else {
        for capability in capabilities {
            response.push_str(&format!("  {}\n", capability));
        }
    }

    // Knowledge
    response.push_str("\nKnowledge:\n");
    if player.knowledge.theories.is_empty() {
        response.push_str("  No theories learned\n");
    } else {
        let mastered_count = player.get_mastered_theories().len();
        response.push_str(&format!("  Theories Mastered: {}/{}\n", mastered_count, player.knowledge.theories.len()));

        for (theory, understanding) in &player.knowledge.theories {
            let status = if *understanding >= 1.0 { "MASTERED" } else { "learning" };
            response.push_str(&format!("  {} ({:.0}% - {})\n", theory, understanding * 100.0, status));
        }
    }

    // Play time
    let hours = player.playtime_minutes / 60;
    let minutes = player.playtime_minutes % 60;
    response.push_str(&format!("\nPlay time: {}h {}m\n", hours, minutes));

    Ok(response)
}

/// Handle crystal status display
fn handle_crystal_status(player: &Player) -> GameResult<String> {
    let mut response = String::new();
    response.push_str("=== CRYSTAL STATUS ===\n\n");

    if player.inventory.crystals.is_empty() {
        response.push_str("You have no crystals.\n");
        return Ok(response);
    }

    for (i, crystal) in player.inventory.crystals.iter().enumerate() {
        let equipped = if Some(i) == player.inventory.active_crystal { " [EQUIPPED]" } else { "" };

        response.push_str(&format!("{}{}:\n", crystal.display_name(), equipped));
        response.push_str(&format!("  Type: {:?}\n", crystal.crystal_type));
        response.push_str(&format!("  Frequency: {}\n", crystal.frequency));
        response.push_str(&format!("  Integrity: {:.1}%\n", crystal.integrity));
        response.push_str(&format!("  Purity: {:.1}%\n", crystal.purity * 100.0));
        response.push_str(&format!("  Size: {:?}\n", crystal.size));
        response.push_str(&format!("  Efficiency: {:.1}%\n", crystal.efficiency() * 100.0));
        response.push_str(&format!("  Power Multiplier: {:.1}x\n\n", crystal.power_multiplier()));
    }

    Ok(response)
}

/// Handle faction status display
fn handle_faction_status(player: &Player) -> GameResult<String> {
    let mut response = String::new();
    response.push_str("=== FACTION STANDINGS ===\n\n");

    use crate::systems::factions::FactionId;

    for faction_id in FactionId::all() {
        let reputation = player.faction_reputation(faction_id);
        let standing = match reputation {
            81..=100 => "Inner Circle",
            51..=80 => "Trusted Ally",
            21..=50 => "Member",
            -20..=20 => "Neutral",
            -50..=-21 => "Suspected",
            -80..=-51 => "Enemy",
            -100..=-81 => "Marked for Elimination",
            _ => "Unknown",
        };

        response.push_str(&format!("{}: {} ({})\n",
            faction_id.display_name(), reputation, standing));
    }

    Ok(response)
}

/// Handle rest command
fn handle_rest(player: &mut Player, world: &mut WorldState) -> GameResult<String> {
    let rest_time = 60; // 1 hour
    let fatigue_reduction = 10;

    player.recover_energy(0, fatigue_reduction);
    world.advance_time(rest_time);
    player.playtime_minutes += rest_time;

    Ok(format!(
        "You rest for an hour, feeling somewhat refreshed.\n\
         Fatigue reduced by {}. Current fatigue: {}/100",
        fatigue_reduction, player.mental_state.fatigue
    ))
}

/// Handle meditate command
fn handle_meditate(player: &mut Player, world: &mut WorldState) -> GameResult<String> {
    let meditation_time = 60; // 1 hour
    let fatigue_reduction = 15;

    player.recover_energy(0, fatigue_reduction);
    world.advance_time(meditation_time);
    player.playtime_minutes += meditation_time;

    Ok(format!(
        "You enter a meditative state, focusing your mind and clearing mental fog.\n\
         Fatigue reduced by {}. Current fatigue: {}/100",
        fatigue_reduction, player.mental_state.fatigue
    ))
}

/// Handle study command with enhanced knowledge system
fn handle_study(
    theory: String,
    player: &mut Player,
    _database: &DatabaseManager,
    knowledge_system: &mut KnowledgeSystem,
    world: &mut WorldState
) -> GameResult<String> {
    let study_time = 30; // 30 minutes

    // Check if player can access this theory
    let accessible_theories = knowledge_system.get_accessible_theories(player)?;
    let theory_available = accessible_theories.iter().any(|t| t.id == theory);

    if !theory_available {
        // Fallback to simple study for backward compatibility
        let current_understanding = player.knowledge.theories.get(&theory).copied().unwrap_or(0.0);
        let progress = 0.1; // 10% progress per study session
        let new_understanding = (current_understanding + progress).min(1.0);
        player.knowledge.theories.insert(theory.clone(), new_understanding);
        player.playtime_minutes += study_time;

        return Ok(format!(
            "You spend {} minutes studying {}.\n\
             Understanding: {:.0}% -> {:.0}%\n\
             (Enhanced theory system not available for this topic)",
            study_time, theory,
            current_understanding * 100.0,
            new_understanding * 100.0
        ));
    }

    // Check if player can use study method
    if !player.can_use_learning_method(&theory, &LearningMethod::Study) {
        return Ok("You cannot use the study method for this theory right now.".to_string());
    }

    // Start learning session
    player.start_learning_session(theory.clone(), LearningMethod::Study)?;

    // Attempt learning through the knowledge system
    match knowledge_system.attempt_learning(&theory, LearningMethod::Study, study_time, player, world) {
        Ok(activity) => {
            // Update player progress
            player.update_theory_progress(&activity)?;
            player.playtime_minutes += study_time;

            let mut response = format!(
                "You spend {} minutes studying {}.\n\n",
                study_time, theory
            );

            response.push_str(&format!(
                "Session Results:\n\
                 - Understanding gained: {:.1}%\n\
                 - Experience gained: {} XP\n\
                 - Success rate: {:.0}%\n",
                activity.understanding_gained * 100.0,
                activity.experience_gained,
                activity.success_rate * 100.0
            ));

            let current_understanding = player.theory_understanding(&theory);
            response.push_str(&format!(
                "\nCurrent understanding: {:.0}%",
                current_understanding * 100.0
            ));

            // Add mastery message if reached 100%
            if current_understanding >= 1.0 {
                response.push_str(&format!(
                    "\n\nCongratulations! You have mastered {}!",
                    theory
                ));
            }

            // Add side effects
            if !activity.side_effects.is_empty() {
                response.push_str("\n\nAdditional notes:\n");
                for effect in &activity.side_effects {
                    response.push_str(&format!("- {}\n", effect));
                }
            }

            player.end_learning_session();
            Ok(response)
        },
        Err(e) => {
            player.end_learning_session();
            Ok(format!("Study session failed: {}", e))
        }
    }
}

/// Handle research command with enhanced knowledge system
fn handle_research(
    topic: String,
    player: &mut Player,
    knowledge_system: &mut KnowledgeSystem,
    world: &mut WorldState
) -> GameResult<String> {
    let research_time = 120; // 2 hours for research

    // Check if player can access this theory for research
    let accessible_theories = knowledge_system.get_accessible_theories(player)?;
    let theory_available = accessible_theories.iter().any(|t| t.id == topic);

    if !theory_available {
        return Ok(format!(
            "You cannot research '{}' yet. You may need to learn prerequisite theories first.",
            topic
        ));
    }

    // Check if player can use research method
    if !player.can_use_learning_method(&topic, &LearningMethod::Research) {
        return Ok(format!(
            "You cannot research '{}' yet. Research requires at least 80% understanding and 60+ Mental Acuity.",
            topic
        ));
    }

    // Start research session
    player.start_learning_session(topic.clone(), LearningMethod::Research)?;

    // Attempt research through the knowledge system
    match knowledge_system.attempt_learning(&topic, LearningMethod::Research, research_time, player, world) {
        Ok(activity) => {
            // Update player progress
            player.update_theory_progress(&activity)?;
            player.playtime_minutes += research_time;

            let mut response = format!(
                "You spend {} hours conducting intensive research on {}.\n\n",
                research_time / 60, topic
            );

            response.push_str(&format!(
                "Research Results:\n\
                 - New insights gained: {:.1}%\n\
                 - Research experience: {} XP\n\
                 - Success rate: {:.0}%\n",
                activity.understanding_gained * 100.0,
                activity.experience_gained,
                activity.success_rate * 100.0
            ));

            let current_understanding = player.theory_understanding(&topic);
            response.push_str(&format!(
                "\nCurrent understanding: {:.0}%",
                current_understanding * 100.0
            ));

            // Add discovery messages based on side effects
            if !activity.side_effects.is_empty() {
                response.push_str("\n\nResearch Discoveries:\n");
                for effect in &activity.side_effects {
                    response.push_str(&format!("- {}\n", effect));
                }
            }

            player.end_learning_session();
            Ok(response)
        },
        Err(e) => {
            player.end_learning_session();
            Ok(format!("Research session failed: {}", e))
        }
    }
}

/// Handle take command
fn handle_take(item: String, _player: &mut Player, _world: &mut WorldState) -> GameResult<String> {
    Ok(format!("You attempt to take the {}. [Item system not yet implemented]", item))
}

/// Handle drop command
fn handle_drop(item: String, _player: &mut Player, _world: &mut WorldState) -> GameResult<String> {
    Ok(format!("You attempt to drop the {}. [Item system not yet implemented]", item))
}

/// Handle equip crystal command
fn handle_equip_crystal(crystal_name: String, player: &mut Player) -> GameResult<String> {
    // Find crystal by name
    for (i, crystal) in player.inventory.crystals.iter().enumerate() {
        if crystal.display_name().to_lowercase().contains(&crystal_name.to_lowercase()) {
            player.inventory.active_crystal = Some(i);
            return Ok(format!("You equip the {}.", crystal.display_name()));
        }
    }

    Ok(format!("You don't have a crystal matching '{}'.", crystal_name))
}

/// Handle unknown commands
fn handle_unknown_command(original: String, suggestions: Vec<String>) -> GameResult<String> {
    let mut response = format!("I don't understand '{}'.\n\n", original);

    if !suggestions.is_empty() {
        response.push_str("Suggestions:\n");
        for suggestion in suggestions {
            response.push_str(&format!("• {}\n", suggestion));
        }
    }

    Ok(response)
}

/// Generate location description
fn generate_location_description(
    location: &crate::core::world_state::Location,
    player: &Player,
) -> String {
    let mut description = format!("=== {} ===\n\n", location.name);
    description.push_str(&location.description);
    description.push_str("\n\n");

    // Add magical information if player has sensitivity
    if player.attributes.resonance_sensitivity > 5 {
        description.push_str(&format!(
            "Magical Properties:\n\
             • Ambient energy: {:.1}\n",
            location.magical_properties.ambient_energy
        ));

        if let Some(freq) = location.magical_properties.dominant_frequency {
            description.push_str(&format!("• Dominant frequency: {}\n", freq));
        }

        if location.magical_properties.interference > 0.1 {
            description.push_str(&format!("• Interference level: {:.1}\n", location.magical_properties.interference));
        }

        if !location.magical_properties.phenomena.is_empty() {
            description.push_str("• Phenomena: ");
            description.push_str(&location.magical_properties.phenomena.join(", "));
            description.push_str("\n");
        }

        description.push_str("\n");
    }

    // Show exits
    if !location.exits.is_empty() {
        description.push_str("Exits: ");
        let exit_list: Vec<String> = location.exits.keys()
            .map(|dir| dir.display_name().to_string())
            .collect();
        description.push_str(&exit_list.join(", "));
        description.push_str("\n");
    }

    description
}

/// Main function to execute a command
pub fn execute_command(
    command: ParsedCommand,
    player: &mut Player,
    world: &mut WorldState,
    database: &DatabaseManager,
    magic_system: &mut MagicSystem,
    dialogue_system: &mut DialogueSystem,
    faction_system: &FactionSystem,
    knowledge_system: &mut KnowledgeSystem,
) -> GameResult<String> {
    let handler = DefaultCommandHandler;
    handler.execute(command, player, world, database, magic_system, dialogue_system, faction_system, knowledge_system)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_inventory() {
        let player = Player::new("Test Player".to_string());
        let result = handle_inventory(&player).unwrap();
        assert!(result.contains("INVENTORY"));
        assert!(result.contains("Crystals:"));
    }

    #[test]
    fn test_handle_status() {
        let player = Player::new("Test Player".to_string());
        let result = handle_status(&player).unwrap();
        assert!(result.contains("Test Player"));
        assert!(result.contains("Mental Acuity:"));
    }

    #[test]
    fn test_handle_crystal_status() {
        let player = Player::new("Test Player".to_string());
        let result = handle_crystal_status(&player).unwrap();
        assert!(result.contains("CRYSTAL STATUS"));
    }
}