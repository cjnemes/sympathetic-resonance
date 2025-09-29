//! Game systems that implement core mechanics
//!
//! This module contains the major gameplay systems:
//! - Magic system with sympathetic resonance calculations
//! - Faction reputation and political relationships
//! - Knowledge progression and theory development
//! - Combat system with magical focus
//! - Comprehensive item system with educational integration

pub mod magic;
pub mod factions;
pub mod knowledge;
pub mod combat;
pub mod dialogue;
pub mod quests;
pub mod quest_examples;
pub mod items;


pub use magic::MagicSystem;
pub use factions::FactionSystem;
pub use knowledge::KnowledgeSystem;
pub use combat::CombatSystem;
pub use dialogue::DialogueSystem;
pub use quests::QuestSystem;
pub use items::ItemSystem;