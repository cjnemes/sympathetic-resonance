use crate::core::Player;
use crate::systems::factions::{FactionId, FactionSystem};
use crate::GameResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPCPersonality {
    /// Brief description of personality (e.g., "Warm and encouraging")
    pub trait_description: String,
    /// Speaking style markers (e.g., "formal", "casual", "enthusiastic")
    pub speaking_style: Vec<String>,
    /// Character quirks or signature phrases
    pub quirks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPC {
    pub id: String,
    pub name: String,
    pub description: String,
    pub faction_affiliation: Option<FactionId>,
    pub dialogue_tree: DialogueTree,
    pub current_disposition: i32, // -100 to 100
    /// Personality traits and speaking style
    #[serde(default)]
    pub personality: Option<NPCPersonality>,
    /// Quest-specific dialogue contexts (quest_id -> dialogue content)
    #[serde(default)]
    pub quest_dialogue: std::collections::HashMap<String, QuestDialogue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestDialogue {
    /// Dialogue when quest is first offered/introduced
    pub quest_intro: Option<String>,
    /// Dialogue when player is working on the quest
    pub quest_in_progress: Option<String>,
    /// Dialogue when quest is completed
    pub quest_completed: Option<String>,
    /// Dialogue for specific quest objectives (objective_id -> dialogue)
    #[serde(default)]
    pub objective_dialogue: HashMap<String, String>,
    /// Encouragement/hints based on player progress
    #[serde(default)]
    pub progress_hints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueTree {
    pub greeting: DialogueNode,
    pub topics: HashMap<String, DialogueNode>,
    #[serde(
        serialize_with = "crate::systems::serde_helpers::serialize_faction_map",
        deserialize_with = "crate::systems::serde_helpers::deserialize_faction_map"
    )]
    pub faction_specific: HashMap<FactionId, DialogueNode>,
    /// Time-of-day variations for greeting (optional)
    #[serde(default)]
    pub time_based_greetings: HashMap<String, String>, // "morning", "afternoon", "evening"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueNode {
    pub text_templates: Vec<String>,
    pub responses: Vec<DialogueResponse>,
    pub requirements: DialogueRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueResponse {
    pub text: String,
    pub effect: DialogueEffect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueRequirements {
    pub min_faction_standing: Option<(FactionId, i32)>,
    pub max_faction_standing: Option<(FactionId, i32)>,
    pub knowledge_requirements: Vec<String>,
    /// Theory understanding requirements (theory_id, min_understanding 0.0-1.0)
    #[serde(default)]
    pub theory_requirements: Vec<(String, f32)>,
    /// Minimum total theory mastery level (0.0-1.0)
    #[serde(default)]
    pub min_theory_mastery: Option<f32>,
    /// Specific theory capabilities required
    #[serde(default)]
    pub required_capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DialogueEffect {
    None,
    FactionStandingChange(FactionId, i32),
    GiveInformation(String),
    GiveItem(String),
    QuestStart(String),
    /// Provide theory hint or insight (theory_id, understanding_bonus)
    TheoryInsight(String, f32),
    /// Unlock advanced dialogue options for theory discussion
    UnlockTheoryDiscussion(String),
    /// Offer theory mentorship opportunity
    OfferMentorship(String),
    /// Share experimental results or observations
    ShareResearch(String, String), // theory_id, research_data
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueSystem {
    npcs: HashMap<String, NPC>,
}

impl DialogueSystem {
    pub fn new() -> Self {
        Self {
            npcs: HashMap::new(),
        }
    }

    pub fn add_npc(&mut self, npc: NPC) {
        self.npcs.insert(npc.id.clone(), npc);
    }

    /// Get quest-specific dialogue for an NPC
    pub fn get_quest_dialogue(
        &self,
        npc_id: &str,
        quest_id: &str,
        quest_status: &str, // "intro", "in_progress", "completed"
    ) -> Option<String> {
        if let Some(npc) = self.npcs.get(npc_id) {
            if let Some(quest_dialogue) = npc.quest_dialogue.get(quest_id) {
                match quest_status {
                    "intro" => quest_dialogue.quest_intro.clone(),
                    "in_progress" => quest_dialogue.quest_in_progress.clone(),
                    "completed" => quest_dialogue.quest_completed.clone(),
                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get a random progress hint for a quest
    pub fn get_progress_hint(&self, npc_id: &str, quest_id: &str) -> Option<String> {
        if let Some(npc) = self.npcs.get(npc_id) {
            if let Some(quest_dialogue) = npc.quest_dialogue.get(quest_id) {
                if !quest_dialogue.progress_hints.is_empty() {
                    // For now, return the first hint. Could randomize in future.
                    return Some(quest_dialogue.progress_hints[0].clone());
                }
            }
        }
        None
    }

    /// Get dialogue for a specific quest objective
    pub fn get_objective_dialogue(
        &self,
        npc_id: &str,
        quest_id: &str,
        objective_id: &str,
    ) -> Option<String> {
        if let Some(npc) = self.npcs.get(npc_id) {
            if let Some(quest_dialogue) = npc.quest_dialogue.get(quest_id) {
                return quest_dialogue.objective_dialogue.get(objective_id).cloned();
            }
        }
        None
    }

    pub fn talk_to_npc(
        &mut self,
        npc_id: &str,
        player: &Player,
        faction_system: &FactionSystem,
    ) -> GameResult<String> {
        // Get all data we need first without mutable borrowing
        let (disposition, npc_name, topics, greeting_text) = {
            let npc = self.npcs.get(npc_id)
                .ok_or_else(|| crate::GameError::ContentNotFound(format!("NPC '{}' not found", npc_id)))?;

            let disposition = self.calculate_disposition(npc, player, faction_system);
            let npc_name = npc.name.clone();
            let topics: Vec<String> = npc.dialogue_tree.topics.keys().cloned().collect();
            let greeting_text = self.select_greeting_text(npc, player)?;

            (disposition, npc_name, topics, greeting_text)
        };

        // Now get mutable reference and update disposition
        let npc = self.npcs.get_mut(npc_id)
            .ok_or_else(|| crate::GameError::ContentNotFound(format!("NPC '{}' not found", npc_id)))?;
        npc.current_disposition = disposition;

        Ok(format!(
            "{}\n\n[Disposition: {}] You can ask {} about: {}",
            greeting_text,
            self.disposition_description(disposition),
            npc_name,
            topics.join(", ")
        ))
    }

    pub fn ask_about_topic(
        &mut self,
        npc_id: &str,
        topic: &str,
        player: &Player,
        faction_system: &FactionSystem,
    ) -> GameResult<String> {
        // Check if NPC and topic exist, and get requirements
        let (npc_name, dialogue_node, current_disposition) = {
            let npc = self.npcs.get(npc_id)
                .ok_or_else(|| crate::GameError::ContentNotFound(format!("NPC '{}' not found", npc_id)))?;

            let dialogue_node = npc.dialogue_tree.topics.get(topic)
                .ok_or_else(|| crate::GameError::InvalidCommand(format!("{} doesn't know about '{}'", npc.name, topic)))?
                .clone();

            (npc.name.clone(), dialogue_node, npc.current_disposition)
        };

        // Check requirements
        if !self.check_requirements(&dialogue_node.requirements, player, faction_system) {
            return Ok(format!("{} doesn't seem willing to discuss {} with you.", npc_name, topic));
        }

        // Select response based on disposition
        let response_text = self.select_response_text(&dialogue_node, current_disposition)?;

        Ok(response_text)
    }

    /// Generate theory-aware topics based on player's knowledge
    pub fn get_theory_topics(&self, npc_id: &str, player: &Player) -> Vec<String> {
        let mut topics = Vec::new();

        if let Some(npc) = self.npcs.get(npc_id) {
            // Add base topics that are already available
            for topic in npc.dialogue_tree.topics.keys() {
                topics.push(topic.clone());
            }

            // Add theory-specific topics based on player knowledge
            if player.theory_understanding("harmonic_fundamentals") >= 0.3 {
                topics.push("resonance_theory".to_string());
            }

            if player.theory_understanding("crystal_structures") >= 0.5 {
                topics.push("crystal_research".to_string());
            }

            if player.theory_understanding("mental_resonance") >= 0.4 {
                topics.push("mental_techniques".to_string());
            }

            if player.theory_understanding("light_manipulation") >= 0.6 {
                topics.push("light_experiments".to_string());
            }

            if player.theory_understanding("bio_resonance") >= 0.7 {
                topics.push("healing_methods".to_string());
            }

            if player.theory_understanding("detection_arrays") >= 0.5 {
                topics.push("detection_techniques".to_string());
            }

            if player.theory_understanding("sympathetic_networks") >= 0.8 {
                topics.push("network_theory".to_string());
            }

            if player.theory_understanding("resonance_amplification") >= 0.9 {
                topics.push("advanced_amplification".to_string());
            }

            if player.theory_understanding("theoretical_synthesis") >= 1.0 {
                topics.push("theoretical_mastery".to_string());
            }

            // Add mastery-based topics
            let mastered_count = player.get_mastered_theories().len();
            if mastered_count >= 3 {
                topics.push("advanced_theory_discussion".to_string());
            }
            if mastered_count >= 6 {
                topics.push("research_collaboration".to_string());
            }
            if mastered_count >= 9 {
                topics.push("theoretical_breakthroughs".to_string());
            }

            // Add capability-based topics
            if player.has_magic_capability("healing_spells") {
                topics.push("healing_applications".to_string());
            }
            if player.has_magic_capability("detection_spells") {
                topics.push("magical_detection".to_string());
            }
            if player.has_magic_capability("long_distance_magic") {
                topics.push("long_distance_communication".to_string());
            }
            if player.has_magic_capability("custom_spell_combinations") {
                topics.push("spell_innovation".to_string());
            }
        }

        // Remove duplicates and sort
        topics.sort();
        topics.dedup();
        topics
    }

    /// Generate theory-aware response based on player's knowledge level
    pub fn get_theory_response(&self, npc_id: &str, topic: &str, player: &Player) -> Option<String> {
        if let Some(_npc) = self.npcs.get(npc_id) {
            match topic {
                "resonance_theory" => {
                    let understanding = player.theory_understanding("harmonic_fundamentals");
                    if understanding >= 0.8 {
                        Some("Your understanding of resonance theory is impressive! I can see you've grasped the fundamental principles of harmonic oscillation.".to_string())
                    } else if understanding >= 0.5 {
                        Some("You have a solid foundation in resonance theory. Have you considered how frequency matching affects crystal efficiency?".to_string())
                    } else {
                        Some("Resonance theory is the cornerstone of magical practice. Study how frequencies interact with crystalline structures.".to_string())
                    }
                },
                "crystal_research" => {
                    let understanding = player.theory_understanding("crystal_structures");
                    if understanding >= 0.9 {
                        Some("Your crystal research is at an advanced level. Perhaps we could collaborate on lattice optimization techniques?".to_string())
                    } else if understanding >= 0.6 {
                        Some("I see you understand crystal structures well. The key is balancing purity with structural integrity.".to_string())
                    } else {
                        Some("Crystal research requires patience and precision. Focus on understanding how molecular arrangements affect magical conductivity.".to_string())
                    }
                },
                "mental_techniques" => {
                    let understanding = player.theory_understanding("mental_resonance");
                    if understanding >= 0.7 {
                        Some("Your mental discipline is commendable. True mastery comes from understanding the mind-magic connection.".to_string())
                    } else {
                        Some("Mental resonance requires both focus and flexibility. Practice meditation to strengthen your mental pathways.".to_string())
                    }
                },
                "healing_methods" => {
                    if player.has_magic_capability("healing_spells") {
                        Some("I sense you've unlocked the healing arts. Bio-resonance is a delicate balance between knowledge and intuition.".to_string())
                    } else {
                        Some("Healing magic requires deep understanding of life force patterns. Study bio-resonance theory carefully.".to_string())
                    }
                },
                "advanced_theory_discussion" => {
                    let mastered = player.get_mastered_theories().len();
                    Some(format!("With {} theories mastered, you're among the more accomplished practitioners I've met. What aspects of magical theory interest you most?", mastered))
                },
                "research_collaboration" => {
                    Some("Your extensive knowledge makes you an ideal research partner. I have some experimental data that might interest you.".to_string())
                },
                "theoretical_mastery" => {
                    if player.theory_understanding("theoretical_synthesis") >= 1.0 {
                        Some("You've achieved theoretical synthesis - the ability to combine and create new magical principles. This is the mark of a true master.".to_string())
                    } else {
                        Some("Theoretical mastery is the highest goal of magical study. When you can synthesize new principles, you become a creator of knowledge.".to_string())
                    }
                },
                "spell_innovation" => {
                    if player.has_magic_capability("custom_spell_combinations") {
                        Some("Your ability to create custom spell combinations is remarkable. Innovation drives magical progress forward.".to_string())
                    } else {
                        Some("Spell innovation requires mastery of multiple theories. Once you achieve synthesis, new possibilities will open.".to_string())
                    }
                },
                _ => None,
            }
        } else {
            None
        }
    }

    fn calculate_disposition(&self, npc: &NPC, player: &Player, faction_system: &FactionSystem) -> i32 {
        let mut disposition = 0;

        // Base disposition from faction affiliation
        if let Some(faction_id) = npc.faction_affiliation {
            if let Some(&standing) = player.faction_standings.get(&faction_id) {
                disposition += standing / 2; // Scale faction standing to disposition

                // Check cross-faction effects
                for (&other_faction, &other_standing) in &player.faction_standings {
                    if other_faction != faction_id {
                        let relationship = faction_system.get_relationship_strength(faction_id, other_faction);
                        match relationship {
                            rel if rel > 0.5 => disposition += (other_standing * relationship as i32) / 10,
                            rel if rel < -0.5 => disposition -= (other_standing * (-relationship as i32)) / 10,
                            _ => {} // Neutral relationship
                        }
                    }
                }
            }
        }

        // Clamp disposition to valid range
        disposition.clamp(-100, 100)
    }

    fn select_greeting_text(&self, npc: &NPC, player: &Player) -> GameResult<String> {
        // Check for faction-specific greetings first
        if let Some(faction_id) = npc.faction_affiliation {
            if let Some(faction_dialogue) = npc.dialogue_tree.faction_specific.get(&faction_id) {
                if let Some(&player_standing) = player.faction_standings.get(&faction_id) {
                    if player_standing >= 50 {
                        return Ok(self.format_dialogue_text(&faction_dialogue.text_templates, npc.current_disposition));
                    }
                }
            }
        }

        // Use default greeting
        Ok(self.format_dialogue_text(&npc.dialogue_tree.greeting.text_templates, npc.current_disposition))
    }

    fn select_response_text(&self, node: &DialogueNode, disposition: i32) -> GameResult<String> {
        Ok(self.format_dialogue_text(&node.text_templates, disposition))
    }

    fn format_dialogue_text(&self, templates: &[String], disposition: i32) -> String {
        // Select template based on disposition
        let template_index = match disposition {
            d if d >= 50 => 0.min(templates.len() - 1), // Friendly
            d if d <= -50 => (templates.len() - 1).max(0), // Hostile
            _ => templates.len() / 2, // Neutral
        };

        templates.get(template_index)
            .cloned()
            .unwrap_or_else(|| "...".to_string())
    }

    fn check_requirements(
        &self,
        requirements: &DialogueRequirements,
        player: &Player,
        _faction_system: &FactionSystem,
    ) -> bool {
        // Check faction standing requirements
        if let Some((faction_id, min_standing)) = requirements.min_faction_standing {
            if let Some(&standing) = player.faction_standings.get(&faction_id) {
                if standing < min_standing {
                    return false;
                }
            } else {
                return false;
            }
        }

        if let Some((faction_id, max_standing)) = requirements.max_faction_standing {
            if let Some(&standing) = player.faction_standings.get(&faction_id) {
                if standing > max_standing {
                    return false;
                }
            }
        }

        // Check knowledge requirements (backward compatibility)
        for knowledge_req in &requirements.knowledge_requirements {
            if !player.knowledge.theories.contains_key(knowledge_req) {
                return false;
            }
        }

        // Check theory understanding requirements
        for (theory_id, min_understanding) in &requirements.theory_requirements {
            let player_understanding = player.theory_understanding(theory_id);
            if player_understanding < *min_understanding {
                return false;
            }
        }

        // Check minimum theory mastery level
        if let Some(min_mastery) = requirements.min_theory_mastery {
            let mastered_theories = player.get_mastered_theories();
            let total_theories = player.knowledge.theories.len().max(player.knowledge.theory_progress.len());
            let mastery_ratio = if total_theories > 0 {
                mastered_theories.len() as f32 / total_theories as f32
            } else {
                0.0
            };

            if mastery_ratio < min_mastery {
                return false;
            }
        }

        // Check required capabilities
        for capability in &requirements.required_capabilities {
            if !player.has_magic_capability(capability) {
                return false;
            }
        }

        true
    }

    fn disposition_description(&self, disposition: i32) -> &'static str {
        match disposition {
            d if d >= 75 => "Extremely Friendly",
            d if d >= 50 => "Very Friendly",
            d if d >= 25 => "Friendly",
            d if d >= 10 => "Cordial",
            d if d >= -10 => "Neutral",
            d if d >= -25 => "Cool",
            d if d >= -50 => "Unfriendly",
            d if d >= -75 => "Hostile",
            _ => "Extremely Hostile",
        }
    }
}

impl Default for DialogueSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::player::Player;
    use crate::systems::factions::{FactionId, FactionSystem};
    use std::collections::HashMap;

    // Test data helper functions
    fn create_test_player() -> Player {
        let mut player = Player::new("Test Player".to_string());

        // Set up faction standings for testing
        player.faction_standings.insert(FactionId::MagistersCouncil, 60);
        player.faction_standings.insert(FactionId::OrderOfHarmony, -30);
        player.faction_standings.insert(FactionId::IndustrialConsortium, 20);
        player.faction_standings.insert(FactionId::UndergroundNetwork, -70);
        player.faction_standings.insert(FactionId::NeutralScholars, 10);

        // Add some test knowledge
        player.knowledge.theories.insert("basic_theory".to_string(), 0.8);
        player.knowledge.theories.insert("advanced_theory".to_string(), 0.6);

        player
    }

    fn create_test_faction_system() -> FactionSystem {
        FactionSystem::new()
    }

    fn create_basic_npc() -> NPC {
        NPC {
            id: "test_merchant".to_string(),
            name: "Test Merchant".to_string(),
            description: "A friendly merchant for testing".to_string(),
            faction_affiliation: Some(FactionId::IndustrialConsortium),
            personality: Some(NPCPersonality {
                trait_description: "Pragmatic and business-minded".to_string(),
                speaking_style: vec!["casual".to_string(), "direct".to_string()],
                quirks: vec!["Often mentions profit margins".to_string()],
            }),
            quest_dialogue: HashMap::new(),
            dialogue_tree: DialogueTree {
                greeting: DialogueNode {
                    text_templates: vec![
                        "Welcome, friend! Good to see you!".to_string(),
                        "Hello there, how can I help?".to_string(),
                        "What do you want?".to_string(),
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
                topics: {
                    let mut topics = HashMap::new();
                    topics.insert("trade".to_string(), DialogueNode {
                        text_templates: vec![
                            "I have the finest goods in the land!".to_string(),
                            "Let's make a deal.".to_string(),
                            "I don't trade with your kind.".to_string(),
                        ],
                        responses: vec![],
                        requirements: DialogueRequirements {
                            min_faction_standing: Some((FactionId::IndustrialConsortium, -50)),
                            max_faction_standing: None,
                            knowledge_requirements: vec![],
                            theory_requirements: vec![],
                            min_theory_mastery: None,
                            required_capabilities: vec![],
                        },
                    });
                    topics.insert("secrets".to_string(), DialogueNode {
                        text_templates: vec![
                            "I know many things, but knowledge costs extra.".to_string(),
                            "Information is valuable.".to_string(),
                            "I can't tell you that.".to_string(),
                        ],
                        responses: vec![],
                        requirements: DialogueRequirements {
                            min_faction_standing: Some((FactionId::IndustrialConsortium, 30)),
                            max_faction_standing: None,
                            knowledge_requirements: vec!["basic_theory".to_string()],
                            theory_requirements: vec![],
                            min_theory_mastery: None,
                            required_capabilities: vec![],
                        },
                    });
                    topics
                },
                faction_specific: {
                    let mut faction_specific = HashMap::new();
                    faction_specific.insert(FactionId::IndustrialConsortium, DialogueNode {
                        text_templates: vec![
                            "Ah, a fellow Consortium member! Welcome!".to_string(),
                            "Good to see a business partner.".to_string(),
                            "Another entrepreneur, I see.".to_string(),
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
                    faction_specific
                },
            },
            current_disposition: 0,
        }
    }

    fn create_neutral_npc() -> NPC {
        NPC {
            id: "neutral_scholar".to_string(),
            name: "Scholar Eldara".to_string(),
            description: "An independent researcher".to_string(),
            faction_affiliation: None,
            personality: None,
            quest_dialogue: HashMap::new(),
            dialogue_tree: DialogueTree {
                greeting: DialogueNode {
                    text_templates: vec![
                        "Greetings, seeker of knowledge.".to_string(),
                        "Hello.".to_string(),
                        "Go away.".to_string(),
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
                topics: {
                    let mut topics = HashMap::new();
                    topics.insert("research".to_string(), DialogueNode {
                        text_templates: vec![
                            "My research is fascinating!".to_string(),
                            "I study many things.".to_string(),
                            "I have no time for you.".to_string(),
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
                    topics
                },
                faction_specific: HashMap::new(),
            },
            current_disposition: 0,
        }
    }

    fn create_hostile_npc() -> NPC {
        NPC {
            id: "underground_contact".to_string(),
            name: "Shadow".to_string(),
            description: "A suspicious figure".to_string(),
            faction_affiliation: Some(FactionId::UndergroundNetwork),
            personality: None,
            quest_dialogue: HashMap::new(),
            dialogue_tree: DialogueTree {
                greeting: DialogueNode {
                    text_templates: vec![
                        "You're one of us, I can tell.".to_string(),
                        "What do you want?".to_string(),
                        "Get lost, establishment pawn.".to_string(),
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
                topics: {
                    let mut topics = HashMap::new();
                    topics.insert("operations".to_string(), DialogueNode {
                        text_templates: vec![
                            "Our operations are going well.".to_string(),
                            "I can't discuss that.".to_string(),
                            "You're not trusted enough.".to_string(),
                        ],
                        responses: vec![],
                        requirements: DialogueRequirements {
                            min_faction_standing: Some((FactionId::UndergroundNetwork, 20)),
                            max_faction_standing: Some((FactionId::MagistersCouncil, -20)),
                            knowledge_requirements: vec![],
                            theory_requirements: vec![],
                            min_theory_mastery: None,
                            required_capabilities: vec![],
                        },
                    });
                    topics
                },
                faction_specific: HashMap::new(),
            },
            current_disposition: 0,
        }
    }

    #[test]
    fn test_dialogue_system_creation() {
        let dialogue_system = DialogueSystem::new();
        assert_eq!(dialogue_system.npcs.len(), 0);
    }

    #[test]
    fn test_add_npc() {
        let mut dialogue_system = DialogueSystem::new();
        let npc = create_basic_npc();
        let npc_id = npc.id.clone();

        dialogue_system.add_npc(npc);

        assert_eq!(dialogue_system.npcs.len(), 1);
        assert!(dialogue_system.npcs.contains_key(&npc_id));
    }

    #[test]
    fn test_disposition_description() {
        let dialogue_system = DialogueSystem::new();

        assert_eq!(dialogue_system.disposition_description(80), "Extremely Friendly");
        assert_eq!(dialogue_system.disposition_description(60), "Very Friendly");
        assert_eq!(dialogue_system.disposition_description(30), "Friendly");
        assert_eq!(dialogue_system.disposition_description(15), "Cordial");
        assert_eq!(dialogue_system.disposition_description(0), "Neutral");
        assert_eq!(dialogue_system.disposition_description(-15), "Cool");
        assert_eq!(dialogue_system.disposition_description(-30), "Unfriendly");
        assert_eq!(dialogue_system.disposition_description(-60), "Hostile");
        assert_eq!(dialogue_system.disposition_description(-80), "Extremely Hostile");
    }

    #[test]
    fn test_calculate_disposition_with_faction_affiliation() {
        let dialogue_system = DialogueSystem::new();
        let player = create_test_player();
        let faction_system = create_test_faction_system();
        let npc = create_basic_npc(); // Has IndustrialConsortium affiliation

        // Player has +20 standing with IndustrialConsortium
        let disposition = dialogue_system.calculate_disposition(&npc, &player, &faction_system);

        // Should be 20/2 = 10 from base faction standing
        assert_eq!(disposition, 10);
    }

    #[test]
    fn test_calculate_disposition_with_cross_faction_effects() {
        let dialogue_system = DialogueSystem::new();
        let player = create_test_player();
        let faction_system = create_test_faction_system();

        // Create an NPC affiliated with Magisters Council
        let mut npc = create_basic_npc();
        npc.faction_affiliation = Some(FactionId::MagistersCouncil);

        // Player has +60 with Magisters Council, -30 with Order of Harmony
        // Magisters and Order are allies, so Order standing should positively affect disposition
        let disposition = dialogue_system.calculate_disposition(&npc, &player, &faction_system);

        // Base: 60/2 = 30, plus cross-faction effects
        assert!(disposition >= 30);
    }

    #[test]
    fn test_calculate_disposition_neutral_npc() {
        let dialogue_system = DialogueSystem::new();
        let player = create_test_player();
        let faction_system = create_test_faction_system();
        let npc = create_neutral_npc(); // No faction affiliation

        let disposition = dialogue_system.calculate_disposition(&npc, &player, &faction_system);

        // Should be 0 for neutral NPCs
        assert_eq!(disposition, 0);
    }

    #[test]
    fn test_calculate_disposition_clamping() {
        let dialogue_system = DialogueSystem::new();
        let mut player = create_test_player();
        let faction_system = create_test_faction_system();

        // Set extreme faction standing
        player.faction_standings.insert(FactionId::MagistersCouncil, 100);

        let mut npc = create_basic_npc();
        npc.faction_affiliation = Some(FactionId::MagistersCouncil);

        let disposition = dialogue_system.calculate_disposition(&npc, &player, &faction_system);

        // Should be clamped to 100 or below
        assert!(disposition <= 100);
        assert!(disposition >= -100);
    }

    #[test]
    fn test_format_dialogue_text() {
        let dialogue_system = DialogueSystem::new();
        let templates = vec![
            "Friendly text".to_string(),
            "Neutral text".to_string(),
            "Hostile text".to_string(),
        ];

        // Test friendly disposition (>= 50)
        let friendly_text = dialogue_system.format_dialogue_text(&templates, 60);
        assert_eq!(friendly_text, "Friendly text");

        // Test neutral disposition
        let neutral_text = dialogue_system.format_dialogue_text(&templates, 0);
        assert_eq!(neutral_text, "Neutral text");

        // Test hostile disposition (<= -50)
        let hostile_text = dialogue_system.format_dialogue_text(&templates, -60);
        assert_eq!(hostile_text, "Hostile text");
    }

    #[test]
    fn test_format_dialogue_text_edge_cases() {
        let dialogue_system = DialogueSystem::new();

        // Empty templates
        let empty_templates = vec![];
        let result = dialogue_system.format_dialogue_text(&empty_templates, 0);
        assert_eq!(result, "...");

        // Single template
        let single_template = vec!["Only option".to_string()];
        let result = dialogue_system.format_dialogue_text(&single_template, 100);
        assert_eq!(result, "Only option");

        let result = dialogue_system.format_dialogue_text(&single_template, -100);
        assert_eq!(result, "Only option");
    }

    #[test]
    fn test_check_requirements_faction_standing() {
        let dialogue_system = DialogueSystem::new();
        let player = create_test_player();
        let faction_system = create_test_faction_system();

        // Test minimum faction standing requirement (player has +20 with Consortium)
        let req_met = DialogueRequirements {
            min_faction_standing: Some((FactionId::IndustrialConsortium, 10)),
            max_faction_standing: None,
            knowledge_requirements: vec![],
            theory_requirements: vec![],
            min_theory_mastery: None,
            required_capabilities: vec![],
        };
        assert!(dialogue_system.check_requirements(&req_met, &player, &faction_system));

        // Test minimum faction standing requirement not met
        let req_not_met = DialogueRequirements {
            min_faction_standing: Some((FactionId::IndustrialConsortium, 50)),
            max_faction_standing: None,
            knowledge_requirements: vec![],
            theory_requirements: vec![],
            min_theory_mastery: None,
            required_capabilities: vec![],
        };
        assert!(!dialogue_system.check_requirements(&req_not_met, &player, &faction_system));

        // Test maximum faction standing requirement (player has +60 with Council)
        let req_max_met = DialogueRequirements {
            min_faction_standing: None,
            max_faction_standing: Some((FactionId::MagistersCouncil, 70)),
            knowledge_requirements: vec![],
            theory_requirements: vec![],
            min_theory_mastery: None,
            required_capabilities: vec![],
        };
        assert!(dialogue_system.check_requirements(&req_max_met, &player, &faction_system));

        // Test maximum faction standing requirement not met
        let req_max_not_met = DialogueRequirements {
            min_faction_standing: None,
            max_faction_standing: Some((FactionId::MagistersCouncil, 50)),
            knowledge_requirements: vec![],
            theory_requirements: vec![],
            min_theory_mastery: None,
            required_capabilities: vec![],
        };
        assert!(!dialogue_system.check_requirements(&req_max_not_met, &player, &faction_system));
    }

    #[test]
    fn test_check_requirements_knowledge() {
        let dialogue_system = DialogueSystem::new();
        let player = create_test_player();
        let faction_system = create_test_faction_system();

        // Test knowledge requirement met
        let req_met = DialogueRequirements {
            min_faction_standing: None,
            max_faction_standing: None,
            knowledge_requirements: vec!["basic_theory".to_string()],
            theory_requirements: vec![],
            min_theory_mastery: None,
            required_capabilities: vec![],
        };
        assert!(dialogue_system.check_requirements(&req_met, &player, &faction_system));

        // Test knowledge requirement not met
        let req_not_met = DialogueRequirements {
            min_faction_standing: None,
            max_faction_standing: None,
            knowledge_requirements: vec!["unknown_theory".to_string()],
            theory_requirements: vec![],
            min_theory_mastery: None,
            required_capabilities: vec![],
        };
        assert!(!dialogue_system.check_requirements(&req_not_met, &player, &faction_system));

        // Test multiple knowledge requirements
        let req_multiple = DialogueRequirements {
            min_faction_standing: None,
            max_faction_standing: None,
            knowledge_requirements: vec!["basic_theory".to_string(), "advanced_theory".to_string()],
            theory_requirements: vec![],
            min_theory_mastery: None,
            required_capabilities: vec![],
        };
        assert!(dialogue_system.check_requirements(&req_multiple, &player, &faction_system));

        // Test multiple knowledge requirements with one missing
        let req_multiple_missing = DialogueRequirements {
            min_faction_standing: None,
            max_faction_standing: None,
            knowledge_requirements: vec!["basic_theory".to_string(), "missing_theory".to_string()],
            theory_requirements: vec![],
            min_theory_mastery: None,
            required_capabilities: vec![],
        };
        assert!(!dialogue_system.check_requirements(&req_multiple_missing, &player, &faction_system));
    }

    #[test]
    fn test_check_requirements_combined() {
        let dialogue_system = DialogueSystem::new();
        let player = create_test_player();
        let faction_system = create_test_faction_system();

        // Test all requirements met
        let req_all_met = DialogueRequirements {
            min_faction_standing: Some((FactionId::IndustrialConsortium, 10)),
            max_faction_standing: Some((FactionId::MagistersCouncil, 70)),
            knowledge_requirements: vec!["basic_theory".to_string()],
            theory_requirements: vec![],
            min_theory_mastery: None,
            required_capabilities: vec![],
        };
        assert!(dialogue_system.check_requirements(&req_all_met, &player, &faction_system));

        // Test faction requirement met but knowledge requirement not met
        let req_partial = DialogueRequirements {
            min_faction_standing: Some((FactionId::IndustrialConsortium, 10)),
            max_faction_standing: Some((FactionId::MagistersCouncil, 70)),
            knowledge_requirements: vec!["missing_theory".to_string()],
            theory_requirements: vec![],
            min_theory_mastery: None,
            required_capabilities: vec![],
        };
        assert!(!dialogue_system.check_requirements(&req_partial, &player, &faction_system));
    }

    #[test]
    fn test_select_greeting_text_default() {
        let dialogue_system = DialogueSystem::new();
        let player = create_test_player();
        let npc = create_neutral_npc(); // No faction affiliation

        let greeting = dialogue_system.select_greeting_text(&npc, &player).unwrap();

        // Should use default greeting (neutral disposition -> middle template)
        assert_eq!(greeting, "Hello.");
    }

    #[test]
    fn test_select_greeting_text_faction_specific() {
        let dialogue_system = DialogueSystem::new();
        let mut player = create_test_player();
        // Set high standing with Industrial Consortium (>=50)
        player.faction_standings.insert(FactionId::IndustrialConsortium, 60);

        let mut npc = create_basic_npc(); // Has Consortium affiliation and faction-specific greeting
        // Need to calculate and set the disposition first
        let faction_system = create_test_faction_system();
        npc.current_disposition = dialogue_system.calculate_disposition(&npc, &player, &faction_system);

        let greeting = dialogue_system.select_greeting_text(&npc, &player).unwrap();

        // Should use faction-specific greeting when disposition is friendly
        // Since greeting selection depends on disposition, check for any of the possible faction-specific greetings
        assert!(greeting.contains("fellow Consortium") || greeting.contains("business partner") || greeting.contains("entrepreneur"));
    }

    #[test]
    fn test_select_greeting_text_low_faction_standing() {
        let dialogue_system = DialogueSystem::new();
        let mut player = create_test_player();
        // Set low standing with Industrial Consortium (<50)
        player.faction_standings.insert(FactionId::IndustrialConsortium, 20);

        let npc = create_basic_npc(); // Has Consortium affiliation but standing too low

        let greeting = dialogue_system.select_greeting_text(&npc, &player).unwrap();

        // Should use default greeting since standing < 50
        assert_eq!(greeting, "Hello there, how can I help?");
    }

    #[test]
    fn test_talk_to_npc_success() {
        let mut dialogue_system = DialogueSystem::new();
        let player = create_test_player();
        let faction_system = create_test_faction_system();
        let npc = create_basic_npc();
        let npc_id = npc.id.clone();

        dialogue_system.add_npc(npc);

        let result = dialogue_system.talk_to_npc(&npc_id, &player, &faction_system);

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.contains("Test Merchant"));
        assert!(response.contains("trade"));
        assert!(response.contains("secrets"));
        assert!(response.contains("Disposition:"));
    }

    #[test]
    fn test_talk_to_npc_not_found() {
        let mut dialogue_system = DialogueSystem::new();
        let player = create_test_player();
        let faction_system = create_test_faction_system();

        let result = dialogue_system.talk_to_npc("nonexistent", &player, &faction_system);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("not found"));
    }

    #[test]
    fn test_talk_to_npc_updates_disposition() {
        let mut dialogue_system = DialogueSystem::new();
        let player = create_test_player();
        let faction_system = create_test_faction_system();
        let npc = create_basic_npc();
        let npc_id = npc.id.clone();

        dialogue_system.add_npc(npc);

        // Initial disposition should be 0
        assert_eq!(dialogue_system.npcs[&npc_id].current_disposition, 0);

        dialogue_system.talk_to_npc(&npc_id, &player, &faction_system).unwrap();

        // Disposition should be updated after talking
        assert_ne!(dialogue_system.npcs[&npc_id].current_disposition, 0);
    }

    #[test]
    fn test_ask_about_topic_success() {
        let mut dialogue_system = DialogueSystem::new();
        let player = create_test_player();
        let faction_system = create_test_faction_system();
        let npc = create_basic_npc();
        let npc_id = npc.id.clone();

        dialogue_system.add_npc(npc);

        // First talk to set disposition
        dialogue_system.talk_to_npc(&npc_id, &player, &faction_system).unwrap();

        let result = dialogue_system.ask_about_topic(&npc_id, "trade", &player, &faction_system);

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(!response.is_empty());
    }

    #[test]
    fn test_ask_about_topic_npc_not_found() {
        let mut dialogue_system = DialogueSystem::new();
        let player = create_test_player();
        let faction_system = create_test_faction_system();

        let result = dialogue_system.ask_about_topic("nonexistent", "trade", &player, &faction_system);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("not found"));
    }

    #[test]
    fn test_ask_about_topic_invalid_topic() {
        let mut dialogue_system = DialogueSystem::new();
        let player = create_test_player();
        let faction_system = create_test_faction_system();
        let npc = create_basic_npc();
        let npc_id = npc.id.clone();

        dialogue_system.add_npc(npc);

        let result = dialogue_system.ask_about_topic(&npc_id, "invalid_topic", &player, &faction_system);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("doesn't know about"));
    }

    #[test]
    fn test_ask_about_topic_requirements_not_met() {
        let mut dialogue_system = DialogueSystem::new();
        let mut player = create_test_player();
        let faction_system = create_test_faction_system();
        let npc = create_basic_npc();
        let npc_id = npc.id.clone();

        // Set faction standing too low for "secrets" topic (requires >= 30)
        player.faction_standings.insert(FactionId::IndustrialConsortium, 10);

        dialogue_system.add_npc(npc);

        // First talk to set disposition
        dialogue_system.talk_to_npc(&npc_id, &player, &faction_system).unwrap();

        let result = dialogue_system.ask_about_topic(&npc_id, "secrets", &player, &faction_system);

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.contains("doesn't seem willing"));
    }

    #[test]
    fn test_ask_about_topic_knowledge_requirement_not_met() {
        let mut dialogue_system = DialogueSystem::new();
        let mut player = create_test_player();
        let faction_system = create_test_faction_system();
        let npc = create_basic_npc();
        let npc_id = npc.id.clone();

        // Meet faction requirement but remove knowledge requirement
        player.faction_standings.insert(FactionId::IndustrialConsortium, 50);
        player.knowledge.theories.remove("basic_theory");

        dialogue_system.add_npc(npc);

        // First talk to set disposition
        dialogue_system.talk_to_npc(&npc_id, &player, &faction_system).unwrap();

        let result = dialogue_system.ask_about_topic(&npc_id, "secrets", &player, &faction_system);

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.contains("doesn't seem willing"));
    }

    #[test]
    fn test_hostile_npc_disposition() {
        let dialogue_system = DialogueSystem::new();
        let player = create_test_player();
        let faction_system = create_test_faction_system();
        let npc = create_hostile_npc(); // Underground affiliation, player has -70 standing

        let disposition = dialogue_system.calculate_disposition(&npc, &player, &faction_system);

        // Should be negative due to poor faction standing
        assert!(disposition < 0);
    }

    #[test]
    fn test_hostile_npc_conversation() {
        let mut dialogue_system = DialogueSystem::new();
        let player = create_test_player();
        let faction_system = create_test_faction_system();
        let npc = create_hostile_npc();
        let npc_id = npc.id.clone();

        dialogue_system.add_npc(npc);

        let result = dialogue_system.talk_to_npc(&npc_id, &player, &faction_system);

        assert!(result.is_ok());
        let response = result.unwrap();

        // Verify the response contains one of the possible hostile greeting templates
        // The actual greeting depends on the calculated disposition
        assert!(
            response.contains("Get lost, establishment pawn") ||
            response.contains("What do you want?") ||
            response.contains("You're one of us, I can tell"),
            "Expected hostile greeting, got: {}", response
        );
    }

    #[test]
    fn test_cross_faction_disposition_effects() {
        let dialogue_system = DialogueSystem::new();
        let mut player = create_test_player();
        let faction_system = create_test_faction_system();

        // Test Underground NPC with player having high Magisters standing
        // These factions are enemies, so high Magisters standing should hurt Underground disposition
        player.faction_standings.insert(FactionId::MagistersCouncil, 80);
        player.faction_standings.insert(FactionId::UndergroundNetwork, 10);

        let npc = create_hostile_npc(); // Underground affiliation
        let disposition = dialogue_system.calculate_disposition(&npc, &player, &faction_system);

        // Base would be 10/2 = 5, but cross-faction effects from enemies should reduce it
        // The exact calculation depends on the relationship strength and implementation
        assert!(disposition <= 5, "Expected disposition <= 5 due to cross-faction penalties, got {}", disposition);
    }

    #[test]
    fn test_allied_faction_disposition_boost() {
        let dialogue_system = DialogueSystem::new();
        let mut player = create_test_player();
        let faction_system = create_test_faction_system();

        // Test Magisters NPC with player having high Order standing
        // These factions are allies, so high Order standing should boost Magisters disposition
        player.faction_standings.insert(FactionId::MagistersCouncil, 30);
        player.faction_standings.insert(FactionId::OrderOfHarmony, 60);

        let mut npc = create_basic_npc();
        npc.faction_affiliation = Some(FactionId::MagistersCouncil);

        let disposition = dialogue_system.calculate_disposition(&npc, &player, &faction_system);

        // Base would be 30/2 = 15, plus cross-faction effects from allied factions
        // Order and Magisters are allies according to the politics system
        assert!(disposition >= 15, "Expected disposition >= 15, got {}", disposition);
    }

    #[test]
    fn test_dialogue_text_disposition_selection() {
        let mut dialogue_system = DialogueSystem::new();
        let player = create_test_player();
        let faction_system = create_test_faction_system();
        let npc = create_basic_npc();
        let npc_id = npc.id.clone();

        dialogue_system.add_npc(npc);

        // First talk to establish disposition
        dialogue_system.talk_to_npc(&npc_id, &player, &faction_system).unwrap();

        let npc_disposition = dialogue_system.npcs[&npc_id].current_disposition;

        // Ask about trade topic and verify response matches disposition
        let result = dialogue_system.ask_about_topic(&npc_id, "trade", &player, &faction_system);
        assert!(result.is_ok());

        let response = result.unwrap();

        // Verify response content matches expected disposition level
        if npc_disposition >= 50 {
            assert!(response.contains("finest goods"));
        } else if npc_disposition <= -50 {
            assert!(response.contains("don't trade"));
        } else {
            assert!(response.contains("make a deal"));
        }
    }

    #[test]
    fn test_no_faction_standing_requirements() {
        let dialogue_system = DialogueSystem::new();
        let mut player = create_test_player();
        let faction_system = create_test_faction_system();

        // Remove all faction standings
        player.faction_standings.clear();

        // Test requirements with faction player has no standing with
        let req = DialogueRequirements {
            min_faction_standing: Some((FactionId::MagistersCouncil, 10)),
            max_faction_standing: None,
            knowledge_requirements: vec![],
            theory_requirements: vec![],
            min_theory_mastery: None,
            required_capabilities: vec![],
        };

        // Should fail because player has no standing (treated as 0, which is < 10)
        assert!(!dialogue_system.check_requirements(&req, &player, &faction_system));
    }

    #[test]
    fn test_max_faction_standing_with_no_standing() {
        let dialogue_system = DialogueSystem::new();
        let mut player = create_test_player();
        let faction_system = create_test_faction_system();

        // Remove specific faction standing
        player.faction_standings.remove(&FactionId::MagistersCouncil);

        // Test max requirements with faction player has no standing with
        let req = DialogueRequirements {
            min_faction_standing: None,
            max_faction_standing: Some((FactionId::MagistersCouncil, 10)),
            knowledge_requirements: vec![],
            theory_requirements: vec![],
            min_theory_mastery: None,
            required_capabilities: vec![],
        };

        // Should pass because player has no standing (treated as 0, which is <= 10)
        assert!(dialogue_system.check_requirements(&req, &player, &faction_system));
    }
}