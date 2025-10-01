//! Quest system for educational progression and faction integration
//!
//! This module provides:
//! - Quest definition and tracking
//! - Educational objectives tied to theory mastery
//! - Faction-based quest availability and outcomes
//! - Multi-path quest progression based on player choices
//! - Scientific learning integration with practical applications

use crate::core::Player;
use crate::systems::factions::{FactionId, FactionSystem};
use crate::GameResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Unique identifier for quests
pub type QuestId = String;

/// Main quest system managing all active and completed quests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestSystem {
    /// All available quest definitions
    pub quest_definitions: HashMap<QuestId, QuestDefinition>,
    /// Player's quest progress
    pub player_progress: HashMap<QuestId, QuestProgress>,
    /// Global quest state and unlocks
    pub global_state: QuestGlobalState,
}

/// Complete quest definition with all metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestDefinition {
    pub id: QuestId,
    pub title: String,
    pub description: String,
    pub category: QuestCategory,
    pub difficulty: QuestDifficulty,

    /// Quest requirements to start
    pub requirements: QuestRequirements,
    /// Objectives that must be completed
    pub objectives: Vec<QuestObjective>,
    /// Possible rewards upon completion
    pub rewards: QuestRewards,

    /// Faction implications and standing changes
    pub faction_effects: HashMap<FactionId, i32>,
    /// Educational focus and learning outcomes
    pub educational_focus: EducationalObjectives,
    /// Quest can branch into multiple paths
    pub branching_paths: HashMap<String, QuestBranch>,
    /// Player choices within the quest
    #[serde(default)]
    pub choices: Vec<QuestChoice>,

    /// NPCs involved in this quest
    pub involved_npcs: Vec<String>,
    /// Locations where quest activities occur
    pub locations: Vec<String>,
    /// Estimated completion time in minutes
    pub estimated_duration: i32,
}

/// Categories of quests for organization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QuestCategory {
    /// Tutorial quests for new players
    Tutorial,
    /// Research-focused quests
    Research,
    /// Faction political quests
    Political,
    /// Practical application of theories
    Practical,
    /// Social interaction and diplomacy
    Social,
    /// Dangerous experimental quests
    Experimental,
    /// Story-driven narrative quests
    Narrative,
}

/// Quest difficulty levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QuestDifficulty {
    Beginner,    // 1-2 theories required
    Intermediate, // 3-4 theories required
    Advanced,    // 5-6 theories required
    Expert,      // 7+ theories required
    Master,      // Requires theoretical synthesis
}

/// Requirements to start a quest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestRequirements {
    /// Minimum theory understanding levels (theory_id, min_level)
    pub theory_requirements: Vec<(String, f32)>,
    /// Required faction standings (faction, min_standing)
    pub faction_requirements: Vec<(FactionId, i32)>,
    /// Forbidden faction standings (faction, max_standing)
    pub faction_restrictions: Vec<(FactionId, i32)>,
    /// Prerequisites quests that must be completed
    pub prerequisite_quests: Vec<QuestId>,
    /// Minimum player attributes required
    pub attribute_requirements: AttributeRequirements,
    /// Required magical capabilities
    pub capability_requirements: Vec<String>,
    /// Player must be at specific locations
    pub location_requirements: Vec<String>,
}

/// Player attribute requirements for quests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeRequirements {
    pub min_mental_acuity: Option<i32>,
    pub min_resonance_sensitivity: Option<i32>,
    pub min_total_playtime: Option<i32>,
}

/// Individual quest objective
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestObjective {
    pub id: String,
    pub description: String,
    pub objective_type: ObjectiveType,
    /// Whether this objective is optional
    pub optional: bool,
    /// Whether this objective is currently visible to player
    pub visible: bool,
    /// Experience and rewards for completing this objective
    pub completion_reward: ObjectiveReward,
}

/// Types of objectives players can complete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectiveType {
    /// Talk to specific NPC about topic
    TalkToNPC { npc_id: String, topic: Option<String> },
    /// Learn theory to specific level
    LearnTheory { theory_id: String, min_level: f32 },
    /// Master specific number of theories
    MasterTheories { count: i32, tier: Option<i32> },
    /// Visit specific location
    VisitLocation { location_id: String },
    /// Achieve faction standing with group
    FactionStanding { faction_id: FactionId, target_standing: i32 },
    /// Perform magical experiment or demonstration
    MagicalDemonstration { theory_id: String, success_threshold: f32 },
    /// Teach another NPC about a theory
    TeachTheory { npc_id: String, theory_id: String },
    /// Research new theoretical insights
    Research { theory_id: String, research_points: i32 },
    /// Make diplomatic choice between factions
    DiplomaticChoice { choice_id: String, factions: Vec<FactionId> },
    /// Collect specific items or resources
    CollectItems { item_ids: Vec<String>, quantities: Vec<i32> },
    /// Complete learning activity with specific method
    LearningActivity { theory_id: String, method: String, duration: i32 },
}

