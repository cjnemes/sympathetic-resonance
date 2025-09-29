//! Comprehensive Item System Implementation
//!
//! This module provides a complete item management system with:
//! - Advanced item types and properties
//! - Educational integration with learning bonuses
//! - Equipment system with stat modifications
//! - Inventory management with weight and space limits
//! - Item interactions and combinations
//! - Integration with existing magic and knowledge systems

pub mod core;
pub mod equipment;
pub mod educational;
pub mod inventory;
pub mod interactions;

pub use core::{Item, ItemId, ItemType, ItemRarity, ItemProperties, ItemEffect};
pub use equipment::{Equipment, EquipmentSlot, EquipmentManager, EquipmentBonus};
pub use educational::{EducationalItem, LearningBonus, ResearchTool, CollaborativeTool};
pub use inventory::{InventoryManager, InventoryConstraints, InventoryError};
pub use interactions::{ItemInteraction, InteractionResult, CombinationRule};

use crate::core::Player;
use crate::systems::knowledge::LearningMethod;
use crate::GameResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete item system managing all item-related functionality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemSystem {
    /// Core inventory management
    pub inventory_manager: InventoryManager,
    /// Equipment system for wearable items
    pub equipment_manager: EquipmentManager,
    /// Item interaction and combination rules
    pub interaction_rules: HashMap<String, CombinationRule>,
    /// Educational item database
    pub educational_items: HashMap<ItemId, EducationalItem>,
}

impl ItemSystem {
    /// Create a new item system
    pub fn new() -> Self {
        Self {
            inventory_manager: InventoryManager::new(),
            equipment_manager: EquipmentManager::new(),
            interaction_rules: Self::default_interaction_rules(),
            educational_items: Self::default_educational_items(),
        }
    }

    /// Add an item to the player's inventory
    pub fn add_item(&mut self, player: &mut Player, item: Item) -> GameResult<()> {
        // Check inventory constraints
        self.inventory_manager.validate_addition(&item)?;

        // Add to inventory
        self.inventory_manager.add_item(item.clone())?;

        // Update player's legacy inventory for backward compatibility
        if let Ok(legacy_item) = self.convert_to_legacy_item(&item) {
            player.inventory.items.push(legacy_item);
        }

        Ok(())
    }

    /// Remove an item from inventory
    pub fn remove_item(&mut self, player: &mut Player, item_id: &ItemId) -> GameResult<Option<Item>> {
        if let Some(item) = self.inventory_manager.remove_item(item_id)? {
            // Remove from player's legacy inventory
            if let Some(pos) = player.inventory.items.iter().position(|i| i.name == item.properties.name) {
                player.inventory.items.remove(pos);
            }
            Ok(Some(item))
        } else {
            Ok(None)
        }
    }

    /// Equip an item
    pub fn equip_item(&mut self, player: &mut Player, item_id: &ItemId) -> GameResult<()> {
        let item = self.inventory_manager.get_item(item_id)
            .ok_or_else(|| crate::GameError::InvalidInput("Item not found".to_string()))?;

        if let ItemType::Equipment(equipment) = &item.item_type {
            self.equipment_manager.equip_item(item_id.clone(), equipment.clone())?;

            // Apply stat bonuses to player
            for bonus in &equipment.bonuses {
                self.apply_equipment_bonus(player, bonus);
            }
        } else {
            return Err(crate::GameError::InvalidInput("Item is not equippable".to_string()).into());
        }

        Ok(())
    }

    /// Unequip an item
    pub fn unequip_item(&mut self, player: &mut Player, slot: EquipmentSlot) -> GameResult<Option<ItemId>> {
        if let Some((item_id, equipment)) = self.equipment_manager.unequip_item(slot)? {
            // Remove stat bonuses from player
            for bonus in &equipment.bonuses {
                self.remove_equipment_bonus(player, bonus);
            }
            Ok(Some(item_id))
        } else {
            Ok(None)
        }
    }

