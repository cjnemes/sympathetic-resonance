//! Political relationship system between factions

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::FactionId;

/// Political relationship types between factions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Relationship {
    StrongAllies,   // +25% cross-effect
    Allies,         // +17% cross-effect
    Neutral,        // No cross-effect
    Rivals,         // -12% cross-effect
    Enemies,        // -20% cross-effect
    OpenWar,        // -33% cross-effect
}

impl Relationship {
    /// Convert relationship to numeric strength (-1.0 to 1.0)
    pub fn to_strength(self) -> f32 {
        match self {
            Relationship::StrongAllies => 0.25,
            Relationship::Allies => 0.17,
            Relationship::Neutral => 0.0,
            Relationship::Rivals => -0.12,
            Relationship::Enemies => -0.20,
            Relationship::OpenWar => -0.33,
        }
    }
}

/// System for managing inter-faction political relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliticalSystem {
    /// Relationships between all faction pairs
    relationships: HashMap<(FactionId, FactionId), Relationship>,
    /// Current political events affecting relationships
    events: Vec<PoliticalEvent>,
    /// Historical relationship changes
    relationship_history: Vec<RelationshipChange>,
}

/// A political event that affects faction relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoliticalEvent {
    /// Unique identifier for this event
    pub id: String,
    /// Event description
    pub description: String,
    /// Factions involved in this event
    pub participants: Vec<FactionId>,
    /// How this event modifies relationships
    pub relationship_effects: HashMap<(FactionId, FactionId), RelationshipEffect>,
    /// When this event started (game time in minutes)
    pub start_time: i32,
    /// How long this event lasts (in game minutes, None for permanent)
    pub duration: Option<i32>,
    /// Whether this event is currently active
    pub active: bool,
}

/// Effect on relationship between two factions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipEffect {
    /// How much to shift the relationship (positive = more friendly)
    pub shift: i32,
    /// Whether this is a temporary or permanent change
    pub temporary: bool,
}

/// Historical record of relationship changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipChange {
    /// Factions involved
    pub factions: (FactionId, FactionId),
    /// Old relationship
    pub old_relationship: Relationship,
    /// New relationship
    pub new_relationship: Relationship,
    /// What caused this change
    pub reason: String,
    /// When this occurred
    pub timestamp: i32,
}

impl PoliticalSystem {
    /// Create a new political system with default relationships
    pub fn new() -> Self {
        let mut system = Self {
            relationships: HashMap::new(),
            events: Vec::new(),
            relationship_history: Vec::new(),
        };

        system.initialize_default_relationships();
        system
    }

    /// Set up the default political landscape
    fn initialize_default_relationships(&mut self) {
        use FactionId::*;
        use Relationship::*;

        // Magisters' Council relationships
        self.set_relationship(MagistersCouncil, OrderOfHarmony, Allies);
        self.set_relationship(MagistersCouncil, IndustrialConsortium, Rivals);
        self.set_relationship(MagistersCouncil, UndergroundNetwork, Enemies);
        self.set_relationship(MagistersCouncil, NeutralScholars, Neutral);

        // Order of Harmony relationships
        self.set_relationship(OrderOfHarmony, IndustrialConsortium, Enemies);
        self.set_relationship(OrderOfHarmony, UndergroundNetwork, Rivals);
        self.set_relationship(OrderOfHarmony, NeutralScholars, Allies);

        // Industrial Consortium relationships
        self.set_relationship(IndustrialConsortium, UndergroundNetwork, Rivals);
        self.set_relationship(IndustrialConsortium, NeutralScholars, Neutral);

        // Underground Network relationships
        self.set_relationship(UndergroundNetwork, NeutralScholars, Allies);
    }

    /// Set relationship between two factions
    fn set_relationship(&mut self, faction1: FactionId, faction2: FactionId, relationship: Relationship) {
        // Ensure consistent ordering for lookup
        let key = if (faction1 as u8) < (faction2 as u8) {
            (faction1, faction2)
        } else {
            (faction2, faction1)
        };

        self.relationships.insert(key, relationship);
    }