/// Rewards for completing individual objectives
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectiveReward {
    /// Experience points awarded
    pub experience: i32,
    /// Theory understanding bonuses
    pub theory_insights: HashMap<String, f32>,
    /// Faction standing changes
    pub faction_changes: HashMap<FactionId, i32>,
    /// Items received
    pub items: Vec<String>,
}

/// Complete quest rewards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestRewards {
    /// Base experience points
    pub experience: i32,
    /// Attribute improvements
    pub attribute_bonuses: AttributeBonuses,
    /// Theory mastery bonuses
    pub theory_bonuses: HashMap<String, f32>,
    /// Faction standing changes
    pub faction_changes: HashMap<FactionId, i32>,
    /// Items and equipment
    pub items: Vec<String>,
    /// Unlocked capabilities
    pub new_capabilities: Vec<String>,
    /// Unlocked quest lines
    pub unlocked_quests: Vec<QuestId>,
}

/// Attribute bonuses from quest completion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeBonuses {
    pub mental_acuity: Option<i32>,
    pub resonance_sensitivity: Option<i32>,
}

/// Educational objectives and learning outcomes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EducationalObjectives {
    /// Primary scientific concepts taught
    pub primary_concepts: Vec<String>,
    /// Secondary concepts reinforced
    pub secondary_concepts: Vec<String>,
    /// Real-world applications demonstrated
    pub applications: Vec<String>,
    /// Problem-solving approaches taught
    pub problem_solving_methods: Vec<String>,
    /// Learning assessment methods
    pub assessment_criteria: Vec<String>,
}

/// Quest branching paths for player choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestBranch {
    pub id: String,
    pub name: String,
    pub description: String,
    /// Requirements to access this branch
    pub requirements: QuestRequirements,
    /// Objectives specific to this branch
    pub branch_objectives: Vec<QuestObjective>,
    /// Faction implications of choosing this path
    pub faction_implications: HashMap<FactionId, i32>,
    /// Educational focus of this branch
    pub educational_focus: EducationalObjectives,
}

/// Player choice within a quest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestChoice {
    pub id: String,
    pub prompt: String,
    pub description: String,
    /// Available options for this choice
    pub options: Vec<ChoiceOption>,
    /// When this choice becomes available (objective_id must be complete)
    pub prerequisite_objective: Option<String>,
    /// Whether this choice is required to progress
    pub required: bool,
}

/// Individual option within a quest choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceOption {
    pub id: String,
    pub text: String,
    pub description: String,
    /// Requirements to select this option
    pub requirements: Option<ChoiceRequirements>,
    /// Outcome if this option is chosen
    pub outcome: QuestOutcome,
}

/// Requirements to select a specific choice option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceRequirements {
    /// Minimum theory understanding (theory_id, min_level)
    pub theory_requirements: Vec<(String, f32)>,
    /// Faction standing requirements
    pub faction_requirements: Vec<(FactionId, i32)>,
    /// Required items in inventory
    pub item_requirements: Vec<String>,
}

/// Outcome of a quest choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestOutcome {
    pub outcome_type: OutcomeType,
    /// Experience modifier (multiplier: 1.0 = normal, 1.5 = 50% bonus)
    pub experience_modifier: f32,
    /// Faction standing changes from this choice
    pub faction_changes: HashMap<FactionId, i32>,
    /// Theory insights gained/lost
    pub theory_insights: HashMap<String, f32>,
    /// Items gained/lost
    pub item_changes: Vec<String>,
    /// Narrative text describing the outcome
    pub narrative_result: String,
    /// Follow-up dialogue from NPCs
    pub npc_reactions: HashMap<String, String>,
    /// Unlocks or blocks future quest content
    pub content_unlocks: Vec<String>,
}

/// Type of quest outcome
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OutcomeType {
    /// Full success - optimal outcome
    Success,
    /// Partial success - goals achieved with complications
    PartialSuccess,
    /// Alternative success - different but valid approach
    AlternativeSuccess,
    /// Failure - objectives not met
    Failure,
    /// Mixed outcome - some wins, some losses
    Mixed,
}

/// Player's progress on a specific quest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestProgress {
    pub quest_id: QuestId,
    pub status: QuestStatus,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,

    /// Objective completion status
    pub objective_progress: HashMap<String, ObjectiveProgress>,
    /// Chosen branch path if applicable
    pub chosen_branch: Option<String>,
    /// Player choices made during quest
    pub player_choices: HashMap<String, String>,
    /// Time spent on quest in minutes
    pub time_invested: i32,

    /// Quest-specific variables and state
    pub quest_variables: HashMap<String, String>,
    /// Educational progress tracking
    pub learning_progress: QuestLearningProgress,
}

/// Quest completion status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QuestStatus {
    Available,    // Can be started
    NotAvailable, // Requirements not met
    InProgress,   // Currently active
    Completed,    // Successfully finished
    Failed,       // Failed due to choices or time
    Abandoned,    // Player abandoned quest
}

