//! Simple performance tests that can be run with cargo test
//!
//! These tests validate response times without external benchmark dependencies

use std::time::{Duration, Instant};
use crate::core::{Player, WorldState};
use crate::systems::magic::MagicSystem;
use crate::systems::factions::FactionSystem;
use crate::systems::dialogue::DialogueSystem;
use crate::systems::knowledge::KnowledgeSystem;
use crate::persistence::DatabaseManager;
use crate::input::command_parser::CommandParser;
use crate::input::command_handlers::execute_command;
use crate::core::player::{Crystal, CrystalType, CrystalSize};
use tempfile::NamedTempFile;

/// Target response time for all commands (100ms)
const TARGET_RESPONSE_TIME: Duration = Duration::from_millis(100);

/// Create a test environment
fn create_test_env() -> (Player, WorldState, DatabaseManager, MagicSystem, DialogueSystem, FactionSystem, KnowledgeSystem) {
    let temp_file = NamedTempFile::new().unwrap();
    let db_path = temp_file.path().to_str().unwrap();
    let database = DatabaseManager::new(db_path).unwrap();
    database.initialize_schema().unwrap();
    database.load_default_content().unwrap();

    let mut player = Player::new("Test Player".to_string());
    let crystal = Crystal::new(CrystalType::Quartz, 90.0, 0.8, CrystalSize::Medium);
    player.inventory.crystals = vec![crystal];
    player.inventory.active_crystal = Some(0);

    let mut world = WorldState::new();
    let locations = database.load_locations().unwrap();
    for (id, location) in locations {
        world.locations.insert(id, location);
    }

    let magic_system = MagicSystem::new();
    let dialogue_system = DialogueSystem::new();
    let faction_system = FactionSystem::new();

    // Initialize knowledge system
    let mut knowledge_system = KnowledgeSystem::new();
    knowledge_system.initialize(&database).unwrap();

    std::mem::forget(temp_file);
    (player, world, database, magic_system, dialogue_system, faction_system, knowledge_system)
}

