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