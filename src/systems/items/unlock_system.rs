use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::core::Player;
use crate::systems::items::core::ItemId;

/// Requirements for unlocking an item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnlockRequirement {
    /// Requires specific theory mastery level (theory_id, minimum_percentage)
    TheoryMastery { theory_id: String, min_mastery: f32 },

    /// Requires faction reputation level (faction_id, minimum_reputation)
    FactionReputation { faction_id: String, min_reputation: i32 },

    /// Requires multiple theories at specific levels
    MultipleTheories { requirements: Vec<(String, f32)> },

    /// Requires completing specific achievements
    Achievement { achievement_id: String },

    /// Combines multiple requirements (all must be met)
    Combined { requirements: Vec<UnlockRequirement> },

    /// Alternative requirements (any one must be met)
    Alternative { requirements: Vec<UnlockRequirement> },

    /// Always unlocked (no requirements)
    None,
}

/// Categories of unlock triggers for better organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnlockCategory {
    /// Basic items available from the start
    Starter,

    /// Items unlocked through theory mastery
    TheoryProgression,

    /// Items unlocked through faction reputation
    FactionLoyalty,

    /// Items unlocked through achievements
    Achievement,

    /// Special items with complex requirements
    Special,
}

/// Tracks when and how an item was unlocked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlockEvent {
    pub item_id: ItemId,
    pub unlocked_at: chrono::DateTime<chrono::Utc>,
    pub unlock_source: String,
    pub requirements_met: Vec<String>,
}

/// Manages the progressive unlock system for items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemUnlockSystem {
    /// Map of item ID to unlock requirements
    unlock_requirements: HashMap<ItemId, UnlockRequirement>,

    /// Map of item ID to unlock category
    unlock_categories: HashMap<ItemId, UnlockCategory>,

    /// Track unlocked items for this player session
    unlocked_items: std::collections::HashSet<ItemId>,

    /// Recently unlocked items (for notifications)
    recent_unlocks: Vec<UnlockEvent>,
}

impl ItemUnlockSystem {
    /// Create a new unlock system
    pub fn new() -> Self {
        Self {
            unlock_requirements: HashMap::new(),
            unlock_categories: HashMap::new(),
            unlocked_items: std::collections::HashSet::new(),
            recent_unlocks: Vec::new(),
        }
    }

    /// Register an item with unlock requirements
    pub fn register_item_unlock(&mut self, item_id: ItemId, requirement: UnlockRequirement, category: UnlockCategory) {
        self.unlock_requirements.insert(item_id.clone(), requirement);
        self.unlock_categories.insert(item_id, category);
    }

    /// Check if a player meets the requirements for a specific item
    pub fn check_unlock_requirements(&self, player: &Player, item_id: &ItemId) -> bool {
        match self.unlock_requirements.get(item_id) {
            Some(requirement) => self.evaluate_requirement(player, requirement),
            None => true, // No requirements means always unlocked
        }
    }

    /// Evaluate a specific unlock requirement
    fn evaluate_requirement(&self, player: &Player, requirement: &UnlockRequirement) -> bool {
        match requirement {
            UnlockRequirement::None => true,

            UnlockRequirement::TheoryMastery { theory_id, min_mastery } => {
                self.check_theory_mastery(player, theory_id, *min_mastery)
            }

            UnlockRequirement::FactionReputation { faction_id, min_reputation } => {
                self.check_faction_reputation(player, faction_id, *min_reputation)
            }

            UnlockRequirement::MultipleTheories { requirements } => {
                requirements.iter().all(|(theory_id, min_mastery)| {
                    self.check_theory_mastery(player, theory_id, *min_mastery)
                })
            }

            UnlockRequirement::Achievement { achievement_id: _ } => {
                // TODO: Implement achievement system integration
                true // For now, assume all achievements are met
            }

            UnlockRequirement::Combined { requirements } => {
                requirements.iter().all(|req| self.evaluate_requirement(player, req))
            }

            UnlockRequirement::Alternative { requirements } => {
                requirements.iter().any(|req| self.evaluate_requirement(player, req))
            }
        }
    }

