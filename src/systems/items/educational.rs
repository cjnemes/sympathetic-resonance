//! Educational items system for learning enhancement
//!
//! This module provides:
//! - Educational items that enhance learning
//! - Research tools for advanced study
//! - Collaborative learning items
//! - Integration with knowledge system

use super::core::ItemId;
use serde::{Deserialize, Serialize};
use crate::systems::knowledge::LearningMethod;
use std::collections::HashMap;

/// Educational items that enhance learning and research
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EducationalItem {
    /// Item name
    pub name: String,
    /// Educational function
    pub item_function: EducationalFunction,
    /// Learning bonuses provided
    pub learning_bonuses: Vec<LearningBonus>,
    /// Requirements to use effectively
    pub usage_requirements: UsageRequirements,
}

/// Types of educational item functions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EducationalFunction {
    /// Research tool for advanced study
    ResearchTool(ResearchTool),
    /// Collaborative learning tool
    CollaborativeTool(CollaborativeTool),
    /// Theory unlock item
    TheoryUnlock { theory_id: String },
    /// Learning method enhancer
    MethodEnhancer { method: LearningMethod, bonus: f32 },
    /// Knowledge preservation tool
    KnowledgeArchive { theories: Vec<String> },
}

/// Research tools for advanced theoretical work
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchTool {
    /// Tool type (measuring device, laboratory, etc.)
    pub tool_type: String,
    /// Required theory to use
    pub required_theory: String,
    /// Minimum understanding needed
    pub min_understanding: f32,
    /// Precision bonus for experiments
    pub precision_bonus: f32,
    /// Unlocked research methods
    pub unlocked_methods: Vec<LearningMethod>,
}

/// Collaborative learning tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborativeTool {
    /// Maximum participants
    pub max_participants: i32,
    /// Efficiency multiplier for group learning
    pub group_efficiency: f32,
    /// Required faction standing
    pub faction_requirements: HashMap<String, i32>,
    /// Theories that benefit from collaboration
    pub collaborative_theories: Vec<String>,
}

/// Learning bonuses provided by educational items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningBonus {
    /// Bonus type
    pub bonus_type: BonusType,
    /// Bonus multiplier
    pub bonus_multiplier: f32,
    /// Conditions for bonus application
    pub conditions: BonusConditions,
}

/// Types of learning bonuses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BonusType {
    /// General learning efficiency
    LearningEfficiency,
    /// Understanding gain rate
    UnderstandingGain,
    /// Experience point multiplier
    ExperienceBonus,
    /// Reduced energy cost for learning
    EnergyCostReduction,
    /// Faster research progress
    ResearchSpeed,
    /// Better success rates for experiments
    ExperimentalSuccess,
}

/// Conditions for bonus application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BonusConditions {
    /// Specific theories this bonus applies to
    pub applicable_theories: Option<Vec<String>>,
    /// Learning methods this bonus applies to
    pub applicable_methods: Option<Vec<LearningMethod>>,
    /// Minimum player level required
    pub min_level: Option<i32>,
    /// Required time of day (for astronomy tools, etc.)
    pub time_requirements: Option<TimeRequirement>,
    /// Environmental requirements
    pub environment: Option<String>,
}

/// Time-based requirements for certain items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRequirement {
    /// Hours during which item is effective (0-23)
    pub effective_hours: Vec<i32>,
    /// Seasonal requirements
    pub seasons: Option<Vec<String>>,
}

/// Requirements to use educational items effectively
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageRequirements {
    /// Minimum attributes needed
    pub min_attributes: HashMap<String, i32>,
    /// Required theories
    pub required_theories: Vec<String>,
    /// Faction reputation requirements
    pub faction_requirements: HashMap<String, i32>,
    /// Other items needed
    pub required_items: Vec<ItemId>,
}

impl EducationalItem {
    /// Create a basic research tool
    pub fn new_research_tool(
        name: String,
        tool_type: String,
        required_theory: String,
        min_understanding: f32,
        precision_bonus: f32,
    ) -> Self {
        Self {
            name: name.clone(),
            item_function: EducationalFunction::ResearchTool(ResearchTool {
                tool_type,
                required_theory: required_theory.clone(),
                min_understanding,
                precision_bonus,
                unlocked_methods: vec![LearningMethod::Experimentation, LearningMethod::Research],
            }),
            learning_bonuses: vec![
                LearningBonus {
                    bonus_type: BonusType::ExperimentalSuccess,
                    bonus_multiplier: precision_bonus,
                    conditions: BonusConditions {
                        applicable_theories: Some(vec![required_theory]),
                        applicable_methods: Some(vec![LearningMethod::Experimentation]),
                        min_level: None,
                        time_requirements: None,
                        environment: None,
                    },
                }
            ],
            usage_requirements: UsageRequirements::default(),
        }
    }

