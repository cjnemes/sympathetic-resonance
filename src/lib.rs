//! # Sympathetic Resonance
//!
//! A text adventure game featuring science-based magic in a low fantasy world.
//!
//! ## Core Modules
//!
//! - [`core`] - Core game engine and fundamental data structures
//! - [`systems`] - Game systems (magic, factions, knowledge progression)
//! - [`input`] - Command parsing and natural language processing
//! - [`content`] - Content loading and narrative management
//! - [`persistence`] - Save/load system and database operations
//! - [`ui`] - User interface and terminal display systems

pub mod core;
pub mod systems;
pub mod input;
pub mod content;
pub mod persistence;
pub mod ui;

#[cfg(test)]
pub mod integration_tests;

#[cfg(test)]
pub mod performance_tests;

// Re-export commonly used types
pub use core::{
    game_engine::GameEngine,
    player::Player,
    world_state::WorldState,
};

pub use systems::{
    magic::MagicSystem,
    factions::FactionSystem,
    knowledge::KnowledgeSystem,
};

pub use persistence::database::DatabaseManager;

/// Game version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const GAME_NAME: &str = "Sympathetic Resonance";

/// Common result type for game operations
pub type GameResult<T> = anyhow::Result<T>;

/// Common error types used throughout the game
#[derive(thiserror::Error, Debug)]
pub enum GameError {
    #[error("Invalid command: {0}")]
    InvalidCommand(String),

    #[error("Insufficient resources: {0}")]
    InsufficientResources(String),

    #[error("Magic failed: {0}")]
    MagicFailure(String),

    #[error("Content not found: {0}")]
    ContentNotFound(String),

    #[error("Save/Load error: {0}")]
    SaveLoadError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("IO error: {0}")]
    IoError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_available() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_game_name() {
        assert_eq!(GAME_NAME, "Sympathetic Resonance");
    }
}