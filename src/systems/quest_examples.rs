//! Example quest definitions showcasing the educational quest system
//!
//! This module contains carefully designed quests that demonstrate:
//! - Progressive difficulty and theory requirements
//! - Faction integration and political consequences
//! - Educational objectives tied to scientific concepts
//! - Multiple solution paths and player agency
//! - Real-world learning applications

use crate::systems::quests::*;
use crate::systems::factions::FactionId;
use crate::systems::dialogue::{NPC, NPCPersonality, QuestDialogue, DialogueTree, DialogueNode, DialogueRequirements};
use std::collections::HashMap;

/// Create the complete set of example quests for the game
pub fn create_example_quests() -> Vec<QuestDefinition> {
    vec![
        create_resonance_foundation_quest(),
        create_crystal_analysis_quest(),
        create_diplomatic_balance_quest(),
        create_healing_research_quest(),
        create_unstable_site_investigation_quest(),
    ]
}

/// Quest 1: "Understanding Resonance" - Tutorial Level
/// Teaches fundamental harmonic principles through safe experimentation
fn create_resonance_foundation_quest() -> QuestDefinition {
    let mut faction_effects = HashMap::new();
    faction_effects.insert(FactionId::MagistersCouncil, 5); // Positive for following proper procedures
    faction_effects.insert(FactionId::NeutralScholars, 3); // Positive for academic progress

    let mut theory_bonuses = HashMap::new();
    theory_bonuses.insert("harmonic_fundamentals".to_string(), 0.15);

    let mut objective_reward = HashMap::new();
    objective_reward.insert("harmonic_fundamentals".to_string(), 0.05);

    QuestDefinition {
        id: "resonance_foundation".to_string(),
        title: "Understanding Resonance".to_string(),
        description: "The soft hum of crystals fills the Practice Hall as you begin your journey into the \
                     mysteries of sympathetic resonance. Under the gentle guidance of Tutorial Assistant \
                     Elara Starweaver, you'll discover how the universe itself sings in frequencies that \
                     can be learned, matched, and harmonized.\n\n\
                     This foundational quest will teach you to listen to the crystalline songs that have \
                     echoed through these halls for generations, revealing the elegant principles of frequency \
                     matching and energy conservation that form the bedrock of all magical understanding.\n\n\
                     As you work with your first resonance crystals, you'll begin to sense the invisible \
                     threads that connect all things - the sympathetic bonds that allow energy to flow \
                     from one system to another in perfect harmony. Every great mage's journey begins \
                     with this single, profound realization: magic is not about forcing your will upon \
                     the world, but about finding the frequency at which the world is already singing.".to_string(),
        category: QuestCategory::Tutorial,
        difficulty: QuestDifficulty::Beginner,

        requirements: QuestRequirements {
            theory_requirements: vec![], // No prerequisites - this is a starter quest
            faction_requirements: vec![],
            faction_restrictions: vec![],
            prerequisite_quests: vec![],
            attribute_requirements: AttributeRequirements {
                min_mental_acuity: Some(10),
                min_resonance_sensitivity: None,
                min_total_playtime: None,
            },
            capability_requirements: vec![],
            location_requirements: vec!["practice_hall".to_string()],
        },

        objectives: vec![
            QuestObjective {
                id: "visit_practice_hall".to_string(),
                description: "Step into the Practice Hall and feel the ancient crystals welcoming you to your magical studies. \
                             The carved walls here have witnessed thousands of students take their first steps into resonance theory.".to_string(),
                objective_type: ObjectiveType::VisitLocation {
                    location_id: "practice_hall".to_string()
                },
                optional: false,
                visible: true,
                completion_reward: ObjectiveReward {
                    experience: 10,
                    theory_insights: HashMap::new(),
                    faction_changes: HashMap::new(),
                    items: vec![],
                },
            },
            QuestObjective {
                id: "learn_harmonic_fundamentals".to_string(),
                description: "Immerse yourself in the study of Harmonic Fundamentals until the concepts become clear. \
                             Feel the knowledge settling into your mind like sediment in still water - each insight \
                             building upon the last until you achieve a foundational understanding (30%) of how \
                             frequencies create the music of magic.".to_string(),
                objective_type: ObjectiveType::LearnTheory {
                    theory_id: "harmonic_fundamentals".to_string(),
                    min_level: 0.3
                },
                optional: false,
                visible: true,
                completion_reward: ObjectiveReward {
                    experience: 25,
                    theory_insights: objective_reward.clone(),
                    faction_changes: HashMap::new(),
                    items: vec![],
                },
            },
            QuestObjective {
                id: "demonstrate_resonance".to_string(),
                description: "Channel your newfound understanding into action - perform a resonance demonstration \
                             that proves you can match frequencies and create sympathetic vibrations. Watch as \
                             theory transforms into reality before your eyes, the crystal responding to your \
                             careful attunement with a gentle, harmonic glow.".to_string(),
                objective_type: ObjectiveType::MagicalDemonstration {
                    theory_id: "harmonic_fundamentals".to_string(),
                    success_threshold: 0.7
                },
                optional: false,
                visible: false, // Becomes visible after learning theory
                completion_reward: ObjectiveReward {
                    experience: 30,
                    theory_insights: HashMap::new(),
                    faction_changes: HashMap::new(),
                    items: vec![],
                },
            },
            QuestObjective {
                id: "discuss_with_tutorial_assistant".to_string(),
                description: "Share your discoveries with Tutorial Assistant Elara Starweaver, whose encouraging \
                             smile and thoughtful questions will help you reflect on what you've learned. In this \
                             conversation, you'll begin to see how your individual insights connect to the larger \
                             tapestry of magical knowledge.".to_string(),
                objective_type: ObjectiveType::TalkToNPC {
                    npc_id: "tutorial_assistant".to_string(),
                    topic: Some("resonance_results".to_string())
                },
                optional: true,
                visible: true,
                completion_reward: ObjectiveReward {
                    experience: 15,
                    theory_insights: HashMap::new(),
                    faction_changes: HashMap::new(),
                    items: vec![],
                },
            },
        ],

        rewards: QuestRewards {
            experience: 100,
            attribute_bonuses: AttributeBonuses {
                mental_acuity: Some(2),
                resonance_sensitivity: Some(1),
            },
            theory_bonuses,
            faction_changes: faction_effects.clone(),
            items: vec!["basic_resonance_crystal".to_string()],
            new_capabilities: vec!["basic_frequency_matching".to_string()],
            unlocked_quests: vec!["crystal_analysis".to_string()],
        },

        faction_effects,
        educational_focus: EducationalObjectives {
            primary_concepts: vec![
                "Wave Physics".to_string(),
                "Harmonic Oscillation".to_string(),
                "Energy Conservation".to_string(),
            ],
            secondary_concepts: vec![
                "Frequency Matching".to_string(),
                "Resonance Phenomena".to_string(),
            ],
            applications: vec![
                "Crystal Tuning".to_string(),
                "Energy Efficiency Calculations".to_string(),
            ],
            problem_solving_methods: vec![
                "Systematic Observation".to_string(),
                "Hypothesis Testing".to_string(),
                "Data Recording".to_string(),
            ],
            assessment_criteria: vec![
                "Understanding of wave properties".to_string(),
                "Ability to predict resonance frequencies".to_string(),
                "Application of energy conservation principles".to_string(),
            ],
        },

        branching_paths: HashMap::new(), // Simple linear quest for beginners
        choices: vec![], // Will be added in Phase 1C
        involved_npcs: vec!["tutorial_assistant".to_string()],
        locations: vec!["practice_hall".to_string(), "tutorial_chamber".to_string()],
        estimated_duration: 45,
    }
}

