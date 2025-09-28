//! Faction system for political relationships and reputation
//!
//! This module provides:
//! - Faction identification and properties
//! - Reputation tracking and modification
//! - Inter-faction relationship modeling

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod reputation;
pub mod politics;

pub use reputation::ReputationSystem;
pub use politics::PoliticalSystem;

/// Unique identifiers for the five major factions
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum FactionId {
    MagistersCouncil,    // Academic/Regulatory
    OrderOfHarmony,      // Conservative/Religious
    IndustrialConsortium, // Commercial/Progressive
    UndergroundNetwork,   // Libertarian/Revolutionary
    NeutralScholars,     // Academic/Independent
}

/// Complete faction information and properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Faction {
    /// Unique faction identifier
    pub id: FactionId,
    /// Public faction name
    pub name: String,
    /// Brief faction description
    pub description: String,
    /// Core philosophical principles
    pub philosophy: String,
    /// Primary goals and motivations
    pub goals: Vec<String>,
    /// Resources and capabilities
    pub resources: FactionResources,
    /// Current political power and influence
    pub influence: FactionInfluence,
}

/// Faction resources and capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionResources {
    /// Economic power (0-100)
    pub wealth: i32,
    /// Military/security capabilities (0-100)
    pub security: i32,
    /// Information networks and intelligence (0-100)
    pub information: i32,
    /// Magical knowledge and artifacts (0-100)
    pub magical_assets: i32,
    /// Political connections and legitimacy (0-100)
    pub political_power: i32,
}

/// Faction influence in different spheres
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionInfluence {
    /// Academic and research institutions
    pub academic: i32,
    /// Government and legal systems
    pub governmental: i32,
    /// Economic and commercial sectors
    pub economic: i32,
    /// Military and security forces
    pub military: i32,
    /// Underground and criminal networks
    pub underground: i32,
}

/// System for managing all faction-related mechanics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionSystem {
    /// All faction definitions
    pub factions: HashMap<FactionId, Faction>,
    /// Reputation management
    pub reputation: ReputationSystem,
    /// Political relationships
    pub politics: PoliticalSystem,
}

impl FactionSystem {
    /// Create a new faction system with default factions
    pub fn new() -> Self {
        let mut factions = HashMap::new();

        // Initialize all five major factions
        factions.insert(FactionId::MagistersCouncil, Faction::magisters_council());
        factions.insert(FactionId::OrderOfHarmony, Faction::order_of_harmony());
        factions.insert(FactionId::IndustrialConsortium, Faction::industrial_consortium());
        factions.insert(FactionId::UndergroundNetwork, Faction::underground_network());
        factions.insert(FactionId::NeutralScholars, Faction::neutral_scholars());

        Self {
            factions,
            reputation: ReputationSystem::new(),
            politics: PoliticalSystem::new(),
        }
    }

    /// Get faction information by ID
    pub fn get_faction(&self, id: FactionId) -> Option<&Faction> {
        self.factions.get(&id)
    }

    /// Get player's reputation with a faction (-100 to +100)
    pub fn get_reputation(&self, faction: FactionId) -> i32 {
        self.reputation.get_reputation(faction)
    }

    /// Modify player's reputation with a faction
    pub fn modify_reputation(&mut self, faction: FactionId, change: i32) {
        self.reputation.modify_reputation(faction, change);

        // Apply cross-faction effects
        self.apply_cross_faction_effects(faction, change);
    }

    /// Apply reputation changes to opposing/allied factions
    fn apply_cross_faction_effects(&mut self, primary_faction: FactionId, primary_change: i32) {
        let relationships = self.politics.get_relationships(primary_faction);

        for (other_faction, relationship) in relationships {
            let cross_effect = match relationship {
                politics::Relationship::StrongAllies => primary_change / 4,      // 25% positive
                politics::Relationship::Allies => primary_change / 6,           // ~17% positive
                politics::Relationship::Neutral => 0,                           // No effect
                politics::Relationship::Rivals => -primary_change / 8,          // ~12% negative
                politics::Relationship::Enemies => -primary_change / 5,         // 20% negative
                politics::Relationship::OpenWar => -primary_change / 3,         // ~33% negative
            };

            if cross_effect != 0 {
                self.reputation.modify_reputation(other_faction, cross_effect);
            }
        }
    }

