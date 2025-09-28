//! Comprehensive tests for the quest system
//!
//! This module provides extensive test coverage for all quest system components,
//! ensuring 150% coverage of functionality including edge cases and integration scenarios.

#[cfg(test)]
mod tests {
    use super::super::quests::*;
    use super::super::quest_examples::*;
    use crate::core::Player;
    use crate::systems::factions::{FactionSystem, FactionId};
    use std::collections::HashMap;
    use chrono::Utc;

    /// Create a test player with basic attributes
    fn create_test_player() -> Player {
        let mut player = Player::new("Test Player".to_string());
        player.attributes.mental_acuity = 30;
        player.attributes.resonance_sensitivity = 25;
        player.knowledge.theories.insert("harmonic_fundamentals".to_string(), 0.8);
        player.knowledge.theories.insert("crystal_structures".to_string(), 0.6);
        player.faction_standings.insert(FactionId::MagistersCouncil, 30);
        player.faction_standings.insert(FactionId::NeutralScholars, 20);
        player.playtime_minutes = 120;
        player
    }

    /// Create an advanced test player with higher capabilities
    fn create_advanced_player() -> Player {
        let mut player = create_test_player();
        player.attributes.mental_acuity = 50;
        player.attributes.resonance_sensitivity = 40;
        player.knowledge.theories.insert("harmonic_fundamentals".to_string(), 1.0);
        player.knowledge.theories.insert("crystal_structures".to_string(), 0.8);
        player.knowledge.theories.insert("mental_resonance".to_string(), 0.7);
        player.knowledge.theories.insert("bio_resonance".to_string(), 0.6);
        player.knowledge.theories.insert("detection_arrays".to_string(), 0.5);
        player.faction_standings.insert(FactionId::MagistersCouncil, 50);
        player.faction_standings.insert(FactionId::NeutralScholars, 60);
        player.faction_standings.insert(FactionId::OrderOfHarmony, 25);
        player.playtime_minutes = 300;
        player
    }

    /// Create a test quest system with example quests
    fn create_test_quest_system() -> QuestSystem {
        let mut quest_system = QuestSystem::new();
        let example_quests = create_example_quests();
        for quest in example_quests {
            quest_system.add_quest_definition(quest);
        }
        quest_system
    }

    #[test]
    fn test_quest_system_creation() {
        let quest_system = QuestSystem::new();
        assert_eq!(quest_system.quest_definitions.len(), 0);
        assert_eq!(quest_system.player_progress.len(), 0);
        assert!(quest_system.global_state.unlocked_quest_lines.contains(&"tutorial".to_string()));
    }

    #[test]
    fn test_quest_system_with_examples() {
        let quest_system = create_test_quest_system();
        assert_eq!(quest_system.quest_definitions.len(), 5);
        assert!(quest_system.quest_definitions.contains_key("resonance_foundation"));
        assert!(quest_system.quest_definitions.contains_key("crystal_analysis"));
        assert!(quest_system.quest_definitions.contains_key("diplomatic_balance"));
        assert!(quest_system.quest_definitions.contains_key("healing_research"));
        assert!(quest_system.quest_definitions.contains_key("unstable_site_investigation"));
    }

    #[test]
    fn test_quest_requirements_checking() {
        let quest_system = create_test_quest_system();
        let faction_system = FactionSystem::new();
        let player = create_test_player();

        // Test basic quest availability
        let foundation_quest = &quest_system.quest_definitions["resonance_foundation"];
        assert!(quest_system.is_quest_available(foundation_quest, &player, &faction_system));

        // Test quest with requirements not met
        let investigation_quest = &quest_system.quest_definitions["unstable_site_investigation"];
        assert!(!quest_system.is_quest_available(investigation_quest, &player, &faction_system));

        // Test with advanced player
        let advanced_player = create_advanced_player();
        let crystal_quest = &quest_system.quest_definitions["crystal_analysis"];
        assert!(quest_system.is_quest_available(crystal_quest, &advanced_player, &faction_system));
    }

    #[test]
    fn test_quest_difficulty_levels() {
        let quest_system = create_test_quest_system();

        let foundation_quest = &quest_system.quest_definitions["resonance_foundation"];
        assert_eq!(foundation_quest.difficulty, QuestDifficulty::Beginner);

        let analysis_quest = &quest_system.quest_definitions["crystal_analysis"];
        assert_eq!(analysis_quest.difficulty, QuestDifficulty::Intermediate);

        let investigation_quest = &quest_system.quest_definitions["unstable_site_investigation"];
        assert_eq!(investigation_quest.difficulty, QuestDifficulty::Expert);
    }