/// Quest 2: "Crystal Analysis Project" - Intermediate Level
/// Deep dive into crystal structures with faction choice consequences
fn create_crystal_analysis_quest() -> QuestDefinition {
    let mut faction_effects = HashMap::new();
    faction_effects.insert(FactionId::NeutralScholars, 10); // Primary beneficiary
    faction_effects.insert(FactionId::IndustrialConsortium, 5); // Benefits from research

    let mut theory_bonuses = HashMap::new();
    theory_bonuses.insert("crystal_structures".to_string(), 0.2);
    theory_bonuses.insert("harmonic_fundamentals".to_string(), 0.1);

    // Create branching paths for different approaches
    let mut branching_paths = HashMap::new();

    // Academic approach - focuses on pure research
    let academic_requirements = QuestRequirements {
        theory_requirements: vec![("harmonic_fundamentals".to_string(), 0.4)],
        faction_requirements: vec![(FactionId::NeutralScholars, 10)],
        faction_restrictions: vec![],
        prerequisite_quests: vec![],
        attribute_requirements: AttributeRequirements {
            min_mental_acuity: Some(25),
            min_resonance_sensitivity: None,
            min_total_playtime: None,
        },
        capability_requirements: vec![],
        location_requirements: vec![],
    };

    let mut academic_faction_effects = HashMap::new();
    academic_faction_effects.insert(FactionId::NeutralScholars, 15);
    academic_faction_effects.insert(FactionId::MagistersCouncil, 8);

    branching_paths.insert("academic_approach".to_string(), QuestBranch {
        id: "academic_approach".to_string(),
        name: "Pure Research Path".to_string(),
        description: "Focus on fundamental understanding and theoretical implications".to_string(),
        requirements: academic_requirements,
        branch_objectives: vec![
            QuestObjective {
                id: "academic_crystal_study".to_string(),
                description: "Conduct detailed lattice structure analysis with Dr. Felix".to_string(),
                objective_type: ObjectiveType::TalkToNPC {
                    npc_id: "dr_felix".to_string(),
                    topic: Some("advanced_crystal_analysis".to_string())
                },
                optional: false,
                visible: true,
                completion_reward: ObjectiveReward {
                    experience: 40,
                    theory_insights: {
                        let mut insights = HashMap::new();
                        insights.insert("crystal_structures".to_string(), 0.1);
                        insights
                    },
                    faction_changes: HashMap::new(),
                    items: vec![],
                },
            },
            QuestObjective {
                id: "research_publication".to_string(),
                description: "Contribute to crystal research publication in the Archives".to_string(),
                objective_type: ObjectiveType::Research {
                    theory_id: "crystal_structures".to_string(),
                    research_points: 50
                },
                optional: false,
                visible: true,
                completion_reward: ObjectiveReward {
                    experience: 50,
                    theory_insights: HashMap::new(),
                    faction_changes: academic_faction_effects.clone(),
                    items: vec![],
                },
            },
        ],
        faction_implications: academic_faction_effects,
        educational_focus: EducationalObjectives {
            primary_concepts: vec![
                "Crystallography".to_string(),
                "Materials Science".to_string(),
                "Research Methodology".to_string(),
            ],
            secondary_concepts: vec![],
            applications: vec!["Academic Publishing".to_string()],
            problem_solving_methods: vec!["Peer Review".to_string(), "Literature Analysis".to_string()],
            assessment_criteria: vec!["Research Quality".to_string()],
        },
    });

    // Commercial approach - focuses on practical applications
    let commercial_requirements = QuestRequirements {
        theory_requirements: vec![("harmonic_fundamentals".to_string(), 0.3)],
        faction_requirements: vec![(FactionId::IndustrialConsortium, 15)],
        faction_restrictions: vec![],
        prerequisite_quests: vec![],
        attribute_requirements: AttributeRequirements {
            min_mental_acuity: Some(20),
            min_resonance_sensitivity: Some(15),
            min_total_playtime: None,
        },
        capability_requirements: vec![],
        location_requirements: vec![],
    };

    let mut commercial_faction_effects = HashMap::new();
    commercial_faction_effects.insert(FactionId::IndustrialConsortium, 20);
    commercial_faction_effects.insert(FactionId::OrderOfHarmony, -5); // Slightly negative due to commercialization

    branching_paths.insert("commercial_approach".to_string(), QuestBranch {
        id: "commercial_approach".to_string(),
        name: "Commercial Development Path".to_string(),
        description: "Focus on practical applications and market potential".to_string(),
        requirements: commercial_requirements,
        branch_objectives: vec![
            QuestObjective {
                id: "commercial_crystal_optimization".to_string(),
                description: "Work with Technician Marcus to optimize crystal efficiency".to_string(),
                objective_type: ObjectiveType::TalkToNPC {
                    npc_id: "technician_marcus".to_string(),
                    topic: Some("efficiency_optimization".to_string())
                },
                optional: false,
                visible: true,
                completion_reward: ObjectiveReward {
                    experience: 35,
                    theory_insights: HashMap::new(),
                    faction_changes: HashMap::new(),
                    items: vec!["optimized_crystal_prototype".to_string()],
                },
            },
            QuestObjective {
                id: "market_analysis".to_string(),
                description: "Analyze commercial applications for improved crystals".to_string(),
                objective_type: ObjectiveType::Research {
                    theory_id: "crystal_structures".to_string(),
                    research_points: 30
                },
                optional: false,
                visible: true,
                completion_reward: ObjectiveReward {
                    experience: 40,
                    theory_insights: HashMap::new(),
                    faction_changes: commercial_faction_effects.clone(),
                    items: vec![],
                },
            },
        ],
        faction_implications: commercial_faction_effects,
        educational_focus: EducationalObjectives {
            primary_concepts: vec![
                "Materials Engineering".to_string(),
                "Quality Optimization".to_string(),
                "Market Analysis".to_string(),
            ],
            secondary_concepts: vec![],
            applications: vec!["Industrial Production".to_string(), "Cost Optimization".to_string()],
            problem_solving_methods: vec!["Efficiency Analysis".to_string(), "Market Research".to_string()],
            assessment_criteria: vec!["Practical Implementation".to_string()],
        },
    });

    QuestDefinition {
        id: "crystal_analysis".to_string(),
        title: "Crystal Analysis Project".to_string(),
        description: "The Crystal Garden Laboratory awaits your deeper exploration into the intricate world \
                     of crystalline structures and their magical properties. Here, among towering crystal \
                     formations that pulse with inner light, you'll embark on a research journey that will \
                     shape not only your understanding but also your standing with the various factions.\n\n\
                     Dr. Felix Verdant and Technician Marcus Clearview represent two divergent paths: will you \
                     pursue the pure academic quest for knowledge, documenting crystalline phenomena for the \
                     advancement of theoretical understanding? Or will you focus on practical applications, \
                     optimizing crystal efficiency for the Industrial Consortium's commercial ventures?\n\n\
                     Each crystal tells a story written in molecular lattices and harmonic frequencies. As you \
                     learn to read these crystalline narratives, you'll discover that the choices you make \
                     in your research approach will echo through the political landscape of the magical \
                     community, influencing how different factions view your work and your potential.".to_string(),
        category: QuestCategory::Research,
        difficulty: QuestDifficulty::Intermediate,

        requirements: QuestRequirements {
            theory_requirements: vec![
                ("harmonic_fundamentals".to_string(), 0.5),
                ("crystal_structures".to_string(), 0.2),
            ],
            faction_requirements: vec![],
            faction_restrictions: vec![],
            prerequisite_quests: vec!["resonance_foundation".to_string()],
            attribute_requirements: AttributeRequirements {
                min_mental_acuity: Some(20),
                min_resonance_sensitivity: Some(10),
                min_total_playtime: Some(60),
            },
            capability_requirements: vec!["basic_frequency_matching".to_string()],
            location_requirements: vec![],
        },

        objectives: vec![
            QuestObjective {
                id: "visit_crystal_garden".to_string(),
                description: "Visit the Crystal Garden Laboratory to begin your analysis".to_string(),
                objective_type: ObjectiveType::VisitLocation {
                    location_id: "crystal_garden_lab".to_string()
                },
                optional: false,
                visible: true,
                completion_reward: ObjectiveReward {
                    experience: 15,
                    theory_insights: HashMap::new(),
                    faction_changes: HashMap::new(),
                    items: vec![],
                },
            },
            QuestObjective {
                id: "master_crystal_theory".to_string(),
                description: "Achieve 60% understanding in Crystal Lattice Theory".to_string(),
                objective_type: ObjectiveType::LearnTheory {
                    theory_id: "crystal_structures".to_string(),
                    min_level: 0.6
                },
                optional: false,
                visible: true,
                completion_reward: ObjectiveReward {
                    experience: 50,
                    theory_insights: {
                        let mut insights = HashMap::new();
                        insights.insert("crystal_structures".to_string(), 0.1);
                        insights
                    },
                    faction_changes: HashMap::new(),
                    items: vec![],
                },
            },
            QuestObjective {
                id: "choose_approach".to_string(),
                description: "Choose your research approach: Academic or Commercial".to_string(),
                objective_type: ObjectiveType::DiplomaticChoice {
                    choice_id: "research_approach".to_string(),
                    factions: vec![FactionId::NeutralScholars, FactionId::IndustrialConsortium]
                },
                optional: false,
                visible: false, // Becomes visible after mastering theory
                completion_reward: ObjectiveReward {
                    experience: 25,
                    theory_insights: HashMap::new(),
                    faction_changes: HashMap::new(),
                    items: vec![],
                },
            },
        ],

        rewards: QuestRewards {
            experience: 200,
            attribute_bonuses: AttributeBonuses {
                mental_acuity: Some(3),
                resonance_sensitivity: Some(2),
            },
            theory_bonuses,
            faction_changes: faction_effects.clone(),
            items: vec!["advanced_analysis_tools".to_string()],
            new_capabilities: vec!["crystal_quality_assessment".to_string()],
            unlocked_quests: vec!["diplomatic_balance".to_string(), "healing_research".to_string()],
        },

        faction_effects,
        educational_focus: EducationalObjectives {
            primary_concepts: vec![
                "Crystallography".to_string(),
                "Solid State Physics".to_string(),
                "Materials Science".to_string(),
            ],
            secondary_concepts: vec![
                "Quality Control".to_string(),
                "Research Design".to_string(),
            ],
            applications: vec![
                "Crystal Optimization".to_string(),
                "Efficiency Improvement".to_string(),
                "Quality Assessment".to_string(),
            ],
            problem_solving_methods: vec![
                "Systematic Analysis".to_string(),
                "Comparative Study".to_string(),
                "Data Interpretation".to_string(),
            ],
            assessment_criteria: vec![
                "Technical accuracy".to_string(),
                "Research methodology".to_string(),
                "Practical application".to_string(),
            ],
        },

        branching_paths,
        choices: vec![], // Will be added in Phase 1C
        involved_npcs: vec!["dr_felix".to_string(), "technician_marcus".to_string()],
        locations: vec!["crystal_garden_lab".to_string(), "resonance_observatory".to_string()],
        estimated_duration: 90,
    }
}

