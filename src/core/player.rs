//! Player character state and management
//!
//! This module handles all aspects of the player character including:
//! - Core attributes (Mental Acuity, Resonance Sensitivity)
//! - Mental energy and fatigue tracking
//! - Inventory and crystal management
//! - Theory knowledge and progression

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::systems::factions::FactionId;
use crate::systems::knowledge::{TheoryProgress, LearningActivity, LearningMethod};
use crate::GameResult;

/// Core player attributes that define magical capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAttributes {
    /// Mental processing power affecting energy pool and learning speed (0-100)
    pub mental_acuity: i32,
    /// Sensitivity to magical resonance affecting success rates (0-100)
    pub resonance_sensitivity: i32,
    /// Experience points in each attribute for progression tracking
    pub experience: AttributeExperience,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeExperience {
    pub mental_acuity_xp: i32,
    pub resonance_sensitivity_xp: i32,
}

/// Mental energy and fatigue state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentalState {
    /// Current mental energy available for magic (calculated from Mental Acuity × 1.5)
    pub current_energy: i32,
    /// Maximum mental energy capacity
    pub max_energy: i32,
    /// Accumulated fatigue from magical use (0-100)
    pub fatigue: i32,
}

/// Crystal in player's possession
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crystal {
    /// Type of crystal determining resonance frequency
    pub crystal_type: CrystalType,
    /// Structural integrity percentage (0-100)
    pub integrity: f32,
    /// Crystal purity affecting efficiency (0.0-1.0)
    pub purity: f32,
    /// Crystal size affecting power output
    pub size: CrystalSize,
    /// Current resonance frequency (1-10)
    pub frequency: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrystalType {
    Quartz,     // Frequency 4, good for basic magic
    Amethyst,   // Frequency 7, excellent for healing
    Obsidian,   // Frequency 2, strong but volatile
    Garnet,     // Frequency 6, stable and reliable
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrystalSize {
    Tiny,    // 0.5x power multiplier
    Small,   // 0.8x power multiplier
    Medium,  // 1.0x power multiplier
    Large,   // 1.3x power multiplier
}

/// Theory knowledge and progression with enhanced tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeState {
    /// Mastered theories with understanding level (0.0-1.0) - maintained for backward compatibility
    pub theories: HashMap<String, f32>,
    /// Current research progress
    pub active_research: Option<String>,
    /// Research progress percentage (0.0-1.0)
    pub research_progress: f32,
    /// Enhanced theory progress tracking with comprehensive details
    #[serde(default)]
    pub theory_progress: HashMap<String, TheoryProgress>,
    /// History of learning activities for analysis and progression
    #[serde(default)]
    pub learning_history: Vec<LearningActivity>,
    /// Available learning methods by theory (cached for performance)
    #[serde(default)]
    pub available_methods: HashMap<String, Vec<LearningMethod>>,
    /// Current session tracking for learning efficiency calculations
    #[serde(default)]
    pub current_session: Option<LearningSession>,
}

/// Tracks current learning session for efficiency calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningSession {
    /// Theory being studied in this session
    pub theory_id: String,
    /// Learning method being used
    pub method: LearningMethod,
    /// Time when session started (Unix timestamp)
    pub started_at: i64,
    /// Total time invested in this session so far (minutes)
    pub time_invested: i32,
    /// Energy used in this session
    pub energy_used: i32,
    /// Session efficiency factor based on conditions
    pub efficiency_factor: f32,
}

/// Player's inventory and equipment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    /// Crystals in possession
    pub crystals: Vec<Crystal>,
    /// Currently equipped crystal for magic use
    pub active_crystal: Option<usize>,
    /// Other items (notes, books, artifacts) - legacy system
    pub items: Vec<Item>,
    /// Currency in silver pieces
    pub silver: i32,
    /// Enhanced item system integration
    #[serde(default)]
    pub enhanced_items: Option<crate::systems::items::ItemSystem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemType {
    Book(String),      // Theory name it teaches
    Note(String),      // Information content
    Artifact(String),  // Special properties
    Mundane,          // Regular items
}

/// Complete player character state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    /// Player's name
    pub name: String,
    /// Core attributes and capabilities
    pub attributes: PlayerAttributes,
    /// Mental energy and fatigue tracking
    pub mental_state: MentalState,
    /// Faction reputation standings
    pub faction_standings: HashMap<FactionId, i32>,
    /// Theory knowledge and research
    pub knowledge: KnowledgeState,
    /// Inventory and equipment
    pub inventory: Inventory,
    /// Current location in the world
    pub current_location: String,
    /// Total playtime in minutes
    pub playtime_minutes: i32,
}

impl Player {
    /// Create a new player with starting attributes
    pub fn new(name: String) -> Self {
        let mental_acuity = 25; // Starting value
        let max_energy = (mental_acuity as f32 * 1.5) as i32;

        Self {
            name,
            attributes: PlayerAttributes {
                mental_acuity,
                resonance_sensitivity: 20,
                experience: AttributeExperience {
                    mental_acuity_xp: 0,
                    resonance_sensitivity_xp: 0,
                },
            },
            mental_state: MentalState {
                current_energy: max_energy,
                max_energy,
                fatigue: 0,
            },
            faction_standings: HashMap::new(),
            knowledge: KnowledgeState {
                theories: HashMap::new(),
                active_research: None,
                research_progress: 0.0,
                theory_progress: HashMap::new(),
                learning_history: Vec::new(),
                available_methods: HashMap::new(),
                current_session: None,
            },
            inventory: Inventory {
                crystals: vec![
                    // Starting crystal
                    Crystal {
                        crystal_type: CrystalType::Quartz,
                        integrity: 95.0,
                        purity: 0.6,
                        size: CrystalSize::Small,
                        frequency: 4,
                    }
                ],
                active_crystal: Some(0),
                items: Vec::new(),
                silver: 50,
                enhanced_items: Some(crate::systems::items::ItemSystem::new()),
            },
            current_location: "tutorial_chamber".to_string(),
            playtime_minutes: 0,
        }
    }

