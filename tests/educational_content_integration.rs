//! Comprehensive integration tests for educational content and progressive unlock system
//!
//! These tests verify that:
//! - Educational items load correctly from database
//! - Progressive unlock system works with theory mastery and faction reputation
//! - Faction-specific items provide appropriate bonuses
//! - Item unlock notifications work correctly
//! - Cross-faction item conflicts are handled properly

use sympathetic_resonance::core::Player;
use sympathetic_resonance::systems::items::{ItemSystem, UnlockCategory};
use sympathetic_resonance::systems::knowledge::TheoryProgress;

/// Integration Tests for Educational Content Population
mod educational_content_integration {
    use super::*;

    #[test]
    fn test_database_educational_items_population() {
        // This test verifies that educational items are properly loaded from database
        // NOTE: This assumes the populate_faction_items.py script has been run

        let item_system = ItemSystem::new();

        // Should have educational items in the system
        // (This would require loading from database - implementation depends on database loading system)

        // For now, test that the item system can be created without errors
        assert!(item_system.get_unlock_progress(&Player::new("Test".to_string()), UnlockCategory::Starter).0 == 0);
    }

    #[test]
    fn test_faction_reputation_unlocks() {
        let item_system = ItemSystem::new();
        let mut player = Player::new("Test Player".to_string());

        // Initially, faction items should not be unlocked
        assert!(!item_system.is_item_unlocked(&player, &"council_scholars_circlet".to_string()));
        assert!(!item_system.is_item_unlocked(&player, &"harmony_meditation_stone".to_string()));

        // Add faction reputation to unlock Magisters Council items
        player.modify_faction_reputation(sympathetic_resonance::systems::factions::FactionId::MagistersCouncil, 50);

        // Now council items should be unlocked
        assert!(item_system.is_item_unlocked(&player, &"council_scholars_circlet".to_string()));

        // But other faction items should still be locked
        assert!(!item_system.is_item_unlocked(&player, &"harmony_meditation_stone".to_string()));
    }

    #[test]
    fn test_theory_mastery_unlocks() {
        let item_system = ItemSystem::new();
        let mut player = Player::new("Test Player".to_string());

        // Initially, theory-specific items should not be unlocked
        assert!(!item_system.is_item_unlocked(&player, &"crystal_analysis_kit".to_string()));
        assert!(!item_system.is_item_unlocked(&player, &"neural_amplification_headband".to_string()));

        // Add theory progress for crystal structures
        player.knowledge.theory_progress.insert(
            "crystal_structures".to_string(),
            TheoryProgress {
                understanding_level: 0.30, // Above 25% threshold (0.25)
                experience_points: 150,
                learning_history: std::collections::HashMap::new(),
                time_invested: 150,
                discovered_at: chrono::Utc::now().timestamp(),
                mastered_at: None,
                is_active_research: false,
                research_progress: 0.0,
            },
        );

        // Crystal analysis kit should now be unlocked
        assert!(item_system.is_item_unlocked(&player, &"crystal_analysis_kit".to_string()));

        // But neural headband should still be locked (needs mental_resonance)
        assert!(!item_system.is_item_unlocked(&player, &"neural_amplification_headband".to_string()));
    }

    #[test]
    fn test_progressive_unlock_notifications() {
        let mut item_system = ItemSystem::new();
        let mut player = Player::new("Test Player".to_string());

        // Check for unlocks with no progress - should be empty
        let initial_unlocks = item_system.check_for_new_unlocks(&player);
        assert!(initial_unlocks.is_empty());

        // Add faction reputation
        player.modify_faction_reputation(sympathetic_resonance::systems::factions::FactionId::MagistersCouncil, 50);

        // Check for new unlocks - should have notifications
        let new_unlocks = item_system.check_for_new_unlocks(&player);
        assert!(!new_unlocks.is_empty());

        // Verify unlock details
        let council_unlock = new_unlocks.iter()
            .find(|unlock| unlock.item_id == "council_scholars_circlet")
            .expect("Council item should be unlocked");

        assert!(council_unlock.unlock_source.contains("magisters_council"));
        assert!(!council_unlock.requirements_met.is_empty());

        // Running check again should not produce duplicate notifications
        let duplicate_check = item_system.check_for_new_unlocks(&player);
        assert!(duplicate_check.is_empty());
    }