/// Quest 3: "The Diplomatic Balance" - Political Quest
/// Navigate faction tensions while advancing magical understanding
fn create_diplomatic_balance_quest() -> QuestDefinition {
    let _faction_effects: HashMap<crate::systems::factions::FactionId, i32> = HashMap::new();
    // This quest can significantly affect multiple factions based on choices

    let mut theory_bonuses = HashMap::new();
    theory_bonuses.insert("mental_resonance".to_string(), 0.15);
    theory_bonuses.insert("detection_arrays".to_string(), 0.1);

    QuestDefinition {
        id: "diplomatic_balance".to_string(),
        title: "The Diplomatic Balance".to_string(),
        description: "Tensions are rising between the Magisters' Council and the Underground Network \
                     over magical regulation policies. Ambassador Cordelia has requested your help \
                     in mediating a crucial negotiation that could determine the future of magical \
                     education and research freedom.".to_string(),
        category: QuestCategory::Political,
        difficulty: QuestDifficulty::Intermediate,

        requirements: QuestRequirements {
            theory_requirements: vec![
                ("mental_resonance".to_string(), 0.3),
                ("harmonic_fundamentals".to_string(), 0.6),
            ],
            faction_requirements: vec![],
            faction_restrictions: vec![
                // Cannot be too extreme in any faction to be trusted as mediator
                (FactionId::MagistersCouncil, 70),
                (FactionId::UndergroundNetwork, 70),
            ],
            prerequisite_quests: vec!["crystal_analysis".to_string()],
            attribute_requirements: AttributeRequirements {
                min_mental_acuity: Some(25),
                min_resonance_sensitivity: Some(20),
                min_total_playtime: Some(120),
            },
            capability_requirements: vec!["crystal_quality_assessment".to_string()],
            location_requirements: vec![],
        },

        objectives: vec![
            QuestObjective {
                id: "meet_ambassador".to_string(),
                description: "Meet with Ambassador Cordelia in the Diplomacy Hall".to_string(),
                objective_type: ObjectiveType::TalkToNPC {
                    npc_id: "ambassador_cordelia".to_string(),
                    topic: Some("faction_tensions".to_string())
                },
                optional: false,
                visible: true,
                completion_reward: ObjectiveReward {
                    experience: 20,
                    theory_insights: HashMap::new(),
                    faction_changes: {
                        let mut changes = HashMap::new();
                        changes.insert(FactionId::NeutralScholars, 5);
                        changes
                    },
                    items: vec![],
                },
            },
            QuestObjective {
                id: "understand_council_position".to_string(),
                description: "Learn the Magisters' Council perspective from Observer Lyra".to_string(),
                objective_type: ObjectiveType::TalkToNPC {
                    npc_id: "observer_lyra".to_string(),
                    topic: Some("regulation_concerns".to_string())
                },
                optional: false,
                visible: true,
                completion_reward: ObjectiveReward {
                    experience: 25,
                    theory_insights: HashMap::new(),
                    faction_changes: HashMap::new(),
                    items: vec![],
                },
            },
            QuestObjective {
                id: "understand_underground_position".to_string(),
                description: "Learn the Underground Network perspective from Echo Voidwalker".to_string(),
                objective_type: ObjectiveType::TalkToNPC {
                    npc_id: "echo_voidwalker".to_string(),
                    topic: Some("freedom_concerns".to_string())
                },
                optional: false,
                visible: true,
                completion_reward: ObjectiveReward {
                    experience: 25,
                    theory_insights: HashMap::new(),
                    faction_changes: HashMap::new(),
                    items: vec![],
                },
            },
            QuestObjective {
                id: "develop_mental_resonance".to_string(),
                description: "Develop Mental Resonance Theory to better understand diplomatic dynamics".to_string(),
                objective_type: ObjectiveType::LearnTheory {
                    theory_id: "mental_resonance".to_string(),
                    min_level: 0.5
                },
                optional: false,
                visible: false, // Becomes visible after talking to both sides
                completion_reward: ObjectiveReward {
                    experience: 40,
                    theory_insights: {
                        let mut insights = HashMap::new();
                        insights.insert("mental_resonance".to_string(), 0.1);
                        insights
                    },
                    faction_changes: HashMap::new(),
                    items: vec![],
                },
            },
            QuestObjective {
                id: "mediate_negotiation".to_string(),
                description: "Use your understanding to mediate the faction negotiation".to_string(),
                objective_type: ObjectiveType::DiplomaticChoice {
                    choice_id: "negotiation_outcome".to_string(),
                    factions: vec![FactionId::MagistersCouncil, FactionId::UndergroundNetwork]
                },
                optional: false,
                visible: false, // Becomes visible after developing mental resonance
                completion_reward: ObjectiveReward {
                    experience: 60,
                    theory_insights: HashMap::new(),
                    faction_changes: HashMap::new(), // Varies based on choice
                    items: vec![],
                },
            },
        ],

        rewards: QuestRewards {
            experience: 250,
            attribute_bonuses: AttributeBonuses {
                mental_acuity: Some(4),
                resonance_sensitivity: Some(1),
            },
            theory_bonuses,
            faction_changes: HashMap::new(), // Determined by player choices
            items: vec!["diplomatic_resonance_crystal".to_string()],
            new_capabilities: vec!["diplomatic_sensing".to_string(), "faction_mediation".to_string()],
            unlocked_quests: vec!["healing_research".to_string(), "unstable_site_investigation".to_string()],
        },

        faction_effects: HashMap::new(), // Variable based on choices
        educational_focus: EducationalObjectives {
            primary_concepts: vec![
                "Psychology".to_string(),
                "Negotiation Theory".to_string(),
                "Systems Thinking".to_string(),
            ],
            secondary_concepts: vec![
                "Political Science".to_string(),
                "Conflict Resolution".to_string(),
            ],
            applications: vec![
                "Diplomatic Communication".to_string(),
                "Tension Resolution".to_string(),
                "Stakeholder Management".to_string(),
            ],
            problem_solving_methods: vec![
                "Active Listening".to_string(),
                "Perspective Taking".to_string(),
                "Compromise Development".to_string(),
            ],
            assessment_criteria: vec![
                "Understanding of all perspectives".to_string(),
                "Quality of proposed solutions".to_string(),
                "Diplomatic skill demonstration".to_string(),
            ],
        },

        branching_paths: HashMap::new(), // Could add complex negotiation branches
        choices: vec![], // Will be added in future phases
        involved_npcs: vec![
            "ambassador_cordelia".to_string(),
            "observer_lyra".to_string(),
            "echo_voidwalker".to_string()
        ],
        locations: vec![
            "faction_diplomacy_hall".to_string(),
            "resonance_observatory".to_string(),
            "unstable_resonance_site".to_string()
        ],
        estimated_duration: 120,
    }
}

