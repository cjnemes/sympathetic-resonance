//! Command execution handlers
//!
//! This module contains handlers that execute parsed commands

use crate::input::command_parser::{ParsedCommand, CommandResult};
use crate::core::{Player, WorldState};
use crate::persistence::DatabaseManager;
use crate::GameResult;

/// Trait for handling command execution
pub trait CommandHandler {
    fn execute(
        &self,
        command: ParsedCommand,
        player: &mut Player,
        world: &mut WorldState,
        database: &DatabaseManager,
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
                handle_magic(spell_type, crystal, target, player, world)
            }

            ParsedCommand::Talk { target } => {
                handle_talk(target, player, world, database)
            }

            ParsedCommand::Ask { target, topic } => {
                handle_ask(target, topic, player, world, database)
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
                handle_study(theory, player, database)
            }

            ParsedCommand::Research { topic } => {
                handle_research(topic, player)
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

            let location = world.current_location()
                .ok_or_else(|| crate::GameError::ContentNotFound("Current location not found".to_string()))?;

            // Mark location as visited
            if let Some(loc) = world.locations.get_mut(&destination) {
                loc.visited = true;
            }

            let mut response = format!("You head {}.\n\n", direction.display_name());
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
    crystal: Option<String>,
    target: Option<String>,
    player: &mut Player,
    world: &mut WorldState,
) -> GameResult<String> {
    // Basic magic implementation - will be expanded with full calculation engine

    // Check if player has a crystal
    let active_crystal = player.active_crystal()
        .ok_or_else(|| crate::GameError::InsufficientResources("No crystal equipped".to_string()))?;

    // Check mental energy
    let energy_cost = 15; // Basic cost
    let fatigue_cost = 8;

    if player.effective_mental_energy() < energy_cost {
        return Ok(format!(
            "You don't have enough mental energy to cast {}. You need {} energy but only have {} effective energy.",
            spell_type, energy_cost, player.effective_mental_energy()
        ));
    }

    // Use energy
    player.use_mental_energy(energy_cost, fatigue_cost)?;

    // Add magical signature to current location
    world.add_magical_signature(spell_type.clone(), 0.5, active_crystal.frequency);

    // Advance time
    world.advance_time(2);
    player.playtime_minutes += 2;

    // Generate response
    let mut response = format!(
        "You focus your mental energy through the {} crystal, casting {}",
        active_crystal.display_name(),
        spell_type
    );

    if let Some(target_str) = target {
        response.push_str(&format!(" on the {}", target_str));
    }

    response.push_str(".\n\n");

    // Add results based on spell type
    match spell_type.as_str() {
        "light" => {
            response.push_str("A soft, steady light emanates from the crystal, illuminating the area.");
        }
        "healing" => {
            if target.is_some() {
                response.push_str("Warm energy flows through the sympathetic connection, accelerating natural healing processes.");
            } else {
                response.push_str("You feel a warm, healing energy flow through your body.");
            }
        }
        _ => {
            response.push_str("The magical energy manifests according to your will.");
        }
    }

    // Show energy status
    response.push_str(&format!(
        "\n\nMental Energy: {}/{} (Fatigue: {})",
        player.mental_state.current_energy,
        player.mental_state.max_energy,
        player.mental_state.fatigue
    ));

    Ok(response)
}

/// Handle talking to NPCs
fn handle_talk(
    target: String,
    _player: &Player,
    _world: &WorldState,
    _database: &DatabaseManager,
) -> GameResult<String> {
    Ok(format!("You approach the {} to start a conversation. [NPC dialogue system not yet implemented]", target))
}

/// Handle asking NPCs about topics
fn handle_ask(
    target: String,
    topic: String,
    _player: &Player,
    _world: &WorldState,
    _database: &DatabaseManager,
) -> GameResult<String> {
    Ok(format!("You ask the {} about {}. [NPC dialogue system not yet implemented]", target, topic))
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

/// Handle status display
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

    // Knowledge
    response.push_str("\nKnowledge:\n");
    if player.knowledge.theories.is_empty() {
        response.push_str("  No theories learned\n");
    } else {
        for (theory, understanding) in &player.knowledge.theories {
            response.push_str(&format!("  {} ({:.0}% understanding)\n", theory, understanding * 100.0));
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

/// Handle study command
fn handle_study(theory: String, player: &mut Player, _database: &DatabaseManager) -> GameResult<String> {
    // Basic study implementation
    let study_time = 30; // 30 minutes
    player.playtime_minutes += study_time;

    // Add some understanding to the theory
    let current_understanding = player.knowledge.theories.get(&theory).copied().unwrap_or(0.0);
    let progress = 0.1; // 10% progress per study session
    let new_understanding = (current_understanding + progress).min(1.0);

    player.knowledge.theories.insert(theory.clone(), new_understanding);

    Ok(format!(
        "You spend {} minutes studying {}.\n\
         Understanding: {:.0}% -> {:.0}%",
        study_time, theory,
        current_understanding * 100.0,
        new_understanding * 100.0
    ))
}

/// Handle research command
fn handle_research(topic: String, _player: &mut Player) -> GameResult<String> {
    Ok(format!("You begin researching {}. [Research system not yet implemented]", topic))
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
) -> GameResult<String> {
    let handler = DefaultCommandHandler;
    handler.execute(command, player, world, database)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::world_state::Direction;

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