    /// Get effective mental energy accounting for fatigue
    pub fn effective_mental_energy(&self) -> i32 {
        let fatigue_penalty = (self.mental_state.fatigue as f32 * 0.5) as i32;
        (self.mental_state.current_energy - fatigue_penalty).max(0)
    }

    /// Use mental energy for magical actions
    pub fn use_mental_energy(&mut self, amount: i32, fatigue_cost: i32) -> GameResult<()> {
        if self.effective_mental_energy() < amount {
            return Err(crate::GameError::InsufficientResources(
                format!("Need {} mental energy, have {} effective energy",
                        amount, self.effective_mental_energy())
            ).into());
        }

        self.mental_state.current_energy =
            (self.mental_state.current_energy - amount).max(0);
        self.mental_state.fatigue =
            (self.mental_state.fatigue + fatigue_cost).min(100);

        Ok(())
    }

    /// Recover mental energy through rest
    pub fn recover_energy(&mut self, amount: i32, fatigue_reduction: i32) {
        self.mental_state.current_energy =
            (self.mental_state.current_energy + amount).min(self.mental_state.max_energy);
        self.mental_state.fatigue =
            (self.mental_state.fatigue - fatigue_reduction).max(0);
    }

    /// Get currently equipped crystal
    pub fn active_crystal(&self) -> Option<&Crystal> {
        self.inventory.active_crystal
            .and_then(|idx| self.inventory.crystals.get(idx))
    }

    /// Get currently equipped crystal mutably
    pub fn active_crystal_mut(&mut self) -> Option<&mut Crystal> {
        if let Some(idx) = self.inventory.active_crystal {
            self.inventory.crystals.get_mut(idx)
        } else {
            None
        }
    }

    /// Check if player knows a specific theory
    pub fn knows_theory(&self, theory: &str) -> bool {
        self.knowledge.theories.contains_key(theory)
    }

    /// Get understanding level of a theory (0.0-1.0)
    pub fn theory_understanding(&self, theory: &str) -> f32 {
        // Check enhanced progress first, fall back to basic theories for compatibility
        if let Some(progress) = self.knowledge.theory_progress.get(theory) {
            progress.understanding_level
        } else {
            self.knowledge.theories.get(theory).copied().unwrap_or(0.0)
        }
    }

    /// Add experience to an attribute
    pub fn add_experience(&mut self, attribute: AttributeType, amount: i32) {
        match attribute {
            AttributeType::MentalAcuity => {
                self.attributes.experience.mental_acuity_xp += amount;
                // Check for level up (every 100 XP)
                if self.attributes.experience.mental_acuity_xp >=
                   (self.attributes.mental_acuity + 1) * 100 {
                    self.attributes.mental_acuity += 1;
                    self.recalculate_max_energy();
                }
            }
            AttributeType::ResonanceSensitivity => {
                self.attributes.experience.resonance_sensitivity_xp += amount;
                if self.attributes.experience.resonance_sensitivity_xp >=
                   (self.attributes.resonance_sensitivity + 1) * 100 {
                    self.attributes.resonance_sensitivity += 1;
                }
            }
        }
    }

    /// Recalculate maximum mental energy when Mental Acuity changes
    fn recalculate_max_energy(&mut self) {
        let new_max = (self.attributes.mental_acuity as f32 * 1.5) as i32;
        let energy_increase = new_max - self.mental_state.max_energy;
        self.mental_state.max_energy = new_max;
        self.mental_state.current_energy += energy_increase;
    }

    /// Get faction reputation (-100 to +100)
    pub fn faction_reputation(&self, faction: FactionId) -> i32 {
        self.faction_standings.get(&faction).copied().unwrap_or(0)
    }

    /// Modify faction reputation with bounds checking
    pub fn modify_faction_reputation(&mut self, faction: FactionId, change: i32) {
        let current = self.faction_reputation(faction);
        let new_value = (current + change).clamp(-100, 100);
        self.faction_standings.insert(faction, new_value);
    }

    // Enhanced Knowledge System Integration Methods

    /// Check if a theory is accessible (prerequisites met)
    pub fn can_access_theory(&self, theory_id: &str, knowledge_system: &crate::systems::knowledge::KnowledgeSystem) -> GameResult<bool> {
        knowledge_system.get_accessible_theories(self)
            .map(|theories| theories.iter().any(|t| t.id == theory_id))
    }

    /// Get detailed theory progress information
    pub fn get_theory_progress(&self, theory_id: &str) -> Option<&TheoryProgress> {
        self.knowledge.theory_progress.get(theory_id)
    }