    /// Get standing level description for reputation value
    pub fn get_standing_description(&self, faction: FactionId) -> String {
        let reputation = self.get_reputation(faction);
        match reputation {
            81..=100 => "Inner Circle".to_string(),
            51..=80 => "Trusted Ally".to_string(),
            21..=50 => "Member".to_string(),
            -20..=20 => "Neutral".to_string(),
            -50..=-21 => "Suspected".to_string(),
            -80..=-51 => "Enemy".to_string(),
            -100..=-81 => "Marked for Elimination".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    /// Check if player has access to faction-specific content
    pub fn has_access(&self, faction: FactionId, required_reputation: i32) -> bool {
        self.get_reputation(faction) >= required_reputation
    }

    /// Get price modifier based on faction reputation
    pub fn get_price_modifier(&self, faction: FactionId) -> f32 {
        let reputation = self.get_reputation(faction);
        match reputation {
            81..=100 => 0.7,   // 30% discount
            51..=80 => 0.8,    // 20% discount
            21..=50 => 0.9,    // 10% discount
            -20..=20 => 1.0,   // Normal price
            -50..=-21 => 1.2,  // 20% markup
            -80..=-51 => 1.5,  // 50% markup
            -100..=-81 => 2.0, // 100% markup (if they trade at all)
            _ => 1.0,
        }
    }

    /// Get all faction standings for display
    pub fn get_all_standings(&self) -> Vec<(FactionId, i32, String)> {
        let mut standings = Vec::new();
        for &faction_id in &[
            FactionId::MagistersCouncil,
            FactionId::OrderOfHarmony,
            FactionId::IndustrialConsortium,
            FactionId::UndergroundNetwork,
            FactionId::NeutralScholars,
        ] {
            let reputation = self.get_reputation(faction_id);
            let description = self.get_standing_description(faction_id);
            standings.push((faction_id, reputation, description));
        }
        standings
    }

    /// Get relationship strength between two factions (-1.0 to 1.0)
    pub fn get_relationship_strength(&self, faction1: FactionId, faction2: FactionId) -> f32 {
        self.politics.get_relationship(faction1, faction2).to_strength()
    }
}

impl Faction {
    /// Create the Magisters' Council faction
    fn magisters_council() -> Self {
        Self {
            id: FactionId::MagistersCouncil,
            name: "The Magisters' Council".to_string(),
            description: "The official governing body for magical research and regulation".to_string(),
            philosophy: "Magic must be studied systematically and regulated to prevent disasters".to_string(),
            goals: vec![
                "Maintain magical safety through regulation".to_string(),
                "Advance understanding through controlled research".to_string(),
                "Prevent magical disasters and accidents".to_string(),
                "Train certified magical practitioners".to_string(),
            ],
            resources: FactionResources {
                wealth: 70,
                security: 60,
                information: 80,
                magical_assets: 85,
                political_power: 90,
            },
            influence: FactionInfluence {
                academic: 85,
                governmental: 90,
                economic: 60,
                military: 50,
                underground: 20,
            },
        }
    }

    /// Create the Order of Natural Harmony faction
    fn order_of_harmony() -> Self {
        Self {
            id: FactionId::OrderOfHarmony,
            name: "The Order of Natural Harmony".to_string(),
            description: "Traditional organization promoting magical integration with natural order".to_string(),
            philosophy: "Magic should supplement, not replace, the natural world and its cycles".to_string(),
            goals: vec![
                "Preserve traditional ways of life".to_string(),
                "Prevent magical disruption of natural order".to_string(),
                "Provide community support and healing".to_string(),
                "Maintain moral and spiritual guidance".to_string(),
            ],
            resources: FactionResources {
                wealth: 40,
                security: 50,
                information: 60,
                magical_assets: 65,
                political_power: 55,
            },
            influence: FactionInfluence {
                academic: 30,
                governmental: 40,
                economic: 35,
                military: 30,
                underground: 15,
            },
        }
    }

    /// Create the Industrial Consortium faction
    fn industrial_consortium() -> Self {
        Self {
            id: FactionId::IndustrialConsortium,
            name: "The Industrial Consortium".to_string(),
            description: "Commercial alliance focused on magical applications for progress and profit".to_string(),
            philosophy: "Magic is a tool for advancement that should drive economic progress and innovation".to_string(),
            goals: vec![
                "Develop profitable magical applications".to_string(),
                "Expand magical technology and infrastructure".to_string(),
                "Compete with traditional industries".to_string(),
                "Create magical economic opportunities".to_string(),
            ],
            resources: FactionResources {
                wealth: 90,
                security: 65,
                information: 70,
                magical_assets: 75,
                political_power: 70,
            },
            influence: FactionInfluence {
                academic: 60,
                governmental: 65,
                economic: 90,
                military: 40,
                underground: 25,
            },
        }
    }

    /// Create the Underground Network faction
    fn underground_network() -> Self {
        Self {
            id: FactionId::UndergroundNetwork,
            name: "The Underground Network".to_string(),
            description: "Decentralized organization opposing magical regulation and promoting free access".to_string(),
            philosophy: "Magical knowledge belongs to everyone and should not be controlled by authorities".to_string(),
            goals: vec![
                "Share magical knowledge freely".to_string(),
                "Resist government magical control".to_string(),
                "Support unregulated magical practice".to_string(),
                "Protect magical practitioners from persecution".to_string(),
            ],
            resources: FactionResources {
                wealth: 30,
                security: 45,
                information: 85,
                magical_assets: 60,
                political_power: 20,
            },
            influence: FactionInfluence {
                academic: 25,
                governmental: 10,
                economic: 30,
                military: 20,
                underground: 90,
            },
        }
    }

    /// Create the Neutral Scholars faction
    fn neutral_scholars() -> Self {
        Self {
            id: FactionId::NeutralScholars,
            name: "The Neutral Scholars".to_string(),
            description: "Independent researchers focused on pure magical knowledge without political agenda".to_string(),
            philosophy: "Truth and knowledge should be pursued objectively without political interference".to_string(),
            goals: vec![
                "Advance pure magical research".to_string(),
                "Maintain academic independence".to_string(),
                "Share knowledge across faction boundaries".to_string(),
                "Preserve magical discoveries for posterity".to_string(),
            ],
            resources: FactionResources {
                wealth: 50,
                security: 30,
                information: 75,
                magical_assets: 80,
                political_power: 35,
            },
            influence: FactionInfluence {
                academic: 70,
                governmental: 25,
                economic: 40,
                military: 15,
                underground: 35,
            },
        }
    }
}

impl FactionId {
    /// Get all faction IDs
    pub fn all() -> Vec<Self> {
        vec![
            FactionId::MagistersCouncil,
            FactionId::OrderOfHarmony,
            FactionId::IndustrialConsortium,
            FactionId::UndergroundNetwork,
            FactionId::NeutralScholars,
        ]
    }

    /// Get display name for faction
    pub fn display_name(&self) -> &str {
        match self {
            FactionId::MagistersCouncil => "Magisters' Council",
            FactionId::OrderOfHarmony => "Order of Natural Harmony",
            FactionId::IndustrialConsortium => "Industrial Consortium",
            FactionId::UndergroundNetwork => "Underground Network",
            FactionId::NeutralScholars => "Neutral Scholars",
        }
    }

    /// Get short name for display
    pub fn short_name(&self) -> &str {
        match self {
            FactionId::MagistersCouncil => "Council",
            FactionId::OrderOfHarmony => "Order",
            FactionId::IndustrialConsortium => "Consortium",
            FactionId::UndergroundNetwork => "Underground",
            FactionId::NeutralScholars => "Scholars",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_faction_system_creation() {
        let faction_system = FactionSystem::new();
        assert_eq!(faction_system.factions.len(), 5);
        assert!(faction_system.get_faction(FactionId::MagistersCouncil).is_some());
    }

    #[test]
    fn test_reputation_modification() {
        let mut faction_system = FactionSystem::new();

        faction_system.modify_reputation(FactionId::MagistersCouncil, 25);
        assert_eq!(faction_system.get_reputation(FactionId::MagistersCouncil), 25);
    }

    #[test]
    fn test_cross_faction_effects() {
        let mut faction_system = FactionSystem::new();

        // Positive action for Council should negatively affect Underground (enemies)
        faction_system.modify_reputation(FactionId::MagistersCouncil, 30);

        let underground_rep = faction_system.get_reputation(FactionId::UndergroundNetwork);
        assert!(underground_rep < 0); // Should be negative due to cross-effect
    }

    #[test]
    fn test_standing_descriptions() {
        let faction_system = FactionSystem::new();

        // Test various reputation levels
        assert_eq!(faction_system.get_standing_description(FactionId::MagistersCouncil), "Neutral");
    }

    #[test]
    fn test_price_modifiers() {
        let mut faction_system = FactionSystem::new();

        // High reputation should give discount
        faction_system.modify_reputation(FactionId::MagistersCouncil, 85);
        let modifier = faction_system.get_price_modifier(FactionId::MagistersCouncil);
        assert!(modifier < 1.0);

        // Low reputation should give markup
        faction_system.modify_reputation(FactionId::IndustrialConsortium, -85);
        let modifier = faction_system.get_price_modifier(FactionId::IndustrialConsortium);
        assert!(modifier > 1.0);
    }
}