/// Progress on individual objective
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectiveProgress {
    pub completed: bool,
    pub progress_value: f32, // 0.0 to 1.0
    pub completed_at: Option<DateTime<Utc>>,
    /// Detailed progress data
    pub progress_data: HashMap<String, String>,
}

/// Educational progress tracking for quest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestLearningProgress {
    /// Concepts the player has demonstrated understanding of
    pub mastered_concepts: Vec<String>,
    /// Problem-solving approaches successfully used
    pub demonstrated_methods: Vec<String>,
    /// Assessment scores (concept -> score 0.0-1.0)
    pub assessment_scores: HashMap<String, f32>,
    /// Learning efficiency metrics
    pub learning_metrics: LearningMetrics,
}

/// Learning efficiency metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningMetrics {
    /// Time to complete vs estimated
    pub completion_efficiency: f32,
    /// Success rate on first attempts
    pub first_attempt_success_rate: f32,
    /// Help seeking behavior
    pub help_requests: i32,
    /// Theory application accuracy
    pub application_accuracy: f32,
}

/// Global quest system state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestGlobalState {
    /// Unlocked quest lines
    pub unlocked_quest_lines: Vec<String>,
    /// Global events affecting all quests
    pub global_events: HashMap<String, String>,
    /// Faction relationship changes from quest outcomes
    pub faction_relationship_modifiers: HashMap<(FactionId, FactionId), f32>,
}

impl QuestSystem {
    /// Create a new quest system
    pub fn new() -> Self {
        Self {
            quest_definitions: HashMap::new(),
            player_progress: HashMap::new(),
            global_state: QuestGlobalState {
                unlocked_quest_lines: vec!["tutorial".to_string()],
                global_events: HashMap::new(),
                faction_relationship_modifiers: HashMap::new(),
            },
        }
    }

    /// Add quest definition to the system
    pub fn add_quest_definition(&mut self, quest: QuestDefinition) {
        self.quest_definitions.insert(quest.id.clone(), quest);
    }

    /// Get available quests for player
    pub fn get_available_quests(&self, player: &Player, faction_system: &FactionSystem) -> Vec<&QuestDefinition> {
        self.quest_definitions
            .values()
            .filter(|quest| self.is_quest_available(quest, player, faction_system))
            .collect()
    }

    /// Check if player can start a specific quest
    pub fn is_quest_available(&self, quest: &QuestDefinition, player: &Player, faction_system: &FactionSystem) -> bool {
        // Check if already completed or in progress
        if let Some(progress) = self.player_progress.get(&quest.id) {
            return progress.status == QuestStatus::Available;
        }

        // Check all requirements
        self.check_quest_requirements(&quest.requirements, player, faction_system)
    }