    /// Use an item
    pub fn use_item(&mut self, player: &mut Player, item_id: &ItemId, target: Option<&str>) -> GameResult<String> {
        let item = self.inventory_manager.get_item(item_id)
            .ok_or_else(|| crate::GameError::InvalidInput("Item not found".to_string()))?
            .clone();

        match &item.item_type {
            ItemType::Consumable { effect, uses_remaining } => {
                if *uses_remaining <= 0 {
                    return Err(crate::GameError::InvalidInput("Item has no uses remaining".to_string()).into());
                }

                let result = self.apply_item_effect(player, effect)?;

                // Reduce uses or remove item if depleted
                self.inventory_manager.reduce_item_uses(item_id)?;

                Ok(result)
            }
            ItemType::Tool { tool_function } => {
                self.use_tool(player, tool_function, target)
            }
            ItemType::Educational(educational) => {
                self.use_educational_item(player, educational, target)
            }
            _ => Err(crate::GameError::InvalidInput("Item cannot be used".to_string()).into())
        }
    }

    /// Examine an item for detailed information
    pub fn examine_item(&self, item_id: &ItemId) -> GameResult<String> {
        let item = self.inventory_manager.get_item(item_id)
            .ok_or_else(|| crate::GameError::InvalidInput("Item not found".to_string()))?;

        let mut description = format!("{}\n{}\n", item.properties.name, item.properties.description);

        // Add type-specific information
        match &item.item_type {
            ItemType::Equipment(equipment) => {
                description.push_str(&format!("Equipment Slot: {:?}\n", equipment.slot));
                if !equipment.bonuses.is_empty() {
                    description.push_str("Bonuses:\n");
                    for bonus in &equipment.bonuses {
                        description.push_str(&format!("  - {:?}\n", bonus));
                    }
                }
            }
            ItemType::Consumable { effect, uses_remaining } => {
                description.push_str(&format!("Uses Remaining: {}\n", uses_remaining));
                description.push_str(&format!("Effect: {:?}\n", effect));
            }
            ItemType::Educational(educational) => {
                description.push_str("Educational Benefits:\n");
                for bonus in &educational.learning_bonuses {
                    description.push_str(&format!("  - {:?}\n", bonus));
                }
            }
            _ => {}
        }

        // Add physical properties
        description.push_str(&format!("Weight: {:.1} kg\n", item.properties.weight));
        description.push_str(&format!("Value: {} silver\n", item.properties.value));
        description.push_str(&format!("Rarity: {:?}\n", item.properties.rarity));

        if item.properties.durability < item.properties.max_durability {
            let condition = if item.properties.durability as f32 > item.properties.max_durability as f32 * 0.8 {
                "Good"
            } else if item.properties.durability as f32 > item.properties.max_durability as f32 * 0.5 {
                "Fair"
            } else if item.properties.durability as f32 > item.properties.max_durability as f32 * 0.2 {
                "Poor"
            } else {
                "Broken"
            };
            description.push_str(&format!("Condition: {} ({}/{})\n",
                condition, item.properties.durability, item.properties.max_durability));
        }

        Ok(description)
    }

    /// Get inventory summary
    pub fn get_inventory_summary(&self) -> String {
        self.inventory_manager.get_summary()
    }

    /// Get equipped items summary
    pub fn get_equipment_summary(&self) -> String {
        self.equipment_manager.get_summary()
    }

    /// Calculate total learning bonus for a theory and method
    pub fn calculate_learning_bonus(&self, theory_id: &str, method: &LearningMethod) -> f32 {
        let mut total_bonus = 0.0;

        // Check equipped educational items
        for equipment_id in self.equipment_manager.get_equipped_items() {
            if let Some(item) = self.inventory_manager.get_item(equipment_id) {
                if let ItemType::Educational(educational) = &item.item_type {
                    for bonus in &educational.learning_bonuses {
                        if bonus.applies_to_theory(theory_id) && bonus.applies_to_method(method) {
                            total_bonus += bonus.bonus_multiplier;
                        }
                    }
                }
            }
        }

        // Check inventory for applicable educational items
        for item in self.inventory_manager.get_all_items() {
            if let ItemType::Educational(educational) = &item.item_type {
                for bonus in &educational.learning_bonuses {
                    if bonus.applies_to_theory(theory_id) && bonus.applies_to_method(method) {
                        // Non-equipped items provide reduced bonus
                        total_bonus += bonus.bonus_multiplier * 0.5;
                    }
                }
            }
        }

        total_bonus
    }

    /// Check if player has required items for an action
    pub fn has_required_items(&self, requirements: &[ItemId]) -> bool {
        requirements.iter().all(|req| self.inventory_manager.has_item(req))
    }

    /// Private helper methods