/// Quest 4: "Healing Research Initiative" - Practical Application Quest
/// Apply bio-resonance theory to develop healing techniques
fn create_healing_research_quest() -> QuestDefinition {
    let mut faction_effects = HashMap::new();
    faction_effects.insert(FactionId::OrderOfHarmony, 15); // Strong positive for healing work
    faction_effects.insert(FactionId::NeutralScholars, 8); // Positive for research
    faction_effects.insert(FactionId::IndustrialConsortium, 5); // Some interest in applications

    let mut theory_bonuses = HashMap::new();
    theory_bonuses.insert("bio_resonance".to_string(), 0.25);
    theory_bonuses.insert("mental_resonance".to_string(), 0.1);

    QuestDefinition {
        id: "healing_research".to_string(),
        title: "Healing Research Initiative".to_string(),
        description: "Healer Seraphina has discovered promising connections between plant biology \
                     and bio-resonance theory. She needs your help to develop new healing \
                     techniques that could revolutionize magical medicine while staying true \
                     to natural harmony principles.".to_string(),
        category: QuestCategory::Practical,
        difficulty: QuestDifficulty::Advanced,

        requirements: QuestRequirements {
            theory_requirements: vec![
                ("bio_resonance".to_string(), 0.2),
                ("crystal_structures".to_string(), 0.4),
                ("mental_resonance".to_string(), 0.3),
            ],
            faction_requirements: vec![],
            faction_restrictions: vec![
                // Cannot be too opposed to natural harmony
                (FactionId::OrderOfHarmony, -30),
            ],
            prerequisite_quests: vec!["crystal_analysis".to_string()],
            attribute_requirements: AttributeRequirements {
                min_mental_acuity: Some(30),
                min_resonance_sensitivity: Some(25),
                min_total_playtime: Some(180),
            },
            capability_requirements: vec!["crystal_quality_assessment".to_string()],
            location_requirements: vec![],
        },

        objectives: vec![
            QuestObjective {
                id: "consult_healer_seraphina".to_string(),
                description: "Discuss the healing research project with Healer Seraphina".to_string(),
                objective_type: ObjectiveType::TalkToNPC {
                    npc_id: "healer_seraphina".to_string(),
                    topic: Some("healing_research_proposal".to_string())
                },
                optional: false,
                visible: true,
                completion_reward: ObjectiveReward {
                    experience: 25,
                    theory_insights: HashMap::new(),
                    faction_changes: {
                        let mut changes = HashMap::new();
                        changes.insert(FactionId::OrderOfHarmony, 5);
                        changes
                    },
                    items: vec![],
                },
            },
            QuestObjective {
                id: "study_plant_crystal_interactions".to_string(),
                description: "Study how crystals affect plant growth in the Garden Laboratory".to_string(),
                objective_type: ObjectiveType::LearningActivity {
                    theory_id: "bio_resonance".to_string(),
                    method: "Experimentation".to_string(),
                    duration: 60
                },
                optional: false,
                visible: true,
                completion_reward: ObjectiveReward {
                    experience: 40,
                    theory_insights: {
                        let mut insights = HashMap::new();
                        insights.insert("bio_resonance".to_string(), 0.1);
                        insights
                    },
                    faction_changes: HashMap::new(),
                    items: vec!["plant_resonance_data".to_string()],
                },
            },
            QuestObjective {
                id: "master_bio_resonance".to_string(),
                description: "Achieve 70% understanding in Biological Sympathetic Healing".to_string(),
                objective_type: ObjectiveType::LearnTheory {
                    theory_id: "bio_resonance".to_string(),
                    min_level: 0.7
                },
                optional: false,
                visible: true,
                completion_reward: ObjectiveReward {
                    experience: 60,
                    theory_insights: {
                        let mut insights = HashMap::new();
                        insights.insert("bio_resonance".to_string(), 0.15);
                        insights
                    },
                    faction_changes: HashMap::new(),
                    items: vec![],
                },
            },
            QuestObjective {
                id: "develop_healing_technique".to_string(),
                description: "Develop a new healing technique combining your research".to_string(),
                objective_type: ObjectiveType::MagicalDemonstration {
                    theory_id: "bio_resonance".to_string(),
                    success_threshold: 0.8
                },
                optional: false,
                visible: false, // Becomes visible after mastering bio-resonance
                completion_reward: ObjectiveReward {
                    experience: 80,
                    theory_insights: HashMap::new(),
                    faction_changes: {
                        let mut changes = HashMap::new();
                        changes.insert(FactionId::OrderOfHarmony, 10);
                        changes.insert(FactionId::NeutralScholars, 5);
                        changes
                    },
                    items: vec!["healing_technique_documentation".to_string()],
                },
            },
            QuestObjective {
                id: "teach_dr_felix".to_string(),
                description: "Share your healing technique insights with Dr. Felix".to_string(),
                objective_type: ObjectiveType::TeachTheory {
                    npc_id: "dr_felix".to_string(),
                    theory_id: "bio_resonance".to_string()
                },
                optional: true,
                visible: true,
                completion_reward: ObjectiveReward {
                    experience: 50,
                    theory_insights: {
                        let mut insights = HashMap::new();
                        insights.insert("bio_resonance".to_string(), 0.05);
                        insights
                    },
                    faction_changes: {
                        let mut changes = HashMap::new();
                        changes.insert(FactionId::NeutralScholars, 8);
                        changes
                    },
                    items: vec![],
                },
            },
        ],

        rewards: QuestRewards {
            experience: 400,
            attribute_bonuses: AttributeBonuses {
                mental_acuity: Some(3),
                resonance_sensitivity: Some(5),
            },
            theory_bonuses,
            faction_changes: faction_effects.clone(),
            items: vec!["advanced_healing_crystals".to_string(), "bio_resonance_equipment".to_string()],
            new_capabilities: vec![
                "healing_spells".to_string(),
                "plant_magic_interaction".to_string(),
                "bio_resonance_diagnosis".to_string()
            ],
            unlocked_quests: vec!["unstable_site_investigation".to_string()],
        },

        faction_effects,
        educational_focus: EducationalObjectives {
            primary_concepts: vec![
                "Biology".to_string(),
                "Physiology".to_string(),
                "Biochemistry".to_string(),
                "Medical Physics".to_string(),
            ],
            secondary_concepts: vec![
                "Ecology".to_string(),
                "Plant Biology".to_string(),
            ],
            applications: vec![
                "Medical Treatment".to_string(),
                "Pain Relief".to_string(),
                "Tissue Regeneration".to_string(),
                "Diagnostic Techniques".to_string(),
            ],
            problem_solving_methods: vec![
                "Controlled Experimentation".to_string(),
                "Biological Observation".to_string(),
                "Safety Protocols".to_string(),
                "Ethical Considerations".to_string(),
            ],
            assessment_criteria: vec![
                "Understanding of biological systems".to_string(),
                "Safe application of techniques".to_string(),
                "Ethical use of healing magic".to_string(),
                "Effectiveness of treatments".to_string(),
            ],
        },

        branching_paths: HashMap::new(), // Could add different healing specializations
        choices: vec![], // Will be added in future phases
        involved_npcs: vec!["healer_seraphina".to_string(), "dr_felix".to_string()],
        locations: vec!["crystal_garden_lab".to_string()],
        estimated_duration: 150,
    }
}