    /// Update theory progress from learning activity (maintains backward compatibility)
    pub fn update_theory_progress(&mut self, activity: &LearningActivity) -> GameResult<()> {
        let theory_id = &activity.theory_id;

        // Update basic theories map for backward compatibility
        let current_understanding = self.theory_understanding(theory_id);
        let new_understanding = (current_understanding + activity.understanding_gained).min(1.0);
        self.knowledge.theories.insert(theory_id.clone(), new_understanding);

        // Update or create enhanced progress tracking
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let progress = self.knowledge.theory_progress.entry(theory_id.clone())
            .or_insert_with(|| TheoryProgress {
                understanding_level: current_understanding,
                experience_points: 0,
                learning_history: HashMap::new(),
                time_invested: 0,
                discovered_at: now,
                mastered_at: None,
                is_active_research: false,
                research_progress: 0.0,
            });

        // Update progress with activity results
        progress.understanding_level = new_understanding;
        progress.experience_points += activity.experience_gained;
        progress.time_invested += activity.duration;

        // Track learning method usage
        let method_experience = progress.learning_history.entry(activity.method.clone()).or_insert(0);
        *method_experience += activity.experience_gained;

        // Mark as mastered if reached 100%
        if new_understanding >= 1.0 && progress.mastered_at.is_none() {
            progress.mastered_at = Some(now);
        }

        // Add to learning history
        self.knowledge.learning_history.push(activity.clone());

        // Limit history size to prevent memory bloat
        if self.knowledge.learning_history.len() > 1000 {
            self.knowledge.learning_history.drain(0..100); // Remove oldest 100 entries
        }

        Ok(())
    }

    /// Start a new learning session
    pub fn start_learning_session(&mut self, theory_id: String, method: LearningMethod) -> GameResult<()> {
        // End any existing session first
        self.end_learning_session();

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        // Calculate efficiency factor based on current conditions
        let efficiency_factor = self.calculate_learning_efficiency(&method);

        self.knowledge.current_session = Some(LearningSession {
            theory_id,
            method,
            started_at: now,
            time_invested: 0,
            energy_used: 0,
            efficiency_factor,
        });

        Ok(())
    }

    /// End current learning session
    pub fn end_learning_session(&mut self) {
        self.knowledge.current_session = None;
    }

    /// Calculate current learning efficiency based on mental state and conditions
    pub fn calculate_learning_efficiency(&self, method: &LearningMethod) -> f32 {
        let mut efficiency = 1.0;

        // Mental acuity affects all learning
        efficiency *= (self.attributes.mental_acuity as f32 / 100.0).max(0.1);

        // Fatigue reduces efficiency
        let fatigue_penalty = (self.mental_state.fatigue as f32 / 100.0) * 0.5;
        efficiency *= (1.0 - fatigue_penalty).max(0.1);

        // Method-specific modifiers
        match method {
            LearningMethod::Study => {
                // Study benefits from mental acuity
                efficiency *= 1.0 + (self.attributes.mental_acuity as f32 / 200.0);
            },
            LearningMethod::Experimentation => {
                // Experimentation benefits from resonance sensitivity
                efficiency *= 1.0 + (self.attributes.resonance_sensitivity as f32 / 200.0);
            },
            LearningMethod::Observation => {
                // Observation heavily depends on resonance sensitivity
                efficiency *= (self.attributes.resonance_sensitivity as f32 / 100.0).max(0.1);
            },
            _ => {} // Other methods use base efficiency
        }

        efficiency.max(0.1).min(2.0) // Clamp between 10% and 200% efficiency
    }

    /// Get all mastered theories
    pub fn get_mastered_theories(&self) -> Vec<String> {
        let mut mastered = Vec::new();

        // Check enhanced progress first
        for (theory_id, progress) in &self.knowledge.theory_progress {
            if progress.understanding_level >= 1.0 {
                mastered.push(theory_id.clone());
            }
        }

        // Check basic theories for backward compatibility
        for (theory_id, understanding) in &self.knowledge.theories {
            if *understanding >= 1.0 && !mastered.contains(theory_id) {
                mastered.push(theory_id.clone());
            }
        }

        mastered
    }

    /// Get learning statistics for a theory
    pub fn get_learning_stats(&self, theory_id: &str) -> Option<LearningStats> {
        self.knowledge.theory_progress.get(theory_id).map(|progress| {
            let total_time = progress.time_invested;
            let total_experience = progress.experience_points;
            let methods_used = progress.learning_history.len();
            let avg_efficiency = if total_time > 0 {
                total_experience as f32 / total_time as f32
            } else {
                0.0
            };

            LearningStats {
                total_time_minutes: total_time,
                total_experience,
                methods_used,
                average_efficiency: avg_efficiency,
                mastery_date: progress.mastered_at,
                discovery_date: progress.discovered_at,
            }
        })
    }

    /// Get available learning methods for theories the player can access
    pub fn get_available_learning_methods(&self, theory_id: &str) -> Vec<LearningMethod> {
        // Return cached methods if available
        if let Some(methods) = self.knowledge.available_methods.get(theory_id) {
            return methods.clone();
        }

        // Fallback to all basic methods if not cached
        vec![LearningMethod::Study, LearningMethod::Observation]
    }

    /// Update available learning methods cache
    pub fn update_available_methods(&mut self, theory_id: String, methods: Vec<LearningMethod>) {
        self.knowledge.available_methods.insert(theory_id, methods);
    }