    /// Create a collaborative learning tool
    pub fn new_collaborative_tool(
        name: String,
        max_participants: i32,
        group_efficiency: f32,
        collaborative_theories: Vec<String>,
    ) -> Self {
        Self {
            name: name.clone(),
            item_function: EducationalFunction::CollaborativeTool(CollaborativeTool {
                max_participants,
                group_efficiency,
                faction_requirements: HashMap::new(),
                collaborative_theories: collaborative_theories.clone(),
            }),
            learning_bonuses: vec![
                LearningBonus {
                    bonus_type: BonusType::LearningEfficiency,
                    bonus_multiplier: group_efficiency,
                    conditions: BonusConditions {
                        applicable_theories: Some(collaborative_theories),
                        applicable_methods: Some(vec![LearningMethod::Study, LearningMethod::Teaching]),
                        min_level: None,
                        time_requirements: None,
                        environment: Some("group_setting".to_string()),
                    },
                }
            ],
            usage_requirements: UsageRequirements::default(),
        }
    }

    /// Create a theory unlock item (book, scroll, etc.)
    pub fn new_theory_unlock(name: String, theory_id: String) -> Self {
        Self {
            name: name.clone(),
            item_function: EducationalFunction::TheoryUnlock { theory_id: theory_id.clone() },
            learning_bonuses: vec![
                LearningBonus {
                    bonus_type: BonusType::UnderstandingGain,
                    bonus_multiplier: 0.5, // 50% faster initial understanding
                    conditions: BonusConditions {
                        applicable_theories: Some(vec![theory_id]),
                        applicable_methods: Some(vec![LearningMethod::Study]),
                        min_level: None,
                        time_requirements: None,
                        environment: None,
                    },
                }
            ],
            usage_requirements: UsageRequirements::default(),
        }
    }

    /// Create a method enhancer (specialized tool for specific learning methods)
    pub fn new_method_enhancer(name: String, method: LearningMethod, bonus: f32) -> Self {
        Self {
            name: name.clone(),
            item_function: EducationalFunction::MethodEnhancer { method: method.clone(), bonus },
            learning_bonuses: vec![
                LearningBonus {
                    bonus_type: BonusType::LearningEfficiency,
                    bonus_multiplier: bonus,
                    conditions: BonusConditions {
                        applicable_theories: None, // Applies to all theories
                        applicable_methods: Some(vec![method]),
                        min_level: None,
                        time_requirements: None,
                        environment: None,
                    },
                }
            ],
            usage_requirements: UsageRequirements::default(),
        }
    }

    /// Add usage requirement
    pub fn add_attribute_requirement(mut self, attribute: String, minimum: i32) -> Self {
        self.usage_requirements.min_attributes.insert(attribute, minimum);
        self
    }

    /// Add theory requirement
    pub fn add_theory_requirement(mut self, theory: String) -> Self {
        self.usage_requirements.required_theories.push(theory);
        self
    }

    /// Add faction requirement
    pub fn add_faction_requirement(mut self, faction: String, reputation: i32) -> Self {
        self.usage_requirements.faction_requirements.insert(faction, reputation);
        self
    }

    /// Check if item can be used by player
    pub fn can_be_used_by(
        &self,
        player_attributes: &HashMap<String, i32>,
        player_theories: &[String],
        player_factions: &HashMap<String, i32>,
    ) -> bool {
        self.usage_requirements.can_be_met(player_attributes, player_theories, player_factions)
    }

    /// Get all applicable bonuses for a specific theory and method
    pub fn get_applicable_bonuses(&self, theory_id: &str, method: &LearningMethod) -> Vec<&LearningBonus> {
        self.learning_bonuses
            .iter()
            .filter(|bonus| bonus.applies_to_theory(theory_id) && bonus.applies_to_method(method))
            .collect()
    }
}

impl LearningBonus {
    /// Check if bonus applies to a specific theory
    pub fn applies_to_theory(&self, theory_id: &str) -> bool {
        if let Some(ref theories) = self.conditions.applicable_theories {
            theories.contains(&theory_id.to_string())
        } else {
            true // Applies to all theories if none specified
        }
    }

    /// Check if bonus applies to a specific learning method
    pub fn applies_to_method(&self, method: &LearningMethod) -> bool {
        if let Some(ref methods) = self.conditions.applicable_methods {
            methods.contains(method)
        } else {
            true // Applies to all methods if none specified
        }
    }