/// Quest 5: "Investigation of the Unstable Site" - Expert Level
/// Advanced quest dealing with dangerous magical phenomena
fn create_unstable_site_investigation_quest() -> QuestDefinition {
    let mut faction_effects = HashMap::new();
    faction_effects.insert(FactionId::MagistersCouncil, 8); // Positive for following safety
    faction_effects.insert(FactionId::UndergroundNetwork, 12); // Positive for brave research
    faction_effects.insert(FactionId::NeutralScholars, 15); // Highest positive for advancing knowledge

    let mut theory_bonuses = HashMap::new();
    theory_bonuses.insert("sympathetic_networks".to_string(), 0.3);
    theory_bonuses.insert("resonance_amplification".to_string(), 0.2);
    theory_bonuses.insert("theoretical_synthesis".to_string(), 0.1);

    QuestDefinition {
        id: "unstable_site_investigation".to_string(),
        title: "Investigation of the Unstable Site".to_string(),
        description: "The Unstable Resonance Site contains dangerous but invaluable magical phenomena \
                     that could unlock advanced theoretical understanding. This high-risk research \
                     requires mastery of multiple theories and careful navigation of safety protocols \
                     versus scientific discovery. Only the most skilled practitioners should attempt this quest.".to_string(),
        category: QuestCategory::Experimental,
        difficulty: QuestDifficulty::Expert,

        requirements: QuestRequirements {
            theory_requirements: vec![
                ("sympathetic_networks".to_string(), 0.4),
                ("resonance_amplification".to_string(), 0.3),
                ("detection_arrays".to_string(), 0.5),
                ("bio_resonance".to_string(), 0.6),
            ],
            faction_requirements: vec![],
            faction_restrictions: vec![],
            prerequisite_quests: vec!["healing_research".to_string(), "diplomatic_balance".to_string()],
            attribute_requirements: AttributeRequirements {
                min_mental_acuity: Some(40),
                min_resonance_sensitivity: Some(35),
                min_total_playtime: Some(300),
            },
            capability_requirements: vec![
                "healing_spells".to_string(),
                "diplomatic_sensing".to_string(),
                "bio_resonance_diagnosis".to_string(),
            ],
            location_requirements: vec![],
        },

        objectives: vec![
            QuestObjective {
                id: "safety_briefing".to_string(),
                description: "Get safety briefing from Safety Warden Gareth".to_string(),
                objective_type: ObjectiveType::TalkToNPC {
                    npc_id: "warden_gareth".to_string(),
                    topic: Some("unstable_site_safety".to_string())
                },
                optional: false,
                visible: true,
                completion_reward: ObjectiveReward {
                    experience: 30,
                    theory_insights: HashMap::new(),
                    faction_changes: {
                        let mut changes = HashMap::new();
                        changes.insert(FactionId::MagistersCouncil, 3);
                        changes
                    },
                    items: vec!["safety_detection_equipment".to_string()],
                },
            },
            QuestObjective {
                id: "master_sympathetic_networks".to_string(),
                description: "Master Sympathetic Networks theory to 80% understanding".to_string(),
                objective_type: ObjectiveType::LearnTheory {
                    theory_id: "sympathetic_networks".to_string(),
                    min_level: 0.8
                },
                optional: false,
                visible: true,
                completion_reward: ObjectiveReward {
                    experience: 100,
                    theory_insights: {
                        let mut insights = HashMap::new();
                        insights.insert("sympathetic_networks".to_string(), 0.1);
                        insights
                    },
                    faction_changes: HashMap::new(),
                    items: vec![],
                },
            },
            QuestObjective {
                id: "visit_unstable_site".to_string(),
                description: "Carefully visit the Unstable Resonance Site with proper protection".to_string(),
                objective_type: ObjectiveType::VisitLocation {
                    location_id: "unstable_resonance_site".to_string()
                },
                optional: false,
                visible: false, // Only after safety briefing and theory mastery
                completion_reward: ObjectiveReward {
                    experience: 50,
                    theory_insights: HashMap::new(),
                    faction_changes: HashMap::new(),
                    items: vec!["unstable_energy_readings".to_string()],
                },
            },
            QuestObjective {
                id: "analyze_instability_patterns".to_string(),
                description: "Analyze the magical instability patterns using advanced detection".to_string(),
                objective_type: ObjectiveType::MagicalDemonstration {
                    theory_id: "detection_arrays".to_string(),
                    success_threshold: 0.9
                },
                optional: false,
                visible: false, // After visiting site
                completion_reward: ObjectiveReward {
                    experience: 80,
                    theory_insights: {
                        let mut insights = HashMap::new();
                        insights.insert("sympathetic_networks".to_string(), 0.15);
                        insights.insert("resonance_amplification".to_string(), 0.1);
                        insights
                    },
                    faction_changes: HashMap::new(),
                    items: vec!["instability_analysis_data".to_string()],
                },
            },
            QuestObjective {
                id: "develop_stabilization_theory".to_string(),
                description: "Develop a theoretical framework for magical stabilization".to_string(),
                objective_type: ObjectiveType::Research {
                    theory_id: "theoretical_synthesis".to_string(),
                    research_points: 100
                },
                optional: false,
                visible: false, // After analysis
                completion_reward: ObjectiveReward {
                    experience: 120,
                    theory_insights: {
                        let mut insights = HashMap::new();
                        insights.insert("theoretical_synthesis".to_string(), 0.2);
                        insights
                    },
                    faction_changes: {
                        let mut changes = HashMap::new();
                        changes.insert(FactionId::NeutralScholars, 15);
                        changes.insert(FactionId::UndergroundNetwork, 10);
                        changes
                    },
                    items: vec!["stabilization_theory_manuscript".to_string()],
                },
            },
            QuestObjective {
                id: "coordinate_with_captain_vera".to_string(),
                description: "Coordinate your findings with Captain Vera for site security".to_string(),
                objective_type: ObjectiveType::TalkToNPC {
                    npc_id: "captain_vera".to_string(),
                    topic: Some("stabilization_recommendations".to_string())
                },
                optional: true,
                visible: true,
                completion_reward: ObjectiveReward {
                    experience: 60,
                    theory_insights: HashMap::new(),
                    faction_changes: {
                        let mut changes = HashMap::new();
                        changes.insert(FactionId::MagistersCouncil, 12);
                        changes
                    },
                    items: vec![],
                },
            },
            QuestObjective {
                id: "share_with_echo".to_string(),
                description: "Share theoretical insights with Echo Voidwalker".to_string(),
                objective_type: ObjectiveType::TalkToNPC {
                    npc_id: "echo_voidwalker".to_string(),
                    topic: Some("theoretical_breakthroughs".to_string())
                },
                optional: true,
                visible: true,
                completion_reward: ObjectiveReward {
                    experience: 70,
                    theory_insights: {
                        let mut insights = HashMap::new();
                        insights.insert("theoretical_synthesis".to_string(), 0.1);
                        insights
                    },
                    faction_changes: {
                        let mut changes = HashMap::new();
                        changes.insert(FactionId::UndergroundNetwork, 15);
                        changes
                    },
                    items: vec![],
                },
            },
        ],

        rewards: QuestRewards {
            experience: 600,
            attribute_bonuses: AttributeBonuses {
                mental_acuity: Some(5),
                resonance_sensitivity: Some(4),
            },
            theory_bonuses,
            faction_changes: faction_effects.clone(),
            items: vec![
                "unstable_energy_stabilizer".to_string(),
                "advanced_detection_array".to_string(),
                "theoretical_synthesis_notes".to_string(),
            ],
            new_capabilities: vec![
                "advanced_magical_analysis".to_string(),
                "instability_detection".to_string(),
                "theoretical_innovation".to_string(),
                "high_energy_magic".to_string(),
            ],
            unlocked_quests: vec![], // This is the capstone quest
        },

        faction_effects,
        educational_focus: EducationalObjectives {
            primary_concepts: vec![
                "Quantum Mechanics".to_string(),
                "Network Theory".to_string(),
                "Information Theory".to_string(),
                "Systems Theory".to_string(),
                "Mathematical Modeling".to_string(),
            ],
            secondary_concepts: vec![
                "Risk Assessment".to_string(),
                "Safety Engineering".to_string(),
                "Advanced Mathematics".to_string(),
            ],
            applications: vec![
                "Magical Stabilization".to_string(),
                "High-Energy Applications".to_string(),
                "Theoretical Innovation".to_string(),
                "Safety Systems".to_string(),
            ],
            problem_solving_methods: vec![
                "Advanced Mathematical Analysis".to_string(),
                "Multi-Theory Synthesis".to_string(),
                "Risk-Benefit Analysis".to_string(),
                "Collaborative Research".to_string(),
                "Safety-First Approach".to_string(),
            ],
            assessment_criteria: vec![
                "Theoretical innovation".to_string(),
                "Safety consciousness".to_string(),
                "Integration of multiple concepts".to_string(),
                "Practical application potential".to_string(),
                "Collaborative effectiveness".to_string(),
            ],
        },

        branching_paths: HashMap::new(), // Could add different approaches to stabilization
        choices: vec![], // Will be added in future phases
        involved_npcs: vec![
            "warden_gareth".to_string(),
            "captain_vera".to_string(),
            "echo_voidwalker".to_string()
        ],
        locations: vec![
            "unstable_resonance_site".to_string(),
            "harmonic_testing_chambers".to_string(),
            "crystalline_archives".to_string(),
        ],
        estimated_duration: 240,
    }
}

/// Create NPCs with quest-aware dialogue for the example quests
pub fn create_quest_npcs() -> Vec<NPC> {
    vec![
        create_tutorial_assistant(),
        create_dr_felix(),
        create_ambassador_cordelia(),
        create_observer_lyra(),
        create_echo_voidwalker(),
    ]
}