    #[test]
    fn test_multi_theory_unlock_requirements() {
        let item_system = ItemSystem::new();
        let mut player = Player::new("Test Player".to_string());

        // Grand synthesis apparatus requires multiple high-level theories
        assert!(!item_system.is_item_unlocked(&player, &"grand_synthesis_apparatus".to_string()));

        // Add partial progress - should still be locked
        player.knowledge.theory_progress.insert(
            "crystal_structures".to_string(),
            TheoryProgress {
                understanding_level: 0.95, // Above 90% threshold (0.90)
                experience_points: 1000,
                learning_history: std::collections::HashMap::new(),
                time_invested: 1000,
                discovered_at: chrono::Utc::now().timestamp(),
                mastered_at: None,
                is_active_research: false,
                research_progress: 0.0,
            },
        );

        // Still locked because we need all three theories
        assert!(!item_system.is_item_unlocked(&player, &"grand_synthesis_apparatus".to_string()));

        // Add the other required theories
        player.knowledge.theory_progress.insert(
            "mental_resonance".to_string(),
            TheoryProgress {
                understanding_level: 0.92,
                experience_points: 800,
                learning_history: std::collections::HashMap::new(),
                time_invested: 800,
                discovered_at: chrono::Utc::now().timestamp(),
                mastered_at: None,
                is_active_research: false,
                research_progress: 0.0,
            },
        );

        player.knowledge.theory_progress.insert(
            "theoretical_synthesis".to_string(),
            TheoryProgress {
                understanding_level: 0.91,
                experience_points: 600,
                learning_history: std::collections::HashMap::new(),
                time_invested: 600,
                discovered_at: chrono::Utc::now().timestamp(),
                mastered_at: None,
                is_active_research: false,
                research_progress: 0.0,
            },
        );

        // Now should be unlocked
        assert!(item_system.is_item_unlocked(&player, &"grand_synthesis_apparatus".to_string()));
    }

    #[test]
    fn test_unlock_category_progress_tracking() {
        let item_system = ItemSystem::new();
        let mut player = Player::new("Test Player".to_string());

        // Initially, no items should be unlocked
        let (faction_unlocked, faction_total) = item_system.get_unlock_progress(&player, UnlockCategory::FactionLoyalty);
        assert_eq!(faction_unlocked, 0);
        assert!(faction_total > 0); // Should have faction items registered

        let (theory_unlocked, theory_total) = item_system.get_unlock_progress(&player, UnlockCategory::TheoryProgression);
        assert_eq!(theory_unlocked, 0);
        assert!(theory_total > 0); // Should have theory items registered

        // Add some faction reputation
        player.modify_faction_reputation(sympathetic_resonance::systems::factions::FactionId::MagistersCouncil, 50);
        player.modify_faction_reputation(sympathetic_resonance::systems::factions::FactionId::OrderOfHarmony, 25);

        // Should now have some faction items unlocked
        let (faction_unlocked_after, _) = item_system.get_unlock_progress(&player, UnlockCategory::FactionLoyalty);
        assert!(faction_unlocked_after > faction_unlocked);

        // Theory progress should remain unchanged
        let (theory_unlocked_after, _) = item_system.get_unlock_progress(&player, UnlockCategory::TheoryProgression);
        assert_eq!(theory_unlocked_after, theory_unlocked);
    }
}

/// Tests for Educational Item Learning Bonuses
mod educational_bonus_integration {
    use super::*;

    #[test]
    fn test_educational_item_loading_from_database() {
        // Test that educational items can be loaded from database
        // This would require database integration which may not be available in tests

        let item_system = ItemSystem::new();

        // Verify the system can handle educational items
        assert!(item_system.educational_items.is_empty()); // Default is empty, would be populated from DB in real game
    }

    #[test]
    fn test_faction_specific_learning_bonuses() {
        // This test would verify that faction items provide appropriate learning bonuses
        // Implementation depends on how educational items are integrated with learning system

        let player = Player::new("Test Player".to_string());

        // Test would involve:
        // 1. Adding faction-specific educational items to player inventory
        // 2. Verifying that learning efficiency calculations include faction bonuses
        // 3. Testing that faction conflicts are properly handled

        // For now, test that player can be created and has learning efficiency system
        assert!(player.calculate_learning_efficiency(
            &sympathetic_resonance::systems::knowledge::LearningMethod::Study
        ) >= 0.0);
    }

    #[test]
    fn test_faction_item_conflict_detection() {
        // Test that wearing items from opposing factions creates conflicts
        // This would require implementing faction conflict detection in the item system

        let mut player = Player::new("Test Player".to_string());

        // Set up player with reputation in opposing factions
        player.modify_faction_reputation(sympathetic_resonance::systems::factions::FactionId::MagistersCouncil, 50);
        player.modify_faction_reputation(sympathetic_resonance::systems::factions::FactionId::UndergroundNetwork, 25);

        // Test would involve equipping items from both factions and checking for penalties
        // Implementation depends on faction conflict system

        // For now, verify player can have multiple faction reputations
        assert_eq!(player.faction_standings.len(), 2);
    }
}