    /// Check if player meets learning method requirements
    pub fn can_use_learning_method(&self, theory_id: &str, method: &LearningMethod) -> bool {
        let understanding = self.theory_understanding(theory_id);

        match method {
            LearningMethod::Study => true, // Always available
            LearningMethod::Experimentation => {
                // Requires active crystal and sufficient understanding
                self.active_crystal().is_some() && understanding >= 0.1
            },
            LearningMethod::Observation => true, // Always available
            LearningMethod::Teaching => {
                // Requires high understanding to teach effectively
                understanding >= 0.6
            },
            LearningMethod::Research => {
                // Requires very high understanding and mental acuity
                understanding >= 0.8 && self.attributes.mental_acuity >= 60
            },
            LearningMethod::Mentorship => {
                // Depends on finding appropriate NPCs (context-dependent)
                true
            },
        }
    }

    // Theory Benefit Calculations

    /// Calculate magic success rate bonus from mastered theories
    pub fn calculate_theory_magic_bonus(&self) -> f32 {
        let mut total_bonus = 0.0;

        // Harmonic Fundamentals: +15% magic success rate, reduces energy costs
        if self.theory_understanding("harmonic_fundamentals") >= 1.0 {
            total_bonus += 0.15;
        } else {
            total_bonus += self.theory_understanding("harmonic_fundamentals") * 0.15;
        }

        // Crystal Structures: +20% crystal efficiency, slower degradation
        if self.theory_understanding("crystal_structures") >= 1.0 {
            total_bonus += 0.10; // Indirect bonus to magic through crystal efficiency
        } else {
            total_bonus += self.theory_understanding("crystal_structures") * 0.10;
        }

        // Mental Resonance: +10% mental energy regeneration, fatigue resistance
        if self.theory_understanding("mental_resonance") >= 1.0 {
            total_bonus += 0.05; // Indirect bonus through better energy management
        } else {
            total_bonus += self.theory_understanding("mental_resonance") * 0.05;
        }

        // Light Manipulation: Unlocks advanced light spells, +25% light magic effectiveness
        if self.theory_understanding("light_manipulation") >= 1.0 {
            total_bonus += 0.08; // General magic improvement
        } else {
            total_bonus += self.theory_understanding("light_manipulation") * 0.08;
        }

        // Bio-resonance: Unlocks healing spells, +30% healing effectiveness
        if self.theory_understanding("bio_resonance") >= 1.0 {
            total_bonus += 0.08; // General magic improvement
        } else {
            total_bonus += self.theory_understanding("bio_resonance") * 0.08;
        }

        // Detection Arrays: Unlocks detection spells, environmental awareness
        if self.theory_understanding("detection_arrays") >= 1.0 {
            total_bonus += 0.05;
        } else {
            total_bonus += self.theory_understanding("detection_arrays") * 0.05;
        }

        // Sympathetic Networks: Long-distance magic capabilities
        if self.theory_understanding("sympathetic_networks") >= 1.0 {
            total_bonus += 0.12;
        } else {
            total_bonus += self.theory_understanding("sympathetic_networks") * 0.12;
        }

        // Resonance Amplification: Power multiplication for all spells
        if self.theory_understanding("resonance_amplification") >= 1.0 {
            total_bonus += 0.20;
        } else {
            total_bonus += self.theory_understanding("resonance_amplification") * 0.20;
        }

        // Theoretical Synthesis: Ability to create custom spell combinations
        if self.theory_understanding("theoretical_synthesis") >= 1.0 {
            total_bonus += 0.25;
        } else {
            total_bonus += self.theory_understanding("theoretical_synthesis") * 0.25;
        }

        total_bonus
    }

    /// Calculate energy cost reduction from theories
    pub fn calculate_theory_energy_reduction(&self) -> f32 {
        let mut reduction = 0.0;

        // Harmonic Fundamentals reduces energy costs
        if self.theory_understanding("harmonic_fundamentals") >= 1.0 {
            reduction += 0.15;
        } else {
            reduction += self.theory_understanding("harmonic_fundamentals") * 0.15;
        }

        // Mental Resonance improves energy efficiency
        if self.theory_understanding("mental_resonance") >= 1.0 {
            reduction += 0.20;
        } else {
            reduction += self.theory_understanding("mental_resonance") * 0.20;
        }

        // Resonance Amplification optimizes energy use
        if self.theory_understanding("resonance_amplification") >= 1.0 {
            reduction += 0.10;
        } else {
            reduction += self.theory_understanding("resonance_amplification") * 0.10;
        }

        reduction.min(0.5) // Cap at 50% reduction
    }

    /// Calculate crystal degradation reduction from theories
    pub fn calculate_theory_crystal_protection(&self) -> f32 {
        let mut protection = 0.0;

        // Crystal Structures provides crystal protection
        if self.theory_understanding("crystal_structures") >= 1.0 {
            protection += 0.30;
        } else {
            protection += self.theory_understanding("crystal_structures") * 0.30;
        }

        // Harmonic Fundamentals provides some protection through better resonance
        if self.theory_understanding("harmonic_fundamentals") >= 1.0 {
            protection += 0.10;
        } else {
            protection += self.theory_understanding("harmonic_fundamentals") * 0.10;
        }

        protection.min(0.5) // Cap at 50% protection
    }

