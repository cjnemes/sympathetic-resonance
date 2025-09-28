//! Performance benchmarks for Sympathetic Resonance game
//!
//! These benchmarks validate that all critical operations meet the <100ms response time requirement

use std::time::{Duration, Instant};
use std::collections::HashMap;
use sympathetic_resonance::*;
use sympathetic_resonance::core::{Player, WorldState};
use sympathetic_resonance::systems::magic::{MagicSystem, MagicAttempt};
use sympathetic_resonance::systems::factions::FactionSystem;
use sympathetic_resonance::systems::dialogue::DialogueSystem;
use sympathetic_resonance::persistence::DatabaseManager;
use sympathetic_resonance::input::command_parser::CommandParser;
use sympathetic_resonance::input::command_handlers::execute_command;
use sympathetic_resonance::core::player::{Crystal, CrystalType, CrystalSize};
use tempfile::NamedTempFile;

/// Target response time for all commands (100ms)
const TARGET_RESPONSE_TIME: Duration = Duration::from_millis(100);

/// Number of iterations for each benchmark
const BENCHMARK_ITERATIONS: usize = 50;

/// Performance test results
#[derive(Debug)]
pub struct BenchmarkResult {
    pub operation: String,
    pub avg_duration: Duration,
    pub max_duration: Duration,
    pub min_duration: Duration,
    pub meets_requirement: bool,
    pub iterations: usize,
}

impl BenchmarkResult {
    pub fn new(operation: String, durations: Vec<Duration>) -> Self {
        let avg_duration = Duration::from_nanos(
            durations.iter().map(|d| d.as_nanos()).sum::<u128>() / durations.len() as u128
        );
        let max_duration = *durations.iter().max().unwrap();
        let min_duration = *durations.iter().min().unwrap();
        let meets_requirement = avg_duration <= TARGET_RESPONSE_TIME;

        Self {
            operation,
            avg_duration,
            max_duration,
            min_duration,
            meets_requirement,
            iterations: durations.len(),
        }
    }
}

/// Create test environment with realistic data
fn create_test_environment() -> (Player, WorldState, DatabaseManager, MagicSystem, DialogueSystem, FactionSystem) {
    // Create temporary database
    let temp_file = NamedTempFile::new().unwrap();
    let db_path = temp_file.path().to_str().unwrap();
    let mut database = DatabaseManager::new(db_path).unwrap();
    database.initialize_schema().unwrap();
    database.load_default_content().unwrap();

    // Create player with realistic state
    let mut player = Player::new("Test Player".to_string());

    // Add multiple crystals
    let crystals = vec![
        Crystal::new(CrystalType::Quartz, 90.0, 0.8, CrystalSize::Medium),
        Crystal::new(CrystalType::Amethyst, 85.0, 0.9, CrystalSize::Large),
        Crystal::new(CrystalType::Garnet, 95.0, 0.7, CrystalSize::Small),
        Crystal::new(CrystalType::Obsidian, 88.0, 0.85, CrystalSize::Medium),
    ];

    player.inventory.crystals = crystals;
    player.inventory.active_crystal = Some(0);

    // Set realistic attribute levels
    player.attributes.mental_acuity = 35;
    player.attributes.resonance_sensitivity = 45;
    player.mental_state.current_energy = 80;
    player.mental_state.max_energy = 100;
    player.mental_state.fatigue = 20;

    // Create world with multiple locations
    let mut world = WorldState::new();
    let locations = database.load_locations().unwrap();
    for (id, location) in locations {
        world.locations.insert(id, location);
    }

    let magic_system = MagicSystem::new();
    let dialogue_system = DialogueSystem::new();
    let faction_system = FactionSystem::new();

    // Leak the temp file to prevent deletion during benchmarks
    std::mem::forget(temp_file);

    (player, world, database, magic_system, dialogue_system, faction_system)
}

/// Benchmark command parsing performance
fn benchmark_command_parsing() -> BenchmarkResult {
    let parser = CommandParser::new();
    let commands = vec![
        "cast light",
        "move north",
        "examine crystal",
        "talk to guard about magic",
        "inventory",
        "status",
        "meditate",
        "study harmonic fundamentals",
        "look around",
        "cast healing on self",
    ];

    let mut durations = Vec::new();

    for _ in 0..BENCHMARK_ITERATIONS {
        let start = Instant::now();

        // Parse multiple commands to simulate realistic load
        for command in &commands {
            let _ = parser.parse(command);
        }

        durations.push(start.elapsed());
    }

    BenchmarkResult::new("Command Parsing (10 commands)".to_string(), durations)
}

