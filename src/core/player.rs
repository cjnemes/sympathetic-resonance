//! Player character state and management
//!
//! This module handles all aspects of the player character including:
//! - Core attributes (Mental Acuity, Resonance Sensitivity)
//! - Mental energy and fatigue tracking
//! - Inventory and crystal management
//! - Theory knowledge and progression

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::systems::factions::FactionId;
use crate::GameResult;

/// Core player attributes that define magical capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAttributes {
    /// Mental processing power affecting energy pool and learning speed (0-100)
    pub mental_acuity: i32,
    /// Sensitivity to magical resonance affecting success rates (0-100)
    pub resonance_sensitivity: i32,
    /// Experience points in each attribute for progression tracking
    pub experience: AttributeExperience,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeExperience {
    pub mental_acuity_xp: i32,
    pub resonance_sensitivity_xp: i32,
}

/// Mental energy and fatigue state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentalState {
    /// Current mental energy available for magic (calculated from Mental Acuity × 1.5)
    pub current_energy: i32,
    /// Maximum mental energy capacity
    pub max_energy: i32,
    /// Accumulated fatigue from magical use (0-100)
    pub fatigue: i32,
}

/// Crystal in player's possession
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crystal {
    /// Type of crystal determining resonance frequency
    pub crystal_type: CrystalType,
    /// Structural integrity percentage (0-100)
    pub integrity: f32,
    /// Crystal purity affecting efficiency (0.0-1.0)
    pub purity: f32,
    /// Crystal size affecting power output
    pub size: CrystalSize,
    /// Current resonance frequency (1-10)
    pub frequency: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrystalType {
    Quartz,     // Frequency 4, good for basic magic
    Amethyst,   // Frequency 7, excellent for healing
    Obsidian,   // Frequency 2, strong but volatile
    Garnet,     // Frequency 6, stable and reliable
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrystalSize {
    Tiny,    // 0.5x power multiplier
    Small,   // 0.8x power multiplier
    Medium,  // 1.0x power multiplier
    Large,   // 1.3x power multiplier
}

/// Theory knowledge and progression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeState {
    /// Mastered theories with understanding level (0.0-1.0)
    pub theories: HashMap<String, f32>,
    /// Current research progress
    pub active_research: Option<String>,
    /// Research progress percentage (0.0-1.0)
    pub research_progress: f32,
}

/// Player's inventory and equipment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    /// Crystals in possession
    pub crystals: Vec<Crystal>,
    /// Currently equipped crystal for magic use
    pub active_crystal: Option<usize>,
    /// Other items (notes, books, artifacts)
    pub items: Vec<Item>,
    /// Currency in silver pieces
    pub silver: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemType {
    Book(String),      // Theory name it teaches
    Note(String),      // Information content
    Artifact(String),  // Special properties
    Mundane,          // Regular items
}

/// Complete player character state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    /// Player's name
    pub name: String,
    /// Core attributes and capabilities
    pub attributes: PlayerAttributes,
    /// Mental energy and fatigue tracking
    pub mental_state: MentalState,
    /// Faction reputation standings
    pub faction_standings: HashMap<FactionId, i32>,
    /// Theory knowledge and research
    pub knowledge: KnowledgeState,
    /// Inventory and equipment
    pub inventory: Inventory,
    /// Current location in the world
    pub current_location: String,
    /// Total playtime in minutes
    pub playtime_minutes: i32,
}

impl Player {
    /// Create a new player with starting attributes
    pub fn new(name: String) -> Self {
        let mental_acuity = 25; // Starting value
        let max_energy = (mental_acuity as f32 * 1.5) as i32;

        Self {
            name,
            attributes: PlayerAttributes {
                mental_acuity,
                resonance_sensitivity: 20,
                experience: AttributeExperience {
                    mental_acuity_xp: 0,
                    resonance_sensitivity_xp: 0,
                },
            },
            mental_state: MentalState {
                current_energy: max_energy,
                max_energy,
                fatigue: 0,
            },
            faction_standings: HashMap::new(),
            knowledge: KnowledgeState {
                theories: HashMap::new(),
                active_research: None,
                research_progress: 0.0,
            },
            inventory: Inventory {
                crystals: vec![
                    // Starting crystal
                    Crystal {
                        crystal_type: CrystalType::Quartz,
                        integrity: 95.0,
                        purity: 0.6,
                        size: CrystalSize::Small,
                        frequency: 4,
                    }
                ],
                active_crystal: Some(0),
                items: Vec::new(),
                silver: 50,
            },
            current_location: "tutorial_chamber".to_string(),
            playtime_minutes: 0,
        }
    }

