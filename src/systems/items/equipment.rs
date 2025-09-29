//! Equipment system for wearable and wieldable items
//!
//! This module handles:
//! - Equipment slots and restrictions
//! - Stat bonuses and magical effects
//! - Equipment management and conflicts
//! - Integration with player attributes

use super::core::ItemId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::systems::knowledge::LearningMethod;
use crate::GameResult;

/// Equipment that can be worn or wielded
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    /// Equipment slot this item occupies
    pub slot: EquipmentSlot,
    /// Stat bonuses provided
    pub bonuses: Vec<EquipmentBonus>,
    /// Requirements to equip
    pub requirements: EquipmentRequirements,
    /// Special abilities granted
    pub special_abilities: Vec<SpecialAbility>,
}

/// Equipment slots for different body parts/functions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EquipmentSlot {
    /// Head gear (hats, circlets, etc.)
    Head,
    /// Neck accessories (amulets, chains)
    Neck,
    /// Chest armor and robes
    Chest,
    /// Hand wear (gloves, gauntlets)
    Hands,
    /// Finger accessories (rings)
    Ring1,
    Ring2,
    /// Waist items (belts, sashes)
    Waist,
    /// Leg protection
    Legs,
    /// Footwear
    Feet,
    /// Main hand weapon/tool
    MainHand,
    /// Off hand item (shield, secondary tool)
    OffHand,
    /// Back items (cloaks, backpacks)
    Back,
}

/// Equipment bonuses and effects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EquipmentBonus {
    /// Direct attribute boost
    AttributeBoost {
        attribute: String,
        amount: i32,
    },
    /// Learning efficiency boost for specific methods
    LearningEfficiency {
        method: LearningMethod,
        bonus: f32,
    },
    /// Magic spell effectiveness bonus
    MagicBonus {
        spell_type: String,
        bonus: f32,
    },
    /// Crystal degradation protection
    CrystalProtection(f32),
    /// Energy cost reduction
    EnergyCostReduction(f32),
    /// Fatigue resistance
    FatigueResistance(f32),
    /// Theory-specific learning bonus
    TheoryBonus {
        theory_id: String,
        bonus: f32,
    },
    /// Faction reputation boost
    FactionBonus {
        faction_id: String,
        bonus: i32,
    },
}

/// Requirements to equip an item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipmentRequirements {
    /// Minimum attribute requirements
    pub min_attributes: HashMap<String, i32>,
    /// Required theories
    pub required_theories: Vec<String>,
    /// Minimum faction reputation
    pub faction_requirements: HashMap<String, i32>,
    /// Level requirements (for future use)
    pub min_level: Option<i32>,
}

/// Special abilities granted by equipment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialAbility {
    /// Ability name
    pub name: String,
    /// Ability description
    pub description: String,
    /// Activation method
    pub activation: AbilityActivation,
    /// Cooldown in minutes
    pub cooldown: i32,
    /// Effect when activated
    pub effect: AbilityEffect,
}

/// How abilities are activated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AbilityActivation {
    /// Activated manually by player command
    Manual,
    /// Activates automatically when conditions are met
    Automatic { conditions: Vec<String> },
    /// Passive effect, always active
    Passive,
    /// Activates during specific actions
    Triggered { trigger: String },
}

/// Effects of special abilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AbilityEffect {
    /// Temporary stat boost
    StatBoost {
        attribute: String,
        amount: i32,
        duration: i32,
    },
    /// Cast a spell
    CastSpell {
        spell_type: String,
        power: f32,
    },
    /// Heal or restore resources
    Restore {
        resource: String,
        amount: i32,
    },
    /// Grant temporary immunity
    Immunity {
        effect_type: String,
        duration: i32,
    },
    /// Multiple effects
    Multiple(Vec<AbilityEffect>),
}

/// Equipment manager handling all equipped items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipmentManager {
    /// Currently equipped items by slot
    pub equipped_items: HashMap<EquipmentSlot, (ItemId, Equipment)>,
    /// Active ability cooldowns
    pub ability_cooldowns: HashMap<String, i64>,
}