    /// Check if player meets quest requirements
    fn check_quest_requirements(&self, requirements: &QuestRequirements, player: &Player, _faction_system: &FactionSystem) -> bool {
        // Check theory requirements
        for (theory_id, min_level) in &requirements.theory_requirements {
            if player.theory_understanding(theory_id) < *min_level {
                return false;
            }
        }

        // Check faction requirements
        for (faction_id, min_standing) in &requirements.faction_requirements {
            if let Some(&standing) = player.faction_standings.get(faction_id) {
                if standing < *min_standing {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Check faction restrictions
        for (faction_id, max_standing) in &requirements.faction_restrictions {
            if let Some(&standing) = player.faction_standings.get(faction_id) {
                if standing > *max_standing {
                    return false;
                }
            }
        }

        // Check prerequisite quests
        for prereq_quest in &requirements.prerequisite_quests {
            if let Some(progress) = self.player_progress.get(prereq_quest) {
                if progress.status != QuestStatus::Completed {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Check attribute requirements
        if let Some(min_acuity) = requirements.attribute_requirements.min_mental_acuity {
            if player.attributes.mental_acuity < min_acuity {
                return false;
            }
        }

        if let Some(min_sensitivity) = requirements.attribute_requirements.min_resonance_sensitivity {
            if player.attributes.resonance_sensitivity < min_sensitivity {
                return false;
            }
        }

        if let Some(min_playtime) = requirements.attribute_requirements.min_total_playtime {
            if player.playtime_minutes < min_playtime {
                return false;
            }
        }

        // Check capability requirements
        for capability in &requirements.capability_requirements {
            if !player.has_magic_capability(capability) {
                return false;
            }
        }

        // Check location requirements
        if !requirements.location_requirements.is_empty() {
            if !requirements.location_requirements.contains(&player.current_location) {
                return false;
            }
        }

        true
    }

    /// Start a quest for the player
    pub fn start_quest(&mut self, quest_id: &str, player: &Player, faction_system: &FactionSystem) -> GameResult<String> {
        let quest = self.quest_definitions.get(quest_id)
            .ok_or_else(|| crate::GameError::ContentNotFound(format!("Quest '{}' not found", quest_id)))?;

        if !self.is_quest_available(quest, player, faction_system) {
            return Err(crate::GameError::InvalidCommand("Quest requirements not met".to_string()).into());
        }

        // Create quest progress
        let mut objective_progress = HashMap::new();
        for objective in &quest.objectives {
            objective_progress.insert(objective.id.clone(), ObjectiveProgress {
                completed: false,
                progress_value: 0.0,
                completed_at: None,
                progress_data: HashMap::new(),
            });
        }

        let progress = QuestProgress {
            quest_id: quest_id.to_string(),
            status: QuestStatus::InProgress,
            started_at: Utc::now(),
            completed_at: None,
            objective_progress,
            chosen_branch: None,
            player_choices: HashMap::new(),
            time_invested: 0,
            quest_variables: HashMap::new(),
            learning_progress: QuestLearningProgress {
                mastered_concepts: Vec::new(),
                demonstrated_methods: Vec::new(),
                assessment_scores: HashMap::new(),
                learning_metrics: LearningMetrics {
                    completion_efficiency: 0.0,
                    first_attempt_success_rate: 0.0,
                    help_requests: 0,
                    application_accuracy: 0.0,
                },
            },
        };

        self.player_progress.insert(quest_id.to_string(), progress);

        Ok(format!("Started quest: {}\n{}", quest.title, quest.description))
    }

    /// Update objective progress
    pub fn update_objective_progress(
        &mut self,
        quest_id: &str,
        objective_id: &str,
        progress_value: f32,
        completed: bool,
    ) -> GameResult<()> {
        let quest_progress = self.player_progress.get_mut(quest_id)
            .ok_or_else(|| crate::GameError::ContentNotFound(format!("Quest progress for '{}' not found", quest_id)))?;

        if let Some(obj_progress) = quest_progress.objective_progress.get_mut(objective_id) {
            obj_progress.progress_value = progress_value.clamp(0.0, 1.0);
            if completed && !obj_progress.completed {
                obj_progress.completed = true;
                obj_progress.completed_at = Some(Utc::now());
            }
        }

        // Check if quest is complete
        self.check_quest_completion(quest_id)?;

        Ok(())
    }

    /// Check if all quest objectives are complete
    fn check_quest_completion(&mut self, quest_id: &str) -> GameResult<bool> {
        let quest_def = self.quest_definitions.get(quest_id)
            .ok_or_else(|| crate::GameError::ContentNotFound(format!("Quest definition '{}' not found", quest_id)))?;

        let quest_progress = self.player_progress.get_mut(quest_id)
            .ok_or_else(|| crate::GameError::ContentNotFound(format!("Quest progress '{}' not found", quest_id)))?;

        // Check required objectives
        let required_objectives: Vec<_> = quest_def.objectives
            .iter()
            .filter(|obj| !obj.optional)
            .collect();

        let completed_required = required_objectives
            .iter()
            .all(|obj| {
                quest_progress.objective_progress
                    .get(&obj.id)
                    .map(|progress| progress.completed)
                    .unwrap_or(false)
            });

        if completed_required {
            quest_progress.status = QuestStatus::Completed;
            quest_progress.completed_at = Some(Utc::now());
            return Ok(true);
        }

        Ok(false)
    }

    /// Get player's active quests
    pub fn get_active_quests(&self) -> Vec<&QuestProgress> {
        self.player_progress
            .values()
            .filter(|progress| progress.status == QuestStatus::InProgress)
            .collect()
    }

    /// Get detailed quest status for player
    pub fn get_quest_status(&self, quest_id: &str) -> GameResult<String> {
        let quest_def = self.quest_definitions.get(quest_id)
            .ok_or_else(|| crate::GameError::ContentNotFound(format!("Quest '{}' not found", quest_id)))?;

        let progress = self.player_progress.get(quest_id);

        if let Some(progress) = progress {
            let mut status = format!("=== {} ===\n", quest_def.title);
            status.push_str(&format!("Status: {:?}\n", progress.status));
            status.push_str(&format!("Description: {}\n\n", quest_def.description));

            status.push_str("Objectives:\n");
            for objective in &quest_def.objectives {
                let obj_progress = progress.objective_progress.get(&objective.id);
                let completed = obj_progress.map(|p| p.completed).unwrap_or(false);
                let progress_val = obj_progress.map(|p| p.progress_value).unwrap_or(0.0);

                let status_icon = if completed { "✓" } else { "○" };
                let optional_tag = if objective.optional { " (Optional)" } else { "" };

                status.push_str(&format!(
                    "  {} {} - {:.0}%{}\n",
                    status_icon,
                    objective.description,
                    progress_val * 100.0,
                    optional_tag
                ));
            }

            if progress.status == QuestStatus::InProgress {
                status.push_str(&format!("\nTime invested: {} minutes\n", progress.time_invested));
            }

            Ok(status)
        } else {
            Ok(format!("=== {} ===\n{}\n\nStatus: Not Started", quest_def.title, quest_def.description))
        }
    }

    /// Abandon an active quest
    pub fn abandon_quest(&mut self, quest_id: &str, faction_system: &mut FactionSystem) -> GameResult<String> {
        // Check if quest exists
        let quest_def = self.quest_definitions.get(quest_id)
            .ok_or_else(|| crate::GameError::ContentNotFound(format!("Quest '{}' not found", quest_id)))?;

        // Check if player has this quest active
        let progress = self.player_progress.get_mut(quest_id)
            .ok_or_else(|| crate::GameError::InvalidCommand(format!("You haven't started the quest '{}'", quest_def.title)))?;

        // Can only abandon quests that are in progress
        if progress.status != QuestStatus::InProgress {
            return Err(crate::GameError::InvalidCommand(
                format!("Cannot abandon quest '{}' with status {:?}", quest_def.title, progress.status)
            ).into());
        }

        // Tutorial quests cannot be abandoned
        if quest_def.category == QuestCategory::Tutorial {
            return Err(crate::GameError::InvalidCommand(
                "Tutorial quests cannot be abandoned. Please complete them to learn the game.".to_string()
            ).into());
        }

        // Mark as abandoned
        progress.status = QuestStatus::Abandoned;
        progress.completed_at = Some(Utc::now());

        let mut result = format!("You have abandoned the quest: {}\n\n", quest_def.title);

        // Apply faction reputation penalties if appropriate
        let mut total_penalty = 0;
        for (faction_id, effect) in &quest_def.faction_effects {
            // Only apply penalty if the effect was positive (they were expecting you to help)
            if *effect > 0 {
                let penalty = -(*effect / 2); // Penalty is half the reward
                faction_system.modify_reputation(*faction_id, penalty);
                total_penalty += penalty.abs();

                result.push_str(&format!("• {:?} faction reputation: {} (abandonment penalty)\n",
                    faction_id, penalty));
            }
        }

        if total_penalty == 0 {
            result.push_str("No faction reputation penalties.\n");
        }

        result.push_str("\nThe quest may become available again in the future.");

        Ok(result)
    }

    /// Handle quest-related dialogue trigger
    pub fn handle_dialogue_trigger(
        &mut self,
        npc_id: &str,
        topic: Option<&str>,
        _player: &Player,
    ) -> GameResult<Vec<String>> {
        let mut quest_updates = Vec::new();

        // Check all active quests for dialogue objectives
        let active_quest_ids: Vec<String> = self.get_active_quests()
            .iter()
            .map(|progress| progress.quest_id.clone())
            .collect();

        // Collect updates to apply later
        let mut updates_to_apply = Vec::new();

        for quest_id in active_quest_ids {
            if let Some(quest_def) = self.quest_definitions.get(&quest_id) {
                for objective in &quest_def.objectives {
                    if let ObjectiveType::TalkToNPC { npc_id: required_npc, topic: required_topic } = &objective.objective_type {
                        if required_npc == npc_id {
                            let topic_matches = match (&required_topic, topic) {
                                (Some(req_topic), Some(actual_topic)) => req_topic == actual_topic,
                                (None, _) => true, // Any topic accepted
                                _ => false,
                            };

                            if topic_matches {
                                updates_to_apply.push((quest_id.clone(), objective.id.clone(), objective.description.clone()));
                            }
                        }
                    }
                }
            }
        }

        // Apply collected updates
        for (quest_id, objective_id, description) in updates_to_apply {
            self.update_objective_progress(&quest_id, &objective_id, 1.0, true)?;
            quest_updates.push(format!("Quest objective completed: {}", description));
        }

        Ok(quest_updates)
    }

    /// Handle theory learning progress for quest objectives
    pub fn handle_theory_progress(
        &mut self,
        theory_id: &str,
        new_understanding_level: f32,
        player: &Player,
    ) -> GameResult<Vec<String>> {
        let mut quest_updates = Vec::new();

        let active_quest_ids: Vec<String> = self.get_active_quests()
            .iter()
            .map(|progress| progress.quest_id.clone())
            .collect();

        // Collect all updates that need to be made first
        let mut updates_to_apply = Vec::new();

        for quest_id in &active_quest_ids {
            if let Some(quest_def) = self.quest_definitions.get(quest_id) {
                for objective in &quest_def.objectives {
                    match &objective.objective_type {
                        ObjectiveType::LearnTheory { theory_id: req_theory, min_level } => {
                            if req_theory == theory_id && new_understanding_level >= *min_level {
                                updates_to_apply.push((quest_id.clone(), objective.id.clone(), objective.description.clone()));
                            }
                        },
                        ObjectiveType::MasterTheories { count, tier } => {
                            let mastered_theories = player.get_mastered_theories();
                            let relevant_theories = if let Some(_tier_req) = tier {
                                // Filter by tier if specified (would need theory tier info)
                                mastered_theories.len()
                            } else {
                                mastered_theories.len()
                            };

                            if relevant_theories >= *count as usize {
                                updates_to_apply.push((quest_id.clone(), objective.id.clone(), objective.description.clone()));
                            }
                        },
                        _ => {},
                    }
                }
            }
        }

        // Now apply all the updates
        for (quest_id, objective_id, description) in updates_to_apply {
            self.update_objective_progress(&quest_id, &objective_id, 1.0, true)?;
            quest_updates.push(format!("Quest objective completed: {}", description));
        }

        Ok(quest_updates)
    }

    /// Handle location visit for quest objectives
    pub fn handle_location_visit(&mut self, location_id: &str) -> GameResult<Vec<String>> {
        let mut quest_updates = Vec::new();

        let active_quest_ids: Vec<String> = self.get_active_quests()
            .iter()
            .map(|progress| progress.quest_id.clone())
            .collect();

        // Collect all updates that need to be made first
        let mut updates_to_apply = Vec::new();

        for quest_id in &active_quest_ids {
            if let Some(quest_def) = self.quest_definitions.get(quest_id) {
                for objective in &quest_def.objectives {
                    if let ObjectiveType::VisitLocation { location_id: req_location } = &objective.objective_type {
                        if req_location == location_id {
                            updates_to_apply.push((quest_id.clone(), objective.id.clone(), objective.description.clone()));
                        }
                    }
                }
            }
        }

        // Now apply all the updates
        for (quest_id, objective_id, description) in updates_to_apply {
            self.update_objective_progress(&quest_id, &objective_id, 1.0, true)?;
            quest_updates.push(format!("Quest objective completed: {}", description));
        }

        Ok(quest_updates)
    }

    /// Apply quest rewards to player
    pub fn apply_quest_rewards(
        &self,
        quest_id: &str,
        player: &mut Player,
        faction_system: &mut FactionSystem,
    ) -> GameResult<String> {
        let quest_def = self.quest_definitions.get(quest_id)
            .ok_or_else(|| crate::GameError::ContentNotFound(format!("Quest '{}' not found", quest_id)))?;

        let progress = self.player_progress.get(quest_id)
            .ok_or_else(|| crate::GameError::ContentNotFound(format!("Quest progress for '{}' not found", quest_id)))?;

        if progress.status != QuestStatus::Completed {
            return Err(crate::GameError::InvalidCommand("Quest not completed".to_string()).into());
        }

        let mut reward_summary = format!("Quest Completed: {}\n\nRewards:\n", quest_def.title);

        // Apply experience
        if quest_def.rewards.experience > 0 {
            reward_summary.push_str(&format!("• {} experience points\n", quest_def.rewards.experience));
        }

        // Apply attribute bonuses
        if let Some(acuity_bonus) = quest_def.rewards.attribute_bonuses.mental_acuity {
            player.attributes.mental_acuity += acuity_bonus;
            reward_summary.push_str(&format!("• +{} Mental Acuity\n", acuity_bonus));
        }

        if let Some(sensitivity_bonus) = quest_def.rewards.attribute_bonuses.resonance_sensitivity {
            player.attributes.resonance_sensitivity += sensitivity_bonus;
            reward_summary.push_str(&format!("• +{} Resonance Sensitivity\n", sensitivity_bonus));
        }

        // Apply theory bonuses
        for (theory_id, bonus) in &quest_def.rewards.theory_bonuses {
            if let Some(current_level) = player.knowledge.theories.get_mut(theory_id) {
                *current_level = (*current_level + bonus).min(1.0);
                reward_summary.push_str(&format!("• +{:.1}% understanding in {}\n", bonus * 100.0, theory_id));
            }
        }

        // Apply faction changes
        for (faction_id, change) in &quest_def.rewards.faction_changes {
            faction_system.modify_reputation(*faction_id, *change);
            reward_summary.push_str(&format!("• {} faction standing with {}\n",
                if *change > 0 { format!("+{}", change) } else { change.to_string() },
                faction_id.display_name()
            ));
        }

        // Add new capabilities
        for capability in &quest_def.rewards.new_capabilities {
            reward_summary.push_str(&format!("• New capability unlocked: {}\n", capability));
        }

        // Items would be added to inventory (not implemented in this snippet)
        if !quest_def.rewards.items.is_empty() {
            reward_summary.push_str(&format!("• Items received: {}\n", quest_def.rewards.items.join(", ")));
        }

        Ok(reward_summary)
    }

    /// Get quest recommendations based on player progress
    pub fn get_quest_recommendations(&self, player: &Player, faction_system: &FactionSystem) -> Vec<(QuestId, String)> {
        let mut recommendations = Vec::new();

        for quest in self.get_available_quests(player, faction_system) {
            let mut score = 0;
            let mut reason = String::new();

            // Score based on difficulty match
            match quest.difficulty {
                QuestDifficulty::Beginner if player.get_mastered_theories().len() <= 2 => {
                    score += 10;
                    reason = "Good for your current level".to_string();
                },
                QuestDifficulty::Intermediate if player.get_mastered_theories().len() >= 3 => {
                    score += 8;
                    reason = "Matches your intermediate knowledge".to_string();
                },
                QuestDifficulty::Advanced if player.get_mastered_theories().len() >= 5 => {
                    score += 6;
                    reason = "Challenges your advanced skills".to_string();
                },
                _ => score += 2,
            }

            // Score based on educational alignment
            if !quest.educational_focus.primary_concepts.is_empty() {
                score += 5;
                if reason.is_empty() {
                    reason = "Teaches important concepts".to_string();
                }
            }

            // Score based on faction alignment
            for (faction_id, _) in &quest.faction_effects {
                if let Some(&standing) = player.faction_standings.get(faction_id) {
                    if standing > 20 {
                        score += 3;
                        if reason.is_empty() {
                            reason = format!("Aligns with your {} standing", faction_id.display_name());
                        }
                    }
                }
            }

            if score >= 6 {
                recommendations.push((quest.id.clone(), reason));
            }
        }

        // Sort by relevance score (implicit in the scoring above)
        recommendations.sort_by(|a, b| b.1.len().cmp(&a.1.len())); // Simple heuristic
        recommendations.truncate(5); // Limit to top 5

        recommendations
    }
}

impl Default for QuestSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Player;
    use crate::systems::factions::FactionSystem;

    fn create_test_player() -> Player {
        let mut player = Player::new("Test Player".to_string());
        player.knowledge.theories.insert("harmonic_fundamentals".to_string(), 0.8);
        player.knowledge.theories.insert("crystal_structures".to_string(), 0.6);
        player.faction_standings.insert(FactionId::MagistersCouncil, 30);
        player.faction_standings.insert(FactionId::NeutralScholars, 20);
        player
    }

    fn create_test_quest() -> QuestDefinition {
        QuestDefinition {
            id: "test_quest".to_string(),
            title: "Test Quest".to_string(),
            description: "A quest for testing".to_string(),
            category: QuestCategory::Tutorial,
            difficulty: QuestDifficulty::Beginner,
            requirements: QuestRequirements {
                theory_requirements: vec![("harmonic_fundamentals".to_string(), 0.5)],
                faction_requirements: vec![(FactionId::MagistersCouncil, 10)],
                faction_restrictions: vec![],
                prerequisite_quests: vec![],
                attribute_requirements: AttributeRequirements {
                    min_mental_acuity: Some(20),
                    min_resonance_sensitivity: None,
                    min_total_playtime: None,
                },
                capability_requirements: vec![],
                location_requirements: vec![],
            },
            objectives: vec![
                QuestObjective {
                    id: "obj1".to_string(),
                    description: "Talk to test NPC".to_string(),
                    objective_type: ObjectiveType::TalkToNPC {
                        npc_id: "test_npc".to_string(),
                        topic: Some("test_topic".to_string()),
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
            ],
            rewards: QuestRewards {
                experience: 100,
                attribute_bonuses: AttributeBonuses {
                    mental_acuity: Some(1),
                    resonance_sensitivity: None,
                },
                theory_bonuses: HashMap::new(),
                faction_changes: HashMap::new(),
                items: vec![],
                new_capabilities: vec![],
                unlocked_quests: vec![],
            },
            faction_effects: HashMap::new(),
            educational_focus: EducationalObjectives {
                primary_concepts: vec!["Basic Magic Theory".to_string()],
                secondary_concepts: vec![],
                applications: vec![],
                problem_solving_methods: vec![],
                assessment_criteria: vec![],
            },
            branching_paths: HashMap::new(),
            choices: vec![],
            involved_npcs: vec!["test_npc".to_string()],
            locations: vec!["test_location".to_string()],
            estimated_duration: 30,
        }
    }

    #[test]
    fn test_quest_system_creation() {
        let quest_system = QuestSystem::new();
        assert_eq!(quest_system.quest_definitions.len(), 0);
        assert_eq!(quest_system.player_progress.len(), 0);
    }

    #[test]
    fn test_add_quest_definition() {
        let mut quest_system = QuestSystem::new();
        let quest = create_test_quest();
        let quest_id = quest.id.clone();

        quest_system.add_quest_definition(quest);

        assert!(quest_system.quest_definitions.contains_key(&quest_id));
    }

    #[test]
    fn test_quest_availability() {
        let mut quest_system = QuestSystem::new();
        let quest = create_test_quest();
        let player = create_test_player();
        let faction_system = FactionSystem::new();

        quest_system.add_quest_definition(quest);

        assert!(quest_system.is_quest_available(&quest_system.quest_definitions["test_quest"], &player, &faction_system));
    }

    #[test]
    fn test_quest_requirements_not_met() {
        let mut quest_system = QuestSystem::new();
        let mut quest = create_test_quest();
        let player = create_test_player();
        let faction_system = FactionSystem::new();

        // Set requirement that player doesn't meet
        quest.requirements.theory_requirements = vec![("unknown_theory".to_string(), 0.5)];
        quest_system.add_quest_definition(quest);

        assert!(!quest_system.is_quest_available(&quest_system.quest_definitions["test_quest"], &player, &faction_system));
    }

    #[test]
    fn test_start_quest() {
        let mut quest_system = QuestSystem::new();
        let quest = create_test_quest();
        let player = create_test_player();
        let faction_system = FactionSystem::new();

        quest_system.add_quest_definition(quest);

        let result = quest_system.start_quest("test_quest", &player, &faction_system);
        assert!(result.is_ok());
        assert!(quest_system.player_progress.contains_key("test_quest"));

        let progress = &quest_system.player_progress["test_quest"];
        assert_eq!(progress.status, QuestStatus::InProgress);
    }

    #[test]
    fn test_objective_progress() {
        let mut quest_system = QuestSystem::new();
        let quest = create_test_quest();
        let player = create_test_player();
        let faction_system = FactionSystem::new();

        quest_system.add_quest_definition(quest);
        quest_system.start_quest("test_quest", &player, &faction_system).unwrap();

        let result = quest_system.update_objective_progress("test_quest", "obj1", 1.0, true);
        assert!(result.is_ok());

        let progress = &quest_system.player_progress["test_quest"];
        assert!(progress.objective_progress["obj1"].completed);
        assert_eq!(progress.status, QuestStatus::Completed);
    }

    #[test]
    fn test_dialogue_trigger() {
        let mut quest_system = QuestSystem::new();
        let quest = create_test_quest();
        let player = create_test_player();
        let faction_system = FactionSystem::new();

        quest_system.add_quest_definition(quest);
        quest_system.start_quest("test_quest", &player, &faction_system).unwrap();

        let result = quest_system.handle_dialogue_trigger("test_npc", Some("test_topic"), &player);
        assert!(result.is_ok());

        let updates = result.unwrap();
        assert!(!updates.is_empty());
        assert!(updates[0].contains("completed"));
    }

    #[test]
    fn test_get_quest_status() {
        let mut quest_system = QuestSystem::new();
        let quest = create_test_quest();

        quest_system.add_quest_definition(quest);

        let status = quest_system.get_quest_status("test_quest").unwrap();
        assert!(status.contains("Test Quest"));
        assert!(status.contains("Not Started"));
    }

    #[test]
    fn test_quest_recommendations() {
        let mut quest_system = QuestSystem::new();
        let quest = create_test_quest();
        let player = create_test_player();
        let faction_system = FactionSystem::new();

        quest_system.add_quest_definition(quest);

        let recommendations = quest_system.get_quest_recommendations(&player, &faction_system);
        assert!(!recommendations.is_empty());
    }

    #[test]
    fn test_abandon_quest() {
        let mut quest_system = QuestSystem::new();
        let mut quest = create_test_quest();
        // Change category to non-tutorial so it can be abandoned
        quest.category = QuestCategory::Practical;
        let player = create_test_player();
        let mut faction_system = FactionSystem::new();

        quest_system.add_quest_definition(quest);
        quest_system.start_quest("test_quest", &player, &faction_system).unwrap();

        // Abandon the quest
        let result = quest_system.abandon_quest("test_quest", &mut faction_system);
        assert!(result.is_ok());

        // Check quest status is Abandoned
        let progress = quest_system.player_progress.get("test_quest").unwrap();
        assert_eq!(progress.status, QuestStatus::Abandoned);
        assert!(progress.completed_at.is_some());
    }

    #[test]
    fn test_cannot_abandon_tutorial() {
        let mut quest_system = QuestSystem::new();
        let quest = create_test_quest(); // Tutorial quest
        let player = create_test_player();
        let mut faction_system = FactionSystem::new();

        quest_system.add_quest_definition(quest);
        quest_system.start_quest("test_quest", &player, &faction_system).unwrap();

        // Try to abandon tutorial quest (should fail)
        let result = quest_system.abandon_quest("test_quest", &mut faction_system);
        assert!(result.is_err());

        // Quest should still be in progress
        let progress = quest_system.player_progress.get("test_quest").unwrap();
        assert_eq!(progress.status, QuestStatus::InProgress);
    }

    #[test]
    fn test_cannot_abandon_non_active_quest() {
        let mut quest_system = QuestSystem::new();
        let mut quest = create_test_quest();
        quest.category = QuestCategory::Practical;
        let mut faction_system = FactionSystem::new();

        quest_system.add_quest_definition(quest);

        // Try to abandon quest that hasn't been started
        let result = quest_system.abandon_quest("test_quest", &mut faction_system);
        assert!(result.is_err());
    }

    #[test]
    fn test_abandon_quest_with_faction_penalty() {
        let mut quest_system = QuestSystem::new();
        let mut quest = create_test_quest();
        quest.category = QuestCategory::Practical;
        // Add faction effects
        quest.faction_effects.insert(FactionId::MagistersCouncil, 20);
        let player = create_test_player();
        let mut faction_system = FactionSystem::new();

        // Get initial reputation
        let initial_rep = faction_system.get_reputation(FactionId::MagistersCouncil);

        quest_system.add_quest_definition(quest);
        quest_system.start_quest("test_quest", &player, &faction_system).unwrap();

        // Abandon the quest
        let result = quest_system.abandon_quest("test_quest", &mut faction_system);
        assert!(result.is_ok());

        // Check that faction reputation decreased
        let new_rep = faction_system.get_reputation(FactionId::MagistersCouncil);
        assert!(new_rep < initial_rep, "Reputation should decrease after abandoning quest");
    }
}