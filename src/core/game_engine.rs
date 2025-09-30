//! Main game engine coordinating all systems

use crate::core::{Player, WorldState, EventBus};
use crate::systems::{MagicSystem, FactionSystem, DialogueSystem, KnowledgeSystem, QuestSystem};
use crate::input::{CommandParser, execute_command};
use crate::persistence::{DatabaseManager, SaveManager};
use crate::GameResult;
use std::io::{self, Write};

/// Main game engine that coordinates all systems
pub struct GameEngine {
    /// Player character
    player: Player,
    /// World state
    world: WorldState,
    /// Magic system
    magic_system: MagicSystem,
    /// Faction system
    faction_system: FactionSystem,
    /// Dialogue system
    dialogue_system: DialogueSystem,
    /// Knowledge system
    knowledge_system: KnowledgeSystem,
    /// Quest system
    quest_system: QuestSystem,
    /// Command parser
    command_parser: CommandParser,
    /// Database manager
    database: DatabaseManager,
    /// Save manager
    save_manager: SaveManager,
    /// Event bus
    #[allow(dead_code)]
    event_bus: EventBus,
    /// Debug mode flag
    debug_mode: bool,
    /// Game running flag
    running: bool,
}

impl GameEngine {
    /// Create a new game engine
    pub fn new(database: DatabaseManager) -> GameResult<Self> {
        let player = Player::new("Adventurer".to_string());
        let mut world = WorldState::new();

        // Load locations from database
        let locations = database.load_locations()?;
        world.locations = locations;

        let save_manager = SaveManager::new()?;

        // Initialize knowledge system
        let mut knowledge_system = KnowledgeSystem::new();
        knowledge_system.initialize(&database)?;

        // Initialize dialogue system and load NPCs from database
        let mut dialogue_system = DialogueSystem::new();
        // Try to load NPCs, but don't fail if they don't exist or are malformed
        if let Ok(npcs) = database.load_npcs() {
            for npc in npcs {
                dialogue_system.add_npc(npc);
            }
        }

        // Initialize quest system with example quests
        let mut quest_system = QuestSystem::new();
        // Load quest definitions from database or create examples
        let example_quests = crate::systems::quest_examples::create_example_quests();
        for quest in example_quests {
            quest_system.add_quest_definition(quest);
        }

        Ok(Self {
            player,
            world,
            magic_system: MagicSystem::new(),
            faction_system: FactionSystem::new(),
            dialogue_system,
            knowledge_system,
            quest_system,
            command_parser: CommandParser::new(),
            database,
            save_manager,
            event_bus: EventBus::new(),
            debug_mode: false,
            running: false,
        })
    }

    /// Start the main game loop
    pub fn run(&mut self) -> GameResult<()> {
        self.running = true;
        self.show_initial_location()?;

        while self.running {
            // Get player input
            print!("> ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input.is_empty() {
                continue;
            }

            // Process command
            match self.process_command(input) {
                Ok(response) => {
                    if response == "QUIT_GAME" {
                        self.running = false;
                        println!("Goodbye!");
                    } else {
                        println!("{}\n", response);
                    }
                }
                Err(e) => {
                    println!("Error: {}\n", e);
                }
            }
        }

        Ok(())
    }

    /// Process a player command
    fn process_command(&mut self, input: &str) -> GameResult<String> {
        // Parse command
        let parse_result = self.command_parser.parse_advanced(input);

        match parse_result {
            crate::input::CommandResult::Success(command) => {
                execute_command(command, &mut self.player, &mut self.world, &self.database, &mut self.magic_system, &mut self.dialogue_system, &mut self.faction_system, &mut self.knowledge_system, &mut self.quest_system)
            }
            crate::input::CommandResult::Error(msg) => {
                Ok(msg)
            }
            crate::input::CommandResult::Help(help_text) => {
                Ok(help_text)
            }
        }
    }

    /// Show the initial location description
    fn show_initial_location(&self) -> GameResult<()> {
        if let Some(location) = self.world.current_location() {
            println!("=== Welcome to Sympathetic Resonance ===\n");
            println!("{}\n", location.description);

            if !location.exits.is_empty() {
                println!("Exits: {}",
                    location.exits.keys()
                        .map(|dir| dir.display_name())
                        .collect::<Vec<_>>()
                        .join(", "));
            }
            println!();
        }
        Ok(())
    }

    /// Load a save file
    pub fn load_save(&mut self, save_path: &str) -> GameResult<()> {
        let (player, world, quest_system) = self.save_manager.load_game(save_path)?;
        self.player = player;
        self.world = world;
        self.quest_system = quest_system;
        Ok(())
    }

    /// Set debug mode
    pub fn set_debug_mode(&mut self, enabled: bool) {
        self.debug_mode = enabled;
    }

    /// Get current player reference
    pub fn player(&self) -> &Player {
        &self.player
    }

    /// Get current world reference
    pub fn world(&self) -> &WorldState {
        &self.world
    }

