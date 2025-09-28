//! Resonance analysis system for sympathetic connections

use crate::core::Player;
use crate::core::world_state::WorldState;

/// Analyzes sympathetic resonance connections
pub struct ResonanceAnalyzer {
    /// Connection strength cache
    connection_cache: std::collections::HashMap<String, f32>,
}

/// Context for resonance analysis
pub struct ResonanceContext {
    pub caster_location: String,
    pub target_description: Option<String>,
    pub environmental_factors: Vec<String>,
}

impl ResonanceAnalyzer {
    pub fn new() -> Self {
        Self {
            connection_cache: std::collections::HashMap::new(),
        }
    }

    /// Calculate sympathetic connection strength
    pub fn calculate_connection_strength(
        &mut self,
        caster: &Player,
        target: Option<&str>,
        world: &WorldState,
    ) -> f32 {
        match target {
            Some(target_str) => {
                // Check cache first
                let cache_key = format!("{}:{}", caster.name, target_str);
                if let Some(&cached_strength) = self.connection_cache.get(&cache_key) {
                    return cached_strength;
                }

                let strength = self.analyze_target_connection(target_str, caster, world);
                self.connection_cache.insert(cache_key, strength);
                strength
            }
            None => 1.0, // Self-targeting has perfect connection
        }
    }

    /// Analyze connection to a specific target
    fn analyze_target_connection(&self, target: &str, caster: &Player, world: &WorldState) -> f32 {
        let mut base_strength = 0.3; // Default weak connection

        // Personal item connections
        if self.is_personal_item(target, caster) {
            base_strength = 0.9;
        }
        // Recent interaction connections
        else if self.has_recent_interaction(target, caster) {
            base_strength = 0.7;
        }
        // Same location connection
        else if self.in_same_location(target, world) {
            base_strength = 0.5;
        }

        // Environmental modifiers
        let env_modifier = self.calculate_environmental_modifier(world);

        (base_strength * env_modifier).clamp(0.1, 1.0)
    }

    fn is_personal_item(&self, target: &str, caster: &Player) -> bool {
        // Check if target matches player's items
        caster.inventory.items.iter()
            .any(|item| item.name.to_lowercase().contains(&target.to_lowercase()))
    }

    fn has_recent_interaction(&self, _target: &str, _caster: &Player) -> bool {
        // Placeholder for interaction history
        false
    }

    fn in_same_location(&self, _target: &str, _world: &WorldState) -> bool {
        // Placeholder for location-based connections
        true
    }

    fn calculate_environmental_modifier(&self, world: &WorldState) -> f32 {
        if let Some(location) = world.current_location() {
            // Higher ambient energy improves connections
            location.magical_properties.ambient_energy * 0.8 + 0.2
        } else {
            1.0
        }
    }
}