    /// Get relationship between two factions
    pub fn get_relationship(&self, faction1: FactionId, faction2: FactionId) -> Relationship {
        if faction1 == faction2 {
            return Relationship::StrongAllies; // Faction is allied with itself
        }

        let key = if (faction1 as u8) < (faction2 as u8) {
            (faction1, faction2)
        } else {
            (faction2, faction1)
        };

        self.relationships.get(&key).copied().unwrap_or(Relationship::Neutral)
    }

    /// Get all relationships for a specific faction
    pub fn get_relationships(&self, faction: FactionId) -> HashMap<FactionId, Relationship> {
        let mut relationships = HashMap::new();

        for &other_faction in &FactionId::all() {
            if other_faction != faction {
                relationships.insert(other_faction, self.get_relationship(faction, other_faction));
            }
        }

        relationships
    }

    /// Add a political event that affects relationships
    pub fn add_event(&mut self, mut event: PoliticalEvent, current_time: i32) {
        event.start_time = current_time;
        event.active = true;

        // Apply immediate relationship effects
        for ((faction1, faction2), effect) in &event.relationship_effects {
            if !effect.temporary {
                let current = self.get_relationship(*faction1, *faction2);
                let new_relationship = self.shift_relationship(current, effect.shift);

                if new_relationship != current {
                    self.relationship_history.push(RelationshipChange {
                        factions: (*faction1, *faction2),
                        old_relationship: current,
                        new_relationship,
                        reason: event.description.clone(),
                        timestamp: current_time,
                    });

                    self.set_relationship(*faction1, *faction2, new_relationship);
                }
            }
        }

        self.events.push(event);
    }

    /// Update political events based on current time
    pub fn update_events(&mut self, current_time: i32) {
        // Collect relationship changes to apply after iteration
        let mut relationship_changes = Vec::new();

        for event in &mut self.events {
            if event.active {
                if let Some(duration) = event.duration {
                    if current_time >= event.start_time + duration {
                        event.active = false;

                        // Collect temporary effects to reverse
                        for ((faction1, faction2), effect) in &event.relationship_effects {
                            if effect.temporary {
                                relationship_changes.push((*faction1, *faction2, -effect.shift));
                            }
                        }
                    }
                }
            }
        }

        // Apply collected relationship changes
        for (faction1, faction2, shift) in relationship_changes {
            let current = self.get_relationship(faction1, faction2);
            let new_relationship = self.shift_relationship(current, shift);
            self.set_relationship(faction1, faction2, new_relationship);
        }

        // Remove expired events
        self.events.retain(|event| event.active || event.duration.is_some());
    }

    /// Shift a relationship by a certain amount
    fn shift_relationship(&self, current: Relationship, shift: i32) -> Relationship {
        let current_value = match current {
            Relationship::OpenWar => -3,
            Relationship::Enemies => -2,
            Relationship::Rivals => -1,
            Relationship::Neutral => 0,
            Relationship::Allies => 1,
            Relationship::StrongAllies => 2,
        };

        let new_value = (current_value + shift).clamp(-3, 2);

        match new_value {
            -3 => Relationship::OpenWar,
            -2 => Relationship::Enemies,
            -1 => Relationship::Rivals,
            0 => Relationship::Neutral,
            1 => Relationship::Allies,
            2 => Relationship::StrongAllies,
            _ => current, // Fallback
        }
    }

    /// Get active political events
    pub fn get_active_events(&self) -> Vec<&PoliticalEvent> {
        self.events.iter().filter(|event| event.active).collect()
    }

    /// Get political tension level (average hostility between all factions)
    pub fn get_political_tension(&self) -> f32 {
        let mut total_tension = 0.0;
        let mut pair_count = 0;

        for &faction1 in &FactionId::all() {
            for &faction2 in &FactionId::all() {
                if faction1 != faction2 && (faction1 as u8) < (faction2 as u8) {
                    let relationship = self.get_relationship(faction1, faction2);
                    let tension = match relationship {
                        Relationship::OpenWar => 3.0,
                        Relationship::Enemies => 2.0,
                        Relationship::Rivals => 1.0,
                        Relationship::Neutral => 0.0,
                        Relationship::Allies => -1.0,
                        Relationship::StrongAllies => -2.0,
                    };
                    total_tension += tension;
                    pair_count += 1;
                }
            }
        }

        if pair_count > 0 {
            total_tension / pair_count as f32
        } else {
            0.0
        }
    }

