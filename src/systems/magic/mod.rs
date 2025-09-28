//! Magic system implementation with sympathetic resonance calculations
//!
//! This module provides:
//! - Magic calculation engine with scientific principles
//! - Crystal resonance and degradation mechanics
//! - Mental energy and fatigue management
//! - Theory-based magical applications

pub mod calculation_engine;
pub mod resonance_system;
pub mod crystal_management;

pub use calculation_engine::{MagicCalculationEngine, MagicAttempt, MagicResult};
pub use resonance_system::{ResonanceAnalyzer, ResonanceContext};
pub use crystal_management::{CrystalManager, CrystalEfficiency};

use crate::core::Player;
use crate::core::world_state::WorldState;
use crate::GameResult;

/// Complete magic system coordinating all magical mechanics
pub struct MagicSystem {
    /// Core calculation engine
    calculation_engine: MagicCalculationEngine,
    /// Resonance analysis system
    #[allow(dead_code)]
    resonance_analyzer: ResonanceAnalyzer,
    /// Crystal management system
    #[allow(dead_code)]
    crystal_manager: CrystalManager,
}

impl MagicSystem {
    /// Create a new magic system
    pub fn new() -> Self {
        Self {
            calculation_engine: MagicCalculationEngine::new(),
            resonance_analyzer: ResonanceAnalyzer::new(),
            crystal_manager: CrystalManager::new(),
        }
    }

    /// Attempt to cast magic with full system integration
    pub fn attempt_magic(
        &mut self,
        spell_type: &str,
        caster: &mut Player,
        world: &mut WorldState,
        target: Option<&str>,
    ) -> GameResult<MagicResult> {
        // Get active crystal info before any mutable operations
        let crystal_frequency = caster.active_crystal()
            .map(|c| c.frequency)
            .ok_or_else(|| crate::GameError::InsufficientResources("No crystal equipped".to_string()))?;

        // Create magic attempt
        let attempt = MagicAttempt::new(spell_type, crystal_frequency, target);

        // Calculate result
        let result = self.calculation_engine.calculate_attempt(
            &attempt,
            caster,
            world,
        )?;

        // Apply costs and effects if successful
        if result.success {
            // Use mental energy
            caster.use_mental_energy(result.energy_cost, result.fatigue_cost)?;

            // Degrade crystal
            if let Some(crystal) = caster.active_crystal_mut() {
                crystal.degrade(result.crystal_degradation);
            }

            // Add magical signature to location
            world.add_magical_signature(
                spell_type.to_string(),
                result.power_level,
                crystal_frequency,
            );

            // Advance time
            world.advance_time(result.time_cost);
            caster.playtime_minutes += result.time_cost;

            // Add experience
            caster.add_experience(crate::core::player::AttributeType::ResonanceSensitivity, result.experience_gained);
        }

        Ok(result)
    }

    /// Get magic system status for debugging
    pub fn get_status(&self) -> String {
        format!(
            "Magic System Status:\n\
             - Calculation Engine: Active\n\
             - Resonance Analyzer: Active\n\
             - Crystal Manager: Active"
        )
    }
}