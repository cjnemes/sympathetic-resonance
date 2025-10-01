//! Integration tests for quest-aware dialogue system
//!
//! These tests verify that the dialogue system properly integrates with
//! the quest system and provides rich, context-aware NPC interactions.

use sympathetic_resonance::{DatabaseManager, GameEngine};
use sympathetic_resonance::systems::quest_examples::create_quest_npcs;
use tempfile::NamedTempFile;

/// Helper function to create a test game engine with default content and quest NPCs
fn create_test_engine_with_npcs() -> GameEngine {
    let temp_file = NamedTempFile::new().unwrap();
    let db_path = temp_file.path().to_str().unwrap();
    let db = DatabaseManager::new(db_path).unwrap();
    db.initialize_schema().unwrap();
    db.load_default_content().unwrap();

    let mut engine = GameEngine::new(db).unwrap();

    // Load quest NPCs into the dialogue system
    let quest_npcs = create_quest_npcs();
    for npc in quest_npcs {
        engine.dialogue_system_mut().add_npc(npc);
    }

    engine
}

#[test]
fn test_tutorial_assistant_creation() {
    let npcs = create_quest_npcs();
    assert!(!npcs.is_empty(), "Should create at least one quest NPC");

    let tutorial_assistant = npcs.iter()
        .find(|npc| npc.id == "tutorial_assistant")
        .expect("Should create tutorial assistant NPC");

    assert_eq!(tutorial_assistant.name, "Elara Starweaver");
    assert!(tutorial_assistant.personality.is_some(), "Should have personality");
    assert!(!tutorial_assistant.quest_dialogue.is_empty(), "Should have quest dialogue");
}

#[test]
fn test_all_quest_npcs_created() {
    let npcs = create_quest_npcs();

    // Phase 1B should create 5 NPCs total
    assert_eq!(npcs.len(), 5, "Should create 5 quest NPCs");

    let expected_npcs = vec![
        "tutorial_assistant",
        "dr_felix",
        "ambassador_cordelia",
        "observer_lyra",
        "echo_voidwalker",
    ];

    for expected_id in expected_npcs {
        assert!(
            npcs.iter().any(|npc| npc.id == expected_id),
            "Missing NPC: {}",
            expected_id
        );
    }
}

#[test]
fn test_faction_affiliations() {
    let npcs = create_quest_npcs();

    let tutorial_assistant = npcs.iter().find(|npc| npc.id == "tutorial_assistant").unwrap();
    assert_eq!(tutorial_assistant.faction_affiliation, Some(sympathetic_resonance::systems::factions::FactionId::MagistersCouncil));

    let dr_felix = npcs.iter().find(|npc| npc.id == "dr_felix").unwrap();
    assert_eq!(dr_felix.faction_affiliation, Some(sympathetic_resonance::systems::factions::FactionId::NeutralScholars));

    let observer_lyra = npcs.iter().find(|npc| npc.id == "observer_lyra").unwrap();
    assert_eq!(observer_lyra.faction_affiliation, Some(sympathetic_resonance::systems::factions::FactionId::MagistersCouncil));

    let echo_voidwalker = npcs.iter().find(|npc| npc.id == "echo_voidwalker").unwrap();
    assert_eq!(echo_voidwalker.faction_affiliation, Some(sympathetic_resonance::systems::factions::FactionId::UndergroundNetwork));
}

#[test]
fn test_tutorial_assistant_has_faction_dialogue() {
    let npcs = create_quest_npcs();
    let tutorial_assistant = npcs.iter()
        .find(|npc| npc.id == "tutorial_assistant")
        .unwrap();

    let faction_specific = &tutorial_assistant.dialogue_tree.faction_specific;

    // Should have faction-specific greetings for 3 factions
    assert!(faction_specific.contains_key(&sympathetic_resonance::systems::factions::FactionId::MagistersCouncil));
    assert!(faction_specific.contains_key(&sympathetic_resonance::systems::factions::FactionId::NeutralScholars));
    assert!(faction_specific.contains_key(&sympathetic_resonance::systems::factions::FactionId::UndergroundNetwork));
}

#[test]
fn test_dr_felix_has_personality() {
    let npcs = create_quest_npcs();
    let dr_felix = npcs.iter()
        .find(|npc| npc.id == "dr_felix")
        .unwrap();

    assert!(dr_felix.personality.is_some());

    let personality = dr_felix.personality.as_ref().unwrap();
    assert!(personality.speaking_style.contains(&"analytical".to_string()));
    assert!(!personality.quirks.is_empty());
}

#[test]
fn test_dr_felix_has_quest_dialogue() {
    let npcs = create_quest_npcs();
    let dr_felix = npcs.iter()
        .find(|npc| npc.id == "dr_felix")
        .unwrap();

    // Should have dialogue for crystal_analysis quest
    assert!(dr_felix.quest_dialogue.contains_key("crystal_analysis"));

    let quest_dialogue = &dr_felix.quest_dialogue["crystal_analysis"];
    assert!(quest_dialogue.quest_intro.is_some());
    assert!(quest_dialogue.quest_in_progress.is_some());
    assert!(quest_dialogue.quest_completed.is_some());
}

