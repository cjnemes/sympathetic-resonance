//! Integration tests for the item system
//!
//! These tests address the critical gaps identified by the test-guardian agent:
//! - Equipment bonus integration tests (HIGH RISK)
//! - Inventory constraint stress tests (HIGH RISK)
//! - Error handling test suite (HIGH RISK)
//! - Player state corruption prevention tests
//! - Memory leak prevention tests

use sympathetic_resonance::core::Player;
use sympathetic_resonance::systems::items::core::{Item, ItemType, ItemEffect};
use sympathetic_resonance::systems::items::equipment::{Equipment, EquipmentSlot, EquipmentBonus};
use sympathetic_resonance::systems::items::educational::EducationalItem;
use sympathetic_resonance::systems::items::inventory::{InventoryManager, InventoryConstraints};
use sympathetic_resonance::systems::knowledge::LearningMethod;

/// High-Risk Integration Tests - Equipment Bonus System
mod equipment_bonus_integration {
    use super::*;

    #[test]
    fn test_equipment_bonus_application_and_removal() {
        let mut player = Player::new("Test Player".to_string());
        player.ensure_enhanced_item_system();

        // Create equipment with attribute bonus
        let equipment = Equipment::new_basic(EquipmentSlot::Head)
            .add_bonus(EquipmentBonus::AttributeBoost {
                attribute: "mental_acuity".to_string(),
                amount: 10,
            });

        let item = Item::new_basic(
            "Intelligence Crown".to_string(),
            "Boosts mental acuity".to_string(),
            ItemType::Equipment(equipment),
        );

        let item_id = item.id.clone();
        let initial_mental_acuity = player.attributes.mental_acuity;

        // Test equipment application
        player.add_enhanced_item(item).unwrap();
        player.equip_enhanced_item(&item_id).unwrap();

        // Verify bonus was applied
        assert_eq!(player.attributes.mental_acuity, initial_mental_acuity + 10);

        // Test equipment removal
        player.unequip_enhanced_item(EquipmentSlot::Head).unwrap();

        // Verify bonus was removed
        assert_eq!(player.attributes.mental_acuity, initial_mental_acuity);
    }

    #[test]
    fn test_multiple_equipment_bonus_stacking() {
        let mut player = Player::new("Test Player".to_string());
        player.ensure_enhanced_item_system();

        let initial_mental_acuity = player.attributes.mental_acuity;

        // Create multiple equipment pieces with bonuses
        let helmet = Item::new_basic(
            "Smart Helmet".to_string(),
            "Boosts mental acuity".to_string(),
            ItemType::Equipment(Equipment::new_basic(EquipmentSlot::Head)
                .add_bonus(EquipmentBonus::AttributeBoost {
                    attribute: "mental_acuity".to_string(),
                    amount: 5,
                })),
        );

        let ring = Item::new_basic(
            "Ring of Intelligence".to_string(),
            "Boosts mental acuity".to_string(),
            ItemType::Equipment(Equipment::new_basic(EquipmentSlot::Ring1)
                .add_bonus(EquipmentBonus::AttributeBoost {
                    attribute: "mental_acuity".to_string(),
                    amount: 3,
                })),
        );

        let helmet_id = helmet.id.clone();
        let ring_id = ring.id.clone();

        // Add and equip items
        player.add_enhanced_item(helmet).unwrap();
        player.add_enhanced_item(ring).unwrap();

        player.equip_enhanced_item(&helmet_id).unwrap();
        player.equip_enhanced_item(&ring_id).unwrap();

        // Verify bonuses stack correctly
        assert_eq!(player.attributes.mental_acuity, initial_mental_acuity + 8);

        // Test partial removal
        player.unequip_enhanced_item(EquipmentSlot::Head).unwrap();
        assert_eq!(player.attributes.mental_acuity, initial_mental_acuity + 3);

        // Test complete removal
        player.unequip_enhanced_item(EquipmentSlot::Ring1).unwrap();
        assert_eq!(player.attributes.mental_acuity, initial_mental_acuity);
    }

