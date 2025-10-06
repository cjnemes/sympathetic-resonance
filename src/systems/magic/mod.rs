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
use serde::{Serialize, Deserialize};

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

// Custom serialization - MagicSystem has no state, just recreate on deserialize
impl Serialize for MagicSystem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize an empty struct since there's no state to preserve
        serializer.serialize_unit_struct("MagicSystem")
    }
}

impl<'de> Deserialize<'de> for MagicSystem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Just create a new instance - all calculators are stateless
        deserializer.deserialize_unit_struct("MagicSystem", MagicSystemVisitor)?;
        Ok(MagicSystem::new())
    }
}

struct MagicSystemVisitor;

impl<'de> serde::de::Visitor<'de> for MagicSystemVisitor {
    type Value = ();

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("unit struct MagicSystem")
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(())
    }
}

impl std::fmt::Debug for MagicSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MagicSystem").finish()
    }
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

        // Apply costs regardless of success to prevent zero-cost exploitation
        // Failed attempts still consume resources, but at reduced rates
        let cost_multiplier = if result.success { 1.0 } else { 0.5 };

        // Use mental energy (always applied, scaled for failures)
        let actual_energy_cost = (result.energy_cost as f32 * cost_multiplier) as i32;
        let actual_fatigue_cost = (result.fatigue_cost as f32 * cost_multiplier) as i32;
        caster.use_mental_energy(actual_energy_cost, actual_fatigue_cost)?;

        // Degrade crystal (always applied, scaled for failures)
        if let Some(crystal) = caster.active_crystal_mut() {
            let actual_degradation = result.crystal_degradation * cost_multiplier;
            crystal.degrade(actual_degradation);
        }

        // Apply time cost (always applied, full cost regardless of success)
        world.advance_time(result.time_cost);
        caster.playtime_minutes += result.time_cost;

        // Only successful spells leave magical signatures and grant full experience
        if result.success {
            // Add magical signature to location
            world.add_magical_signature(
                spell_type.to_string(),
                result.power_level,
                crystal_frequency,
            );

            // Add full experience for successful casts
            caster.add_experience(crate::core::player::AttributeType::ResonanceSensitivity, result.experience_gained);
        } else {
            // Failed attempts still provide some learning experience
            let reduced_experience = (result.experience_gained as f32 * 0.25) as i32;
            caster.add_experience(crate::core::player::AttributeType::ResonanceSensitivity, reduced_experience);
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