    fn apply_item_effect(&self, player: &mut Player, effect: &ItemEffect) -> GameResult<String> {
        match effect {
            ItemEffect::RestoreEnergy(amount) => {
                player.recover_energy(*amount, 0);
                Ok(format!("Restored {} mental energy", amount))
            }
            ItemEffect::ReduceFatigue(amount) => {
                player.recover_energy(0, *amount);
                Ok(format!("Reduced fatigue by {}", amount))
            }
            ItemEffect::TemporaryAttributeBoost { attribute, amount, duration: _ } => {
                // For now, apply permanent boost (temporary effects would need game time tracking)
                match attribute.as_str() {
                    "mental_acuity" => {
                        player.attributes.mental_acuity += amount;
                        Ok(format!("Mental acuity increased by {}", amount))
                    }
                    "resonance_sensitivity" => {
                        player.attributes.resonance_sensitivity += amount;
                        Ok(format!("Resonance sensitivity increased by {}", amount))
                    }
                    _ => Ok("Unknown attribute boost".to_string())
                }
            }
            ItemEffect::LearnTheory { theory_id, understanding_boost } => {
                let current = player.theory_understanding(theory_id);
                let new_understanding = (current + understanding_boost).min(1.0);
                player.knowledge.theories.insert(theory_id.clone(), new_understanding);
                Ok(format!("Gained understanding of {}", theory_id))
            }
            ItemEffect::HealDamage(amount) => {
                // For future health system - for now just report it
                Ok(format!("Would heal {} damage (health system not implemented)", amount))
            }
            ItemEffect::TemporarySpell { spell_type, duration: _ } => {
                // For future temporary spell system
                Ok(format!("Granted temporary {} spell ability", spell_type))
            }
            ItemEffect::EnhanceCrystal { property, amount } => {
                if let Some(crystal) = player.active_crystal_mut() {
                    match property.as_str() {
                        "integrity" => {
                            crystal.integrity = (crystal.integrity + amount).min(100.0);
                            Ok(format!("Crystal integrity improved by {:.1}", amount))
                        }
                        "purity" => {
                            crystal.purity = (crystal.purity + amount).min(1.0);
                            Ok(format!("Crystal purity improved by {:.1}", amount))
                        }
                        _ => Ok(format!("Enhanced crystal {} by {:.1}", property, amount))
                    }
                } else {
                    Ok("No crystal equipped to enhance".to_string())
                }
            }
            ItemEffect::Multiple(effects) => {
                let mut results = Vec::new();
                for effect in effects {
                    results.push(self.apply_item_effect(player, effect)?);
                }
                Ok(results.join("; "))
            }
        }
    }

    fn apply_equipment_bonus(&self, player: &mut Player, bonus: &EquipmentBonus) {
        match bonus {
            EquipmentBonus::AttributeBoost { attribute, amount } => {
                match attribute.as_str() {
                    "mental_acuity" => player.attributes.mental_acuity += amount,
                    "resonance_sensitivity" => player.attributes.resonance_sensitivity += amount,
                    _ => {}
                }
            }
            EquipmentBonus::LearningEfficiency { method: _, bonus: _ } => {
                // Learning efficiency bonuses are applied during learning calculation
            }
            EquipmentBonus::MagicBonus { spell_type: _, bonus: _ } => {
                // Magic bonuses are applied during magic calculation
            }
            EquipmentBonus::CrystalProtection(_) => {
                // Crystal protection is applied during crystal degradation
            }
            EquipmentBonus::EnergyCostReduction(_) => {
                // Energy cost reduction is applied during magic calculation
            }
            EquipmentBonus::FatigueResistance(_) => {
                // Fatigue resistance is applied during fatigue calculation
            }
            EquipmentBonus::TheoryBonus { theory_id: _, bonus: _ } => {
                // Theory bonuses are applied during learning calculation
            }
            EquipmentBonus::FactionBonus { faction_id: _, bonus: _ } => {
                // Faction bonuses would be applied to faction reputation
                // For now, just noted but not implemented
            }
        }
    }

    fn remove_equipment_bonus(&self, player: &mut Player, bonus: &EquipmentBonus) {
        match bonus {
            EquipmentBonus::AttributeBoost { attribute, amount } => {
                match attribute.as_str() {
                    "mental_acuity" => player.attributes.mental_acuity -= amount,
                    "resonance_sensitivity" => player.attributes.resonance_sensitivity -= amount,
                    _ => {}
                }
            }
            _ => {} // Other bonuses are removed by not being applied
        }
    }