impl EquipmentManager {
    /// Create new equipment manager
    pub fn new() -> Self {
        Self {
            equipped_items: HashMap::new(),
            ability_cooldowns: HashMap::new(),
        }
    }

    /// Equip an item to a slot
    pub fn equip_item(&mut self, item_id: ItemId, equipment: Equipment) -> GameResult<Option<(ItemId, Equipment)>> {
        let slot = equipment.slot;

        // Check if slot is already occupied
        let previous = self.equipped_items.remove(&slot);

        // Equip the new item
        self.equipped_items.insert(slot, (item_id, equipment));

        Ok(previous)
    }

    /// Unequip an item from a slot
    pub fn unequip_item(&mut self, slot: EquipmentSlot) -> GameResult<Option<(ItemId, Equipment)>> {
        Ok(self.equipped_items.remove(&slot))
    }

    /// Get equipped item in a slot
    pub fn get_equipped_item(&self, slot: EquipmentSlot) -> Option<&(ItemId, Equipment)> {
        self.equipped_items.get(&slot)
    }

    /// Get all equipped item IDs
    pub fn get_equipped_items(&self) -> Vec<&ItemId> {
        self.equipped_items.values().map(|(id, _)| id).collect()
    }

    /// Check if a slot is occupied
    pub fn is_slot_occupied(&self, slot: EquipmentSlot) -> bool {
        self.equipped_items.contains_key(&slot)
    }

    /// Get all active bonuses
    pub fn get_active_bonuses(&self) -> Vec<&EquipmentBonus> {
        self.equipped_items
            .values()
            .flat_map(|(_, equipment)| &equipment.bonuses)
            .collect()
    }

