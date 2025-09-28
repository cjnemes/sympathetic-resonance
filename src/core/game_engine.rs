//! Main game engine coordinating all systems

use crate::core::{Player, WorldState, EventBus};
use crate::systems::{MagicSystem, FactionSystem};
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
    /// Command parser
    command_parser: CommandParser,
    /// Database manager
    database: DatabaseManager,
    /// Save manager
    save_manager: SaveManager,
    /// Event bus
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

        Ok(Self {
            player,
            world,
            magic_system: MagicSystem::new(),
            faction_system: FactionSystem::new(),
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
                execute_command(command, &mut self.player, &mut self.world, &self.database)
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
        let (player, world) = self.save_manager.load_game(save_path)?;
        self.player = player;
        self.world = world;
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