#[test]
fn test_dr_felix_faction_specific_dialogue() {
    let npcs = create_quest_npcs();
    let dr_felix = npcs.iter()
        .find(|npc| npc.id == "dr_felix")
        .unwrap();

    let faction_specific = &dr_felix.dialogue_tree.faction_specific;

    // Dr. Felix should have opinions about Scholars and Consortium
    assert!(faction_specific.contains_key(&sympathetic_resonance::systems::factions::FactionId::NeutralScholars));
    assert!(faction_specific.contains_key(&sympathetic_resonance::systems::factions::FactionId::IndustrialConsortium));
}

#[test]
fn test_tutorial_assistant_personality() {
    let npcs = create_quest_npcs();
    let tutorial_assistant = npcs.iter()
        .find(|npc| npc.id == "tutorial_assistant")
        .unwrap();

    let personality = tutorial_assistant.personality.as_ref().unwrap();

    assert!(!personality.trait_description.is_empty());
    assert!(!personality.speaking_style.is_empty());
    assert!(!personality.quirks.is_empty());

    // Verify personality traits match character
    assert!(personality.speaking_style.contains(&"encouraging".to_string()));
    assert!(personality.speaking_style.contains(&"patient".to_string()));
}

#[test]
fn test_quest_dialogue_present() {
    let npcs = create_quest_npcs();
    let tutorial_assistant = npcs.iter()
        .find(|npc| npc.id == "tutorial_assistant")
        .unwrap();

    // Should have dialogue for resonance_foundation quest
    assert!(tutorial_assistant.quest_dialogue.contains_key("resonance_foundation"));

    let resonance_dialogue = &tutorial_assistant.quest_dialogue["resonance_foundation"];

    // Verify all quest stages have dialogue
    assert!(resonance_dialogue.quest_intro.is_some());
    assert!(resonance_dialogue.quest_in_progress.is_some());
    assert!(resonance_dialogue.quest_completed.is_some());

    // Verify objective-specific dialogue exists
    assert!(resonance_dialogue.objective_dialogue.contains_key("visit_practice_hall"));
    assert!(resonance_dialogue.objective_dialogue.contains_key("learn_harmonic_fundamentals"));
    assert!(resonance_dialogue.objective_dialogue.contains_key("demonstrate_resonance"));

    // Verify progress hints exist
    assert!(!resonance_dialogue.progress_hints.is_empty());
}

#[test]
fn test_dialogue_system_get_quest_dialogue() {
    let engine = create_test_engine_with_npcs();

    // Test getting quest intro dialogue
    let intro = engine.dialogue_system().get_quest_dialogue(
        "tutorial_assistant",
        "resonance_foundation",
        "intro"
    );

    assert!(intro.is_some(), "Should have intro dialogue");
    let intro_text = intro.unwrap();
    assert!(intro_text.contains("Elara Starweaver"));
    assert!(intro_text.contains("sympathetic resonance"));

    // Test getting in-progress dialogue
    let in_progress = engine.dialogue_system().get_quest_dialogue(
        "tutorial_assistant",
        "resonance_foundation",
        "in_progress"
    );

    assert!(in_progress.is_some(), "Should have in-progress dialogue");
    let progress_text = in_progress.unwrap();
    assert!(progress_text.contains("frequencies interact"));

    // Test getting completed dialogue
    let completed = engine.dialogue_system().get_quest_dialogue(
        "tutorial_assistant",
        "resonance_foundation",
        "completed"
    );

    assert!(completed.is_some(), "Should have completed dialogue");
    let completed_text = completed.unwrap();
    assert!(completed_text.contains("proud of you"));
}

#[test]
fn test_dialogue_system_get_objective_dialogue() {
    let engine = create_test_engine_with_npcs();

    // Test getting dialogue for visit_practice_hall objective
    let obj_dialogue = engine.dialogue_system().get_objective_dialogue(
        "tutorial_assistant",
        "resonance_foundation",
        "visit_practice_hall"
    );

    assert!(obj_dialogue.is_some(), "Should have objective dialogue");
    let dialogue_text = obj_dialogue.unwrap();
    assert!(dialogue_text.contains("Practice Hall"));

    // Test getting dialogue for demonstrate_resonance objective
    let demo_dialogue = engine.dialogue_system().get_objective_dialogue(
        "tutorial_assistant",
        "resonance_foundation",
        "demonstrate_resonance"
    );

    assert!(demo_dialogue.is_some(), "Should have demonstration dialogue");
    let demo_text = demo_dialogue.unwrap();
    assert!(demo_text.contains("practice crystal"));
}

#[test]
fn test_dialogue_system_get_progress_hint() {
    let engine = create_test_engine_with_npcs();

    let hint = engine.dialogue_system().get_progress_hint(
        "tutorial_assistant",
        "resonance_foundation"
    );

    assert!(hint.is_some(), "Should have progress hints");
    let hint_text = hint.unwrap();
    assert!(!hint_text.is_empty());
}