/// Create Tutorial Assistant Elara Starweaver for the "Understanding Resonance" quest
fn create_tutorial_assistant() -> NPC {
    let personality = NPCPersonality {
        trait_description: "Warm, encouraging, and endlessly patient with new students. Elara radiates genuine enthusiasm for magical theory and believes every student can succeed with proper guidance.".to_string(),
        speaking_style: vec!["encouraging".to_string(), "patient".to_string(), "enthusiastic".to_string()],
        quirks: vec![
            "Often uses musical metaphors when explaining resonance".to_string(),
            "Smiles warmly when students have breakthroughs".to_string(),
            "Hums softly when deep in thought about theory".to_string(),
        ],
    };

    // Create quest-specific dialogue for "Understanding Resonance"
    let mut quest_dialogue_map = HashMap::new();

    let resonance_quest_dialogue = QuestDialogue {
        quest_intro: Some(
            "Welcome to the Practice Hall! I'm Tutorial Assistant Elara Starweaver, and I'm delighted \
            to guide you through your first steps into the beautiful world of sympathetic resonance.\n\n\
            Today, you'll discover something truly magical - and I mean that in both senses of the word. \
            You'll learn that magic isn't some mysterious force that defies understanding, but rather a \
            natural phenomenon as elegant as music itself. Every crystal, every object around us, sings \
            its own unique frequency. Our task as practitioners is simply to learn how to listen, and \
            then to sing in harmony.\n\n\
            We'll start slowly, with the fundamentals. First, you'll need to study the theory of Harmonic \
            Fundamentals - take your time with it, let the concepts settle naturally. Then, once you feel \
            ready, we'll move to a hands-on demonstration. I'll be here if you need any guidance!".to_string()
        ),

        quest_in_progress: Some(
            "You're doing wonderfully! I can see you're beginning to grasp how frequencies interact. \
            Remember, there's no rush - every student learns at their own pace, and that's perfectly natural.\n\n\
            If you're working on the theory, try to visualize the waves as they oscillate. Imagine them like \
            ripples on a pond, spreading outward, intersecting, sometimes amplifying each other when they're \
            in sync. That's resonance - it's when two systems find their common song.\n\n\
            When you're ready for the demonstration, find a practice crystal and focus on matching its natural \
            frequency. You'll feel it when you get it right - the crystal will respond with a gentle warmth, \
            a subtle glow. It's one of my favorite moments in teaching, seeing that first connection.\n\n\
            Feel free to ask me about 'resonance_results' when you want to discuss your progress!".to_string()
        ),

        quest_completed: Some(
            "Oh, how wonderful! I'm so proud of you! You've achieved something truly special today - you've \
            taken your first real step into magical understanding.\n\n\
            I saw the way the crystal responded to you. That wasn't just technique, though your technique was \
            excellent. That was understanding made manifest. You didn't just follow instructions - you grasped \
            the *why* behind the practice, and that's what separates true practitioners from mere followers of recipes.\n\n\
            This foundation you've built today will support everything else you learn. Harmonic Fundamentals isn't \
            just one theory among many - it's the bedrock on which all other theories are built. Energy conservation, \
            frequency matching, resonance phenomena - these principles appear everywhere in magical practice.\n\n\
            Take this crystal - you've earned it. It's tuned specifically to your personal resonance frequency now. \
            Treat it well, and it will serve you faithfully in your studies ahead.\n\n\
            I look forward to hearing about your future discoveries. The path ahead of you is full of wonder!".to_string()
        ),

        objective_dialogue: {
            let mut obj_dialogue = HashMap::new();

            obj_dialogue.insert(
                "visit_practice_hall".to_string(),
                "Ah, you've found the Practice Hall! Isn't it beautiful? These crystals have been singing here for \
                generations, patiently waiting to teach new students. Every scratch on these walls tells a story of \
                discovery, every scorch mark a lesson learned. You're part of that lineage now.".to_string()
            );

            obj_dialogue.insert(
                "learn_harmonic_fundamentals".to_string(),
                "The theory materials are over on that shelf - take whichever tome speaks to you. Some students prefer \
                'Resonance for Beginners' with its many diagrams, others find 'The Singing Crystal' more poetic and \
                memorable. Both cover the same principles, just in different voices.\n\n\
                As you study, try to *feel* the concepts, not just memorize them. When you read about wave interference, \
                picture ocean waves merging and splitting. When you learn about frequency matching, think about how you can \
                recognize a friend's voice in a crowd. The math is important, but the intuition is what will guide your practice.".to_string()
            );

            obj_dialogue.insert(
                "demonstrate_resonance".to_string(),
                "Ready for the practical demonstration? Wonderful! Here's a practice crystal - feel its weight, its \
                temperature, its subtle vibration. Every crystal has a natural frequency, a tone at which it most easily resonates.\n\n\
                Close your eyes if it helps. Reach out with your senses - not your physical ones, but that inner awareness you've \
                been developing through your studies. You're looking for that sweet spot, that frequency where everything just... \
                clicks into place.\n\n\
                Don't worry if it takes a few tries. Even I still sometimes need to adjust my attunement. That's not failure - \
                that's the process of learning, of fine-tuning your understanding through practice.".to_string()
            );

            obj_dialogue
        },

        progress_hints: vec![
            "If you're finding the theory challenging, try breaking it down into smaller pieces. Start with wave basics, \
            then move to interference patterns, then finally to resonance phenomena. Each concept builds naturally on the last.".to_string(),

            "For the practical demonstration, remember that your mental state affects your ability to sense resonance. Take a \
            deep breath, clear your mind of distractions, and approach the crystal with calm confidence.".to_string(),

            "Theory and practice should reinforce each other. After studying a concept, try to observe it in the practice \
            crystals. After a hands-on session, return to the theory to understand what you experienced.".to_string(),
        ],
    };

    quest_dialogue_map.insert("resonance_foundation".to_string(), resonance_quest_dialogue);

    // Create dialogue tree with base topics
    let mut topics = HashMap::new();

    // Topic: Resonance
    topics.insert("resonance".to_string(), DialogueNode {
        text_templates: vec![
            "Resonance is the heart of magic! When two systems share compatible frequencies, energy can flow between them \
            with remarkable efficiency. It's like... imagine two friends humming the same tune. Their voices naturally amplify \
            each other, creating something more beautiful than either could alone.".to_string(),
        ],
        responses: vec![],
        requirements: DialogueRequirements {
            min_faction_standing: None,
            max_faction_standing: None,
            knowledge_requirements: vec![],
            theory_requirements: vec![],
            min_theory_mastery: None,
            required_capabilities: vec![],
        },
    });

    // Topic: Crystals
    topics.insert("crystals".to_string(), DialogueNode {
        text_templates: vec![
            "Crystals are wonderful teachers! Their molecular structure creates stable, predictable frequencies - they're \
            like tuning forks for magical practice. A well-cared-for crystal can serve a practitioner for decades, holding \
            its resonance true through countless attunements.".to_string(),
        ],
        responses: vec![],
        requirements: DialogueRequirements {
            min_faction_standing: None,
            max_faction_standing: None,
            knowledge_requirements: vec![],
            theory_requirements: vec![],
            min_theory_mastery: None,
            required_capabilities: vec![],
        },
    });

    // Topic: Practice Tips
    topics.insert("practice_tips".to_string(), DialogueNode {
        text_templates: vec![
            "The best practice advice I can give? Be patient with yourself. Everyone learns differently, and that's a \
            strength, not a weakness. Some students grasp theory instantly but struggle with practical work. Others have \
            an intuitive feel for resonance but need time to understand the underlying mathematics. Both paths lead to mastery.\n\n\
            Also, don't underestimate rest. Your mind needs time to process and integrate new concepts. Sometimes a good \
            night's sleep teaches you more than another hour of study.".to_string(),
        ],
        responses: vec![],
        requirements: DialogueRequirements {
            min_faction_standing: None,
            max_faction_standing: None,
            knowledge_requirements: vec![],
            theory_requirements: vec![],
            min_theory_mastery: None,
            required_capabilities: vec![],
        },
    });

    // Topic: Resonance Results (Quest-specific, available during quest)
    topics.insert("resonance_results".to_string(), DialogueNode {
        text_templates: vec![
            "Tell me about your experience! Did you feel that moment when the crystal first responded? That's what keeps me \
            teaching, year after year - seeing that spark of understanding in a student's eyes when theory becomes reality.\n\n\
            What you're learning now will stay with you forever. Years from now, when you're working with advanced theories and \
            complex applications, you'll still be relying on these fundamental principles. Harmonic Fundamentals isn't called \
            'fundamental' for nothing!".to_string(),
        ],
        responses: vec![],
        requirements: DialogueRequirements {
            min_faction_standing: None,
            max_faction_standing: None,
            knowledge_requirements: vec!["harmonic_fundamentals".to_string()],
            theory_requirements: vec![("harmonic_fundamentals".to_string(), 0.3)],
            min_theory_mastery: None,
            required_capabilities: vec![],
        },
    });

    NPC {
        id: "tutorial_assistant".to_string(),
        name: "Elara Starweaver".to_string(),
        description: "A warm, encouraging Tutorial Assistant with flowing robes decorated with crystalline patterns. \
                     Her eyes sparkle with genuine enthusiasm for teaching, and she moves with the practiced grace of \
                     someone who has spent years working in harmony with resonance crystals. She radiates patience and \
                     the kind of deep understanding that comes from truly mastering the fundamentals.".to_string(),
        faction_affiliation: Some(FactionId::MagistersCouncil),
        personality: Some(personality),
        quest_dialogue: quest_dialogue_map,
        dialogue_tree: DialogueTree {
            greeting: DialogueNode {
                text_templates: vec![
                    "Welcome, my dear student! The crystals seem particularly harmonious today - a good omen for learning!".to_string(),
                    "Hello again! How wonderful to see you continuing your studies. The path of magical understanding is a journey worth taking.".to_string(),
                    "Ah, another eager mind ready to discover the wonders of resonance! Please, come in, come in.".to_string(),
                ],
                responses: vec![],
                requirements: DialogueRequirements {
                    min_faction_standing: None,
                    max_faction_standing: None,
                    knowledge_requirements: vec![],
                    theory_requirements: vec![],
                    min_theory_mastery: None,
                    required_capabilities: vec![],
                },
            },
            time_based_greetings: {
                let mut time_greetings = HashMap::new();
                time_greetings.insert("morning".to_string(),
                    "Good morning! The crystals are just beginning to warm with the day's energy - perfect timing for study.".to_string());
                time_greetings.insert("afternoon".to_string(),
                    "Good afternoon! I hope the day's lessons are treating you well.".to_string());
                time_greetings.insert("evening".to_string(),
                    "Good evening! Don't study too late - rest is as important as practice for true understanding.".to_string());
                time_greetings
            },
            topics,
            faction_specific: {
                let mut faction_specific = HashMap::new();

                // High Magisters' Council reputation (50+)
                faction_specific.insert(FactionId::MagistersCouncil, DialogueNode {
                    text_templates: vec![
                        "Ah, a fellow Council member! It's wonderful to see dedicated students advancing through our structured curriculum. \
                        The Council's methods have proven themselves over centuries - systematic, thorough, and always grounded in proven theory.".to_string(),
                        "Welcome back! The Council takes great pride in students like you who demonstrate both aptitude and dedication to proper \
                        magical study. Your progress reflects well on our educational philosophy.".to_string(),
                        "Greetings, colleague! I'm pleased to see you're following the Council's recommended learning path. There's comfort in \
                        knowing the methods we use have been refined by generations of masters.".to_string(),
                    ],
                    responses: vec![],
                    requirements: DialogueRequirements {
                        min_faction_standing: Some((FactionId::MagistersCouncil, 50)),
                        max_faction_standing: None,
                        knowledge_requirements: vec![],
                        theory_requirements: vec![],
                        min_theory_mastery: None,
                        required_capabilities: vec![],
                    },
                });

                // High Neutral Scholars reputation (40+)
                faction_specific.insert(FactionId::NeutralScholars, DialogueNode {
                    text_templates: vec![
                        "Welcome, fellow seeker of knowledge! I appreciate the Scholars' approach - learning for its own sake, free from political \
                        entanglements. There's something pure about pursuing understanding simply because it's worth understanding.".to_string(),
                        "Ah, another independent mind! While I teach within the Council's structure, I admire those who study with open curiosity \
                        rather than political agenda. The Scholars represent something important in our community.".to_string(),
                        "Greetings! Your reputation among the Neutral Scholars speaks well of your dedication to genuine learning. Sometimes the \
                        best insights come from those who study without predetermined conclusions.".to_string(),
                    ],
                    responses: vec![],
                    requirements: DialogueRequirements {
                        min_faction_standing: Some((FactionId::NeutralScholars, 40)),
                        max_faction_standing: None,
                        knowledge_requirements: vec![],
                        theory_requirements: vec![],
                        min_theory_mastery: None,
                        required_capabilities: vec![],
                    },
                });

                // High Underground Network reputation (30+) - concerned but still teaching
                faction_specific.insert(FactionId::UndergroundNetwork, DialogueNode {
                    text_templates: vec![
                        "I... see you've been spending time with the Underground Network. *pauses thoughtfully* I won't pretend to approve of all \
                        their methods, but I recognize that different perspectives can illuminate different truths. Please, just be careful.".to_string(),
                        "Welcome. Your associations... concern me, I'll admit. The Council and the Underground have fundamental disagreements about \
                        proper magical practice. But you're still a student, and I'm still a teacher. Let's focus on the knowledge itself.".to_string(),
                        "Ah. *slight frown* Word of your Underground connections has reached the Practice Hall. I hope you're learning genuine \
                        theory and not just... unconventional shortcuts. The fundamentals matter, regardless of who teaches them.".to_string(),
                    ],
                    responses: vec![],
                    requirements: DialogueRequirements {
                        min_faction_standing: Some((FactionId::UndergroundNetwork, 30)),
                        max_faction_standing: None,
                        knowledge_requirements: vec![],
                        theory_requirements: vec![],
                        min_theory_mastery: None,
                        required_capabilities: vec![],
                    },
                });

                faction_specific
            },
        },
        current_disposition: 0,
    }
}