    /// Calculate spell-specific bonuses based on theories
    pub fn calculate_spell_type_bonus(&self, spell_type: &str) -> f32 {
        let mut bonus = 0.0;

        match spell_type {
            "light" => {
                if self.theory_understanding("light_manipulation") >= 1.0 {
                    bonus += 0.25; // +25% light magic effectiveness
                } else {
                    bonus += self.theory_understanding("light_manipulation") * 0.25;
                }
            },
            "healing" => {
                if self.theory_understanding("bio_resonance") >= 1.0 {
                    bonus += 0.30; // +30% healing effectiveness
                } else {
                    bonus += self.theory_understanding("bio_resonance") * 0.30;
                }
            },
            "detection" => {
                if self.theory_understanding("detection_arrays") >= 1.0 {
                    bonus += 0.20; // +20% detection effectiveness
                } else {
                    bonus += self.theory_understanding("detection_arrays") * 0.20;
                }
            },
            "communication" => {
                if self.theory_understanding("sympathetic_networks") >= 1.0 {
                    bonus += 0.25; // +25% communication effectiveness
                } else {
                    bonus += self.theory_understanding("sympathetic_networks") * 0.25;
                }
            },
            "manipulation" => {
                if self.theory_understanding("resonance_amplification") >= 1.0 {
                    bonus += 0.15; // +15% manipulation effectiveness
                } else {
                    bonus += self.theory_understanding("resonance_amplification") * 0.15;
                }
            },
            _ => {} // No specific bonus for unknown spell types
        }

        bonus
    }

    /// Check if specific magic capabilities are unlocked by theories
    pub fn has_magic_capability(&self, capability: &str) -> bool {
        match capability {
            "advanced_light_spells" => self.theory_understanding("light_manipulation") >= 1.0,
            "healing_spells" => self.theory_understanding("bio_resonance") >= 0.8,
            "detection_spells" => self.theory_understanding("detection_arrays") >= 0.8,
            "long_distance_magic" => self.theory_understanding("sympathetic_networks") >= 1.0,
            "power_amplification" => self.theory_understanding("resonance_amplification") >= 1.0,
            "custom_spell_combinations" => self.theory_understanding("theoretical_synthesis") >= 1.0,
            _ => false,
        }
    }

    /// Calculate mental energy regeneration bonus from theories
    pub fn calculate_theory_regeneration_bonus(&self) -> f32 {
        let mut bonus = 0.0;

        // Mental Resonance improves mental energy regeneration
        if self.theory_understanding("mental_resonance") >= 1.0 {
            bonus += 0.10; // +10% regeneration
        } else {
            bonus += self.theory_understanding("mental_resonance") * 0.10;
        }

        bonus
    }

    /// Calculate fatigue resistance from theories
    pub fn calculate_theory_fatigue_resistance(&self) -> f32 {
        let mut resistance = 0.0;

        // Mental Resonance provides fatigue resistance
        if self.theory_understanding("mental_resonance") >= 1.0 {
            resistance += 0.20; // 20% less fatigue accumulation
        } else {
            resistance += self.theory_understanding("mental_resonance") * 0.20;
        }

        resistance.min(0.4) // Cap at 40% resistance
    }

    // Enhanced Item System Integration Methods

    /// Get reference to enhanced item system
    pub fn enhanced_item_system(&self) -> Option<&crate::systems::items::ItemSystem> {
        self.inventory.enhanced_items.as_ref()
    }

    /// Get mutable reference to enhanced item system
    pub fn enhanced_item_system_mut(&mut self) -> Option<&mut crate::systems::items::ItemSystem> {
        self.inventory.enhanced_items.as_mut()
    }

    /// Initialize enhanced item system if not present
    pub fn ensure_enhanced_item_system(&mut self) {
        if self.inventory.enhanced_items.is_none() {
            self.inventory.enhanced_items = Some(crate::systems::items::ItemSystem::new());
        }
    }

    /// Add an item using the enhanced system
    pub fn add_enhanced_item(&mut self, item: crate::systems::items::core::Item) -> GameResult<()> {
        self.ensure_enhanced_item_system();

        // Extract the item system temporarily to avoid borrowing conflicts
        if let Some(mut item_system) = self.inventory.enhanced_items.take() {
            let result = item_system.add_item(self, item);
            self.inventory.enhanced_items = Some(item_system);
            result
        } else {
            Err(crate::GameError::InvalidInput("Enhanced item system not available".to_string()).into())
        }
    }

    /// Remove an item using the enhanced system
    pub fn remove_enhanced_item(&mut self, item_id: &str) -> GameResult<Option<crate::systems::items::core::Item>> {
        if let Some(mut item_system) = self.inventory.enhanced_items.take() {
            let result = item_system.remove_item(self, &item_id.to_string());
            self.inventory.enhanced_items = Some(item_system);
            result
        } else {
            Ok(None)
        }
    }

    /// Use an item from the enhanced system
    pub fn use_enhanced_item(&mut self, item_id: &str, target: Option<&str>) -> GameResult<String> {
        if let Some(mut item_system) = self.inventory.enhanced_items.take() {
            let result = item_system.use_item(self, &item_id.to_string(), target);
            self.inventory.enhanced_items = Some(item_system);
            result
        } else {
            Err(crate::GameError::InvalidInput("Enhanced item system not available".to_string()).into())
        }
    }

    /// Equip an item from the enhanced system
    pub fn equip_enhanced_item(&mut self, item_id: &str) -> GameResult<()> {
        if let Some(mut item_system) = self.inventory.enhanced_items.take() {
            let result = item_system.equip_item(self, &item_id.to_string());
            self.inventory.enhanced_items = Some(item_system);
            result
        } else {
            Err(crate::GameError::InvalidInput("Enhanced item system not available".to_string()).into())
        }
    }

    /// Unequip an item from the enhanced system
    pub fn unequip_enhanced_item(&mut self, slot: crate::systems::items::equipment::EquipmentSlot) -> GameResult<Option<String>> {
        if let Some(mut item_system) = self.inventory.enhanced_items.take() {
            let result = item_system.unequip_item(self, slot);
            self.inventory.enhanced_items = Some(item_system);
            result
        } else {
            Ok(None)
        }
    }