/// Benchmark magic calculation performance
fn benchmark_magic_calculations() -> BenchmarkResult {
    let (mut player, mut world, _database, mut magic_system, _dialogue, _faction) = create_test_environment();

    let spells = vec!["light", "healing", "detection", "manipulation", "communication"];
    let mut durations = Vec::new();

    for _ in 0..BENCHMARK_ITERATIONS {
        let start = Instant::now();

        // Test multiple magic types
        for spell in &spells {
            let _ = magic_system.attempt_magic(spell, &mut player, &mut world, None);
        }

        durations.push(start.elapsed());
    }

    BenchmarkResult::new("Magic Calculations (5 spells)".to_string(), durations)
}

/// Benchmark database operations
fn benchmark_database_operations() -> BenchmarkResult {
    let temp_file = NamedTempFile::new().unwrap();
    let db_path = temp_file.path().to_str().unwrap();

    let mut durations = Vec::new();

    for _ in 0..BENCHMARK_ITERATIONS {
        let start = Instant::now();

        let mut database = DatabaseManager::new(db_path).unwrap();
        database.initialize_schema().unwrap();
        database.load_default_content().unwrap();

        // Load all data
        let _ = database.load_locations().unwrap();
        let _ = database.load_theories().unwrap();

        durations.push(start.elapsed());
    }

    std::mem::forget(temp_file);
    BenchmarkResult::new("Database Operations (full load)".to_string(), durations)
}

/// Benchmark faction system calculations
fn benchmark_faction_calculations() -> BenchmarkResult {
    let mut faction_system = FactionSystem::new();
    let mut durations = Vec::new();

    for _ in 0..BENCHMARK_ITERATIONS {
        let start = Instant::now();

        // Perform complex faction calculations
        let _ = faction_system.get_faction_influence_summary();
        let _ = faction_system.calculate_cross_faction_effects();

        // Simulate reputation changes
        use sympathetic_resonance::systems::factions::FactionId;
        faction_system.adjust_reputation(FactionId::MagistersCouncil, 5);
        faction_system.adjust_reputation(FactionId::UndergroundNetwork, -3);

        durations.push(start.elapsed());
    }

    BenchmarkResult::new("Faction Calculations".to_string(), durations)
}

/// Benchmark world state management
fn benchmark_world_state_operations() -> BenchmarkResult {
    let (mut player, mut world, _database, _magic, _dialogue, _faction) = create_test_environment();

    let mut durations = Vec::new();

    for _ in 0..BENCHMARK_ITERATIONS {
        let start = Instant::now();

        // Perform world state operations
        world.advance_time(30);
        let _ = world.calculate_magical_modifier(4);
        world.add_magical_signature("test_magic".to_string(), 0.8, 4);
        let _ = world.available_exits();

        // Test movement
        if let Some(location) = world.current_location() {
            for (direction, _) in location.exits.clone() {
                let _ = world.move_to_location(direction);
                break; // Only test one movement
            }
        }

        durations.push(start.elapsed());
    }

    BenchmarkResult::new("World State Operations".to_string(), durations)
}

/// Benchmark complete command execution pipeline
fn benchmark_complete_command_execution() -> BenchmarkResult {
    let (mut player, mut world, database, mut magic_system, mut dialogue_system, faction_system) = create_test_environment();
    let parser = CommandParser::new();

    let commands = vec![
        "look",
        "status",
        "inventory",
        "cast light",
        "examine crystal",
    ];

    let mut durations = Vec::new();

    for _ in 0..BENCHMARK_ITERATIONS {
        let start = Instant::now();

        for command_str in &commands {
            if let Ok(command) = parser.parse(command_str) {
                let _ = execute_command(
                    command,
                    &mut player,
                    &mut world,
                    &database,
                    &mut magic_system,
                    &mut dialogue_system,
                    &faction_system,
                );
            }
        }

        durations.push(start.elapsed());
    }

    BenchmarkResult::new("Complete Command Pipeline (5 commands)".to_string(), durations)
}