    /// Get effective mental energy accounting for fatigue
    pub fn effective_mental_energy(&self) -> i32 {
        let fatigue_penalty = (self.mental_state.fatigue as f32 * 0.5) as i32;
        (self.mental_state.current_energy - fatigue_penalty).max(0)
    }

    /// Use mental energy for magical actions
    pub fn use_mental_energy(&mut self, amount: i32, fatigue_cost: i32) -> GameResult<()> {
        if self.effective_mental_energy() < amount {
            return Err(crate::GameError::InsufficientResources(
                format!("Need {} mental energy, have {} effective energy",
                        amount, self.effective_mental_energy())
            ).into());
        }

        self.mental_state.current_energy =
            (self.mental_state.current_energy - amount).max(0);
        self.mental_state.fatigue =
            (self.mental_state.fatigue + fatigue_cost).min(100);

        Ok(())
    }

    /// Recover mental energy through rest
    pub fn recover_energy(&mut self, amount: i32, fatigue_reduction: i32) {
        self.mental_state.current_energy =
            (self.mental_state.current_energy + amount).min(self.mental_state.max_energy);
        self.mental_state.fatigue =
            (self.mental_state.fatigue - fatigue_reduction).max(0);
    }

    /// Get currently equipped crystal
    pub fn active_crystal(&self) -> Option<&Crystal> {
        self.inventory.active_crystal
            .and_then(|idx| self.inventory.crystals.get(idx))
    }

    /// Get currently equipped crystal mutably
    pub fn active_crystal_mut(&mut self) -> Option<&mut Crystal> {
        if let Some(idx) = self.inventory.active_crystal {
            self.inventory.crystals.get_mut(idx)
        } else {
            None
        }
    }

    /// Check if player knows a specific theory
    pub fn knows_theory(&self, theory: &str) -> bool {
        self.knowledge.theories.contains_key(theory)
    }

    /// Get understanding level of a theory (0.0-1.0)
    pub fn theory_understanding(&self, theory: &str) -> f32 {
        self.knowledge.theories.get(theory).copied().unwrap_or(0.0)
    }

    /// Add experience to an attribute
    pub fn add_experience(&mut self, attribute: AttributeType, amount: i32) {
        match attribute {
            AttributeType::MentalAcuity => {
                self.attributes.experience.mental_acuity_xp += amount;
                // Check for level up (every 100 XP)
                if self.attributes.experience.mental_acuity_xp >=
                   (self.attributes.mental_acuity + 1) * 100 {
                    self.attributes.mental_acuity += 1;
                    self.recalculate_max_energy();
                }
            }
            AttributeType::ResonanceSensitivity => {
                self.attributes.experience.resonance_sensitivity_xp += amount;
                if self.attributes.experience.resonance_sensitivity_xp >=
                   (self.attributes.resonance_sensitivity + 1) * 100 {
                    self.attributes.resonance_sensitivity += 1;
                }
            }
        }
    }

    /// Recalculate maximum mental energy when Mental Acuity changes
    fn recalculate_max_energy(&mut self) {
        let new_max = (self.attributes.mental_acuity as f32 * 1.5) as i32;
        let energy_increase = new_max - self.mental_state.max_energy;
        self.mental_state.max_energy = new_max;
        self.mental_state.current_energy += energy_increase;
    }

    /// Get faction reputation (-100 to +100)
    pub fn faction_reputation(&self, faction: FactionId) -> i32 {
        self.faction_standings.get(&faction).copied().unwrap_or(0)
    }

    /// Modify faction reputation with bounds checking
    pub fn modify_faction_reputation(&mut self, faction: FactionId, change: i32) {
        let current = self.faction_reputation(faction);
        let new_value = (current + change).max(-100).min(100);
        self.faction_standings.insert(faction, new_value);
    }
}