/// Create Dr. Felix for the "Crystal Analysis Fundamentals" quest (Academic path)
fn create_dr_felix() -> NPC {
    let personality = NPCPersonality {
        trait_description: "Brilliant, detail-oriented, and slightly absent-minded. Dr. Felix is passionate about pure research \
                           and often gets lost in theoretical discussions. Values precision and intellectual rigor above all.".to_string(),
        speaking_style: vec!["analytical".to_string(), "precise".to_string(), "academic".to_string()],
        quirks: vec![
            "Adjusts spectacles when making important points".to_string(),
            "Often references obscure research papers".to_string(),
            "Gets excited about lattice structures and crystallography".to_string(),
        ],
    };

    let mut quest_dialogue_map = HashMap::new();

    let crystal_quest_dialogue = QuestDialogue {
        quest_intro: Some(
            "Ah, excellent! A research assistant! *adjusts spectacles enthusiastically* Welcome to the Crystalline Research Laboratory. \
            I'm Dr. Felix Stoneweaver, senior crystallographer with the Neutral Scholars.\n\n\
            Crystal structures - now there's a topic worthy of a lifetime's study! The way molecular arrangements affect magical \
            conductivity, the subtle variations in lattice geometries, the quantum interactions at the atomic level... \
            *trails off, lost in thought*\n\n\
            Ahem. Yes. You're here for the advanced crystal analysis project. This isn't just about using crystals as tools - \
            anyone can do that. We're going to understand them at a fundamental level. The mathematics, the physics, the elegant \
            geometry of it all.\n\n\
            I hope you're prepared for rigorous study. The Neutral Scholars maintain the highest academic standards. But if you \
            persevere, you'll gain insights that go far beyond mere practical application.".to_string()
        ),
        quest_in_progress: Some(
            "How are your lattice structure analyses progressing? *peers at you intently through spectacles*\n\n\
            Remember, precision is everything in crystallography. A single misaligned atom in the molecular matrix can alter the \
            entire resonance profile. Take your measurements carefully, document everything, verify your results.\n\n\
            I'm currently working on a fascinating paper about hexagonal vs. cubic crystal systems and their relative efficiency \
            in sympathetic resonance applications. The data is quite compelling, though I suspect the review committee will \
            quibble with my methodology. They always do. *sighs*\n\n\
            When you're ready to discuss your findings, I'll be here. I've cleared my afternoon - well, except for the \
            Crystallographers' Symposium at three, and the peer review meeting at four-thirty, and... where did I put my \
            schedule?".to_string()
        ),
        quest_completed: Some(
            "Remarkable work! *reviews your research with evident satisfaction* Your lattice analysis shows genuine understanding, \
            not just rote application of formulas. You've grasped the underlying principles.\n\n\
            *makes enthusiastic notes* This data on resonance frequency variations across different crystal geometries - \
            excellent, excellent! I might reference this in my next publication, if you don't mind? Proper academic credit, \
            of course. Stoneweaver and... what was your name again?\n\n\
            *chuckles* I jest, I jest. You've earned recognition for this research. The Neutral Scholars will be pleased to \
            count you among those who pursue knowledge for its own sake, untainted by political maneuvering or commercial interests.\n\n\
            Keep this mindset. Pure research, rigorous methodology, peer-reviewed findings - these are the foundations of \
            genuine scientific progress. The world needs more scholars and fewer opportunists.".to_string()
        ),
        objective_dialogue: {
            let mut obj_dialogue = HashMap::new();
            obj_dialogue.insert(
                "academic_crystal_study".to_string(),
                "Ah, ready for the advanced analysis? Splendid! *pulls out complex diagrams*\n\n\
                Let's begin with hexagonal lattice structures - particularly elegant, in my opinion. Notice how the sixty-degree \
                bond angles create natural resonance pathways... *launches into detailed technical explanation*\n\n\
                Now, compare that to cubic structures - less aesthetically pleasing, perhaps, but with fascinating harmonic \
                properties when stressed at specific frequencies...\n\n\
                *several hours later* ...and that's why the relative atomic mass of the constituent elements matters so much for \
                sustained resonance. Any questions?".to_string()
            );
            obj_dialogue
        },
        progress_hints: vec![
            "Don't just measure - understand what you're measuring. Every data point tells a story about molecular geometry and \
            quantum interactions. Ask yourself: why does this crystal behave this way?".to_string(),
            "The best research comes from curiosity, not obligation. If you find yourself fascinated by crystal structures, you're \
            on the right path. If it feels like tedious work... well, perhaps the commercial approach suits you better.".to_string(),
        ],
    };

    quest_dialogue_map.insert("crystal_analysis".to_string(), crystal_quest_dialogue);

    let mut topics = HashMap::new();

    topics.insert("crystal_research".to_string(), DialogueNode {
        text_templates: vec![
            "Crystal research is the foundation of modern magical theory! Every advancement in the past century traces back to \
            better understanding of crystalline structures. *becomes animated* The potential for future discoveries is limitless!".to_string(),
        ],
        responses: vec![],
        requirements: DialogueRequirements {
            min_faction_standing: None,
            max_faction_standing: None,
            knowledge_requirements: vec![],
            theory_requirements: vec![],
            min_theory_mastery: None,
            required_capabilities: vec![],
        },
    });

    topics.insert("academic_standards".to_string(), DialogueNode {
        text_templates: vec![
            "The Neutral Scholars maintain rigorous academic standards precisely because we have no political agenda. We can't \
            afford to let ideology corrupt our research - the truth must speak for itself. *adjusts spectacles firmly*\n\n\
            That's why peer review matters. That's why we publish our methodologies openly. That's why we welcome criticism and \
            replication studies. Science without scrutiny is just... opinion.".to_string(),
        ],
        responses: vec![],
        requirements: DialogueRequirements {
            min_faction_standing: Some((FactionId::NeutralScholars, 20)),
            max_faction_standing: None,
            knowledge_requirements: vec![],
            theory_requirements: vec![],
            min_theory_mastery: None,
            required_capabilities: vec![],
        },
    });

    NPC {
        id: "dr_felix".to_string(),
        name: "Dr. Felix Stoneweaver".to_string(),
        description: "A brilliant crystallographer in his late fifties, with wild gray hair and perpetually ink-stained fingers. \
                     His spectacles are always slightly askew, and his robes are covered in chalk dust from countless hours at the \
                     research boards. Despite his absent-minded professor demeanor, his eyes sharpen with intense focus when \
                     discussing crystal structures or research methodology.".to_string(),
        faction_affiliation: Some(FactionId::NeutralScholars),
        personality: Some(personality),
        quest_dialogue: quest_dialogue_map,
        dialogue_tree: DialogueTree {
            greeting: DialogueNode {
                text_templates: vec![
                    "Ah! A visitor! *peers over spectacles* I hope you're not here to interrupt important research... unless you're interested in lattice theory?".to_string(),
                    "Hmm? Oh! Welcome to the laboratory. Mind the equipment - some of these crystals are quite temperamental.".to_string(),
                    "*looks up from notes* Ah, yes, hello. Forgive the mess - organization is not my strong suit. Research is.".to_string(),
                ],
                responses: vec![],
                requirements: DialogueRequirements {
                    min_faction_standing: None,
                    max_faction_standing: None,
                    knowledge_requirements: vec![],
                    theory_requirements: vec![],
                    min_theory_mastery: None,
                    required_capabilities: vec![],
                },
            },
            time_based_greetings: HashMap::new(),
            topics,
            faction_specific: {
                let mut faction_specific = HashMap::new();

                // High Neutral Scholars reputation
                faction_specific.insert(FactionId::NeutralScholars, DialogueNode {
                    text_templates: vec![
                        "Ah, a fellow Scholar! *brightens considerably* It's refreshing to speak with someone who values knowledge for its own sake. \
                        Come, let me show you my latest findings!".to_string(),
                    ],
                    responses: vec![],
                    requirements: DialogueRequirements {
                        min_faction_standing: Some((FactionId::NeutralScholars, 40)),
                        max_faction_standing: None,
                        knowledge_requirements: vec![],
                        theory_requirements: vec![],
                        min_theory_mastery: None,
                        required_capabilities: vec![],
                    },
                });

                // High Industrial Consortium reputation - disapproving
                faction_specific.insert(FactionId::IndustrialConsortium, DialogueNode {
                    text_templates: vec![
                        "*eyes narrow slightly* You work with the Consortium, I hear. I hope you're not here to ask me to compromise \
                        research integrity for commercial applications. I've had quite enough of that pressure.".to_string(),
                    ],
                    responses: vec![],
                    requirements: DialogueRequirements {
                        min_faction_standing: Some((FactionId::IndustrialConsortium, 40)),
                        max_faction_standing: None,
                        knowledge_requirements: vec![],
                        theory_requirements: vec![],
                        min_theory_mastery: None,
                        required_capabilities: vec![],
                    },
                });

                faction_specific
            },
        },
        current_disposition: 0,
    }
}
/// Create Ambassador Cordelia for the "Diplomatic Balance" quest
fn create_ambassador_cordelia() -> NPC {
    let personality = NPCPersonality {
        trait_description: "Diplomatic, measured, and keenly observant. Sees all sides of conflicts.".to_string(),
        speaking_style: vec!["diplomatic".to_string(), "measured".to_string()],
        quirks: vec!["Pauses thoughtfully before responding".to_string()],
    };

    NPC {
        id: "ambassador_cordelia".to_string(),
        name: "Ambassador Cordelia".to_string(),
        description: "An elegant diplomat with a reputation for fairness and neutrality.".to_string(),
        faction_affiliation: Some(FactionId::NeutralScholars),
        personality: Some(personality),
        quest_dialogue: HashMap::new(),
        dialogue_tree: DialogueTree {
            greeting: DialogueNode {
                text_templates: vec!["Welcome. I hope we can find common ground.".to_string()],
                responses: vec![],
                requirements: DialogueRequirements {
                    min_faction_standing: None,
                    max_faction_standing: None,
                    knowledge_requirements: vec![],
                    theory_requirements: vec![],
                    min_theory_mastery: None,
                    required_capabilities: vec![],
                },
            },
            time_based_greetings: HashMap::new(),
            topics: HashMap::new(),
            faction_specific: HashMap::new(),
        },
        current_disposition: 0,
    }
}