/// Benchmark memory-intensive operations
fn benchmark_memory_operations() -> BenchmarkResult {
    let mut durations = Vec::new();

    for _ in 0..BENCHMARK_ITERATIONS {
        let start = Instant::now();

        // Create multiple complex objects to test memory allocation patterns
        let _large_world = create_large_world_state();
        let _players: Vec<Player> = (0..50).map(|i| Player::new(format!("Player {}", i))).collect();
        let _hash_maps: Vec<HashMap<String, String>> = (0..20)
            .map(|_| {
                let mut map = HashMap::new();
                for j in 0..100 {
                    map.insert(format!("key_{}", j), format!("value_{}", j));
                }
                map
            })
            .collect();

        durations.push(start.elapsed());
    }

    BenchmarkResult::new("Memory Allocation Operations".to_string(), durations)
}

/// Create a large world state for memory testing
fn create_large_world_state() -> WorldState {
    let mut world = WorldState::new();

    // Add many locations
    for i in 0..100 {
        let mut location = sympathetic_resonance::core::world_state::Location::new(
            format!("location_{}", i),
            format!("Location {}", i),
            format!("This is test location number {} with detailed description text.", i),
        );

        location.magical_properties.ambient_energy = 1.0 + (i as f32 * 0.01);
        location.magical_properties.dominant_frequency = Some((i % 10) + 1);

        world.add_location(location);
    }

    world
}

/// Run all benchmarks and return results
pub fn run_all_benchmarks() -> Vec<BenchmarkResult> {
    println!("Running performance benchmarks...");

    let benchmarks = vec![
        benchmark_command_parsing,
        benchmark_magic_calculations,
        benchmark_database_operations,
        benchmark_faction_calculations,
        benchmark_world_state_operations,
        benchmark_complete_command_execution,
        benchmark_memory_operations,
    ];

    let mut results = Vec::new();

    for (i, benchmark) in benchmarks.iter().enumerate() {
        println!("Running benchmark {} of {}...", i + 1, benchmarks.len());
        results.push(benchmark());
    }

    results
}

/// Generate performance report
pub fn generate_performance_report(results: &[BenchmarkResult]) -> String {
    let mut report = String::new();

    report.push_str("=== PERFORMANCE BENCHMARK RESULTS ===\n\n");
    report.push_str(&format!("Target Response Time: {}ms\n", TARGET_RESPONSE_TIME.as_millis()));
    report.push_str(&format!("Benchmark Iterations: {}\n\n", BENCHMARK_ITERATIONS));

    let mut all_pass = true;

    for result in results {
        let status = if result.meets_requirement { "✓ PASS" } else { "✗ FAIL" };
        if !result.meets_requirement {
            all_pass = false;
        }

        report.push_str(&format!(
            "{} | {}\n\
             ├─ Average: {:.2}ms\n\
             ├─ Maximum: {:.2}ms\n\
             ├─ Minimum: {:.2}ms\n\
             └─ Iterations: {}\n\n",
            status,
            result.operation,
            result.avg_duration.as_secs_f64() * 1000.0,
            result.max_duration.as_secs_f64() * 1000.0,
            result.min_duration.as_secs_f64() * 1000.0,
            result.iterations
        ));
    }

    report.push_str("=== SUMMARY ===\n");
    let passed = results.iter().filter(|r| r.meets_requirement).count();
    let total = results.len();

    report.push_str(&format!(
        "Overall Status: {}\n\
         Benchmarks Passed: {}/{}\n\
         Success Rate: {:.1}%\n",
        if all_pass { "✓ ALL TESTS PASS" } else { "✗ SOME TESTS FAIL" },
        passed,
        total,
        (passed as f64 / total as f64) * 100.0
    ));

    if !all_pass {
        report.push_str("\nFAILED BENCHMARKS:\n");
        for result in results.iter().filter(|r| !r.meets_requirement) {
            report.push_str(&format!(
                "• {}: {:.2}ms (exceeds target by {:.2}ms)\n",
                result.operation,
                result.avg_duration.as_secs_f64() * 1000.0,
                (result.avg_duration.as_secs_f64() - TARGET_RESPONSE_TIME.as_secs_f64()) * 1000.0
            ));
        }
    }

    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_command_benchmark() {
        let result = benchmark_command_parsing();
        assert!(result.iterations > 0);
        assert!(result.avg_duration.as_millis() < 1000); // Should be much faster than 1 second
    }

    #[test]
    fn test_magic_calculation_benchmark() {
        let result = benchmark_magic_calculations();
        assert!(result.iterations > 0);
        // Magic calculations should be fast
        assert!(result.avg_duration.as_millis() < 500);
    }

    #[test]
    fn test_database_benchmark() {
        let result = benchmark_database_operations();
        assert!(result.iterations > 0);
        // Database operations can be slower but should be reasonable
        assert!(result.avg_duration.as_millis() < 1000);
    }
}