#[derive(Debug, Clone)]
pub enum AttributeType {
    MentalAcuity,
    ResonanceSensitivity,
}

impl Crystal {
    /// Create a new crystal with specified properties
    pub fn new(crystal_type: CrystalType, integrity: f32, purity: f32, size: CrystalSize) -> Self {
        let frequency = match crystal_type {
            CrystalType::Quartz => 4,
            CrystalType::Amethyst => 7,
            CrystalType::Obsidian => 2,
            CrystalType::Garnet => 6,
        };

        Self {
            crystal_type,
            integrity: integrity.max(0.0).min(100.0),
            purity: purity.max(0.0).min(1.0),
            size,
            frequency,
        }
    }

    /// Calculate power multiplier based on size
    pub fn power_multiplier(&self) -> f32 {
        match self.size {
            CrystalSize::Tiny => 0.5,
            CrystalSize::Small => 0.8,
            CrystalSize::Medium => 1.0,
            CrystalSize::Large => 1.3,
        }
    }

    /// Calculate efficiency based on integrity and purity
    pub fn efficiency(&self) -> f32 {
        let integrity_factor = self.integrity / 100.0;
        let purity_factor = self.purity;
        (integrity_factor * purity_factor).max(0.1) // Minimum 10% efficiency
    }

    /// Degrade crystal from use
    pub fn degrade(&mut self, base_degradation: f32) {
        let purity_protection = self.purity * 0.5; // High purity reduces degradation
        let actual_degradation = base_degradation * (1.0 - purity_protection);
        self.integrity = (self.integrity - actual_degradation).max(0.0);
    }

    /// Check if crystal is still usable
    pub fn is_usable(&self) -> bool {
        self.integrity > 5.0 // Crystals become unusable below 5% integrity
    }

    /// Get crystal display name
    pub fn display_name(&self) -> String {
        let size_str = match self.size {
            CrystalSize::Tiny => "tiny",
            CrystalSize::Small => "small",
            CrystalSize::Medium => "",
            CrystalSize::Large => "large",
        };

        let type_str = match self.crystal_type {
            CrystalType::Quartz => "quartz",
            CrystalType::Amethyst => "amethyst",
            CrystalType::Obsidian => "obsidian",
            CrystalType::Garnet => "garnet",
        };

        if size_str.is_empty() {
            type_str.to_string()
        } else {
            format!("{} {}", size_str, type_str)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let player = Player::new("Test Player".to_string());
        assert_eq!(player.name, "Test Player");
        assert_eq!(player.attributes.mental_acuity, 25);
        assert_eq!(player.mental_state.max_energy, 37); // 25 * 1.5 = 37.5 -> 37
        assert!(player.inventory.crystals.len() > 0);
    }

    #[test]
    fn test_mental_energy_usage() {
        let mut player = Player::new("Test".to_string());
        let initial_energy = player.mental_state.current_energy;

        player.use_mental_energy(10, 5).unwrap();

        assert_eq!(player.mental_state.current_energy, initial_energy - 10);
        assert_eq!(player.mental_state.fatigue, 5);
    }

    #[test]
    fn test_crystal_degradation() {
        let mut crystal = Crystal::new(CrystalType::Quartz, 100.0, 0.8, CrystalSize::Medium);
        let initial_integrity = crystal.integrity;

        crystal.degrade(2.0);

        assert!(crystal.integrity < initial_integrity);
        assert!(crystal.is_usable());
    }

    #[test]
    fn test_faction_reputation() {
        let mut player = Player::new("Test".to_string());

        player.modify_faction_reputation(FactionId::MagistersCouncil, 25);
        assert_eq!(player.faction_reputation(FactionId::MagistersCouncil), 25);

        player.modify_faction_reputation(FactionId::MagistersCouncil, -150); // Should clamp to -100
        assert_eq!(player.faction_reputation(FactionId::MagistersCouncil), -100);
    }

    #[test]
    fn test_effective_mental_energy() {
        let mut player = Player::new("Test".to_string());
        player.mental_state.current_energy = 50;
        player.mental_state.fatigue = 20;

        // Effective = 50 - (20 * 0.5) = 40
        assert_eq!(player.effective_mental_energy(), 40);
    }
}