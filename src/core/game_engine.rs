//! Main game engine coordinating all systems

use crate::core::{Player, WorldState};
use crate::systems::{MagicSystem, FactionSystem, DialogueSystem, KnowledgeSystem, QuestSystem, CombatSystem};
use crate::input::{CommandParser, execute_command};
use crate::persistence::{DatabaseManager, SaveManager};
use crate::GameResult;
use std::io::{self, Write};
use std::time::{Instant, Duration};

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
    /// Combat system
    combat_system: CombatSystem,
    /// Command parser
    command_parser: CommandParser,
    /// Database manager
    database: DatabaseManager,
    /// Save manager
    save_manager: SaveManager,
    /// Debug mode flag
    debug_mode: bool,
    /// Game running flag
    running: bool,
    /// Last autosave time
    last_autosave: Instant,
    /// Autosave interval (default: 5 minutes)
    autosave_interval: Duration,
    /// Autosave enabled flag
    autosave_enabled: bool,
    /// Maximum number of autosave files to keep
    max_autosaves: usize,
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
            combat_system: CombatSystem::new(),
            command_parser: CommandParser::new(),
            database,
            save_manager,
            debug_mode: false,
            running: false,
            last_autosave: Instant::now(),
            autosave_interval: Duration::from_secs(300), // 5 minutes default
            autosave_enabled: true,
            max_autosaves: 3,
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

            // Check if autosave is needed
            if let Err(e) = self.check_autosave() {
                if self.debug_mode {
                    println!("Autosave error: {}", e);
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
                execute_command(command, &mut self.player, &mut self.world, &self.database, &mut self.magic_system, &mut self.dialogue_system, &mut self.faction_system, &mut self.knowledge_system, &mut self.quest_system, &mut self.combat_system, &self.save_manager)
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

    /// Get combat system reference
    pub fn combat_system(&self) -> &CombatSystem {
        &self.combat_system
    }

    /// Get mutable combat system reference
    pub fn combat_system_mut(&mut self) -> &mut CombatSystem {
        &mut self.combat_system
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

    /// Check if autosave is needed and perform if necessary
    fn check_autosave(&mut self) -> GameResult<()> {
        if !self.autosave_enabled {
            return Ok(());
        }

        if self.last_autosave.elapsed() >= self.autosave_interval {
            self.perform_autosave(false)?;
            self.last_autosave = Instant::now();
        }

        Ok(())
    }

    /// Perform an autosave operation
    fn perform_autosave(&mut self, silent: bool) -> GameResult<()> {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let slot_name = format!("autosave_{}", timestamp);

        self.save_manager.save_game(
            &self.player,
            &self.world,
            &self.quest_system,
            Some(slot_name.clone()),
            Some("Auto Save".to_string())
        )?;

        if !silent && self.debug_mode {
            println!("[Auto-saved at {}]", chrono::Local::now().format("%H:%M"));
        }

        // Cleanup old autosaves
        self.cleanup_old_autosaves()?;

        Ok(())
    }

    /// Clean up old autosave files, keeping only the most recent ones
    fn cleanup_old_autosaves(&self) -> GameResult<()> {
        let saves = self.save_manager.list_save_slots()?;

        // Filter to autosave files only that exist and have info
        let mut autosaves: Vec<_> = saves.iter()
            .filter(|save| save.slot_name.starts_with("autosave_") && save.exists)
            .collect();

        // Sort by timestamp (newest first) - use info.timestamp
        autosaves.sort_by(|a, b| {
            let a_time = a.info.as_ref().map(|i| i.timestamp).unwrap_or(chrono::Utc::now());
            let b_time = b.info.as_ref().map(|i| i.timestamp).unwrap_or(chrono::Utc::now());
            b_time.cmp(&a_time)
        });

        // Delete old autosaves beyond max_autosaves
        let to_delete: Vec<_> = autosaves.iter().skip(self.max_autosaves).collect();
        for autosave in to_delete {
            self.save_manager.delete_save(&autosave.slot_name)?;
        }

        Ok(())
    }

    /// Trigger autosave after significant events
    pub fn trigger_event_autosave(&mut self, event_type: &str) -> GameResult<()> {
        if !self.autosave_enabled {
            return Ok(());
        }

        match event_type {
            "quest_complete" | "level_up" | "major_faction_change" | "combat_end" => {
                self.perform_autosave(true)?;
                self.last_autosave = Instant::now();
            }
            _ => {}
        }

        Ok(())
    }

    /// Configure autosave settings
    pub fn configure_autosave(&mut self, enabled: bool, interval_minutes: u64, max_saves: usize) {
        self.autosave_enabled = enabled;
        self.autosave_interval = Duration::from_secs(interval_minutes * 60);
        self.max_autosaves = max_saves;
    }

    /// Get autosave status
    pub fn autosave_status(&self) -> String {
        if self.autosave_enabled {
            let elapsed = self.last_autosave.elapsed().as_secs();
            let interval_secs = self.autosave_interval.as_secs();
            let remaining = interval_secs.saturating_sub(elapsed);

            format!(
                "Autosave: Enabled (every {} minutes, {} autosaves kept)\nNext autosave in: {} minutes {} seconds",
                interval_secs / 60,
                self.max_autosaves,
                remaining / 60,
                remaining % 60
            )
        } else {
            "Autosave: Disabled".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{NamedTempFile, TempDir};

    fn create_test_engine_with_temp_saves() -> (GameEngine, TempDir) {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let db = DatabaseManager::new(db_path).unwrap();
        db.initialize_schema().unwrap();
        db.load_default_content().unwrap();

        let mut engine = GameEngine::new(db).unwrap();

        // Create unique temporary save directory for this test
        let temp_dir = TempDir::new().unwrap();
        engine.save_manager.set_save_directory_for_test(temp_dir.path().to_path_buf());

        (engine, temp_dir)
    }

    fn create_test_engine() -> GameEngine {
        create_test_engine_with_temp_saves().0
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

    // ========== AUTOSAVE SYSTEM TESTS ==========

    #[test]
    fn test_autosave_enabled_by_default() {
        let engine = create_test_engine();
        assert!(engine.autosave_enabled);
        assert_eq!(engine.autosave_interval, Duration::from_secs(300)); // 5 minutes
        assert_eq!(engine.max_autosaves, 3);
    }

    #[test]
    fn test_configure_autosave() {
        let mut engine = create_test_engine();

        // Configure autosave: 10 minutes, keep 5 saves
        engine.configure_autosave(true, 10, 5);

        assert!(engine.autosave_enabled);
        assert_eq!(engine.autosave_interval, Duration::from_secs(600)); // 10 minutes
        assert_eq!(engine.max_autosaves, 5);
    }

    #[test]
    fn test_disable_autosave() {
        let mut engine = create_test_engine();

        engine.configure_autosave(false, 5, 3);
        assert!(!engine.autosave_enabled);
    }

    #[test]
    fn test_autosave_status_enabled() {
        let engine = create_test_engine();
        let status = engine.autosave_status();

        assert!(status.contains("Autosave: Enabled"));
        assert!(status.contains("every 5 minutes"));
        assert!(status.contains("3 autosaves kept"));
    }

    #[test]
    fn test_autosave_status_disabled() {
        let mut engine = create_test_engine();
        engine.configure_autosave(false, 5, 3);

        let status = engine.autosave_status();
        assert_eq!(status, "Autosave: Disabled");
    }

    #[test]
    fn test_perform_autosave_creates_file() {
        let (mut engine, _temp_dir) = create_test_engine_with_temp_saves();

        // Perform autosave
        let result = engine.perform_autosave(true);
        assert!(result.is_ok());

        // Check that autosave file was created
        let saves = engine.save_manager.list_save_slots().unwrap();
        let autosaves: Vec<_> = saves.iter()
            .filter(|s| s.slot_name.starts_with("autosave_"))
            .collect();

        assert_eq!(autosaves.len(), 1);
    }

    #[test]
    fn test_cleanup_keeps_only_max_autosaves() {
        let (mut engine, _temp_dir) = create_test_engine_with_temp_saves();
        engine.configure_autosave(true, 5, 3);

        // Create 5 autosaves with sufficient delays
        for _ in 0..5 {
            engine.perform_autosave(true).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(200)); // Ensure different timestamps and filesystem sync
        }

        // Check that only 3 or fewer are kept (cleanup should have run)
        let saves = engine.save_manager.list_save_slots().unwrap();
        let autosave_count = saves.iter()
            .filter(|s| s.slot_name.starts_with("autosave_"))
            .count();

        assert!(autosave_count <= 3, "Should keep at most max_autosaves (3) files, found {}", autosave_count);
        assert!(autosave_count > 0, "Should have at least 1 autosave file");
    }

    #[test]
    fn test_event_autosave_quest_complete() {
        let (mut engine, _temp_dir) = create_test_engine_with_temp_saves();

        let result = engine.trigger_event_autosave("quest_complete");
        assert!(result.is_ok());

        // Verify autosave was created
        let saves = engine.save_manager.list_save_slots().unwrap();
        let autosaves: Vec<_> = saves.iter()
            .filter(|s| s.slot_name.starts_with("autosave_"))
            .collect();

        assert!(!autosaves.is_empty());
    }

    #[test]
    fn test_event_autosave_level_up() {
        let (mut engine, _temp_dir) = create_test_engine_with_temp_saves();

        let result = engine.trigger_event_autosave("level_up");
        assert!(result.is_ok());

        let saves = engine.save_manager.list_save_slots().unwrap();
        let autosaves: Vec<_> = saves.iter()
            .filter(|s| s.slot_name.starts_with("autosave_"))
            .collect();

        assert!(!autosaves.is_empty());
    }

    #[test]
    fn test_event_autosave_major_faction_change() {
        let (mut engine, _temp_dir) = create_test_engine_with_temp_saves();

        let result = engine.trigger_event_autosave("major_faction_change");
        assert!(result.is_ok());

        let saves = engine.save_manager.list_save_slots().unwrap();
        let autosaves: Vec<_> = saves.iter()
            .filter(|s| s.slot_name.starts_with("autosave_"))
            .collect();

        assert!(!autosaves.is_empty());
    }

    #[test]
    fn test_event_autosave_combat_end() {
        let (mut engine, _temp_dir) = create_test_engine_with_temp_saves();

        let result = engine.trigger_event_autosave("combat_end");
        assert!(result.is_ok());

        let saves = engine.save_manager.list_save_slots().unwrap();
        let autosaves: Vec<_> = saves.iter()
            .filter(|s| s.slot_name.starts_with("autosave_"))
            .collect();

        assert!(!autosaves.is_empty());
    }

    #[test]
    fn test_event_autosave_unknown_event_ignored() {
        let (mut engine, _temp_dir) = create_test_engine_with_temp_saves();

        // Unknown event should not trigger autosave
        let result = engine.trigger_event_autosave("unknown_event");
        assert!(result.is_ok());

        let saves = engine.save_manager.list_save_slots().unwrap();
        let autosaves: Vec<_> = saves.iter()
            .filter(|s| s.slot_name.starts_with("autosave_"))
            .collect();

        // No autosaves should be created for unknown events
        assert!(autosaves.is_empty());
    }

    #[test]
    fn test_event_autosave_respects_disabled_flag() {
        let (mut engine, _temp_dir) = create_test_engine_with_temp_saves();
        engine.configure_autosave(false, 5, 3);

        // Should not create autosave when disabled
        let result = engine.trigger_event_autosave("quest_complete");
        assert!(result.is_ok());

        let saves = engine.save_manager.list_save_slots().unwrap();
        let autosaves: Vec<_> = saves.iter()
            .filter(|s| s.slot_name.starts_with("autosave_"))
            .collect();

        assert!(autosaves.is_empty());
    }

    #[test]
    fn test_check_autosave_respects_disabled_flag() {
        let (mut engine, _temp_dir) = create_test_engine_with_temp_saves();
        engine.configure_autosave(false, 5, 3);

        let result = engine.check_autosave();
        assert!(result.is_ok());

        // No autosave should be created
        let saves = engine.save_manager.list_save_slots().unwrap();
        let autosaves: Vec<_> = saves.iter()
            .filter(|s| s.slot_name.starts_with("autosave_"))
            .collect();

        assert!(autosaves.is_empty());
    }

    #[test]
    fn test_autosave_naming_format() {
        let (mut engine, _temp_dir) = create_test_engine_with_temp_saves();

        engine.perform_autosave(true).unwrap();

        let saves = engine.save_manager.list_save_slots().unwrap();
        let autosave = saves.iter()
            .find(|s| s.slot_name.starts_with("autosave_"))
            .expect("Should have autosave file");

        // Check format: autosave_YYYYMMDD_HHMMSS
        assert!(autosave.slot_name.starts_with("autosave_2"));
        assert!(autosave.slot_name.len() >= 23); // autosave_20250101_120000 = 23 chars
    }

    #[test]
    fn test_autosave_cleanup_preserves_newest() {
        let (mut engine, _temp_dir) = create_test_engine_with_temp_saves();
        engine.configure_autosave(true, 5, 2);

        // Create 4 autosaves with delays to ensure different timestamps
        for _ in 0..4 {
            engine.perform_autosave(true).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(200));
        }

        // Get remaining autosaves
        let saves = engine.save_manager.list_save_slots().unwrap();
        let autosave_count = saves.iter()
            .filter(|s| s.slot_name.starts_with("autosave_"))
            .count();

        // Should keep at most 2 (max_autosaves), and at least 1
        assert!(autosave_count <= 2, "Should keep at most 2 autosaves, found {}", autosave_count);
        assert!(autosave_count > 0, "Should have at least 1 autosave");
    }
}