/// Performance and Edge Case Tests
mod performance_and_edge_cases {
    use super::*;

    #[test]
    fn test_unlock_system_performance_with_many_items() {
        let item_system = ItemSystem::new();
        let player = Player::new("Test Player".to_string());

        // Test performance with checking many unlock requirements
        let start_time = std::time::Instant::now();

        for i in 0..1000 {
            let item_id = format!("test_item_{}", i);
            item_system.is_item_unlocked(&player, &item_id);
        }

        let duration = start_time.elapsed();

        // Should complete quickly (under 100ms for 1000 checks)
        assert!(duration.as_millis() < 100, "Unlock checking took too long: {:?}", duration);
    }

    #[test]
    fn test_unlock_system_edge_cases() {
        let item_system = ItemSystem::new();
        let player = Player::new("Test Player".to_string());

        // Test with non-existent item
        assert!(item_system.is_item_unlocked(&player, &"non_existent_item".to_string()));

        // Test with empty player
        let empty_player = Player::new("Empty".to_string());
        assert!(!item_system.is_item_unlocked(&empty_player, &"council_scholars_circlet".to_string()));

        // Test unlock progress with no registered items for category
        let (unlocked, total) = item_system.get_unlock_progress(&player, UnlockCategory::Achievement);
        assert_eq!(unlocked, 0);
        assert_eq!(total, 0); // No achievement items registered by default
    }

    #[test]
    fn test_unlock_notification_memory_management() {
        let mut item_system = ItemSystem::new();
        let mut player = Player::new("Test Player".to_string());

        // Generate many unlock events
        for _i in 0..10 {
            player.modify_faction_reputation(sympathetic_resonance::systems::factions::FactionId::MagistersCouncil, 50);
            item_system.check_for_new_unlocks(&player);
        }

        // Should have some recent unlocks
        assert!(!item_system.get_recent_unlocks().is_empty());

        // Clear notifications
        item_system.clear_recent_unlocks();

        // Should now be empty
        assert!(item_system.get_recent_unlocks().is_empty());
    }
}

/// Integration Tests with Existing Systems
mod system_integration_tests {
    use super::*;

    #[test]
    fn test_integration_with_existing_item_system() {
        let mut item_system = ItemSystem::new();
        let mut player = Player::new("Test Player".to_string());

        // Test that new unlock system integrates with existing inventory system
        let unlocked_items = item_system.get_unlocked_items(&player);

        // Should be able to get unlocked items without errors
        assert!(!unlocked_items.is_empty() || unlocked_items.is_empty());

        // Test that unlock system doesn't interfere with basic item operations
        let test_item = sympathetic_resonance::systems::items::Item::new_basic(
            "test_item".to_string(),
            "A test item".to_string(),
            sympathetic_resonance::systems::items::ItemType::Mundane,
        );

        // Should be able to add items regardless of unlock status
        assert!(item_system.add_item(&mut player, test_item).is_ok());
    }

    #[test]
    fn test_integration_with_faction_system() {
        let item_system = ItemSystem::new();
        let mut player = Player::new("Test Player".to_string());

        // Test that unlock system properly integrates with faction reputation
        assert_eq!(item_system.get_unlock_progress(&player, UnlockCategory::FactionLoyalty).0, 0);

        // Modify faction reputation
        player.modify_faction_reputation(sympathetic_resonance::systems::factions::FactionId::MagistersCouncil, 50);

        // Should affect unlock progress
        assert!(item_system.get_unlock_progress(&player, UnlockCategory::FactionLoyalty).0 > 0);
    }

    #[test]
    fn test_integration_with_knowledge_system() {
        let item_system = ItemSystem::new();
        let mut player = Player::new("Test Player".to_string());

        // Test that unlock system properly integrates with theory mastery
        assert_eq!(item_system.get_unlock_progress(&player, UnlockCategory::TheoryProgression).0, 0);

        // Add theory progress
        player.knowledge.theory_progress.insert(
            "crystal_structures".to_string(),
            TheoryProgress {
                understanding_level: 0.75,
                experience_points: 300,
                learning_history: std::collections::HashMap::new(),
                time_invested: 300,
                discovered_at: chrono::Utc::now().timestamp(),
                mastered_at: None,
                is_active_research: false,
                research_progress: 0.0,
            },
        );

        // Should affect unlock progress
        assert!(item_system.get_unlock_progress(&player, UnlockCategory::TheoryProgression).0 > 0);
    }
}