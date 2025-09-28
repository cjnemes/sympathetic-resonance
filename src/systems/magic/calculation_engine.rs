//! Core magic calculation engine implementing sympathetic resonance mathematics
//!
//! This module implements the scientific foundation of the magic system,
//! calculating success rates, energy costs, and magical effects based on
//! established physical principles and game balance framework.

use crate::core::{Player, player::Crystal};
use crate::core::world_state::WorldState;
use crate::GameResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core magic calculation engine
pub struct MagicCalculationEngine {
    /// Calculators for different magic types
    calculators: HashMap<String, Box<dyn MagicCalculator>>,
    /// Base formulas and constants
    formulas: MagicFormulas,
}

/// A specific magical attempt with all parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagicAttempt {
    /// Type of magic being attempted
    pub spell_type: String,
    /// Crystal frequency being used
    pub crystal_frequency: i32,
    /// Target of the magic (optional)
    pub target: Option<String>,
    /// Difficulty modifier (1.0 = normal)
    pub difficulty_modifier: f32,
}

/// Result of a magic attempt calculation
#[derive(Debug, Clone)]
pub struct MagicResult {
    /// Whether the attempt succeeded
    pub success: bool,
    /// Power level achieved (0.0-1.0+)
    pub power_level: f32,
    /// Mental energy cost
    pub energy_cost: i32,
    /// Fatigue accumulation
    pub fatigue_cost: i32,
    /// Crystal degradation amount
    pub crystal_degradation: f32,
    /// Time taken in minutes
    pub time_cost: i32,
    /// Experience points gained
    pub experience_gained: i32,
    /// Detailed explanation of calculation
    pub explanation: String,
    /// Success probability that was calculated
    pub success_probability: f32,
}

/// Magic formulas and constants from balance framework
struct MagicFormulas {
    /// Base energy costs for different magic types
    base_energy_costs: HashMap<String, i32>,
    /// Base fatigue costs for different magic types
    base_fatigue_costs: HashMap<String, i32>,
    /// Optimal frequencies for different magic types
    optimal_frequencies: HashMap<String, i32>,
    /// Difficulty multipliers for magic types
    difficulty_multipliers: HashMap<String, f32>,
}

/// Trait for magic type-specific calculations
trait MagicCalculator: Send + Sync {
    fn calculate(&self, attempt: &MagicAttempt, context: &MagicContext) -> MagicCalculationResult;
    fn get_description(&self) -> &str;
}

/// Context for magic calculations
pub struct MagicContext {
    pub caster: &Player,
    pub world: &WorldState,
    pub crystal: &Crystal,
}

/// Raw calculation result before applying success/failure
struct MagicCalculationResult {
    success_probability: f32,
    power_level: f32,
    energy_cost: i32,
    fatigue_cost: i32,
    crystal_degradation: f32,
    time_cost: i32,
    explanation_parts: Vec<String>,
}

impl MagicCalculationEngine {
    /// Create a new calculation engine with all magic types
    pub fn new() -> Self {
        let mut engine = Self {
            calculators: HashMap::new(),
            formulas: MagicFormulas::new(),
        };

        // Register magic calculators
        engine.register_calculator("light", Box::new(LightMagicCalculator));
        engine.register_calculator("healing", Box::new(HealingMagicCalculator));
        engine.register_calculator("detection", Box::new(DetectionMagicCalculator));
        engine.register_calculator("manipulation", Box::new(ManipulationMagicCalculator));
        engine.register_calculator("communication", Box::new(CommunicationMagicCalculator));

        engine
    }

    /// Register a calculator for a magic type
    fn register_calculator(&mut self, magic_type: &str, calculator: Box<dyn MagicCalculator>) {
        self.calculators.insert(magic_type.to_string(), calculator);
    }