    /// Get mutable player reference
    pub fn player_mut(&mut self) -> &mut Player {
        &mut self.player
    }

    /// Get mutable world reference
    pub fn world_mut(&mut self) -> &mut WorldState {
        &mut self.world
    }

    /// Get quest system reference
    pub fn quest_system(&self) -> &QuestSystem {
        &self.quest_system
    }

    /// Get mutable quest system reference
    pub fn quest_system_mut(&mut self) -> &mut QuestSystem {
        &mut self.quest_system
    }

    /// Get faction system reference
    pub fn faction_system(&self) -> &FactionSystem {
        &self.faction_system
    }

    /// Get mutable faction system reference
    pub fn faction_system_mut(&mut self) -> &mut FactionSystem {
        &mut self.faction_system
    }

    /// Get dialogue system reference
    pub fn dialogue_system(&self) -> &DialogueSystem {
        &self.dialogue_system
    }

    /// Get mutable dialogue system reference
    pub fn dialogue_system_mut(&mut self) -> &mut DialogueSystem {
        &mut self.dialogue_system
    }

    /// Get knowledge system reference
    pub fn knowledge_system(&self) -> &KnowledgeSystem {
        &self.knowledge_system
    }

    /// Get mutable knowledge system reference
    pub fn knowledge_system_mut(&mut self) -> &mut KnowledgeSystem {
        &mut self.knowledge_system
    }

    /// Handle cross-system quest integration
    pub fn handle_quest_events(&mut self) -> GameResult<Vec<String>> {
        let mut all_updates = Vec::new();

        // Check for dialogue-triggered quest objectives
        // This would be called after dialogue interactions

        // Check for location-triggered quest objectives
        let location_updates = self.quest_system.handle_location_visit(&self.player.current_location)?;
        all_updates.extend(location_updates);

        // Check for theory progress quest objectives
        for (theory_id, level) in &self.player.knowledge.theories {
            let theory_updates = self.quest_system.handle_theory_progress(theory_id, *level, &self.player)?;
            all_updates.extend(theory_updates);
        }

        Ok(all_updates)
    }

    /// Handle NPC dialogue with quest integration
    pub fn talk_to_npc(&mut self, npc_id: &str, topic: Option<&str>) -> GameResult<String> {
        // First handle the dialogue
        let dialogue_result = self.dialogue_system.talk_to_npc(npc_id, &self.player, &self.faction_system)?;

        // Then check for quest triggers
        let quest_updates = self.quest_system.handle_dialogue_trigger(npc_id, topic, &self.player)?;

        // Combine results
        let mut result = dialogue_result;
        if !quest_updates.is_empty() {
            result.push_str("\n\n--- Quest Updates ---\n");
            for update in quest_updates {
                result.push_str(&format!("• {}\n", update));
            }
        }

        Ok(result)
    }

    /// Handle theory learning with quest integration
    pub fn learn_theory(&mut self, theory_id: &str, method: &str, duration: i32) -> GameResult<String> {
        // First handle the learning
        let learning_method = match method {
            "study" => crate::systems::knowledge::LearningMethod::Study,
            "experiment" => crate::systems::knowledge::LearningMethod::Experimentation,
            "observe" => crate::systems::knowledge::LearningMethod::Observation,
            "teach" => crate::systems::knowledge::LearningMethod::Teaching,
            "research" => crate::systems::knowledge::LearningMethod::Research,
            "mentor" => crate::systems::knowledge::LearningMethod::Mentorship,
            _ => crate::systems::knowledge::LearningMethod::Study,
        };

        let learning_activity = self.knowledge_system.attempt_learning(
            theory_id,
            learning_method,
            duration,
            &mut self.player,
            &mut self.world,
        )?;

        // Then check for quest updates
        let new_level = self.player.theory_understanding(theory_id);
        let quest_updates = self.quest_system.handle_theory_progress(theory_id, new_level, &self.player)?;

        // Create result message from learning activity
        let mut result = format!(
            "Learning session completed! Gained {:.2} understanding in {}. Experience: {}",
            learning_activity.understanding_gained,
            learning_activity.theory_id,
            learning_activity.experience_gained
        );
        if !quest_updates.is_empty() {
            result.push_str("\n\n--- Quest Updates ---\n");
            for update in quest_updates {
                result.push_str(&format!("• {}\n", update));
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn create_test_engine() -> GameEngine {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let db = DatabaseManager::new(db_path).unwrap();
        db.initialize_schema().unwrap();
        db.load_default_content().unwrap();

        GameEngine::new(db).unwrap()
    }

    #[test]
    fn test_engine_creation() {
        let _engine = create_test_engine();
    }

    #[test]
    fn test_command_processing() {
        let mut engine = create_test_engine();
        let result = engine.process_command("look");
        assert!(result.is_ok());
    }

    #[test]
    fn test_debug_mode() {
        let mut engine = create_test_engine();
        engine.set_debug_mode(true);
        assert!(engine.debug_mode);
    }
}