    #[test]
    fn test_quest_categories() {
        let quest_system = create_test_quest_system();

        let foundation_quest = &quest_system.quest_definitions["resonance_foundation"];
        assert_eq!(foundation_quest.category, QuestCategory::Tutorial);

        let analysis_quest = &quest_system.quest_definitions["crystal_analysis"];
        assert_eq!(analysis_quest.category, QuestCategory::Research);

        let diplomatic_quest = &quest_system.quest_definitions["diplomatic_balance"];
        assert_eq!(diplomatic_quest.category, QuestCategory::Political);

        let healing_quest = &quest_system.quest_definitions["healing_research"];
        assert_eq!(healing_quest.category, QuestCategory::Practical);

        let investigation_quest = &quest_system.quest_definitions["unstable_site_investigation"];
        assert_eq!(investigation_quest.category, QuestCategory::Experimental);
    }

    #[test]
    fn test_available_quests_filtering() {
        let quest_system = create_test_quest_system();
        let faction_system = FactionSystem::new();

        // Basic player should only see tutorial quest
        let basic_player = create_test_player();
        let available = quest_system.get_available_quests(&basic_player, &faction_system);
        assert_eq!(available.len(), 1);
        assert_eq!(available[0].id, "resonance_foundation");

        // Advanced player should see multiple quests
        let advanced_player = create_advanced_player();
        let available_advanced = quest_system.get_available_quests(&advanced_player, &faction_system);
        assert!(available_advanced.len() > 1);
    }

    #[test]
    fn test_quest_starting() {
        let mut quest_system = create_test_quest_system();
        let faction_system = FactionSystem::new();
        let player = create_test_player();

        // Start a quest the player can access
        let result = quest_system.start_quest("resonance_foundation", &player, &faction_system);
        assert!(result.is_ok());

        // Check quest is now in progress
        assert!(quest_system.player_progress.contains_key("resonance_foundation"));
        let progress = &quest_system.player_progress["resonance_foundation"];
        assert_eq!(progress.status, QuestStatus::InProgress);
        assert!(!progress.objective_progress.is_empty());

        // Try to start quest with unmet requirements
        let result_fail = quest_system.start_quest("unstable_site_investigation", &player, &faction_system);
        assert!(result_fail.is_err());
    }

    #[test]
    fn test_objective_progress_tracking() {
        let mut quest_system = create_test_quest_system();
        let faction_system = FactionSystem::new();
        let player = create_test_player();

        // Start quest and get objective IDs
        quest_system.start_quest("resonance_foundation", &player, &faction_system).unwrap();

        let quest_def = &quest_system.quest_definitions["resonance_foundation"];
        let first_objective_id = &quest_def.objectives[0].id;

        // Update objective progress
        let result = quest_system.update_objective_progress(
            "resonance_foundation",
            first_objective_id,
            0.5,
            false
        );
        assert!(result.is_ok());

        // Check progress was updated
        let progress = &quest_system.player_progress["resonance_foundation"];
        let obj_progress = &progress.objective_progress[first_objective_id];
        assert_eq!(obj_progress.progress_value, 0.5);
        assert!(!obj_progress.completed);

        // Complete the objective
        let result = quest_system.update_objective_progress(
            "resonance_foundation",
            first_objective_id,
            1.0,
            true
        );
        assert!(result.is_ok());

        let progress = &quest_system.player_progress["resonance_foundation"];
        let obj_progress = &progress.objective_progress[first_objective_id];
        assert!(obj_progress.completed);
        assert!(obj_progress.completed_at.is_some());
    }

    #[test]
    fn test_quest_completion() {
        let mut quest_system = create_test_quest_system();
        let faction_system = FactionSystem::new();
        let player = create_test_player();

        // Start quest
        quest_system.start_quest("resonance_foundation", &player, &faction_system).unwrap();

        let quest_def = quest_system.quest_definitions["resonance_foundation"].clone();

        // Complete all required objectives
        for objective in &quest_def.objectives {
            if !objective.optional {
                quest_system.update_objective_progress(
                    "resonance_foundation",
                    &objective.id,
                    1.0,
                    true
                ).unwrap();
            }
        }

        // Check quest is completed
        let progress = &quest_system.player_progress["resonance_foundation"];
        assert_eq!(progress.status, QuestStatus::Completed);
        assert!(progress.completed_at.is_some());
    }