    /// Calculate total attribute bonus
    pub fn calculate_attribute_bonus(&self, attribute: &str) -> i32 {
        self.get_active_bonuses()
            .iter()
            .filter_map(|bonus| {
                if let EquipmentBonus::AttributeBoost { attribute: attr, amount } = bonus {
                    if attr == attribute {
                        Some(*amount)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .sum()
    }

    /// Calculate learning efficiency bonus
    pub fn calculate_learning_bonus(&self, method: &LearningMethod) -> f32 {
        self.get_active_bonuses()
            .iter()
            .filter_map(|bonus| {
                if let EquipmentBonus::LearningEfficiency { method: bonus_method, bonus } = bonus {
                    if bonus_method == method {
                        Some(*bonus)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .sum()
    }

    /// Calculate magic bonus for spell type
    pub fn calculate_magic_bonus(&self, spell_type: &str) -> f32 {
        self.get_active_bonuses()
            .iter()
            .filter_map(|bonus| {
                if let EquipmentBonus::MagicBonus { spell_type: bonus_spell, bonus } = bonus {
                    if bonus_spell == spell_type || bonus_spell == "all" {
                        Some(*bonus)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .sum()
    }

    /// Calculate crystal protection
    pub fn calculate_crystal_protection(&self) -> f32 {
        self.get_active_bonuses()
            .iter()
            .filter_map(|bonus| {
                if let EquipmentBonus::CrystalProtection(protection) = bonus {
                    Some(*protection)
                } else {
                    None
                }
            })
            .sum()
    }

    /// Calculate energy cost reduction
    pub fn calculate_energy_reduction(&self) -> f32 {
        self.get_active_bonuses()
            .iter()
            .filter_map(|bonus| {
                if let EquipmentBonus::EnergyCostReduction(reduction) = bonus {
                    Some(*reduction)
                } else {
                    None
                }
            })
            .sum()
    }

    /// Get all available special abilities
    pub fn get_available_abilities(&self) -> Vec<&SpecialAbility> {
        self.equipped_items
            .values()
            .flat_map(|(_, equipment)| &equipment.special_abilities)
            .collect()
    }

    /// Activate a special ability
    pub fn activate_ability(&mut self, ability_name: &str) -> GameResult<String> {
        // Check if ability exists and is not on cooldown
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        if let Some(&cooldown_end) = self.ability_cooldowns.get(ability_name) {
            if current_time < cooldown_end {
                let remaining = cooldown_end - current_time;
                return Err(crate::GameError::InvalidInput(
                    format!("Ability {} is on cooldown for {} more seconds", ability_name, remaining)
                ).into());
            }
        }

        // Find the ability and extract necessary data
        let abilities = self.get_available_abilities();
        let ability_data = abilities
            .iter()
            .find(|a| a.name == ability_name)
            .map(|a| (a.activation.clone(), a.cooldown, a.description.clone()))
            .ok_or_else(|| crate::GameError::InvalidInput(
                format!("Ability {} not found", ability_name)
            ))?;

        // Check activation type
        match &ability_data.0 {
            AbilityActivation::Manual => {
                // Set cooldown
                let cooldown_end = current_time + (ability_data.1 as i64 * 60);
                self.ability_cooldowns.insert(ability_name.to_string(), cooldown_end);

                Ok(format!("Activated ability: {}", ability_data.2))
            }
            _ => Err(crate::GameError::InvalidInput(
                format!("Ability {} cannot be manually activated", ability_name)
            ).into())
        }
    }

    /// Check equipment requirements against player stats
    pub fn check_requirements(
        &self,
        equipment: &Equipment,
        player_attributes: &HashMap<String, i32>,
        player_theories: &[String],
        player_factions: &HashMap<String, i32>,
    ) -> Vec<String> {
        let mut unmet_requirements = Vec::new();

        // Check attribute requirements
        for (attr, required) in &equipment.requirements.min_attributes {
            if let Some(&current) = player_attributes.get(attr) {
                if current < *required {
                    unmet_requirements.push(format!("Need {} {} (have {})", required, attr, current));
                }
            } else {
                unmet_requirements.push(format!("Need {} {}", required, attr));
            }
        }

        // Check theory requirements
        for theory in &equipment.requirements.required_theories {
            if !player_theories.contains(theory) {
                unmet_requirements.push(format!("Must know theory: {}", theory));
            }
        }

        // Check faction requirements
        for (faction, required) in &equipment.requirements.faction_requirements {
            if let Some(&current) = player_factions.get(faction) {
                if current < *required {
                    unmet_requirements.push(format!("Need {} reputation with {} (have {})", required, faction, current));
                }
            } else {
                unmet_requirements.push(format!("Need {} reputation with {}", required, faction));
            }
        }

        unmet_requirements
    }

    /// Get equipment summary for display
    pub fn get_summary(&self) -> String {
        if self.equipped_items.is_empty() {
            return "No equipment equipped".to_string();
        }

        let mut summary = String::from("Equipped Items:\n");

        for (slot, (item_id, equipment)) in &self.equipped_items {
            summary.push_str(&format!("  {:?}: {} ({})\n", slot, item_id, equipment.bonuses.len()));
        }

        summary
    }
}

impl Equipment {
    /// Create basic equipment
    pub fn new_basic(slot: EquipmentSlot) -> Self {
        Self {
            slot,
            bonuses: Vec::new(),
            requirements: EquipmentRequirements::default(),
            special_abilities: Vec::new(),
        }
    }

    /// Add a bonus to equipment
    pub fn add_bonus(mut self, bonus: EquipmentBonus) -> Self {
        self.bonuses.push(bonus);
        self
    }

    /// Add a requirement
    pub fn add_attribute_requirement(mut self, attribute: String, minimum: i32) -> Self {
        self.requirements.min_attributes.insert(attribute, minimum);
        self
    }

    /// Add theory requirement
    pub fn add_theory_requirement(mut self, theory: String) -> Self {
        self.requirements.required_theories.push(theory);
        self
    }

    /// Add special ability
    pub fn add_ability(mut self, ability: SpecialAbility) -> Self {
        self.special_abilities.push(ability);
        self
    }
}

impl EquipmentRequirements {
    /// Create default requirements (no restrictions)
    pub fn default() -> Self {
        Self {
            min_attributes: HashMap::new(),
            required_theories: Vec::new(),
            faction_requirements: HashMap::new(),
            min_level: None,
        }
    }

    /// Check if requirements are met
    pub fn are_met(
        &self,
        player_attributes: &HashMap<String, i32>,
        player_theories: &[String],
        player_factions: &HashMap<String, i32>,
    ) -> bool {
        // Check attributes
        for (attr, required) in &self.min_attributes {
            if let Some(&current) = player_attributes.get(attr) {
                if current < *required {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Check theories
        for theory in &self.required_theories {
            if !player_theories.contains(theory) {
                return false;
            }
        }

        // Check factions
        for (faction, required) in &self.faction_requirements {
            if let Some(&current) = player_factions.get(faction) {
                if current < *required {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::knowledge::LearningMethod;

    #[test]
    fn test_equipment_manager() {
        let mut manager = EquipmentManager::new();

        let equipment = Equipment::new_basic(EquipmentSlot::Head)
            .add_bonus(EquipmentBonus::AttributeBoost {
                attribute: "mental_acuity".to_string(),
                amount: 5,
            });

        // Equip item
        let result = manager.equip_item("test_item".to_string(), equipment);
        assert!(result.is_ok());
        assert!(manager.is_slot_occupied(EquipmentSlot::Head));

        // Check bonus calculation
        let bonus = manager.calculate_attribute_bonus("mental_acuity");
        assert_eq!(bonus, 5);

        // Unequip item
        let unequipped = manager.unequip_item(EquipmentSlot::Head).unwrap();
        assert!(unequipped.is_some());
        assert!(!manager.is_slot_occupied(EquipmentSlot::Head));
    }

    #[test]
    fn test_equipment_bonuses() {
        let mut manager = EquipmentManager::new();

        // Add multiple items with bonuses
        let helmet = Equipment::new_basic(EquipmentSlot::Head)
            .add_bonus(EquipmentBonus::AttributeBoost {
                attribute: "mental_acuity".to_string(),
                amount: 3,
            });

        let ring = Equipment::new_basic(EquipmentSlot::Ring1)
            .add_bonus(EquipmentBonus::LearningEfficiency {
                method: LearningMethod::Study,
                bonus: 0.2,
            });

        manager.equip_item("helmet".to_string(), helmet).unwrap();
        manager.equip_item("ring".to_string(), ring).unwrap();

        // Test cumulative bonuses
        assert_eq!(manager.calculate_attribute_bonus("mental_acuity"), 3);
        assert_eq!(manager.calculate_learning_bonus(&LearningMethod::Study), 0.2);
        assert_eq!(manager.calculate_learning_bonus(&LearningMethod::Experimentation), 0.0);
    }

    #[test]
    fn test_equipment_requirements() {
        let mut requirements = EquipmentRequirements::default();
        requirements.min_attributes.insert("mental_acuity".to_string(), 50);
        requirements.required_theories.push("harmonic_fundamentals".to_string());

        let mut player_attributes = HashMap::new();
        player_attributes.insert("mental_acuity".to_string(), 45);
        let player_theories = vec![];
        let player_factions = HashMap::new();

        // Should not meet requirements
        assert!(!requirements.are_met(&player_attributes, &player_theories, &player_factions));

        // Meet attribute requirement
        player_attributes.insert("mental_acuity".to_string(), 55);
        assert!(!requirements.are_met(&player_attributes, &player_theories, &player_factions));

        // Meet theory requirement
        let player_theories = vec!["harmonic_fundamentals".to_string()];
        assert!(requirements.are_met(&player_attributes, &player_theories, &player_factions));
    }

    #[test]
    fn test_special_abilities() {
        let ability = SpecialAbility {
            name: "Test Ability".to_string(),
            description: "A test ability".to_string(),
            activation: AbilityActivation::Manual,
            cooldown: 5,
            effect: AbilityEffect::StatBoost {
                attribute: "mental_acuity".to_string(),
                amount: 10,
                duration: 60,
            },
        };

        let equipment = Equipment::new_basic(EquipmentSlot::MainHand)
            .add_ability(ability);

        let mut manager = EquipmentManager::new();
        manager.equip_item("magic_wand".to_string(), equipment).unwrap();

        // Test ability activation
        let result = manager.activate_ability("Test Ability");
        assert!(result.is_ok());

        // Test cooldown
        let result2 = manager.activate_ability("Test Ability");
        assert!(result2.is_err());
    }
}