    /// Check if bonus is currently active (time and environment conditions)
    pub fn is_currently_active(&self, current_environment: Option<&str>) -> bool {
        // Check environment requirements
        if let Some(ref required_env) = self.conditions.environment {
            if let Some(current_env) = current_environment {
                if current_env != required_env {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Time requirements could be checked here with world time system
        // For now, assume all time requirements are met

        true
    }
}

impl UsageRequirements {
    /// Create default usage requirements (no restrictions)
    pub fn default() -> Self {
        Self {
            min_attributes: HashMap::new(),
            required_theories: Vec::new(),
            faction_requirements: HashMap::new(),
            required_items: Vec::new(),
        }
    }

    /// Check if requirements can be met by player
    pub fn can_be_met(
        &self,
        player_attributes: &HashMap<String, i32>,
        player_theories: &[String],
        player_factions: &HashMap<String, i32>,
    ) -> bool {
        // Check attribute requirements
        for (attr, required) in &self.min_attributes {
            if let Some(&current) = player_attributes.get(attr) {
                if current < *required {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Check theory requirements
        for theory in &self.required_theories {
            if !player_theories.contains(theory) {
                return false;
            }
        }

        // Check faction requirements
        for (faction, required) in &self.faction_requirements {
            if let Some(&current) = player_factions.get(faction) {
                if current < *required {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

/// Factory functions for common educational items
pub struct EducationalItemFactory;

impl EducationalItemFactory {
    /// Create a resonance measuring device
    pub fn resonance_measuring_device() -> EducationalItem {
        EducationalItem::new_research_tool(
            "Resonance Measuring Device".to_string(),
            "measuring_instrument".to_string(),
            "harmonic_fundamentals".to_string(),
            0.5,
            0.5, // +50% precision for experiments
        )
        .add_attribute_requirement("resonance_sensitivity".to_string(), 30)
    }

    /// Create a collaborative study circle
    pub fn collaborative_study_circle() -> EducationalItem {
        EducationalItem::new_collaborative_tool(
            "Collaborative Study Circle".to_string(),
            5,
            0.75, // +75% efficiency when used with NPCs
            vec![
                "harmonic_fundamentals".to_string(),
                "crystal_structures".to_string(),
                "mental_resonance".to_string(),
            ],
        )
        .add_faction_requirement("MagistersCouncil".to_string(), 25)
    }

    /// Create a crystal synthesis laboratory
    pub fn crystal_synthesis_laboratory() -> EducationalItem {
        EducationalItem::new_research_tool(
            "Crystal Synthesis Laboratory".to_string(),
            "laboratory".to_string(),
            "crystal_structures".to_string(),
            1.0, // Must master crystal structures
            1.0, // +100% precision for crystal research
        )
        .add_attribute_requirement("mental_acuity".to_string(), 60)
        .add_theory_requirement("harmonic_fundamentals".to_string())
    }

    /// Create a theory textbook
    pub fn theory_textbook(theory_id: String, theory_name: String) -> EducationalItem {
        EducationalItem::new_theory_unlock(
            format!("Textbook: {}", theory_name),
            theory_id,
        )
    }

    /// Create an observation telescope
    pub fn observation_telescope() -> EducationalItem {
        EducationalItem::new_method_enhancer(
            "Precision Telescope".to_string(),
            LearningMethod::Observation,
            0.3, // +30% efficiency for observation learning
        )
        .add_attribute_requirement("resonance_sensitivity".to_string(), 25)
    }

    /// Create a meditation focus
    pub fn meditation_focus() -> EducationalItem {
        EducationalItem::new_method_enhancer(
            "Meditation Focus Crystal".to_string(),
            LearningMethod::Study,
            0.4, // +40% efficiency for study learning
        )
        .add_theory_requirement("mental_resonance".to_string())
    }

    /// Create experimental apparatus
    pub fn experimental_apparatus() -> EducationalItem {
        EducationalItem::new_method_enhancer(
            "Advanced Experimental Apparatus".to_string(),
            LearningMethod::Experimentation,
            0.6, // +60% efficiency for experimentation
        )
        .add_attribute_requirement("mental_acuity".to_string(), 40)
        .add_theory_requirement("harmonic_fundamentals".to_string())
    }
}

/// Faction-specific educational item factories
pub struct FactionItemFactory;

impl FactionItemFactory {
    // MAGISTERS' COUNCIL - Academic Excellence Items

    /// Council Scholar's Circlet - Head equipment for academic mastery
    pub fn council_scholars_circlet() -> EducationalItem {
        EducationalItem {
            name: "Council Scholar's Circlet".to_string(),
            item_function: EducationalFunction::MethodEnhancer {
                method: LearningMethod::Study,
                bonus: 0.4
            },
            learning_bonuses: vec![
                LearningBonus {
                    bonus_type: BonusType::LearningEfficiency,
                    bonus_multiplier: 0.4, // +40% Study efficiency
                    conditions: BonusConditions {
                        applicable_theories: None, // All theories
                        applicable_methods: Some(vec![LearningMethod::Study]),
                        min_level: None,
                        time_requirements: None,
                        environment: None,
                    },
                },
                LearningBonus {
                    bonus_type: BonusType::ExperienceBonus,
                    bonus_multiplier: -0.2, // -20% Experimentation (too cautious)
                    conditions: BonusConditions {
                        applicable_theories: None,
                        applicable_methods: Some(vec![LearningMethod::Experimentation]),
                        min_level: None,
                        time_requirements: None,
                        environment: None,
                    },
                }
            ],
            usage_requirements: UsageRequirements {
                min_attributes: {
                    let mut attrs = HashMap::new();
                    attrs.insert("mental_acuity".to_string(), 60);
                    attrs
                },
                required_theories: vec![
                    "harmonic_fundamentals".to_string(),
                    "crystal_structures".to_string(),
                    "mental_resonance".to_string(),
                    "bio_resonance".to_string(),
                    "detection_arrays".to_string(),
                ],
                faction_requirements: {
                    let mut factions = HashMap::new();
                    factions.insert("MagistersCouncil".to_string(), 75); // Allied tier
                    factions
                },
                required_items: Vec::new(),
            },
        }
    }

    /// Codified Theory Compendium - Cross-referenced academic knowledge
    pub fn codified_theory_compendium() -> EducationalItem {
        EducationalItem {
            name: "Codified Theory Compendium".to_string(),
            item_function: EducationalFunction::KnowledgeArchive {
                theories: vec![
                    "harmonic_fundamentals".to_string(),
                    "crystal_structures".to_string(),
                    "mental_resonance".to_string(),
                    "bio_resonance".to_string(),
                    "detection_arrays".to_string(),
                ]
            },
            learning_bonuses: vec![
                LearningBonus {
                    bonus_type: BonusType::UnderstandingGain,
                    bonus_multiplier: 0.3, // +30% Understanding gain for Tier 1-2
                    conditions: BonusConditions {
                        applicable_theories: Some(vec![
                            "harmonic_fundamentals".to_string(),
                            "crystal_structures".to_string(),
                            "mental_resonance".to_string(),
                            "bio_resonance".to_string(),
                            "detection_arrays".to_string(),
                        ]),
                        applicable_methods: Some(vec![LearningMethod::Study, LearningMethod::Research]),
                        min_level: None,
                        time_requirements: None,
                        environment: None,
                    },
                }
            ],
            usage_requirements: UsageRequirements {
                min_attributes: HashMap::new(),
                required_theories: Vec::new(),
                faction_requirements: {
                    let mut factions = HashMap::new();
                    factions.insert("MagistersCouncil".to_string(), 25); // Friendly tier
                    factions
                },
                required_items: Vec::new(),
            },
        }
    }

    /// Academy Research Laboratory - Controlled experimental environment
    pub fn academy_research_laboratory() -> EducationalItem {
        EducationalItem::new_research_tool(
            "Academy Research Laboratory".to_string(),
            "controlled_laboratory".to_string(),
            "crystal_structures".to_string(),
            0.8, // High understanding required
            1.0, // +100% precision for controlled experiments
        )
        .add_attribute_requirement("mental_acuity".to_string(), 80)
        .add_faction_requirement("MagistersCouncil".to_string(), 100) // Champion tier
        .add_theory_requirement("harmonic_fundamentals".to_string())
        .add_theory_requirement("detection_arrays".to_string())
    }

    // ORDER OF NATURAL HARMONY - Spiritual Balance Items

    /// Harmony Meditation Stone - Natural rhythm enhancement
    pub fn harmony_meditation_stone() -> EducationalItem {
        EducationalItem {
            name: "Harmony Meditation Stone".to_string(),
            item_function: EducationalFunction::MethodEnhancer {
                method: LearningMethod::Study,
                bonus: 0.35
            },
            learning_bonuses: vec![
                LearningBonus {
                    bonus_type: BonusType::LearningEfficiency,
                    bonus_multiplier: 0.35, // +35% Study efficiency
                    conditions: BonusConditions {
                        applicable_theories: None,
                        applicable_methods: Some(vec![LearningMethod::Study]),
                        min_level: None,
                        time_requirements: Some(TimeRequirement {
                            effective_hours: vec![5, 6, 18, 19, 20], // Dawn/dusk hours
                            seasons: None,
                        }),
                        environment: Some("natural".to_string()),
                    },
                },
                LearningBonus {
                    bonus_type: BonusType::EnergyCostReduction,
                    bonus_multiplier: 0.25, // +25% energy recovery
                    conditions: BonusConditions {
                        applicable_theories: None,
                        applicable_methods: None, // All methods
                        min_level: None,
                        time_requirements: None,
                        environment: None,
                    },
                }
            ],
            usage_requirements: UsageRequirements {
                min_attributes: {
                    let mut attrs = HashMap::new();
                    attrs.insert("resonance_sensitivity".to_string(), 40);
                    attrs
                },
                required_theories: Vec::new(),
                faction_requirements: {
                    let mut factions = HashMap::new();
                    factions.insert("OrderOfNaturalHarmony".to_string(), 25); // Friendly tier
                    factions
                },
                required_items: Vec::new(),
            },
        }
    }

    /// Living Crystal Garden - Symbiotic growth system
    pub fn living_crystal_garden() -> EducationalItem {
        EducationalItem::new_research_tool(
            "Living Crystal Garden".to_string(),
            "symbiotic_laboratory".to_string(),
            "bio_resonance".to_string(),
            0.6,
            0.4, // +40% precision for bio-resonance research
        )
        .add_attribute_requirement("resonance_sensitivity".to_string(), 50)
        .add_faction_requirement("OrderOfNaturalHarmony".to_string(), 75) // Allied tier
        .add_theory_requirement("bio_resonance".to_string())
    }

    /// Nature's Wisdom Tome - Seasonal knowledge cycles
    pub fn natures_wisdom_tome() -> EducationalItem {
        EducationalItem {
            name: "Nature's Wisdom Tome".to_string(),
            item_function: EducationalFunction::KnowledgeArchive {
                theories: vec![
                    "bio_resonance".to_string(),
                    "detection_arrays".to_string(),
                    "harmonic_fundamentals".to_string(),
                ]
            },
            learning_bonuses: vec![
                LearningBonus {
                    bonus_type: BonusType::LearningEfficiency,
                    bonus_multiplier: 0.45, // +45% for nature-related theories
                    conditions: BonusConditions {
                        applicable_theories: Some(vec![
                            "bio_resonance".to_string(),
                            "detection_arrays".to_string(),
                        ]),
                        applicable_methods: Some(vec![
                            LearningMethod::Study,
                            LearningMethod::Observation
                        ]),
                        min_level: None,
                        time_requirements: Some(TimeRequirement {
                            effective_hours: vec![], // Any time
                            seasons: Some(vec!["spring".to_string(), "summer".to_string()]),
                        }),
                        environment: Some("natural".to_string()),
                    },
                }
            ],
            usage_requirements: UsageRequirements {
                min_attributes: HashMap::new(),
                required_theories: Vec::new(),
                faction_requirements: {
                    let mut factions = HashMap::new();
                    factions.insert("OrderOfNaturalHarmony".to_string(), 75); // Allied tier
                    factions
                },
                required_items: Vec::new(),
            },
        }
    }

    // INDUSTRIAL CONSORTIUM - Technological Innovation Items

    /// Efficiency Optimizer Goggles - Process analysis enhancement
    pub fn efficiency_optimizer_goggles() -> EducationalItem {
        EducationalItem {
            name: "Efficiency Optimizer Goggles".to_string(),
            item_function: EducationalFunction::MethodEnhancer {
                method: LearningMethod::Experimentation,
                bonus: 0.25
            },
            learning_bonuses: vec![
                LearningBonus {
                    bonus_type: BonusType::LearningEfficiency,
                    bonus_multiplier: 0.25, // +25% Experimentation efficiency
                    conditions: BonusConditions {
                        applicable_theories: None,
                        applicable_methods: Some(vec![LearningMethod::Experimentation]),
                        min_level: None,
                        time_requirements: None,
                        environment: None,
                    },
                },
                LearningBonus {
                    bonus_type: BonusType::ResearchSpeed,
                    bonus_multiplier: 0.35, // +35% Research speed
                    conditions: BonusConditions {
                        applicable_theories: None,
                        applicable_methods: Some(vec![LearningMethod::Research]),
                        min_level: None,
                        time_requirements: None,
                        environment: Some("workshop".to_string()),
                    },
                }
            ],
            usage_requirements: UsageRequirements {
                min_attributes: {
                    let mut attrs = HashMap::new();
                    attrs.insert("mental_acuity".to_string(), 45);
                    attrs
                },
                required_theories: Vec::new(),
                faction_requirements: {
                    let mut factions = HashMap::new();
                    factions.insert("IndustrialConsortium".to_string(), 25); // Friendly tier
                    factions
                },
                required_items: Vec::new(),
            },
        }
    }

    /// Advanced Experimental Apparatus - High-risk, high-reward research
    pub fn advanced_experimental_apparatus() -> EducationalItem {
        EducationalItem::new_research_tool(
            "Advanced Experimental Apparatus".to_string(),
            "industrial_laboratory".to_string(),
            "light_manipulation".to_string(),
            0.7,
            0.6, // +60% experimentation precision
        )
        .add_attribute_requirement("mental_acuity".to_string(), 65)
        .add_faction_requirement("IndustrialConsortium".to_string(), 75) // Allied tier
        .add_theory_requirement("resonance_amplification".to_string())
    }

    /// Innovation Database - Commercial magical techniques
    pub fn innovation_database() -> EducationalItem {
        EducationalItem {
            name: "Innovation Database".to_string(),
            item_function: EducationalFunction::KnowledgeArchive {
                theories: vec![
                    "light_manipulation".to_string(),
                    "resonance_amplification".to_string(),
                    "sympathetic_networks".to_string(),
                ]
            },
            learning_bonuses: vec![
                LearningBonus {
                    bonus_type: BonusType::LearningEfficiency,
                    bonus_multiplier: 0.3, // +30% for commercial theories
                    conditions: BonusConditions {
                        applicable_theories: Some(vec![
                            "light_manipulation".to_string(),
                            "resonance_amplification".to_string(),
                        ]),
                        applicable_methods: Some(vec![
                            LearningMethod::Research,
                            LearningMethod::Experimentation
                        ]),
                        min_level: None,
                        time_requirements: None,
                        environment: None,
                    },
                }
            ],
            usage_requirements: UsageRequirements {
                min_attributes: HashMap::new(),
                required_theories: Vec::new(),
                faction_requirements: {
                    let mut factions = HashMap::new();
                    factions.insert("IndustrialConsortium".to_string(), 25); // Friendly tier
                    factions
                },
                required_items: Vec::new(),
            },
        }
    }

    // UNDERGROUND NETWORK - Revolutionary Methods Items

    /// Forbidden Knowledge Cache - Dangerous but powerful learning
    pub fn forbidden_knowledge_cache() -> EducationalItem {
        EducationalItem {
            name: "Forbidden Knowledge Cache".to_string(),
            item_function: EducationalFunction::KnowledgeArchive {
                theories: vec![
                    "sympathetic_networks".to_string(),
                    "theoretical_synthesis".to_string(),
                ]
            },
            learning_bonuses: vec![
                LearningBonus {
                    bonus_type: BonusType::LearningEfficiency,
                    bonus_multiplier: 0.5, // +50% for dangerous theories
                    conditions: BonusConditions {
                        applicable_theories: Some(vec![
                            "sympathetic_networks".to_string(),
                            "theoretical_synthesis".to_string(),
                        ]),
                        applicable_methods: Some(vec![
                            LearningMethod::Research,
                            LearningMethod::Experimentation
                        ]),
                        min_level: None,
                        time_requirements: None,
                        environment: Some("hidden".to_string()),
                    },
                }
            ],
            usage_requirements: UsageRequirements {
                min_attributes: HashMap::new(),
                required_theories: Vec::new(),
                faction_requirements: {
                    let mut factions = HashMap::new();
                    factions.insert("UndergroundNetwork".to_string(), 25); // Friendly tier
                    factions
                },
                required_items: Vec::new(),
            },
        }
    }

    /// Experimental Risk Amplifier - Maximum risk, maximum reward
    pub fn experimental_risk_amplifier() -> EducationalItem {
        EducationalItem::new_research_tool(
            "Experimental Risk Amplifier".to_string(),
            "dangerous_apparatus".to_string(),
            "theoretical_synthesis".to_string(),
            0.8,
            0.8, // +80% experimentation efficiency
        )
        .add_attribute_requirement("resonance_sensitivity".to_string(), 70)
        .add_faction_requirement("UndergroundNetwork".to_string(), 75) // Allied tier
        .add_theory_requirement("sympathetic_networks".to_string())
    }

    /// Black Market Research Tools - Variable quality illegal equipment
    pub fn black_market_research_tools() -> EducationalItem {
        EducationalItem::new_research_tool(
            "Black Market Research Tools".to_string(),
            "illegal_apparatus".to_string(),
            "resonance_amplification".to_string(),
            0.5,
            0.45, // +45% research efficiency (variable)
        )
        .add_faction_requirement("UndergroundNetwork".to_string(), 75) // Allied tier
    }

    // NEUTRAL SCHOLARS - Synthesis & Balance Items

    /// Diplomatic Synthesis Lens - Cross-cultural analysis
    pub fn diplomatic_synthesis_lens() -> EducationalItem {
        EducationalItem {
            name: "Diplomatic Synthesis Lens".to_string(),
            item_function: EducationalFunction::MethodEnhancer {
                method: LearningMethod::Study,
                bonus: 0.3
            },
            learning_bonuses: vec![
                LearningBonus {
                    bonus_type: BonusType::LearningEfficiency,
                    bonus_multiplier: 0.3, // +30% when studying multiple factions
                    conditions: BonusConditions {
                        applicable_theories: None, // All theories
                        applicable_methods: Some(vec![LearningMethod::Study, LearningMethod::Research]),
                        min_level: None,
                        time_requirements: None,
                        environment: None,
                    },
                }
            ],
            usage_requirements: UsageRequirements {
                min_attributes: HashMap::new(),
                required_theories: Vec::new(),
                faction_requirements: {
                    let mut factions = HashMap::new();
                    factions.insert("NeutralScholars".to_string(), 25); // Friendly tier
                    factions
                },
                required_items: Vec::new(),
            },
        }
    }

    /// Universal Theory Framework - Cross-faction synthesis
    pub fn universal_theory_framework() -> EducationalItem {
        EducationalItem {
            name: "Universal Theory Framework".to_string(),
            item_function: EducationalFunction::KnowledgeArchive {
                theories: vec![
                    "harmonic_fundamentals".to_string(),
                    "crystal_structures".to_string(),
                    "mental_resonance".to_string(),
                    "bio_resonance".to_string(),
                    "detection_arrays".to_string(),
                    "light_manipulation".to_string(),
                    "resonance_amplification".to_string(),
                    "sympathetic_networks".to_string(),
                    "theoretical_synthesis".to_string(),
                ]
            },
            learning_bonuses: vec![
                LearningBonus {
                    bonus_type: BonusType::LearningEfficiency,
                    bonus_multiplier: 0.35, // +35% for any theory
                    conditions: BonusConditions {
                        applicable_theories: None, // All theories
                        applicable_methods: None, // All methods
                        min_level: None,
                        time_requirements: None,
                        environment: None,
                    },
                },
                LearningBonus {
                    bonus_type: BonusType::LearningEfficiency,
                    bonus_multiplier: 0.45, // +45% bonus for theory combinations
                    conditions: BonusConditions {
                        applicable_theories: Some(vec![
                            "sympathetic_networks".to_string(),
                            "theoretical_synthesis".to_string(),
                        ]),
                        applicable_methods: Some(vec![LearningMethod::Research]),
                        min_level: None,
                        time_requirements: None,
                        environment: None,
                    },
                }
            ],
            usage_requirements: UsageRequirements {
                min_attributes: HashMap::new(),
                required_theories: vec![
                    "harmonic_fundamentals".to_string(), // At least one from each faction
                    "bio_resonance".to_string(),
                    "light_manipulation".to_string(),
                    "sympathetic_networks".to_string(),
                ],
                faction_requirements: {
                    let mut factions = HashMap::new();
                    factions.insert("NeutralScholars".to_string(), 75); // Allied tier
                    factions
                },
                required_items: Vec::new(),
            },
        }
    }

    /// Synthesis Master's Archive - Grand synthesis capabilities
    pub fn synthesis_masters_archive() -> EducationalItem {
        EducationalItem {
            name: "Synthesis Master's Archive".to_string(),
            item_function: EducationalFunction::KnowledgeArchive {
                theories: vec![
                    "theoretical_synthesis".to_string(),
                    "sympathetic_networks".to_string(),
                    "resonance_amplification".to_string(),
                    "light_manipulation".to_string(),
                ]
            },
            learning_bonuses: vec![
                LearningBonus {
                    bonus_type: BonusType::LearningEfficiency,
                    bonus_multiplier: 0.6, // +60% for synthesis and Tier 3 theories
                    conditions: BonusConditions {
                        applicable_theories: Some(vec![
                            "theoretical_synthesis".to_string(),
                            "sympathetic_networks".to_string(),
                            "resonance_amplification".to_string(),
                            "light_manipulation".to_string(),
                        ]),
                        applicable_methods: None, // All methods
                        min_level: None,
                        time_requirements: None,
                        environment: None,
                    },
                }
            ],
            usage_requirements: UsageRequirements {
                min_attributes: {
                    let mut attrs = HashMap::new();
                    attrs.insert("mental_acuity".to_string(), 90);
                    attrs.insert("resonance_sensitivity".to_string(), 80);
                    attrs
                },
                required_theories: vec![
                    "harmonic_fundamentals".to_string(),
                    "crystal_structures".to_string(),
                    "mental_resonance".to_string(),
                    "bio_resonance".to_string(),
                    "detection_arrays".to_string(),
                    "light_manipulation".to_string(),
                    "resonance_amplification".to_string(),
                    "sympathetic_networks".to_string(),
                ],
                faction_requirements: {
                    let mut factions = HashMap::new();
                    factions.insert("NeutralScholars".to_string(), 100); // Champion tier
                    factions
                },
                required_items: Vec::new(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_educational_item_creation() {
        let research_tool = EducationalItem::new_research_tool(
            "Test Tool".to_string(),
            "instrument".to_string(),
            "test_theory".to_string(),
            0.5,
            0.3,
        );

        assert_eq!(research_tool.name, "Test Tool");
        assert_eq!(research_tool.learning_bonuses.len(), 1);

        match &research_tool.item_function {
            EducationalFunction::ResearchTool(tool) => {
                assert_eq!(tool.required_theory, "test_theory");
                assert_eq!(tool.min_understanding, 0.5);
                assert_eq!(tool.precision_bonus, 0.3);
            }
            _ => panic!("Expected research tool"),
        }
    }

    #[test]
    fn test_learning_bonus_conditions() {
        let bonus = LearningBonus {
            bonus_type: BonusType::LearningEfficiency,
            bonus_multiplier: 0.5,
            conditions: BonusConditions {
                applicable_theories: Some(vec!["theory1".to_string(), "theory2".to_string()]),
                applicable_methods: Some(vec![LearningMethod::Study]),
                min_level: None,
                time_requirements: None,
                environment: None,
            },
        };

        // Should apply to specified theories
        assert!(bonus.applies_to_theory("theory1"));
        assert!(bonus.applies_to_theory("theory2"));
        assert!(!bonus.applies_to_theory("theory3"));

        // Should apply to specified methods
        assert!(bonus.applies_to_method(&LearningMethod::Study));
        assert!(!bonus.applies_to_method(&LearningMethod::Experimentation));
    }

    #[test]
    fn test_usage_requirements() {
        let mut requirements = UsageRequirements::default();
        requirements.min_attributes.insert("mental_acuity".to_string(), 50);
        requirements.required_theories.push("harmonic_fundamentals".to_string());
        requirements.faction_requirements.insert("MagistersCouncil".to_string(), 25);

        let mut player_attributes = HashMap::new();
        player_attributes.insert("mental_acuity".to_string(), 60);

        let player_theories = vec!["harmonic_fundamentals".to_string()];

        let mut player_factions = HashMap::new();
        player_factions.insert("MagistersCouncil".to_string(), 30);

        // Should meet all requirements
        assert!(requirements.can_be_met(&player_attributes, &player_theories, &player_factions));

        // Fail attribute requirement
        player_attributes.insert("mental_acuity".to_string(), 40);
        assert!(!requirements.can_be_met(&player_attributes, &player_theories, &player_factions));
    }

    #[test]
    fn test_collaborative_tool() {
        let collab_tool = EducationalItem::new_collaborative_tool(
            "Study Group".to_string(),
            4,
            0.8,
            vec!["theory1".to_string()],
        );

        if let EducationalFunction::CollaborativeTool(tool) = &collab_tool.item_function {
            assert_eq!(tool.max_participants, 4);
            assert_eq!(tool.group_efficiency, 0.8);
            assert_eq!(tool.collaborative_theories, vec!["theory1".to_string()]);
        } else {
            panic!("Expected collaborative tool");
        }
    }

    #[test]
    fn test_factory_items() {
        let measuring_device = EducationalItemFactory::resonance_measuring_device();
        assert!(measuring_device.name.contains("Resonance"));

        let study_circle = EducationalItemFactory::collaborative_study_circle();
        assert!(study_circle.name.contains("Collaborative"));

        let textbook = EducationalItemFactory::theory_textbook(
            "test_theory".to_string(),
            "Test Theory".to_string(),
        );
        assert!(textbook.name.contains("Textbook"));
    }
}