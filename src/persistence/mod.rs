//! Persistence layer for save/load and database operations
//!
//! This module provides:
//! - Database schema and content management
//! - Save/load system for game state
//! - Data serialization and migration

pub mod database;
pub mod save_system;
pub mod serialization;

pub use database::DatabaseManager;
pub use save_system::SaveManager;
pub use serialization::{GameStateData, serialize_game_state, deserialize_game_state};