    fn use_tool(&self, player: &mut Player, tool_function: &str, target: Option<&str>) -> GameResult<String> {
        match tool_function {
            "resonance_measurement" => {
                if let Some(crystal) = player.active_crystal() {
                    Ok(format!("Crystal resonance frequency: {} Hz, Efficiency: {:.1}%",
                        crystal.frequency, crystal.efficiency() * 100.0))
                } else {
                    Err(crate::GameError::InvalidInput("No crystal equipped".to_string()).into())
                }
            }
            "crystal_analysis" => {
                if let Some(crystal) = player.active_crystal() {
                    Ok(format!("Crystal Analysis:\nType: {:?}\nIntegrity: {:.1}%\nPurity: {:.1}%\nSize: {:?}",
                        crystal.crystal_type, crystal.integrity, crystal.purity * 100.0, crystal.size))
                } else {
                    Err(crate::GameError::InvalidInput("No crystal equipped".to_string()).into())
                }
            }
            _ => Ok(format!("Used tool: {}", tool_function))
        }
    }

    fn use_educational_item(&self, player: &mut Player, educational: &EducationalItem, target: Option<&str>) -> GameResult<String> {
        match &educational.item_function {
            crate::systems::items::educational::EducationalFunction::ResearchTool(tool) => {
                let theory_id = target.unwrap_or(&tool.required_theory);
                if player.theory_understanding(theory_id) >= tool.min_understanding {
                    Ok(format!("Using {} for research on {}", educational.name, theory_id))
                } else {
                    Err(crate::GameError::InvalidInput(
                        format!("Insufficient understanding of {} required", theory_id)
                    ).into())
                }
            }
            crate::systems::items::educational::EducationalFunction::CollaborativeTool(_) => {
                Ok("Started collaborative learning session".to_string())
            }
            crate::systems::items::educational::EducationalFunction::TheoryUnlock { theory_id } => {
                if !player.knows_theory(theory_id) {
                    player.knowledge.theories.insert(theory_id.clone(), 0.1);
                    Ok(format!("Discovered new theory: {}", theory_id))
                } else {
                    Ok(format!("Already know theory: {}", theory_id))
                }
            }
            crate::systems::items::educational::EducationalFunction::MethodEnhancer { method, bonus } => {
                Ok(format!("Enhanced {} learning method with {:.0}% bonus",
                    format!("{:?}", method), bonus * 100.0))
            }
            crate::systems::items::educational::EducationalFunction::KnowledgeArchive { theories } => {
                Ok(format!("Accessed knowledge archive with {} theories available", theories.len()))
            }
        }
    }

    fn convert_to_legacy_item(&self, item: &Item) -> GameResult<crate::core::player::Item> {
        use crate::core::player::ItemType as LegacyItemType;

        let legacy_type = match &item.item_type {
            ItemType::Book { theory_id } => LegacyItemType::Book(theory_id.clone()),
            ItemType::Artifact { properties } => LegacyItemType::Artifact(properties.clone()),
            _ => LegacyItemType::Mundane,
        };

        Ok(crate::core::player::Item {
            name: item.properties.name.clone(),
            description: item.properties.description.clone(),
            item_type: legacy_type,
        })
    }

    fn default_interaction_rules() -> HashMap<String, CombinationRule> {
        // TODO: Implement default item combination rules
        HashMap::new()
    }

    fn default_educational_items() -> HashMap<ItemId, EducationalItem> {
        // TODO: Implement default educational items catalog
        HashMap::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Player;

    #[test]
    fn test_item_system_creation() {
        let item_system = ItemSystem::new();
        assert!(item_system.inventory_manager.get_all_items().is_empty());
        assert!(item_system.equipment_manager.get_equipped_items().is_empty());
    }

    #[test]
    fn test_add_remove_item() {
        let mut item_system = ItemSystem::new();
        let mut player = Player::new("Test".to_string());

        let item = Item::new_basic(
            "test_item".to_string(),
            "A test item".to_string(),
            ItemType::Mundane,
        );

        let item_id = item.id.clone();

        // Add item
        item_system.add_item(&mut player, item).unwrap();
        assert!(item_system.inventory_manager.has_item(&item_id));

        // Remove item
        let removed = item_system.remove_item(&mut player, &item_id).unwrap();
        assert!(removed.is_some());
        assert!(!item_system.inventory_manager.has_item(&item_id));
    }
}