    /// Get enhanced inventory summary
    pub fn enhanced_inventory_summary(&self) -> String {
        if let Some(ref item_system) = self.inventory.enhanced_items {
            item_system.get_inventory_summary()
        } else {
            "Enhanced item system not initialized".to_string()
        }
    }

    /// Get equipment summary
    pub fn equipment_summary(&self) -> String {
        if let Some(ref item_system) = self.inventory.enhanced_items {
            item_system.get_equipment_summary()
        } else {
            "No equipment system available".to_string()
        }
    }

    /// Calculate learning bonus from equipped educational items
    pub fn calculate_educational_item_bonus(&self, theory_id: &str, method: &crate::systems::knowledge::LearningMethod) -> f32 {
        if let Some(ref item_system) = self.inventory.enhanced_items {
            item_system.calculate_learning_bonus(theory_id, method)
        } else {
            0.0
        }
    }
}

#[derive(Debug, Clone)]
pub enum AttributeType {
    MentalAcuity,
    ResonanceSensitivity,
}

/// Learning statistics for a theory
#[derive(Debug, Clone)]
pub struct LearningStats {
    pub total_time_minutes: i32,
    pub total_experience: i32,
    pub methods_used: usize,
    pub average_efficiency: f32,
    pub mastery_date: Option<i64>,
    pub discovery_date: i64,
}

impl KnowledgeState {
    /// Create new enhanced knowledge state with backward compatibility
    pub fn new() -> Self {
        Self {
            theories: HashMap::new(),
            active_research: None,
            research_progress: 0.0,
            theory_progress: HashMap::new(),
            learning_history: Vec::new(),
            available_methods: HashMap::new(),
            current_session: None,
        }
    }

    /// Migrate legacy knowledge state to enhanced version
    pub fn migrate_from_legacy(theories: HashMap<String, f32>, active_research: Option<String>, research_progress: f32) -> Self {
        let mut knowledge = Self::new();
        knowledge.theories = theories.clone();
        knowledge.active_research = active_research;
        knowledge.research_progress = research_progress;

        // Create enhanced progress entries for existing theories
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        for (theory_id, understanding) in theories {
            let mastered_at = if understanding >= 1.0 { Some(now) } else { None };

            knowledge.theory_progress.insert(theory_id, TheoryProgress {
                understanding_level: understanding,
                experience_points: (understanding * 1000.0) as i32, // Estimate XP from understanding
                learning_history: HashMap::new(),
                time_invested: (understanding * 180.0) as i32, // Estimate 3 hours per theory
                discovered_at: now,
                mastered_at,
                is_active_research: false,
                research_progress: 0.0,
            });
        }

        knowledge
    }

    /// Get total learning time across all theories
    pub fn total_learning_time(&self) -> i32 {
        self.theory_progress.values().map(|p| p.time_invested).sum()
    }

    /// Get total experience points across all theories
    pub fn total_experience(&self) -> i32 {
        self.theory_progress.values().map(|p| p.experience_points).sum()
    }

    /// Get number of theories discovered
    pub fn theories_discovered(&self) -> usize {
        self.theories.len().max(self.theory_progress.len())
    }

    /// Get number of theories mastered
    pub fn theories_mastered(&self) -> usize {
        let basic_mastered = self.theories.values().filter(|&&u| u >= 1.0).count();
        let enhanced_mastered = self.theory_progress.values().filter(|p| p.understanding_level >= 1.0).count();
        basic_mastered.max(enhanced_mastered)
    }

    /// Get learning efficiency score (0-100)
    pub fn learning_efficiency_score(&self) -> f32 {
        if self.learning_history.is_empty() {
            return 50.0; // Default score
        }

        let total_time: i32 = self.learning_history.iter().map(|a| a.duration).sum();
        let total_understanding: f32 = self.learning_history.iter().map(|a| a.understanding_gained).sum();

        if total_time > 0 {
            ((total_understanding / (total_time as f32 / 60.0)) * 100.0).min(100.0)
        } else {
            50.0
        }
    }
}

impl Crystal {
    /// Create a new crystal with specified properties
    pub fn new(crystal_type: CrystalType, integrity: f32, purity: f32, size: CrystalSize) -> Self {
        let frequency = match crystal_type {
            CrystalType::Quartz => 4,
            CrystalType::Amethyst => 7,
            CrystalType::Obsidian => 2,
            CrystalType::Garnet => 6,
        };

        Self {
            crystal_type,
            integrity: integrity.clamp(0.0, 100.0),
            purity: purity.clamp(0.0, 1.0),
            size,
            frequency,
        }
    }

    /// Calculate power multiplier based on size
    pub fn power_multiplier(&self) -> f32 {
        match self.size {
            CrystalSize::Tiny => 0.5,
            CrystalSize::Small => 0.8,
            CrystalSize::Medium => 1.0,
            CrystalSize::Large => 1.3,
        }
    }

    /// Calculate efficiency based on integrity and purity
    pub fn efficiency(&self) -> f32 {
        let integrity_factor = self.integrity / 100.0;
        let purity_factor = self.purity;
        (integrity_factor * purity_factor).max(0.1) // Minimum 10% efficiency
    }

