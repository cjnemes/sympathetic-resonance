//! Reputation tracking and management system

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::FactionId;

/// System for tracking player reputation with all factions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationSystem {
    /// Player's current standing with each faction (-100 to +100)
    #[serde(
        serialize_with = "crate::systems::serde_helpers::serialize_faction_map",
        deserialize_with = "crate::systems::serde_helpers::deserialize_faction_map"
    )]
    reputation_scores: HashMap<FactionId, i32>,
    /// History of reputation changes for analytics
    reputation_history: Vec<ReputationChange>,
}

/// Record of a reputation change for tracking player choices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationChange {
    /// Which faction was affected
    pub faction: FactionId,
    /// How much the reputation changed
    pub change: i32,
    /// What action caused this change
    pub reason: String,
    /// When this change occurred (game time in minutes)
    pub timestamp: i32,
}

impl ReputationSystem {
    /// Create a new reputation system with neutral standings
    pub fn new() -> Self {
        Self {
            reputation_scores: HashMap::new(),
            reputation_history: Vec::new(),
        }
    }

    /// Get current reputation with a faction (-100 to +100)
    pub fn get_reputation(&self, faction: FactionId) -> i32 {
        self.reputation_scores.get(&faction).copied().unwrap_or(0)
    }

    /// Modify reputation with bounds checking
    pub fn modify_reputation(&mut self, faction: FactionId, change: i32) {
        let current = self.get_reputation(faction);
        let new_value = (current + change).clamp(-100, 100);
        self.reputation_scores.insert(faction, new_value);
    }

    /// Modify reputation with reason tracking
    pub fn modify_reputation_with_reason(
        &mut self,
        faction: FactionId,
        change: i32,
        reason: String,
        timestamp: i32,
    ) {
        self.modify_reputation(faction, change);

        self.reputation_history.push(ReputationChange {
            faction,
            change,
            reason,
            timestamp,
        });
    }

    /// Get reputation history for a specific faction
    pub fn get_faction_history(&self, faction: FactionId) -> Vec<&ReputationChange> {
        self.reputation_history
            .iter()
            .filter(|change| change.faction == faction)
            .collect()
    }

    /// Get recent reputation changes (last N events)
    pub fn get_recent_changes(&self, count: usize) -> Vec<&ReputationChange> {
        let mut recent: Vec<_> = self.reputation_history.iter().collect();
        recent.sort_by_key(|change| change.timestamp);
        recent.into_iter().rev().take(count).collect()
    }

    /// Calculate reputation momentum (trend over recent changes)
    pub fn get_reputation_momentum(&self, faction: FactionId) -> i32 {
        let recent_changes: Vec<_> = self.reputation_history
            .iter()
            .filter(|change| change.faction == faction)
            .rev()
            .take(5) // Last 5 changes
            .collect();

        recent_changes.iter().map(|change| change.change).sum()
    }

    /// Check if reputation has crossed a significant threshold recently
    pub fn crossed_threshold(&self, faction: FactionId, threshold: i32) -> bool {
        let current = self.get_reputation(faction);

        // Find the last reputation change for this faction
        if let Some(last_change) = self.reputation_history
            .iter()
            .rev()
            .find(|change| change.faction == faction) {

            let previous = current - last_change.change;

            // Check if we crossed the threshold
            (previous < threshold && current >= threshold) ||
            (previous > threshold && current <= threshold)
        } else {
            false
        }
    }

    /// Get all factions where player has positive reputation
    pub fn get_allied_factions(&self) -> Vec<FactionId> {
        self.reputation_scores
            .iter()
            .filter(|(_, &reputation)| reputation > 20)
            .map(|(&faction, _)| faction)
            .collect()
    }

    /// Get all factions where player has negative reputation
    pub fn get_enemy_factions(&self) -> Vec<FactionId> {
        self.reputation_scores
            .iter()
            .filter(|(_, &reputation)| reputation < -20)
            .map(|(&faction, _)| faction)
            .collect()
    }

    /// Calculate total reputation score (sum of all positive reputations)
    pub fn total_positive_reputation(&self) -> i32 {
        self.reputation_scores
            .values()
            .filter(|&&rep| rep > 0)
            .sum()
    }

    /// Get reputation statistics for analytics
    pub fn get_reputation_stats(&self) -> ReputationStats {
        let values: Vec<i32> = FactionId::all()
            .iter()
            .map(|&faction| self.get_reputation(faction))
            .collect();

        let total: i32 = values.iter().sum();
        let count = values.len() as i32;
        let average = if count > 0 { total / count } else { 0 };

        let highest = values.iter().max().copied().unwrap_or(0);
        let lowest = values.iter().min().copied().unwrap_or(0);

        ReputationStats {
            average,
            highest,
            lowest,
            total_positive: values.iter().filter(|&&v| v > 0).sum(),
            total_negative: values.iter().filter(|&&v| v < 0).sum(),
            neutral_count: values.iter().filter(|&&v| (-20..=20).contains(&v)).count(),
        }
    }
}

/// Statistics about player's overall reputation standing
#[derive(Debug)]
pub struct ReputationStats {
    pub average: i32,
    pub highest: i32,
    pub lowest: i32,
    pub total_positive: i32,
    pub total_negative: i32,
    pub neutral_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reputation_modification() {
        let mut system = ReputationSystem::new();

        system.modify_reputation(FactionId::MagistersCouncil, 25);
        assert_eq!(system.get_reputation(FactionId::MagistersCouncil), 25);

        // Test bounds checking
        system.modify_reputation(FactionId::MagistersCouncil, 100);
        assert_eq!(system.get_reputation(FactionId::MagistersCouncil), 100); // Capped at 100

        system.modify_reputation(FactionId::MagistersCouncil, -250);
        assert_eq!(system.get_reputation(FactionId::MagistersCouncil), -100); // Capped at -100
    }

    #[test]
    fn test_reputation_history() {
        let mut system = ReputationSystem::new();

        system.modify_reputation_with_reason(
            FactionId::MagistersCouncil,
            15,
            "Helped with research".to_string(),
            100,
        );

        let history = system.get_faction_history(FactionId::MagistersCouncil);
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].change, 15);
    }

    #[test]
    fn test_reputation_momentum() {
        let mut system = ReputationSystem::new();

        // Add several positive changes
        for i in 0..3 {
            system.modify_reputation_with_reason(
                FactionId::MagistersCouncil,
                10,
                "Good deed".to_string(),
                i * 10,
            );
        }

        let momentum = system.get_reputation_momentum(FactionId::MagistersCouncil);
        assert_eq!(momentum, 30); // 3 × 10
    }

    #[test]
    fn test_threshold_crossing() {
        let mut system = ReputationSystem::new();

        system.modify_reputation_with_reason(
            FactionId::MagistersCouncil,
            25,
            "Major contribution".to_string(),
            100,
        );

        assert!(system.crossed_threshold(FactionId::MagistersCouncil, 20));
        assert!(!system.crossed_threshold(FactionId::MagistersCouncil, 30));
    }

    #[test]
    fn test_allied_and_enemy_factions() {
        let mut system = ReputationSystem::new();

        system.modify_reputation(FactionId::MagistersCouncil, 50);
        system.modify_reputation(FactionId::UndergroundNetwork, -50);

        let allies = system.get_allied_factions();
        let enemies = system.get_enemy_factions();

        assert!(allies.contains(&FactionId::MagistersCouncil));
        assert!(enemies.contains(&FactionId::UndergroundNetwork));
    }
}