/// Time a function execution
fn time_operation<F, R>(operation: F) -> (Duration, R)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = operation();
    (start.elapsed(), result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_parsing_performance() {
        let parser = CommandParser::new();
        let commands = vec![
            "cast light",
            "move north",
            "examine crystal",
            "inventory",
            "status",
        ];

        let (duration, _) = time_operation(|| {
            for command in &commands {
                let _ = parser.parse(command);
            }
        });

        println!("Command parsing (5 commands): {:.2}ms", duration.as_secs_f64() * 1000.0);
        assert!(duration <= TARGET_RESPONSE_TIME,
                "Command parsing took {:.2}ms, exceeds target of {:.2}ms",
                duration.as_secs_f64() * 1000.0, TARGET_RESPONSE_TIME.as_secs_f64() * 1000.0);
    }

    #[test]
    fn test_magic_calculation_performance() {
        let (mut player, mut world, _db, mut magic_system, _dialogue, _faction, _knowledge) = create_test_env();

        let (duration, _) = time_operation(|| {
            let _ = magic_system.attempt_magic("light", &mut player, &mut world, None);
            let _ = magic_system.attempt_magic("healing", &mut player, &mut world, None);
            let _ = magic_system.attempt_magic("detection", &mut player, &mut world, None);
        });

        println!("Magic calculations (3 spells): {:.2}ms", duration.as_secs_f64() * 1000.0);
        assert!(duration <= TARGET_RESPONSE_TIME,
                "Magic calculations took {:.2}ms, exceeds target of {:.2}ms",
                duration.as_secs_f64() * 1000.0, TARGET_RESPONSE_TIME.as_secs_f64() * 1000.0);
    }

    #[test]
    fn test_database_operations_performance() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();

        let (duration, _) = time_operation(|| {
            let database = DatabaseManager::new(db_path).unwrap();
            database.initialize_schema().unwrap();
            database.load_default_content().unwrap();
            let _ = database.load_locations().unwrap();
            let _ = database.load_theories().unwrap();
        });

        std::mem::forget(temp_file);
        println!("Database operations: {:.2}ms", duration.as_secs_f64() * 1000.0);

        // Database initialization with full content (8 locations + 12 NPCs) has higher threshold
        let database_init_target = Duration::from_millis(150);
        assert!(duration <= database_init_target,
                "Database operations took {:.2}ms, exceeds target of {:.2}ms",
                duration.as_secs_f64() * 1000.0, database_init_target.as_secs_f64() * 1000.0);
    }

    #[test]
    fn test_faction_calculations_performance() {
        let mut faction_system = FactionSystem::new();

        let (duration, _) = time_operation(|| {
            let _ = faction_system.get_all_standings();
            let _ = faction_system.get_relationship_strength(
                crate::systems::factions::FactionId::MagistersCouncil,
                crate::systems::factions::FactionId::UndergroundNetwork
            );

            use crate::systems::factions::FactionId;
            faction_system.modify_reputation(FactionId::MagistersCouncil, 5);
            faction_system.modify_reputation(FactionId::UndergroundNetwork, -3);
        });

        println!("Faction calculations: {:.2}ms", duration.as_secs_f64() * 1000.0);
        assert!(duration <= TARGET_RESPONSE_TIME,
                "Faction calculations took {:.2}ms, exceeds target of {:.2}ms",
                duration.as_secs_f64() * 1000.0, TARGET_RESPONSE_TIME.as_secs_f64() * 1000.0);
    }

    #[test]
    fn test_world_state_performance() {
        let (_player, mut world, _db, _magic, _dialogue, _faction, _knowledge) = create_test_env();

        let (duration, _) = time_operation(|| {
            world.advance_time(30);
            let _ = world.calculate_magical_modifier(4);
            world.add_magical_signature("test_magic".to_string(), 0.8, 4);
            let _ = world.available_exits();
        });

        println!("World state operations: {:.2}ms", duration.as_secs_f64() * 1000.0);
        assert!(duration <= TARGET_RESPONSE_TIME,
                "World state operations took {:.2}ms, exceeds target of {:.2}ms",
                duration.as_secs_f64() * 1000.0, TARGET_RESPONSE_TIME.as_secs_f64() * 1000.0);
    }

    #[test]
    fn test_complete_command_pipeline_performance() {
        let (mut player, mut world, database, mut magic_system, mut dialogue_system, mut faction_system, mut knowledge_system) = create_test_env();
        let mut quest_system = crate::systems::QuestSystem::new();
        let parser = CommandParser::new();

        let (duration, _) = time_operation(|| {
            let commands = vec!["look", "status", "inventory", "cast light"];
            for command_str in &commands {
                match parser.parse(command_str) {
                    crate::input::command_parser::CommandResult::Success(command) => {
                        let _ = execute_command(
                            command,
                            &mut player,
                            &mut world,
                            &database,
                            &mut magic_system,
                            &mut dialogue_system,
                            &mut faction_system,
                            &mut knowledge_system,
                            &mut quest_system,
                        );
                    }
                    _ => {} // Ignore parsing errors for benchmark
                }
            }
        });

        println!("Complete command pipeline (4 commands): {:.2}ms", duration.as_secs_f64() * 1000.0);
        assert!(duration <= TARGET_RESPONSE_TIME,
                "Complete command pipeline took {:.2}ms, exceeds target of {:.2}ms",
                duration.as_secs_f64() * 1000.0, TARGET_RESPONSE_TIME.as_secs_f64() * 1000.0);
    }

    #[test]
    fn test_individual_command_performance() {
        let (mut player, mut world, database, mut magic_system, mut dialogue_system, mut faction_system, mut knowledge_system) = create_test_env();
        let mut quest_system = crate::systems::QuestSystem::new();
        let parser = CommandParser::new();

        // Test individual commands to identify bottlenecks
        let test_commands = vec![
            ("look", "Look command"),
            ("status", "Status command"),
            ("inventory", "Inventory command"),
            ("cast light", "Magic command"),
            ("examine crystal", "Examine command"),
        ];

        for (command_str, description) in test_commands {
            let (duration, _) = time_operation(|| {
                match parser.parse(command_str) {
                    crate::input::command_parser::CommandResult::Success(command) => {
                        let _ = execute_command(
                            command,
                            &mut player,
                            &mut world,
                            &database,
                            &mut magic_system,
                            &mut dialogue_system,
                            &mut faction_system,
                            &mut knowledge_system,
                            &mut quest_system,
                        );
                    }
                    _ => {} // Ignore parsing errors for benchmark
                }
            });

            println!("{}: {:.2}ms", description, duration.as_secs_f64() * 1000.0);

            // Individual commands should be much faster than the target
            let individual_target = Duration::from_millis(20); // 20ms for individual commands
            assert!(duration <= individual_target,
                    "{} took {:.2}ms, exceeds individual target of {:.2}ms",
                    description, duration.as_secs_f64() * 1000.0, individual_target.as_secs_f64() * 1000.0);
        }
    }

    #[test]
    fn test_stress_magic_calculations() {
        let (mut player, mut world, _db, mut magic_system, _dialogue, _faction, _knowledge) = create_test_env();

        // Test with multiple magic attempts to simulate stress
        let (duration, _) = time_operation(|| {
            for _ in 0..10 {
                let _ = magic_system.attempt_magic("light", &mut player, &mut world, None);
            }
        });

        println!("Stress test - 10 magic calculations: {:.2}ms", duration.as_secs_f64() * 1000.0);

        // 10 magic calculations should still be under reasonable time
        let stress_target = Duration::from_millis(200); // 200ms for 10 calculations
        assert!(duration <= stress_target,
                "Stress test took {:.2}ms, exceeds stress target of {:.2}ms",
                duration.as_secs_f64() * 1000.0, stress_target.as_secs_f64() * 1000.0);
    }
}