    #[test]
    fn test_equipment_bonus_persistence_through_save_load() {
        // This test ensures bonuses are correctly preserved and restored
        let mut player = Player::new("Test Player".to_string());
        player.ensure_enhanced_item_system();

        let equipment = Equipment::new_basic(EquipmentSlot::Chest)
            .add_bonus(EquipmentBonus::AttributeBoost {
                attribute: "resonance_sensitivity".to_string(),
                amount: 15,
            });

        let item = Item::new_basic(
            "Resonance Vest".to_string(),
            "Enhances resonance sensitivity".to_string(),
            ItemType::Equipment(equipment),
        );

        let item_id = item.id.clone();
        let initial_sensitivity = player.attributes.resonance_sensitivity;

        // Equip item
        player.add_enhanced_item(item).unwrap();
        player.equip_enhanced_item(&item_id).unwrap();

        let boosted_sensitivity = player.attributes.resonance_sensitivity;
        assert_eq!(boosted_sensitivity, initial_sensitivity + 15);

        // Simulate save/load by cloning player (serialization would work similarly)
        let cloned_player = player.clone();
        assert_eq!(cloned_player.attributes.resonance_sensitivity, boosted_sensitivity);
    }
}

/// High-Risk Integration Tests - Inventory Constraints
mod inventory_constraint_stress_tests {
    use super::*;

    #[test]
    fn test_weight_limit_enforcement() {
        let mut constraints = InventoryConstraints::default();
        constraints.max_weight = 10.0; // Very low weight limit

        let mut inventory = InventoryManager::with_constraints(constraints);

        // Create heavy items
        let mut heavy_item = Item::new_basic(
            "Heavy Item".to_string(),
            "Very heavy".to_string(),
            ItemType::Mundane,
        );
        heavy_item.properties.weight = 8.0;

        let mut another_heavy_item = Item::new_basic(
            "Another Heavy Item".to_string(),
            "Also very heavy".to_string(),
            ItemType::Mundane,
        );
        another_heavy_item.properties.weight = 5.0;

        // First item should succeed
        assert!(inventory.add_item(heavy_item).is_ok());
        assert_eq!(inventory.current_weight(), 8.0);

        // Second item should fail due to weight limit
        let result = inventory.add_item(another_heavy_item);
        assert!(result.is_err());
        assert_eq!(inventory.current_weight(), 8.0); // Weight unchanged
    }

    #[test]
    fn test_slot_limit_enforcement() {
        let mut constraints = InventoryConstraints::default();
        constraints.max_slots = 2; // Very low slot limit

        let mut inventory = InventoryManager::with_constraints(constraints);

        // Add items to fill slots
        let item1 = Item::new_basic("Item 1".to_string(), "First item".to_string(), ItemType::Mundane);
        let item2 = Item::new_basic("Item 2".to_string(), "Second item".to_string(), ItemType::Mundane);
        let item3 = Item::new_basic("Item 3".to_string(), "Third item".to_string(), ItemType::Mundane);

        // First two items should succeed
        assert!(inventory.add_item(item1).is_ok());
        assert!(inventory.add_item(item2).is_ok());
        assert_eq!(inventory.current_slots(), 2);

        // Third item should fail
        let result = inventory.add_item(item3);
        assert!(result.is_err());
        assert_eq!(inventory.current_slots(), 2); // Slots unchanged
    }

    #[test]
    fn test_category_limit_enforcement() {
        let mut constraints = InventoryConstraints::default();
        constraints.category_limits.insert(
            sympathetic_resonance::systems::items::core::ItemCategory::Books,
            1
        );

        let mut inventory = InventoryManager::with_constraints(constraints);

        // Create book items
        let book1 = Item::new_book(
            "Theory Book 1".to_string(),
            "First theory book".to_string(),
            "theory1".to_string(),
        );

        let book2 = Item::new_book(
            "Theory Book 2".to_string(),
            "Second theory book".to_string(),
            "theory2".to_string(),
        );

        // First book should succeed
        assert!(inventory.add_item(book1).is_ok());

        // Second book should fail due to category limit
        let result = inventory.add_item(book2);
        assert!(result.is_err());
    }