/// Create Observer Lyra for the "Diplomatic Balance" quest (Magisters' Council)
fn create_observer_lyra() -> NPC {
    let personality = NPCPersonality {
        trait_description: "Formal, traditional, values order and structure.".to_string(),
        speaking_style: vec!["formal".to_string(), "authoritative".to_string()],
        quirks: vec!["References Council precedents frequently".to_string()],
    };

    NPC {
        id: "observer_lyra".to_string(),
        name: "Observer Lyra".to_string(),
        description: "A stern Council observer who upholds traditional magical governance.".to_string(),
        faction_affiliation: Some(FactionId::MagistersCouncil),
        personality: Some(personality),
        quest_dialogue: HashMap::new(),
        dialogue_tree: DialogueTree {
            greeting: DialogueNode {
                text_templates: vec!["The Council values order and proper procedure.".to_string()],
                responses: vec![],
                requirements: DialogueRequirements {
                    min_faction_standing: None,
                    max_faction_standing: None,
                    knowledge_requirements: vec![],
                    theory_requirements: vec![],
                    min_theory_mastery: None,
                    required_capabilities: vec![],
                },
            },
            time_based_greetings: HashMap::new(),
            topics: HashMap::new(),
            faction_specific: HashMap::new(),
        },
        current_disposition: 0,
    }
}

/// Create Echo Voidwalker for the "Diplomatic Balance" quest (Underground Network)
fn create_echo_voidwalker() -> NPC {
    let personality = NPCPersonality {
        trait_description: "Mysterious, anti-authoritarian, values freedom and innovation.".to_string(),
        speaking_style: vec!["cryptic".to_string(), "rebellious".to_string()],
        quirks: vec!["Speaks in riddles sometimes".to_string()],
    };

    NPC {
        id: "echo_voidwalker".to_string(),
        name: "Echo Voidwalker".to_string(),
        description: "An enigmatic member of the Underground Network who questions all authority.".to_string(),
        faction_affiliation: Some(FactionId::UndergroundNetwork),
        personality: Some(personality),
        quest_dialogue: HashMap::new(),
        dialogue_tree: DialogueTree {
            greeting: DialogueNode {
                text_templates: vec!["The shadows hold more truth than the Council's light.".to_string()],
                responses: vec![],
                requirements: DialogueRequirements {
                    min_faction_standing: None,
                    max_faction_standing: None,
                    knowledge_requirements: vec![],
                    theory_requirements: vec![],
                    min_theory_mastery: None,
                    required_capabilities: vec![],
                },
            },
            time_based_greetings: HashMap::new(),
            topics: HashMap::new(),
            faction_specific: HashMap::new(),
        },
        current_disposition: 0,
    }
}