    /// Calculate the result of a magic attempt
    pub fn calculate_attempt(
        &self,
        attempt: &MagicAttempt,
        caster: &Player,
        world: &WorldState,
    ) -> GameResult<MagicResult> {
        // Get active crystal
        let crystal = caster.active_crystal()
            .ok_or_else(|| crate::GameError::InsufficientResources("No crystal equipped".to_string()))?;

        // Create calculation context
        let context = MagicContext {
            caster,
            world,
            crystal,
        };

        // Get calculator for this magic type
        let calculator = self.calculators.get(&attempt.spell_type)
            .ok_or_else(|| crate::GameError::InvalidCommand(format!("Unknown magic type: {}", attempt.spell_type)))?;

        // Perform calculation
        let calc_result = calculator.calculate(attempt, &context);

        // Apply base modifiers and roll for success
        let final_result = self.finalize_result(calc_result, &context);

        Ok(final_result)
    }

    /// Apply final modifiers and determine success
    fn finalize_result(&self, calc_result: MagicCalculationResult, context: &MagicContext) -> MagicResult {
        // Roll for success using calculated probability
        let roll = rand::random::<f32>();
        let success = roll < calc_result.success_probability;

        // Calculate experience gained (always get some, more on success)
        let base_xp = if success { 5 } else { 2 };
        let difficulty_xp = (calc_result.success_probability * 3.0) as i32;
        let experience_gained = base_xp + difficulty_xp;

        // Build explanation
        let mut explanation = calc_result.explanation_parts.join("\n");
        explanation.push_str(&format!(
            "\n\nFinal Roll: {:.3} vs {:.3} probability ({})",
            roll, calc_result.success_probability,
            if success { "SUCCESS" } else { "FAILURE" }
        ));

        if !success {
            explanation.push_str("\nThe resonance fails to stabilize and the magic dissipates harmlessly.");
        }

        MagicResult {
            success,
            power_level: if success { calc_result.power_level } else { 0.0 },
            energy_cost: calc_result.energy_cost,
            fatigue_cost: calc_result.fatigue_cost,
            crystal_degradation: calc_result.crystal_degradation,
            time_cost: calc_result.time_cost,
            experience_gained,
            explanation,
            success_probability: calc_result.success_probability,
        }
    }
}

impl MagicFormulas {
    fn new() -> Self {
        let mut base_energy_costs = HashMap::new();
        base_energy_costs.insert("light".to_string(), 8);
        base_energy_costs.insert("healing".to_string(), 15);
        base_energy_costs.insert("detection".to_string(), 12);
        base_energy_costs.insert("manipulation".to_string(), 20);
        base_energy_costs.insert("communication".to_string(), 10);

        let mut base_fatigue_costs = HashMap::new();
        base_fatigue_costs.insert("light".to_string(), 5);
        base_fatigue_costs.insert("healing".to_string(), 12);
        base_fatigue_costs.insert("detection".to_string(), 8);
        base_fatigue_costs.insert("manipulation".to_string(), 15);
        base_fatigue_costs.insert("communication".to_string(), 6);

        let mut optimal_frequencies = HashMap::new();
        optimal_frequencies.insert("light".to_string(), 4);        // Quartz
        optimal_frequencies.insert("healing".to_string(), 7);      // Amethyst
        optimal_frequencies.insert("detection".to_string(), 6);   // Garnet
        optimal_frequencies.insert("manipulation".to_string(), 2); // Obsidian
        optimal_frequencies.insert("communication".to_string(), 4); // Quartz

        let mut difficulty_multipliers = HashMap::new();
        difficulty_multipliers.insert("light".to_string(), 0.8);         // Easier
        difficulty_multipliers.insert("healing".to_string(), 1.2);       // Harder
        difficulty_multipliers.insert("detection".to_string(), 1.0);     // Normal
        difficulty_multipliers.insert("manipulation".to_string(), 1.5);  // Much harder
        difficulty_multipliers.insert("communication".to_string(), 0.9); // Slightly easier

        Self {
            base_energy_costs,
            base_fatigue_costs,
            optimal_frequencies,
            difficulty_multipliers,
        }
    }

    fn get_base_energy_cost(&self, magic_type: &str) -> i32 {
        self.base_energy_costs.get(magic_type).copied().unwrap_or(15)
    }