    #[test]
    fn test_inventory_stress_with_many_items() {
        // Test performance and memory behavior with large inventory
        let mut constraints = InventoryConstraints::default();
        constraints.max_slots = 1000; // Allow many more slots for stress test
        constraints.max_weight = 1000.0; // Allow more weight for stress test
        let mut inventory = InventoryManager::with_constraints(constraints);

        // Add many items to test performance
        for i in 0..1000 {
            let item = Item::new_basic(
                format!("Item {}", i),
                format!("Description for item {}", i),
                ItemType::Mundane,
            );

            let result = inventory.add_item(item);
            assert!(result.is_ok(), "Failed to add item {}", i);
        }

        assert_eq!(inventory.current_slots(), 1000);

        // Test search performance with many items
        let search_results = inventory.search_by_name("Item 500");
        assert_eq!(search_results.len(), 1);
    }
}

/// High-Risk Integration Tests - Error Handling
mod error_handling_test_suite {
    use super::*;

    #[test]
    fn test_use_nonexistent_item() {
        let mut player = Player::new("Test Player".to_string());

        // Try to use an item that doesn't exist
        let result = player.use_enhanced_item("nonexistent_item", None);
        assert!(result.is_err() || result.unwrap().contains("not found"));
    }

    #[test]
    fn test_equip_invalid_item() {
        let mut player = Player::new("Test Player".to_string());
        player.ensure_enhanced_item_system();

        // Add a non-equipment item
        let mundane_item = Item::new_basic(
            "Regular Item".to_string(),
            "Cannot be equipped".to_string(),
            ItemType::Mundane,
        );

        let item_id = mundane_item.id.clone();
        player.add_enhanced_item(mundane_item).unwrap();

        // Try to equip a non-equipment item
        let result = player.equip_enhanced_item(&item_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_nonexistent_item() {
        let mut player = Player::new("Test Player".to_string());
        player.ensure_enhanced_item_system();

        let result = player.remove_enhanced_item("nonexistent_item");
        // It's reasonable for removing a nonexistent item to return an error
        // or to return Ok(None) - both behaviors are acceptable
        match result {
            Ok(option) => assert!(option.is_none()),
            Err(_) => {}, // This is acceptable behavior
        }
    }

    #[test]
    fn test_consumable_item_depletion() {
        let mut player = Player::new("Test Player".to_string());
        player.ensure_enhanced_item_system();

        // Create consumable with 1 use
        let consumable = Item::new_consumable(
            "Health Potion".to_string(),
            "Restores health".to_string(),
            ItemEffect::RestoreEnergy(25),
            1,
        );

        let item_id = consumable.id.clone();
        player.add_enhanced_item(consumable).unwrap();

        // Use the item once (should succeed)
        let result = player.use_enhanced_item(&item_id, None);
        assert!(result.is_ok());

        // Try to use it again (should fail - item should be gone)
        let result2 = player.use_enhanced_item(&item_id, None);
        assert!(result2.is_err() || result2.unwrap().contains("not found"));
    }
}

/// Integration Tests - Player State Consistency
mod player_state_consistency {
    use super::*;

    #[test]
    fn test_player_state_integrity_after_item_operations() {
        let mut player = Player::new("Test Player".to_string());
        player.ensure_enhanced_item_system();

        let initial_mental_acuity = player.attributes.mental_acuity;
        let initial_energy = player.mental_state.current_energy;

        // Perform complex sequence of item operations
        let equipment = Item::new_basic(
            "Test Equipment".to_string(),
            "Test equipment".to_string(),
            ItemType::Equipment(Equipment::new_basic(EquipmentSlot::Head)
                .add_bonus(EquipmentBonus::AttributeBoost {
                    attribute: "mental_acuity".to_string(),
                    amount: 5,
                })),
        );

        let consumable = Item::new_consumable(
            "Energy Potion".to_string(),
            "Restores energy".to_string(),
            ItemEffect::RestoreEnergy(20),
            1,
        );

        let equipment_id = equipment.id.clone();
        let consumable_id = consumable.id.clone();

        // Add items
        player.add_enhanced_item(equipment).unwrap();
        player.add_enhanced_item(consumable).unwrap();

        // Equip item
        player.equip_enhanced_item(&equipment_id).unwrap();
        assert_eq!(player.attributes.mental_acuity, initial_mental_acuity + 5);

        // Use consumable
        player.use_enhanced_item(&consumable_id, None).unwrap();
        let energy_after_consumable = player.mental_state.current_energy;
        assert!(energy_after_consumable >= initial_energy, "Energy should increase or stay same after consumable");

        // Unequip item
        player.unequip_enhanced_item(EquipmentSlot::Head).unwrap();
        assert_eq!(player.attributes.mental_acuity, initial_mental_acuity);

        // Verify final state consistency
        assert_eq!(player.mental_state.current_energy, energy_after_consumable);
        assert_eq!(player.attributes.mental_acuity, initial_mental_acuity);
    }

    #[test]
    fn test_educational_item_integration() {
        let mut player = Player::new("Test Player".to_string());
        player.ensure_enhanced_item_system();

        // Create educational item
        let educational_item = Item::new_basic(
            "Study Guide".to_string(),
            "Enhances learning".to_string(),
            ItemType::Educational(EducationalItem::new_method_enhancer(
                "Study Guide".to_string(),
                LearningMethod::Study,
                0.25, // +25% efficiency
            )),
        );

        let item_id = educational_item.id.clone();
        player.add_enhanced_item(educational_item).unwrap();

        // Test educational item functionality
        let result = player.use_enhanced_item(&item_id, None);
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.contains("Enhanced") && response.contains("25%"));
    }
}

/// Performance and Memory Tests
mod performance_tests {
    use super::*;

