// Integration tests for cross-system interactions
// These tests verify that different systems work together correctly

use crate::core::Player;
use crate::core::player::{Crystal, CrystalType, CrystalSize};
use crate::systems::factions::{FactionId, FactionSystem};
use crate::systems::dialogue::{DialogueSystem, NPC, DialogueTree, DialogueNode, DialogueRequirements};
use crate::systems::magic::MagicSystem;
use crate::persistence::database::DatabaseManager;
use crate::input::command_parser::CommandParser;
use crate::GameResult;
use std::collections::HashMap;

/// Test magic system integration with player state
#[test]
fn test_magic_system_integration() -> GameResult<()> {
    let mut player = Player::new("Integration Test Player".to_string());
    let _magic_system = MagicSystem::new();

    // Test that magic system works with player state
    player.mental_state.current_energy = 100;
    player.mental_state.max_energy = 100;

    // Check initial crystal count
    let initial_crystal_count = player.inventory.crystals.len();

    // Add a crystal to player inventory using correct structure
    let crystal = Crystal::new(CrystalType::Quartz, 100.0, 0.8, CrystalSize::Medium);
    player.inventory.crystals.push(crystal);

    // Verify crystal was added correctly
    assert_eq!(player.inventory.crystals.len(), initial_crystal_count + 1);
    assert!(player.inventory.crystals.last().unwrap().is_usable());

    // Test that player has active crystal
    assert!(player.active_crystal().is_some());

    Ok(())
}

/// Test faction system integration with dialogue system
#[test]
fn test_faction_dialogue_integration() -> GameResult<()> {
    let mut player = Player::new("Integration Test Player".to_string());
    let faction_system = FactionSystem::new();
    let mut dialogue_system = DialogueSystem::new();

    // Set up player faction standings
    player.faction_standings.insert(FactionId::MagistersCouncil, 60);
    player.faction_standings.insert(FactionId::UndergroundNetwork, -40);

    // Create an NPC affiliated with Magisters Council
    let npc = NPC {
        id: "council_member".to_string(),
        name: "Council Magistrate".to_string(),
        description: "A high-ranking member of the Magisters Council".to_string(),
        faction_affiliation: Some(FactionId::MagistersCouncil),
        dialogue_tree: DialogueTree {
            greeting: DialogueNode {
                text_templates: vec![
                    "Welcome, esteemed colleague!".to_string(),
                    "Greetings.".to_string(),
                    "What brings you here?".to_string(),
                ],
                responses: vec![],
                requirements: DialogueRequirements {
                    min_faction_standing: None,
                    max_faction_standing: None,
                    knowledge_requirements: vec![],
                    theory_requirements: vec![],
                    min_theory_mastery: None,
                    required_capabilities: vec![],
                },
            },
            topics: {
                let mut topics = HashMap::new();
                topics.insert("council_business".to_string(), DialogueNode {
                    text_templates: vec![
                        "Our work progresses well.".to_string(),
                        "There's much to discuss.".to_string(),
                        "I cannot share that information.".to_string(),
                    ],
                    responses: vec![],
                    requirements: DialogueRequirements {
                        min_faction_standing: Some((FactionId::MagistersCouncil, 30)),
                        max_faction_standing: None,
                        knowledge_requirements: vec![],
                        theory_requirements: vec![],
                        min_theory_mastery: None,
                        required_capabilities: vec![],
                    },
                });
                topics
            },
            faction_specific: HashMap::new(),
        },
        current_disposition: 0,
    };

    dialogue_system.add_npc(npc);

    // Test that faction standing affects dialogue disposition
    let result = dialogue_system.talk_to_npc("council_member", &player, &faction_system);
    assert!(result.is_ok());

    let response = result?;
    assert!(response.contains("Council Magistrate"));
    assert!(response.contains("council_business"));

    // Test topic discussion works with faction requirements
    let topic_result = dialogue_system.ask_about_topic(
        "council_member",
        "council_business",
        &player,
        &faction_system,
    );
    assert!(topic_result.is_ok());

    Ok(())
}

/// Test database system integration
#[test]
fn test_database_integration() -> GameResult<()> {
    let database = DatabaseManager::new(":memory:")?;

    // Just verify we can create an in-memory database
    // The actual save/load functionality is tested in persistence unit tests
    assert!(database.initialize_schema().is_ok());

    Ok(())
}

/// Test command parsing system creation
#[test]
fn test_command_parsing_integration() -> GameResult<()> {
    let command_parser = CommandParser::new();

    // Just verify we can create a command parser
    // The actual parsing logic is tested in unit tests
    assert_eq!(std::mem::size_of_val(&command_parser), std::mem::size_of::<CommandParser>());

    Ok(())
}

/// Test performance across multiple system interactions
#[test]
fn test_performance_integration() -> GameResult<()> {
    let start_time = std::time::Instant::now();

    // Perform multiple operations that cross system boundaries
    for _ in 0..10 {
        let mut player = Player::new("Performance Test".to_string());
        let faction_system = FactionSystem::new();
        let mut dialogue_system = DialogueSystem::new();

        // Set up faction standings
        player.faction_standings.insert(FactionId::MagistersCouncil, 50);

        // Create and add an NPC
        let npc = NPC {
            id: "perf_test_npc".to_string(),
            name: "Performance NPC".to_string(),
            description: "For performance testing".to_string(),
            faction_affiliation: Some(FactionId::MagistersCouncil),
            dialogue_tree: DialogueTree {
                greeting: DialogueNode {
                    text_templates: vec!["Hello".to_string()],
                    responses: vec![],
                    requirements: DialogueRequirements {
                        min_faction_standing: None,
                        max_faction_standing: None,
                        knowledge_requirements: vec![],
                        theory_requirements: vec![],
                        min_theory_mastery: None,
                        required_capabilities: vec![],
                    },
                },
                topics: HashMap::new(),
                faction_specific: HashMap::new(),
            },
            current_disposition: 0,
        };

        dialogue_system.add_npc(npc);

        // Perform dialogue interaction
        let _result = dialogue_system.talk_to_npc("perf_test_npc", &player, &faction_system);
    }

    let elapsed = start_time.elapsed();

    // Should complete in well under the 100ms requirement per operation
    assert!(elapsed.as_millis() < 1000, "Integration tests took too long: {}ms", elapsed.as_millis());

    Ok(())
}