    fn get_base_fatigue_cost(&self, magic_type: &str) -> i32 {
        self.base_fatigue_costs.get(magic_type).copied().unwrap_or(10)
    }

    fn get_optimal_frequency(&self, magic_type: &str) -> i32 {
        self.optimal_frequencies.get(magic_type).copied().unwrap_or(4)
    }

    fn get_difficulty_multiplier(&self, magic_type: &str) -> f32 {
        self.difficulty_multipliers.get(magic_type).copied().unwrap_or(1.0)
    }
}

/// Calculate base success probability using core formula
fn calculate_base_success(context: &MagicContext, formulas: &MagicFormulas, magic_type: &str) -> (f32, Vec<String>) {
    let mut explanation = Vec::new();

    // Base success from Resonance Sensitivity
    let base_success = context.caster.attributes.resonance_sensitivity as f32 / 4.0; // 0-25
    explanation.push(format!("Base success from Resonance Sensitivity {}: {:.1}%",
                            context.caster.attributes.resonance_sensitivity, base_success));

    // Crystal frequency matching
    let optimal_freq = formulas.get_optimal_frequency(magic_type);
    let frequency_diff = (context.crystal.frequency - optimal_freq).abs();
    let frequency_modifier = match frequency_diff {
        0 => 25.0,      // Perfect match
        1 => 15.0,      // Very good
        2 => 5.0,       // Good
        3 => -5.0,      // Poor
        4 => -15.0,     // Bad
        _ => -25.0,     // Terrible
    };
    explanation.push(format!("Frequency matching (crystal {} vs optimal {}): {:+.1}%",
                            context.crystal.frequency, optimal_freq, frequency_modifier));

    // Crystal efficiency
    let efficiency_bonus = (context.crystal.efficiency() - 0.5) * 40.0; // -20 to +20
    explanation.push(format!("Crystal efficiency {:.0}%: {:+.1}%",
                            context.crystal.efficiency() * 100.0, efficiency_bonus));

    // Crystal power multiplier
    let power_bonus = (context.crystal.power_multiplier() - 1.0) * 10.0; // -5 to +3
    explanation.push(format!("Crystal size {:?}: {:+.1}%",
                            context.crystal.size, power_bonus));

    // Mental energy state
    let energy_ratio = context.caster.effective_mental_energy() as f32 / context.caster.mental_state.max_energy as f32;
    let energy_modifier = match energy_ratio {
        r if r >= 0.8 => 10.0,   // High energy
        r if r >= 0.6 => 5.0,    // Good energy
        r if r >= 0.4 => 0.0,    // Medium energy
        r if r >= 0.2 => -10.0,  // Low energy
        _ => -20.0,              // Very low energy
    };
    explanation.push(format!("Mental energy state ({}/{}): {:+.1}%",
                            context.caster.effective_mental_energy(),
                            context.caster.mental_state.max_energy, energy_modifier));

    // Environmental modifier
    let env_modifier = (context.world.calculate_magical_modifier(context.crystal.frequency) - 1.0) * 25.0;
    explanation.push(format!("Environmental conditions: {:+.1}%", env_modifier));

    // Difficulty modifier
    let difficulty_penalty = (1.0 - formulas.get_difficulty_multiplier(magic_type)) * 25.0;
    explanation.push(format!("Magic type difficulty: {:+.1}%", difficulty_penalty));

    let total_success = (base_success + frequency_modifier + efficiency_bonus + power_bonus +
                        energy_modifier + env_modifier + difficulty_penalty)
                        .max(5.0)  // Minimum 5% chance
                        .min(95.0); // Maximum 95% chance

    explanation.push(format!("\nTotal Success Probability: {:.1}%", total_success));

    (total_success / 100.0, explanation)
}

impl MagicAttempt {
    pub fn new(spell_type: &str, crystal_frequency: i32, target: Option<&str>) -> Self {
        Self {
            spell_type: spell_type.to_string(),
            crystal_frequency,
            target: target.map(|s| s.to_string()),
            difficulty_modifier: 1.0,
        }
    }

    pub fn with_difficulty(mut self, modifier: f32) -> Self {
        self.difficulty_modifier = modifier;
        self
    }
}