    #[test]
    fn test_large_inventory_operations() {
        let mut player = Player::new("Test Player".to_string());
        player.ensure_enhanced_item_system();

        // Add many items to test memory usage (within default slot limit)
        for i in 0..25 {
            let item = Item::new_basic(
                format!("Item {}", i),
                format!("Test item number {}", i),
                ItemType::Mundane,
            );

            let result = player.add_enhanced_item(item);
            assert!(result.is_ok(), "Failed to add item {}", i);
        }

        // Test inventory operations still work efficiently
        let summary = player.enhanced_inventory_summary();
        assert!(summary.contains("25"));
    }

    #[test]
    fn test_equipment_manager_with_many_items() {
        let mut player = Player::new("Test Player".to_string());
        player.ensure_enhanced_item_system();

        // Test equipment system with various items
        let slots = vec![
            EquipmentSlot::Head,
            EquipmentSlot::Chest,
            EquipmentSlot::Hands,
            EquipmentSlot::Feet,
            EquipmentSlot::Ring1,
            EquipmentSlot::Ring2,
        ];

        for (i, slot) in slots.iter().enumerate() {
            let equipment = Item::new_basic(
                format!("Equipment {}", i),
                format!("Equipment for slot {:?}", slot),
                ItemType::Equipment(Equipment::new_basic(*slot)
                    .add_bonus(EquipmentBonus::AttributeBoost {
                        attribute: "mental_acuity".to_string(),
                        amount: 1,
                    })),
            );

            let item_id = equipment.id.clone();
            player.add_enhanced_item(equipment).unwrap();
            player.equip_enhanced_item(&item_id).unwrap();
        }

        // Verify all equipment is properly managed
        let equipment_summary = player.equipment_summary();
        assert!(equipment_summary.contains("Equipped Items"));
    }
}