#[test]
fn test_tutorial_assistant_has_topics() {
    let npcs = create_quest_npcs();
    let tutorial_assistant = npcs.iter()
        .find(|npc| npc.id == "tutorial_assistant")
        .unwrap();

    let topics = &tutorial_assistant.dialogue_tree.topics;

    // Verify expected topics exist
    assert!(topics.contains_key("resonance"));
    assert!(topics.contains_key("crystals"));
    assert!(topics.contains_key("practice_tips"));
    assert!(topics.contains_key("resonance_results"));
}

#[test]
fn test_resonance_results_topic_has_requirements() {
    let npcs = create_quest_npcs();
    let tutorial_assistant = npcs.iter()
        .find(|npc| npc.id == "tutorial_assistant")
        .unwrap();

    let resonance_results_topic = tutorial_assistant.dialogue_tree.topics
        .get("resonance_results")
        .expect("Should have resonance_results topic");

    // This topic should require harmonic_fundamentals knowledge
    assert!(!resonance_results_topic.requirements.knowledge_requirements.is_empty());
    assert!(resonance_results_topic.requirements.knowledge_requirements.contains(&"harmonic_fundamentals".to_string()));

    // Should also have theory requirement
    assert!(!resonance_results_topic.requirements.theory_requirements.is_empty());
}

#[test]
fn test_tutorial_assistant_has_time_based_greetings() {
    let npcs = create_quest_npcs();
    let tutorial_assistant = npcs.iter()
        .find(|npc| npc.id == "tutorial_assistant")
        .unwrap();

    let time_greetings = &tutorial_assistant.dialogue_tree.time_based_greetings;

    assert!(time_greetings.contains_key("morning"));
    assert!(time_greetings.contains_key("afternoon"));
    assert!(time_greetings.contains_key("evening"));

    // Verify greetings are contextual
    let morning = &time_greetings["morning"];
    assert!(morning.contains("morning") || morning.contains("day"));
}

#[test]
fn test_dialogue_content_quality() {
    let npcs = create_quest_npcs();
    let tutorial_assistant = npcs.iter()
        .find(|npc| npc.id == "tutorial_assistant")
        .unwrap();

    let resonance_dialogue = &tutorial_assistant.quest_dialogue["resonance_foundation"];

    // Verify dialogue has substance (not just placeholders)
    let intro = resonance_dialogue.quest_intro.as_ref().unwrap();
    assert!(intro.len() > 200, "Quest intro should be substantial");
    assert!(intro.contains('\n'), "Should have paragraph breaks");

    let in_progress = resonance_dialogue.quest_in_progress.as_ref().unwrap();
    assert!(in_progress.len() > 200, "In-progress dialogue should be substantial");

    let completed = resonance_dialogue.quest_completed.as_ref().unwrap();
    assert!(completed.len() > 200, "Completed dialogue should be substantial");
}

#[test]
fn test_all_objectives_have_dialogue() {
    let npcs = create_quest_npcs();
    let tutorial_assistant = npcs.iter()
        .find(|npc| npc.id == "tutorial_assistant")
        .unwrap();

    let resonance_dialogue = &tutorial_assistant.quest_dialogue["resonance_foundation"];

    // These are the main objectives from the resonance_foundation quest
    let expected_objectives = vec![
        "visit_practice_hall",
        "learn_harmonic_fundamentals",
        "demonstrate_resonance",
    ];

    for objective in expected_objectives {
        assert!(
            resonance_dialogue.objective_dialogue.contains_key(objective),
            "Missing dialogue for objective: {}",
            objective
        );

        let dialogue = &resonance_dialogue.objective_dialogue[objective];
        assert!(
            !dialogue.is_empty(),
            "Dialogue for objective {} is empty",
            objective
        );
    }
}

#[test]
fn test_npc_description_is_rich() {
    let npcs = create_quest_npcs();
    let tutorial_assistant = npcs.iter()
        .find(|npc| npc.id == "tutorial_assistant")
        .unwrap();

    // Verify description is detailed and immersive
    assert!(tutorial_assistant.description.len() > 100);
    assert!(tutorial_assistant.description.contains("robes"));
    assert!(tutorial_assistant.description.contains("crystal"));
}

#[test]
fn test_multiple_greeting_variations() {
    let npcs = create_quest_npcs();
    let tutorial_assistant = npcs.iter()
        .find(|npc| npc.id == "tutorial_assistant")
        .unwrap();

    let greeting_templates = &tutorial_assistant.dialogue_tree.greeting.text_templates;

    // Should have multiple greeting variations
    assert!(greeting_templates.len() >= 3, "Should have at least 3 greeting variations");

    // Each greeting should be unique
    for i in 0..greeting_templates.len() {
        for j in (i + 1)..greeting_templates.len() {
            assert_ne!(
                greeting_templates[i], greeting_templates[j],
                "Greetings should be unique"
            );
        }
    }
}