// Magic type calculators

struct LightMagicCalculator;

impl MagicCalculator for LightMagicCalculator {
    fn calculate(&self, attempt: &MagicAttempt, context: &MagicContext) -> MagicCalculationResult {
        let formulas = MagicFormulas::new();
        let (success_probability, mut explanation) = calculate_base_success(context, &formulas, &attempt.spell_type);

        explanation.push("\nLight Magic: Creates illumination through crystal resonance".to_string());

        MagicCalculationResult {
            success_probability,
            power_level: 0.6,
            energy_cost: formulas.get_base_energy_cost(&attempt.spell_type),
            fatigue_cost: formulas.get_base_fatigue_cost(&attempt.spell_type),
            crystal_degradation: 0.5,
            time_cost: 1,
            explanation_parts: explanation,
        }
    }

    fn get_description(&self) -> &str {
        "Creates steady illumination by exciting crystal lattice structures"
    }
}

struct HealingMagicCalculator;

impl MagicCalculator for HealingMagicCalculator {
    fn calculate(&self, attempt: &MagicAttempt, context: &MagicContext) -> MagicCalculationResult {
        let formulas = MagicFormulas::new();
        let (mut success_probability, mut explanation) = calculate_base_success(context, &formulas, &attempt.spell_type);

        // Healing requires sympathetic connection
        if attempt.target.is_some() {
            explanation.push("Target healing: Requires sympathetic connection (-10%)".to_string());
            success_probability -= 0.1;
        } else {
            explanation.push("Self-healing: Natural sympathetic connection (+5%)".to_string());
            success_probability += 0.05;
        }

        explanation.push("\nHealing Magic: Accelerates natural healing through bio-resonance".to_string());

        MagicCalculationResult {
            success_probability: success_probability.max(0.05).min(0.95),
            power_level: 0.8,
            energy_cost: formulas.get_base_energy_cost(&attempt.spell_type),
            fatigue_cost: formulas.get_base_fatigue_cost(&attempt.spell_type),
            crystal_degradation: 1.2,
            time_cost: 3,
            explanation_parts: explanation,
        }
    }

    fn get_description(&self) -> &str {
        "Accelerates natural healing by harmonizing with biological energy patterns"
    }
}

struct DetectionMagicCalculator;

impl MagicCalculator for DetectionMagicCalculator {
    fn calculate(&self, attempt: &MagicAttempt, context: &MagicContext) -> MagicCalculationResult {
        let formulas = MagicFormulas::new();
        let (success_probability, mut explanation) = calculate_base_success(context, &formulas, &attempt.spell_type);

        explanation.push("\nDetection Magic: Reveals hidden magical signatures and energies".to_string());

        MagicCalculationResult {
            success_probability,
            power_level: 0.7,
            energy_cost: formulas.get_base_energy_cost(&attempt.spell_type),
            fatigue_cost: formulas.get_base_fatigue_cost(&attempt.spell_type),
            crystal_degradation: 0.8,
            time_cost: 2,
            explanation_parts: explanation,
        }
    }

    fn get_description(&self) -> &str {
        "Enhances perception to detect magical signatures and hidden energies"
    }
}

struct ManipulationMagicCalculator;

impl MagicCalculator for ManipulationMagicCalculator {
    fn calculate(&self, attempt: &MagicAttempt, context: &MagicContext) -> MagicCalculationResult {
        let formulas = MagicFormulas::new();
        let (success_probability, mut explanation) = calculate_base_success(context, &formulas, &attempt.spell_type);

        explanation.push("\nManipulation Magic: Direct force application through electromagnetic fields".to_string());

        MagicCalculationResult {
            success_probability,
            power_level: 1.0,
            energy_cost: formulas.get_base_energy_cost(&attempt.spell_type),
            fatigue_cost: formulas.get_base_fatigue_cost(&attempt.spell_type),
            crystal_degradation: 2.0,
            time_cost: 4,
            explanation_parts: explanation,
        }
    }

