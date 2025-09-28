//! Core game engine components
//!
//! This module contains the fundamental systems that drive the game:
//! - Game engine and main loop
//! - Player state and character management
//! - World state and location tracking
//! - Event system for game-wide notifications

pub mod game_engine;
pub mod player;
pub mod world_state;
pub mod events;

pub use game_engine::GameEngine;
pub use player::Player;
pub use world_state::WorldState;
pub use events::{Event, EventBus};