    /// Check if player has required theory mastery
    fn check_theory_mastery(&self, player: &Player, theory_id: &str, min_mastery: f32) -> bool {
        // Get theory mastery percentage from player
        if let Some(theory_progress) = player.knowledge.theory_progress.get(theory_id) {
            theory_progress.understanding_level >= min_mastery
        } else {
            false
        }
    }

    /// Check if player has required faction reputation
    fn check_faction_reputation(&self, player: &Player, faction_id: &str, min_reputation: i32) -> bool {
        // Convert faction_id string to FactionId enum and get reputation
        if let Ok(faction) = self.parse_faction_id(faction_id) {
            player.faction_reputation(faction) >= min_reputation
        } else {
            false
        }
    }

    /// Helper to parse faction ID strings to FactionId enum
    fn parse_faction_id(&self, faction_id: &str) -> Result<crate::systems::factions::FactionId, ()> {
        use crate::systems::factions::FactionId;
        match faction_id {
            "magisters_council" => Ok(FactionId::MagistersCouncil),
            "order_of_natural_harmony" => Ok(FactionId::OrderOfHarmony),
            "industrial_consortium" => Ok(FactionId::IndustrialConsortium),
            "underground_network" => Ok(FactionId::UndergroundNetwork),
            "neutral_scholars" => Ok(FactionId::NeutralScholars),
            _ => Err(()),
        }
    }

    /// Get all items that the player has unlocked
    pub fn get_unlocked_items(&self, player: &Player) -> Vec<ItemId> {
        self.unlock_requirements
            .keys()
            .filter(|item_id| self.check_unlock_requirements(player, item_id))
            .cloned()
            .collect()
    }

    /// Check for newly unlocked items and return notifications
    pub fn check_for_new_unlocks(&mut self, player: &Player) -> Vec<UnlockEvent> {
        let mut new_unlocks = Vec::new();

        for (item_id, requirement) in &self.unlock_requirements {
            if !self.unlocked_items.contains(item_id) && self.evaluate_requirement(player, requirement) {
                // This item was just unlocked
                let unlock_event = UnlockEvent {
                    item_id: item_id.clone(),
                    unlocked_at: chrono::Utc::now(),
                    unlock_source: self.get_unlock_source_description(requirement),
                    requirements_met: self.get_requirements_description(requirement),
                };

                self.unlocked_items.insert(item_id.clone());
                self.recent_unlocks.push(unlock_event.clone());
                new_unlocks.push(unlock_event);
            }
        }

        new_unlocks
    }

    /// Get human-readable description of unlock source
    fn get_unlock_source_description(&self, requirement: &UnlockRequirement) -> String {
        match requirement {
            UnlockRequirement::None => "Always Available".to_string(),
            UnlockRequirement::TheoryMastery { theory_id, .. } => format!("Theory Mastery: {}", theory_id),
            UnlockRequirement::FactionReputation { faction_id, .. } => format!("Faction Reputation: {}", faction_id),
            UnlockRequirement::MultipleTheories { .. } => "Multiple Theory Mastery".to_string(),
            UnlockRequirement::Achievement { achievement_id } => format!("Achievement: {}", achievement_id),
            UnlockRequirement::Combined { .. } => "Combined Requirements".to_string(),
            UnlockRequirement::Alternative { .. } => "Alternative Requirements".to_string(),
        }
    }

    /// Get detailed description of requirements met
    fn get_requirements_description(&self, requirement: &UnlockRequirement) -> Vec<String> {
        match requirement {
            UnlockRequirement::None => vec!["No requirements".to_string()],
            UnlockRequirement::TheoryMastery { theory_id, min_mastery } => {
                vec![format!("{} theory mastery ≥ {:.0}%", theory_id, min_mastery * 100.0)]
            }
            UnlockRequirement::FactionReputation { faction_id, min_reputation } => {
                vec![format!("{} reputation ≥ {}", faction_id, min_reputation)]
            }
            UnlockRequirement::MultipleTheories { requirements } => {
                requirements.iter().map(|(theory_id, min_mastery)| {
                    format!("{} mastery ≥ {:.0}%", theory_id, min_mastery * 100.0)
                }).collect()
            }
            UnlockRequirement::Achievement { achievement_id } => {
                vec![format!("Achievement: {}", achievement_id)]
            }
            UnlockRequirement::Combined { requirements } => {
                requirements.iter().flat_map(|req| self.get_requirements_description(req)).collect()
            }
            UnlockRequirement::Alternative { requirements } => {
                vec![format!("Any of: {}", requirements.len())]
            }
        }
    }