    /// Check if two factions are at war
    pub fn at_war(&self, faction1: FactionId, faction2: FactionId) -> bool {
        matches!(self.get_relationship(faction1, faction2), Relationship::OpenWar)
    }

    /// Get faction allies (Strong Allies + Allies)
    pub fn get_allies(&self, faction: FactionId) -> Vec<FactionId> {
        FactionId::all()
            .into_iter()
            .filter(|&other| {
                other != faction && matches!(
                    self.get_relationship(faction, other),
                    Relationship::Allies | Relationship::StrongAllies
                )
            })
            .collect()
    }

    /// Get faction enemies (Enemies + Open War)
    pub fn get_enemies(&self, faction: FactionId) -> Vec<FactionId> {
        FactionId::all()
            .into_iter()
            .filter(|&other| {
                other != faction && matches!(
                    self.get_relationship(faction, other),
                    Relationship::Enemies | Relationship::OpenWar
                )
            })
            .collect()
    }

    /// Get relationship history for analysis
    pub fn get_relationship_history(&self) -> &Vec<RelationshipChange> {
        &self.relationship_history
    }
}

impl Relationship {
    /// Get display name for relationship
    pub fn display_name(&self) -> &str {
        match self {
            Relationship::StrongAllies => "Strong Allies",
            Relationship::Allies => "Allies",
            Relationship::Neutral => "Neutral",
            Relationship::Rivals => "Rivals",
            Relationship::Enemies => "Enemies",
            Relationship::OpenWar => "Open War",
        }
    }

    /// Get color indicator for UI display
    pub fn color_indicator(&self) -> &str {
        match self {
            Relationship::StrongAllies => "bright_green",
            Relationship::Allies => "green",
            Relationship::Neutral => "yellow",
            Relationship::Rivals => "orange",
            Relationship::Enemies => "red",
            Relationship::OpenWar => "bright_red",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_political_system_creation() {
        let politics = PoliticalSystem::new();

        // Test some default relationships
        assert_eq!(
            politics.get_relationship(FactionId::MagistersCouncil, FactionId::UndergroundNetwork),
            Relationship::Enemies
        );
        assert_eq!(
            politics.get_relationship(FactionId::OrderOfHarmony, FactionId::NeutralScholars),
            Relationship::Allies
        );
    }

    #[test]
    fn test_relationship_symmetry() {
        let politics = PoliticalSystem::new();

        let rel1 = politics.get_relationship(FactionId::MagistersCouncil, FactionId::OrderOfHarmony);
        let rel2 = politics.get_relationship(FactionId::OrderOfHarmony, FactionId::MagistersCouncil);

        assert_eq!(rel1, rel2); // Relationships should be symmetric
    }

    #[test]
    fn test_faction_self_relationship() {
        let politics = PoliticalSystem::new();

        let self_rel = politics.get_relationship(FactionId::MagistersCouncil, FactionId::MagistersCouncil);
        assert_eq!(self_rel, Relationship::StrongAllies);
    }

    #[test]
    fn test_allies_and_enemies() {
        let politics = PoliticalSystem::new();

        let council_allies = politics.get_allies(FactionId::MagistersCouncil);
        let council_enemies = politics.get_enemies(FactionId::MagistersCouncil);

        assert!(council_allies.contains(&FactionId::OrderOfHarmony));
        assert!(council_enemies.contains(&FactionId::UndergroundNetwork));
    }

    #[test]
    fn test_political_tension() {
        let politics = PoliticalSystem::new();

        let tension = politics.get_political_tension();
        assert!(tension >= 0.0); // Should have some positive tension due to conflicts
    }

    #[test]
    fn test_relationship_shifting() {
        let politics = PoliticalSystem::new();

        let original = Relationship::Neutral;
        let improved = politics.shift_relationship(original, 1);
        assert_eq!(improved, Relationship::Allies);

        let worsened = politics.shift_relationship(original, -2);
        assert_eq!(worsened, Relationship::Enemies);
    }
}