    /// Degrade crystal from use
    pub fn degrade(&mut self, base_degradation: f32) {
        let purity_protection = self.purity * 0.5; // High purity reduces degradation
        let actual_degradation = base_degradation * (1.0 - purity_protection);
        self.integrity = (self.integrity - actual_degradation).max(0.0);
    }

    /// Check if crystal is still usable
    pub fn is_usable(&self) -> bool {
        self.integrity > 5.0 // Crystals become unusable below 5% integrity
    }

    /// Get crystal display name
    pub fn display_name(&self) -> String {
        let size_str = match self.size {
            CrystalSize::Tiny => "tiny",
            CrystalSize::Small => "small",
            CrystalSize::Medium => "",
            CrystalSize::Large => "large",
        };

        let type_str = match self.crystal_type {
            CrystalType::Quartz => "quartz",
            CrystalType::Amethyst => "amethyst",
            CrystalType::Obsidian => "obsidian",
            CrystalType::Garnet => "garnet",
        };

        if size_str.is_empty() {
            type_str.to_string()
        } else {
            format!("{} {}", size_str, type_str)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let player = Player::new("Test Player".to_string());
        assert_eq!(player.name, "Test Player");
        assert_eq!(player.attributes.mental_acuity, 25);
        assert_eq!(player.mental_state.max_energy, 37); // 25 * 1.5 = 37.5 -> 37
        assert!(player.inventory.crystals.len() > 0);
    }

    #[test]
    fn test_mental_energy_usage() {
        let mut player = Player::new("Test".to_string());
        let initial_energy = player.mental_state.current_energy;

        player.use_mental_energy(10, 5).unwrap();

        assert_eq!(player.mental_state.current_energy, initial_energy - 10);
        assert_eq!(player.mental_state.fatigue, 5);
    }

    #[test]
    fn test_crystal_degradation() {
        let mut crystal = Crystal::new(CrystalType::Quartz, 100.0, 0.8, CrystalSize::Medium);
        let initial_integrity = crystal.integrity;

        crystal.degrade(2.0);

        assert!(crystal.integrity < initial_integrity);
        assert!(crystal.is_usable());
    }

    #[test]
    fn test_faction_reputation() {
        let mut player = Player::new("Test".to_string());

        player.modify_faction_reputation(FactionId::MagistersCouncil, 25);
        assert_eq!(player.faction_reputation(FactionId::MagistersCouncil), 25);

        player.modify_faction_reputation(FactionId::MagistersCouncil, -150); // Should clamp to -100
        assert_eq!(player.faction_reputation(FactionId::MagistersCouncil), -100);
    }

    #[test]
    fn test_effective_mental_energy() {
        let mut player = Player::new("Test".to_string());
        player.mental_state.current_energy = 50;
        player.mental_state.fatigue = 20;

        // Effective = 50 - (20 * 0.5) = 40
        assert_eq!(player.effective_mental_energy(), 40);
    }

    #[test]
    fn test_enhanced_knowledge_state() {
        let mut player = Player::new("Test".to_string());

        // Test backward compatibility - basic theory understanding
        player.knowledge.theories.insert("test_theory".to_string(), 0.5);
        assert_eq!(player.theory_understanding("test_theory"), 0.5);

        // Test enhanced progress tracking
        let activity = LearningActivity {
            theory_id: "test_theory".to_string(),
            method: LearningMethod::Study,
            duration: 30,
            success_rate: 0.8,
            experience_gained: 100,
            understanding_gained: 0.2,
            resources_used: HashMap::new(),
            side_effects: vec![],
        };

        player.update_theory_progress(&activity).unwrap();

        // Should have updated both basic and enhanced tracking
        assert_eq!(player.theory_understanding("test_theory"), 0.7);
        assert!(player.get_theory_progress("test_theory").is_some());

        let progress = player.get_theory_progress("test_theory").unwrap();
        assert_eq!(progress.experience_points, 100);
        assert_eq!(progress.time_invested, 30);
    }

    #[test]
    fn test_learning_efficiency_calculation() {
        let mut player = Player::new("Test".to_string());
        player.attributes.mental_acuity = 80;
        player.attributes.resonance_sensitivity = 60;
        player.mental_state.fatigue = 20;

        let study_efficiency = player.calculate_learning_efficiency(&LearningMethod::Study);
        let experiment_efficiency = player.calculate_learning_efficiency(&LearningMethod::Experimentation);
        let observation_efficiency = player.calculate_learning_efficiency(&LearningMethod::Observation);

        // All should be positive and reasonable
        assert!(study_efficiency > 0.0);
        assert!(experiment_efficiency > 0.0);
        assert!(observation_efficiency > 0.0);

        // Study should benefit more from mental acuity
        // Observation should be lower due to fatigue affecting it more
        assert!(study_efficiency > observation_efficiency);
    }

    #[test]
    fn test_learning_method_requirements() {
        let mut player = Player::new("Test".to_string());

        // Basic methods should always be available
        assert!(player.can_use_learning_method("any_theory", &LearningMethod::Study));
        assert!(player.can_use_learning_method("any_theory", &LearningMethod::Observation));

        // Teaching requires understanding
        assert!(!player.can_use_learning_method("new_theory", &LearningMethod::Teaching));

        // Add some understanding
        player.knowledge.theories.insert("known_theory".to_string(), 0.8);
        assert!(player.can_use_learning_method("known_theory", &LearningMethod::Teaching));

        // Research requires high understanding and mental acuity
        assert!(!player.can_use_learning_method("known_theory", &LearningMethod::Research));
        player.attributes.mental_acuity = 70;
        assert!(player.can_use_learning_method("known_theory", &LearningMethod::Research));
    }

    #[test]
    fn test_learning_session_management() {
        let mut player = Player::new("Test".to_string());

        // No session initially
        assert!(player.knowledge.current_session.is_none());

        // Start a session
        player.start_learning_session("test_theory".to_string(), LearningMethod::Study).unwrap();
        assert!(player.knowledge.current_session.is_some());

        let session = player.knowledge.current_session.as_ref().unwrap();
        assert_eq!(session.theory_id, "test_theory");
        assert_eq!(session.method, LearningMethod::Study);

        // End session
        player.end_learning_session();
        assert!(player.knowledge.current_session.is_none());
    }

    #[test]
    fn test_knowledge_state_migration() {
        let mut old_theories = HashMap::new();
        old_theories.insert("theory1".to_string(), 0.5);
        old_theories.insert("theory2".to_string(), 1.0);

        let knowledge = KnowledgeState::migrate_from_legacy(
            old_theories,
            Some("theory1".to_string()),
            0.3
        );

        // Should preserve backward compatibility
        assert_eq!(knowledge.theories.len(), 2);
        assert_eq!(knowledge.theories.get("theory1"), Some(&0.5));
        assert_eq!(knowledge.theories.get("theory2"), Some(&1.0));
        assert_eq!(knowledge.active_research, Some("theory1".to_string()));
        assert_eq!(knowledge.research_progress, 0.3);

        // Should have enhanced tracking
        assert_eq!(knowledge.theory_progress.len(), 2);
        assert!(knowledge.theory_progress.get("theory1").is_some());
        assert!(knowledge.theory_progress.get("theory2").is_some());

        // Theory2 should be marked as mastered
        let theory2_progress = knowledge.theory_progress.get("theory2").unwrap();
        assert!(theory2_progress.mastered_at.is_some());
    }

    #[test]
    fn test_mastered_theories_collection() {
        let mut player = Player::new("Test".to_string());

        // Add some theories with different completion levels
        player.knowledge.theories.insert("basic_mastered".to_string(), 1.0);
        player.knowledge.theories.insert("basic_partial".to_string(), 0.6);

        // Create enhanced progress for another theory
        let activity = LearningActivity {
            theory_id: "enhanced_mastered".to_string(),
            method: LearningMethod::Study,
            duration: 60,
            success_rate: 1.0,
            experience_gained: 200,
            understanding_gained: 1.0,
            resources_used: HashMap::new(),
            side_effects: vec![],
        };
        player.update_theory_progress(&activity).unwrap();

        let mastered = player.get_mastered_theories();
        assert!(mastered.contains(&"basic_mastered".to_string()));
        assert!(mastered.contains(&"enhanced_mastered".to_string()));
        assert!(!mastered.contains(&"basic_partial".to_string()));
        assert_eq!(mastered.len(), 2);
    }

    #[test]
    fn test_theory_magic_bonuses() {
        let mut player = Player::new("Test".to_string());

        // Initial player should have no bonuses
        assert_eq!(player.calculate_theory_magic_bonus(), 0.0);
        assert_eq!(player.calculate_theory_energy_reduction(), 0.0);
        assert_eq!(player.calculate_theory_crystal_protection(), 0.0);

        // Add some theory understanding
        player.knowledge.theories.insert("harmonic_fundamentals".to_string(), 1.0);
        player.knowledge.theories.insert("crystal_structures".to_string(), 0.5);
        player.knowledge.theories.insert("mental_resonance".to_string(), 0.8);

        // Should now have bonuses
        let magic_bonus = player.calculate_theory_magic_bonus();
        assert!(magic_bonus > 0.0);
        assert!(magic_bonus < 1.0); // Should be reasonable

        let energy_reduction = player.calculate_theory_energy_reduction();
        assert!(energy_reduction > 0.0);
        assert!(energy_reduction <= 0.5); // Should be capped at 50%

        let crystal_protection = player.calculate_theory_crystal_protection();
        assert!(crystal_protection > 0.0);
        assert!(crystal_protection <= 0.5); // Should be capped at 50%

        // Test spell-specific bonuses
        let light_bonus = player.calculate_spell_type_bonus("light");
        assert_eq!(light_bonus, 0.0); // No light manipulation theory

        player.knowledge.theories.insert("light_manipulation".to_string(), 1.0);
        let light_bonus_after = player.calculate_spell_type_bonus("light");
        assert_eq!(light_bonus_after, 0.25); // Should be 25% for mastered light manipulation
    }

    #[test]
    fn test_magic_capabilities() {
        let mut player = Player::new("Test".to_string());

        // Initially should have no advanced capabilities
        assert!(!player.has_magic_capability("healing_spells"));
        assert!(!player.has_magic_capability("detection_spells"));
        assert!(!player.has_magic_capability("long_distance_magic"));

        // Add theories to unlock capabilities
        player.knowledge.theories.insert("bio_resonance".to_string(), 0.8);
        assert!(player.has_magic_capability("healing_spells"));

        player.knowledge.theories.insert("detection_arrays".to_string(), 0.8);
        assert!(player.has_magic_capability("detection_spells"));

        player.knowledge.theories.insert("sympathetic_networks".to_string(), 1.0);
        assert!(player.has_magic_capability("long_distance_magic"));

        player.knowledge.theories.insert("theoretical_synthesis".to_string(), 1.0);
        assert!(player.has_magic_capability("custom_spell_combinations"));
    }
}