    /// Get recent unlocks for notification purposes
    pub fn get_recent_unlocks(&self) -> &[UnlockEvent] {
        &self.recent_unlocks
    }

    /// Clear recent unlocks (after showing notifications)
    pub fn clear_recent_unlocks(&mut self) {
        self.recent_unlocks.clear();
    }

    /// Get unlock progress summary for a specific category
    pub fn get_category_progress(&self, player: &Player, category: UnlockCategory) -> (usize, usize) {
        let total_items: Vec<_> = self.unlock_categories
            .iter()
            .filter(|(_, cat)| std::mem::discriminant(*cat) == std::mem::discriminant(&category))
            .map(|(item_id, _)| item_id)
            .collect();

        let unlocked_count = total_items
            .iter()
            .filter(|item_id| self.check_unlock_requirements(player, item_id))
            .count();

        (unlocked_count, total_items.len())
    }
}

impl Default for ItemUnlockSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper functions for creating common unlock patterns
impl ItemUnlockSystem {
    /// Create theory mastery unlock for single theory
    pub fn theory_unlock(theory_id: &str, min_mastery: f32) -> UnlockRequirement {
        UnlockRequirement::TheoryMastery {
            theory_id: theory_id.to_string(),
            min_mastery,
        }
    }

    /// Create faction reputation unlock
    pub fn faction_unlock(faction_id: &str, min_reputation: i32) -> UnlockRequirement {
        UnlockRequirement::FactionReputation {
            faction_id: faction_id.to_string(),
            min_reputation,
        }
    }

    /// Create combined theory + faction unlock
    pub fn theory_and_faction_unlock(theory_id: &str, min_mastery: f32, faction_id: &str, min_reputation: i32) -> UnlockRequirement {
        UnlockRequirement::Combined {
            requirements: vec![
                Self::theory_unlock(theory_id, min_mastery),
                Self::faction_unlock(faction_id, min_reputation),
            ],
        }
    }

    /// Create multi-theory mastery unlock
    pub fn multi_theory_unlock(theory_requirements: Vec<(&str, f32)>) -> UnlockRequirement {
        UnlockRequirement::MultipleTheories {
            requirements: theory_requirements
                .into_iter()
                .map(|(id, mastery)| (id.to_string(), mastery))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unlock_system_creation() {
        let unlock_system = ItemUnlockSystem::new();
        assert!(unlock_system.unlock_requirements.is_empty());
        assert!(unlock_system.unlocked_items.is_empty());
    }

    #[test]
    fn test_unlock_helper_functions() {
        // Test the unlock requirement creation helpers
        let theory_req = ItemUnlockSystem::theory_unlock("crystal_structures", 0.5);
        let faction_req = ItemUnlockSystem::faction_unlock("magisters_council", 25);
        let combined_req = ItemUnlockSystem::theory_and_faction_unlock("crystal_structures", 0.75, "magisters_council", 50);

        // Should create the correct requirement types
        assert!(matches!(theory_req, UnlockRequirement::TheoryMastery { .. }));
        assert!(matches!(faction_req, UnlockRequirement::FactionReputation { .. }));
        assert!(matches!(combined_req, UnlockRequirement::Combined { .. }));
    }

    #[test]
    fn test_unlock_registration() {
        let mut unlock_system = ItemUnlockSystem::new();
        let item_id = "test_item".to_string();

        unlock_system.register_item_unlock(
            item_id.clone(),
            ItemUnlockSystem::theory_unlock("crystal_structures", 0.5),
            UnlockCategory::TheoryProgression,
        );

        // Should have registered the item
        assert!(unlock_system.unlock_requirements.contains_key(&item_id));
        assert!(unlock_system.unlock_categories.contains_key(&item_id));
    }
}