//! Crystal management system for efficiency and degradation

use crate::core::player::Crystal;

/// Manages crystal efficiency and degradation
pub struct CrystalManager {
    /// Degradation rate modifiers
    degradation_modifiers: std::collections::HashMap<String, f32>,
}

/// Crystal efficiency analysis
pub struct CrystalEfficiency {
    pub base_efficiency: f32,
    pub purity_bonus: f32,
    pub integrity_penalty: f32,
    pub size_modifier: f32,
    pub total_efficiency: f32,
}

impl CrystalManager {
    pub fn new() -> Self {
        let mut degradation_modifiers = std::collections::HashMap::new();

        // Different crystal types degrade at different rates
        degradation_modifiers.insert("Quartz".to_string(), 1.0);     // Baseline
        degradation_modifiers.insert("Amethyst".to_string(), 0.8);   // More durable
        degradation_modifiers.insert("Obsidian".to_string(), 1.5);   // More fragile
        degradation_modifiers.insert("Garnet".to_string(), 0.9);     // Slightly more durable

        Self {
            degradation_modifiers,
        }
    }

    /// Calculate detailed crystal efficiency
    pub fn analyze_efficiency(&self, crystal: &Crystal) -> CrystalEfficiency {
        let base_efficiency = 0.7; // 70% base efficiency
        let purity_bonus = crystal.purity * 0.3; // Up to 30% from purity
        let integrity_penalty = (1.0 - crystal.integrity / 100.0) * 0.4; // Up to 40% penalty
        let size_modifier = (crystal.power_multiplier() - 1.0) * 0.1; // Size affects efficiency

        let total_efficiency = (base_efficiency + purity_bonus + size_modifier - integrity_penalty)
            .max(0.1)
            .min(1.0);

        CrystalEfficiency {
            base_efficiency,
            purity_bonus,
            integrity_penalty,
            size_modifier,
            total_efficiency,
        }
    }

    /// Calculate degradation for a specific use
    pub fn calculate_degradation(&self, crystal: &Crystal, base_degradation: f32, overuse_penalty: f32) -> f32 {
        let type_modifier = self.degradation_modifiers
            .get(&format!("{:?}", crystal.crystal_type))
            .copied()
            .unwrap_or(1.0);

        let purity_protection = crystal.purity * 0.5; // High purity reduces degradation
        let integrity_vulnerability = if crystal.integrity < 50.0 { 1.5 } else { 1.0 };

        let total_degradation = base_degradation * type_modifier * integrity_vulnerability
                              * (1.0 - purity_protection) * (1.0 + overuse_penalty);

        total_degradation.max(0.1).min(10.0) // Reasonable bounds
    }

    /// Check if crystal needs maintenance
    pub fn needs_maintenance(&self, crystal: &Crystal) -> bool {
        crystal.integrity < 75.0
    }

    /// Get maintenance recommendations
    pub fn get_maintenance_advice(&self, crystal: &Crystal) -> Vec<String> {
        let mut advice = Vec::new();

        if crystal.integrity < 25.0 {
            advice.push("URGENT: Crystal integrity critically low! Seek professional repair immediately.".to_string());
        } else if crystal.integrity < 50.0 {
            advice.push("Crystal showing significant wear. Consider professional maintenance.".to_string());
        } else if crystal.integrity < 75.0 {
            advice.push("Crystal could benefit from routine maintenance.".to_string());
        }

        if crystal.purity < 0.5 {
            advice.push("Crystal purity is low. Purification rituals may help.".to_string());
        }

        advice
    }
}