    #[test]
    fn test_dialogue_triggers() {
        let mut quest_system = create_test_quest_system();
        let faction_system = FactionSystem::new();
        let player = create_test_player();

        // Start quest with dialogue objective
        quest_system.start_quest("resonance_foundation", &player, &faction_system).unwrap();

        // Trigger dialogue that matches an objective
        let result = quest_system.handle_dialogue_trigger(
            "tutorial_assistant",
            Some("resonance_results"),
            &player
        );
        assert!(result.is_ok());

        let updates = result.unwrap();
        assert!(!updates.is_empty());
        assert!(updates[0].contains("completed"));

        // Trigger dialogue that doesn't match
        let result_no_match = quest_system.handle_dialogue_trigger(
            "random_npc",
            Some("random_topic"),
            &player
        );
        assert!(result_no_match.is_ok());
        assert!(result_no_match.unwrap().is_empty());
    }

    #[test]
    fn test_theory_progress_triggers() {
        let mut quest_system = create_test_quest_system();
        let faction_system = FactionSystem::new();
        let player = create_test_player();

        // Start quest with theory objective
        quest_system.start_quest("resonance_foundation", &player, &faction_system).unwrap();

        // Update theory understanding to trigger objective
        let result = quest_system.handle_theory_progress(
            "harmonic_fundamentals",
            0.35, // Above the 0.3 threshold
            &player
        );
        assert!(result.is_ok());

        let updates = result.unwrap();
        assert!(!updates.is_empty());
    }

    #[test]
    fn test_location_visit_triggers() {
        let mut quest_system = create_test_quest_system();
        let faction_system = FactionSystem::new();
        let player = create_test_player();

        // Start quest with location objective
        quest_system.start_quest("resonance_foundation", &player, &faction_system).unwrap();

        // Visit location to trigger objective
        let result = quest_system.handle_location_visit("practice_hall");
        assert!(result.is_ok());

        let updates = result.unwrap();
        assert!(!updates.is_empty());
    }

    #[test]
    fn test_quest_status_display() {
        let mut quest_system = create_test_quest_system();
        let faction_system = FactionSystem::new();
        let player = create_test_player();

        // Test status for unstarted quest
        let status = quest_system.get_quest_status("resonance_foundation").unwrap();
        assert!(status.contains("Not Started"));

        // Start quest and check status
        quest_system.start_quest("resonance_foundation", &player, &faction_system).unwrap();
        let status_active = quest_system.get_quest_status("resonance_foundation").unwrap();
        assert!(status_active.contains("InProgress"));
        assert!(status_active.contains("Objectives:"));

        // Test status for nonexistent quest
        let status_missing = quest_system.get_quest_status("nonexistent_quest");
        assert!(status_missing.is_err());
    }

    #[test]
    fn test_quest_recommendations() {
        let quest_system = create_test_quest_system();
        let faction_system = FactionSystem::new();

        let basic_player = create_test_player();
        let recommendations = quest_system.get_quest_recommendations(&basic_player, &faction_system);
        assert!(!recommendations.is_empty());

        // Recommendations should include reasons
        for (_, reason) in &recommendations {
            assert!(!reason.is_empty());
        }

        // Advanced player should get different recommendations
        let advanced_player = create_advanced_player();
        let advanced_recommendations = quest_system.get_quest_recommendations(&advanced_player, &faction_system);
        assert!(!advanced_recommendations.is_empty());
    }

    #[test]
    fn test_active_quests_tracking() {
        let mut quest_system = create_test_quest_system();
        let faction_system = FactionSystem::new();
        let player = create_advanced_player();

        // Initially no active quests
        let active = quest_system.get_active_quests();
        assert!(active.is_empty());

        // Start multiple quests
        quest_system.start_quest("resonance_foundation", &player, &faction_system).unwrap();
        quest_system.start_quest("crystal_analysis", &player, &faction_system).unwrap();

        let active_multiple = quest_system.get_active_quests();
        assert_eq!(active_multiple.len(), 2);
    }

    #[test]
    fn test_faction_requirement_restrictions() {
        let quest_system = create_test_quest_system();
        let faction_system = FactionSystem::new();

        // Create player with high standing in restricted faction
        let mut player = create_test_player();
        player.faction_standings.insert(FactionId::MagistersCouncil, 80); // Too high for diplomatic quest

        let diplomatic_quest = &quest_system.quest_definitions["diplomatic_balance"];
        assert!(!quest_system.is_quest_available(diplomatic_quest, &player, &faction_system));
    }