    fn get_description(&self) -> &str {
        "Applies direct physical force through controlled electromagnetic manipulation"
    }
}

struct CommunicationMagicCalculator;

impl MagicCalculator for CommunicationMagicCalculator {
    fn calculate(&self, attempt: &MagicAttempt, context: &MagicContext) -> MagicCalculationResult {
        let formulas = MagicFormulas::new();
        let (success_probability, mut explanation) = calculate_base_success(context, &formulas, &attempt.spell_type);

        explanation.push("\nCommunication Magic: Establishes resonant links for information transfer".to_string());

        MagicCalculationResult {
            success_probability,
            power_level: 0.5,
            energy_cost: formulas.get_base_energy_cost(&attempt.spell_type),
            fatigue_cost: formulas.get_base_fatigue_cost(&attempt.spell_type),
            crystal_degradation: 0.6,
            time_cost: 2,
            explanation_parts: explanation,
        }
    }

    fn get_description(&self) -> &str {
        "Creates resonant communication links across distances"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::player::{CrystalType, CrystalSize};

    fn create_test_context() -> (Player, WorldState, Crystal) {
        let player = Player::new("Test Player".to_string());
        let world = WorldState::new();
        let crystal = Crystal::new(CrystalType::Quartz, 90.0, 0.8, CrystalSize::Medium);
        (player, world, crystal)
    }

    #[test]
    fn test_magic_engine_creation() {
        let engine = MagicCalculationEngine::new();
        assert!(engine.calculators.contains_key("light"));
        assert!(engine.calculators.contains_key("healing"));
    }

    #[test]
    fn test_light_magic_calculation() {
        let engine = MagicCalculationEngine::new();
        let (mut player, world, _crystal) = create_test_context();

        // Ensure player has a crystal equipped
        let crystal = Crystal::new(CrystalType::Quartz, 90.0, 0.8, CrystalSize::Medium);
        player.inventory.crystals = vec![crystal];
        player.inventory.active_crystal = Some(0);

        let attempt = MagicAttempt::new("light", 4, None);
        let result = engine.calculate_attempt(&attempt, &player, &world).unwrap();

        assert!(result.success_probability > 0.0);
        assert!(result.energy_cost > 0);
        assert!(!result.explanation.is_empty());
    }

    #[test]
    fn test_healing_magic_with_target() {
        let engine = MagicCalculationEngine::new();
        let (mut player, world, _crystal) = create_test_context();

        let crystal = Crystal::new(CrystalType::Amethyst, 85.0, 0.9, CrystalSize::Medium);
        player.inventory.crystals = vec![crystal];
        player.inventory.active_crystal = Some(0);

        let attempt = MagicAttempt::new("healing", 7, Some("guard"));
        let result = engine.calculate_attempt(&attempt, &player, &world).unwrap();

        assert!(result.energy_cost > 0);
        assert!(result.explanation.contains("Target healing"));
    }

    #[test]
    fn test_frequency_matching_bonus() {
        let formulas = MagicFormulas::new();
        let (player, world, crystal) = create_test_context();

        let context = MagicContext {
            caster: &player,
            world: &world,
            crystal: &crystal,
        };

        let (success_prob, explanation) = calculate_base_success(&context, &formulas, "light");

        // Should get perfect frequency match bonus for quartz (freq 4) with light magic
        assert!(explanation.iter().any(|line| line.contains("Perfect match") || line.contains("25.0%")));
        assert!(success_prob > 0.0);
    }

    #[test]
    fn test_energy_state_modifier() {
        let formulas = MagicFormulas::new();
        let (mut player, world, crystal) = create_test_context();

        // Set low energy
        player.mental_state.current_energy = 10;
        player.mental_state.fatigue = 80;

        let context = MagicContext {
            caster: &player,
            world: &world,
            crystal: &crystal,
        };

        let (success_prob, explanation) = calculate_base_success(&context, &formulas, "light");

        // Should get penalty for low energy
        assert!(explanation.iter().any(|line| line.contains("Very low energy") || line.contains("-20")));
        assert!(success_prob < 0.5); // Should be quite low due to energy penalty
    }
}