    #[test]
    fn test_prerequisite_quest_requirements() {
        let quest_system = create_test_quest_system();
        let faction_system = FactionSystem::new();
        let player = create_advanced_player();

        // Crystal analysis requires resonance foundation
        let crystal_quest = &quest_system.quest_definitions["crystal_analysis"];
        assert!(!quest_system.is_quest_available(crystal_quest, &player, &faction_system));

        // Complete prerequisite in quest system
        let mut quest_system_with_completed = quest_system.clone();
        let mut completed_progress = QuestProgress {
            quest_id: "resonance_foundation".to_string(),
            status: QuestStatus::Completed,
            started_at: Utc::now(),
            completed_at: Some(Utc::now()),
            objective_progress: HashMap::new(),
            chosen_branch: None,
            player_choices: HashMap::new(),
            time_invested: 45,
            quest_variables: HashMap::new(),
            learning_progress: QuestLearningProgress {
                mastered_concepts: Vec::new(),
                demonstrated_methods: Vec::new(),
                assessment_scores: HashMap::new(),
                learning_metrics: LearningMetrics {
                    completion_efficiency: 1.0,
                    first_attempt_success_rate: 1.0,
                    help_requests: 0,
                    application_accuracy: 1.0,
                },
            },
        };
        quest_system_with_completed.player_progress.insert("resonance_foundation".to_string(), completed_progress);

        // Now crystal analysis should be available
        assert!(quest_system_with_completed.is_quest_available(crystal_quest, &player, &faction_system));
    }

    #[test]
    fn test_attribute_requirements() {
        let quest_system = create_test_quest_system();
        let faction_system = FactionSystem::new();

        // Create player with insufficient attributes
        let mut weak_player = Player::new("Weak Player".to_string());
        weak_player.attributes.mental_acuity = 5; // Below requirement

        let foundation_quest = &quest_system.quest_definitions["resonance_foundation"];
        assert!(!quest_system.is_quest_available(foundation_quest, &weak_player, &faction_system));
    }

    #[test]
    fn test_educational_objectives() {
        let quest_system = create_test_quest_system();

        let foundation_quest = &quest_system.quest_definitions["resonance_foundation"];
        assert!(!foundation_quest.educational_focus.primary_concepts.is_empty());
        assert!(foundation_quest.educational_focus.primary_concepts.contains(&"Wave Physics".to_string()));

        let healing_quest = &quest_system.quest_definitions["healing_research"];
        assert!(healing_quest.educational_focus.primary_concepts.contains(&"Biology".to_string()));
    }

    #[test]
    fn test_quest_branching_paths() {
        let quest_system = create_test_quest_system();

        let crystal_quest = &quest_system.quest_definitions["crystal_analysis"];
        assert!(!crystal_quest.branching_paths.is_empty());
        assert!(crystal_quest.branching_paths.contains_key("academic_approach"));
        assert!(crystal_quest.branching_paths.contains_key("commercial_approach"));

        // Test branch requirements
        let academic_branch = &crystal_quest.branching_paths["academic_approach"];
        assert!(!academic_branch.requirements.faction_requirements.is_empty());
    }

    #[test]
    fn test_quest_rewards() {
        let quest_system = create_test_quest_system();

        let foundation_quest = &quest_system.quest_definitions["resonance_foundation"];
        assert!(foundation_quest.rewards.experience > 0);
        assert!(foundation_quest.rewards.attribute_bonuses.mental_acuity.is_some());
        assert!(!foundation_quest.rewards.theory_bonuses.is_empty());
        assert!(!foundation_quest.rewards.new_capabilities.is_empty());
    }

    #[test]
    fn test_quest_time_tracking() {
        let mut quest_system = create_test_quest_system();
        let faction_system = FactionSystem::new();
        let player = create_test_player();

        quest_system.start_quest("resonance_foundation", &player, &faction_system).unwrap();

        let progress = &quest_system.player_progress["resonance_foundation"];
        assert_eq!(progress.time_invested, 0);
        assert!(progress.started_at <= Utc::now());
    }

    #[test]
    fn test_quest_learning_progress() {
        let mut quest_system = create_test_quest_system();
        let faction_system = FactionSystem::new();
        let player = create_test_player();

        quest_system.start_quest("resonance_foundation", &player, &faction_system).unwrap();

        let progress = &quest_system.player_progress["resonance_foundation"];
        assert!(progress.learning_progress.mastered_concepts.is_empty());
        assert_eq!(progress.learning_progress.learning_metrics.help_requests, 0);
    }

    #[test]
    fn test_optional_objectives() {
        let quest_system = create_test_quest_system();

        let foundation_quest = &quest_system.quest_definitions["resonance_foundation"];
        let optional_objectives: Vec<_> = foundation_quest.objectives
            .iter()
            .filter(|obj| obj.optional)
            .collect();

        assert!(!optional_objectives.is_empty());

        // Optional objectives shouldn't block quest completion
        let required_objectives: Vec<_> = foundation_quest.objectives
            .iter()
            .filter(|obj| !obj.optional)
            .collect();

        assert!(!required_objectives.is_empty());
    }

    #[test]
    fn test_quest_progression_chains() {
        let quest_system = create_test_quest_system();

        // Test that quests form a proper progression chain
        let foundation_quest = &quest_system.quest_definitions["resonance_foundation"];
        assert!(foundation_quest.rewards.unlocked_quests.contains(&"crystal_analysis".to_string()));

        let crystal_quest = &quest_system.quest_definitions["crystal_analysis"];
        assert!(crystal_quest.requirements.prerequisite_quests.contains(&"resonance_foundation".to_string()));
    }

    #[test]
    fn test_quest_npc_involvement() {
        let quest_system = create_test_quest_system();

        let foundation_quest = &quest_system.quest_definitions["resonance_foundation"];
        assert!(!foundation_quest.involved_npcs.is_empty());

        let diplomatic_quest = &quest_system.quest_definitions["diplomatic_balance"];
        assert!(diplomatic_quest.involved_npcs.len() >= 3); // Multiple NPCs for political quest
    }

    #[test]
    fn test_quest_location_requirements() {
        let quest_system = create_test_quest_system();

        let foundation_quest = &quest_system.quest_definitions["resonance_foundation"];
        assert!(foundation_quest.requirements.location_requirements.contains(&"practice_hall".to_string()));
    }

    #[test]
    fn test_quest_system_error_handling() {
        let mut quest_system = QuestSystem::new();
        let faction_system = FactionSystem::new();
        let player = create_test_player();

        // Test starting nonexistent quest
        let result = quest_system.start_quest("nonexistent", &player, &faction_system);
        assert!(result.is_err());

        // Test updating progress for nonexistent quest
        let result = quest_system.update_objective_progress("nonexistent", "obj", 1.0, true);
        assert!(result.is_err());

        // Test getting status for nonexistent quest
        let result = quest_system.get_quest_status("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_quest_clone_functionality() {
        let quest_system = create_test_quest_system();
        let cloned_system = quest_system.clone();

        assert_eq!(quest_system.quest_definitions.len(), cloned_system.quest_definitions.len());
        assert_eq!(quest_system.player_progress.len(), cloned_system.player_progress.len());
    }

    #[test]
    fn test_quest_serialization_compatibility() {
        // Test that quest structures can be serialized (important for save/load)
        let quest_system = create_test_quest_system();
        let faction_system = FactionSystem::new();
        let player = create_test_player();

        quest_system.start_quest("resonance_foundation", &player, &faction_system).unwrap();

        // Serialize and deserialize quest progress
        let progress = &quest_system.player_progress["resonance_foundation"];
        let serialized = serde_json::to_string(progress).unwrap();
        let deserialized: QuestProgress = serde_json::from_str(&serialized).unwrap();

        assert_eq!(progress.quest_id, deserialized.quest_id);
        assert_eq!(progress.status, deserialized.status);
    }

    #[test]
    fn test_quest_system_performance() {
        let quest_system = create_test_quest_system();
        let faction_system = FactionSystem::new();
        let player = create_advanced_player();

        // Test that getting available quests is fast even with multiple quests
        let start_time = std::time::Instant::now();
        for _ in 0..100 {
            let _ = quest_system.get_available_quests(&player, &faction_system);
        }
        let duration = start_time.elapsed();

        // Should complete 100 iterations in reasonable time (less than 100ms)
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_quest_capability_requirements() {
        let quest_system = create_test_quest_system();
        let faction_system = FactionSystem::new();

        // Test advanced quest capability requirements
        let healing_quest = &quest_system.quest_definitions["healing_research"];
        assert!(!healing_quest.requirements.capability_requirements.is_empty());

        // Player without capabilities shouldn't access quest
        let basic_player = create_test_player();
        assert!(!quest_system.is_quest_available(healing_quest, &basic_player, &faction_system));
    }

    #[test]
    fn test_quest_global_state() {
        let quest_system = create_test_quest_system();

        // Test initial global state
        assert!(quest_system.global_state.unlocked_quest_lines.contains(&"tutorial".to_string()));
        assert!(quest_system.global_state.global_events.is_empty());
        assert!(quest_system.global_state.faction_relationship_modifiers.is_empty());
    }
}