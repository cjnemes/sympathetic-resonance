//! Item interaction and combination system
//!
//! This module handles:
//! - Item-to-item interactions and combinations
//! - Crafting and synthesis rules
//! - Special interaction effects
//! - Recipe management

use super::core::{Item, ItemId, ItemEffect, ItemType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::GameResult;

/// Defines how items can interact with each other
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemInteraction {
    /// Unique identifier for this interaction
    pub id: String,
    /// Input items required
    pub inputs: Vec<InteractionInput>,
    /// Output items produced
    pub outputs: Vec<InteractionOutput>,
    /// Conditions that must be met
    pub conditions: InteractionConditions,
    /// Type of interaction
    pub interaction_type: InteractionType,
}

/// Input requirements for an interaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionInput {
    /// Item ID or pattern
    pub item_requirement: ItemRequirement,
    /// Quantity needed
    pub quantity: i32,
    /// Whether item is consumed
    pub consumed: bool,
}

/// Output produced by an interaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionOutput {
    /// Item to produce
    pub item: Item,
    /// Quantity produced
    pub quantity: i32,
    /// Probability of success (0.0-1.0)
    pub success_chance: f32,
}

/// Requirements for items in interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemRequirement {
    /// Specific item by ID
    SpecificItem(ItemId),
    /// Any item of a specific type
    ItemType(ItemType),
    /// Any item with specific properties
    ItemWithProperties {
        name_pattern: Option<String>,
        min_value: Option<i32>,
        min_durability: Option<i32>,
    },
    /// Any crystal with specific properties
    CrystalWithProperties {
        crystal_type: Option<String>,
        min_purity: Option<f32>,
        min_integrity: Option<f32>,
    },
}

/// Conditions required for interaction to succeed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionConditions {
    /// Required player attributes
    pub min_attributes: HashMap<String, i32>,
    /// Required theories
    pub required_theories: Vec<String>,
    /// Required environment or location
    pub required_environment: Option<String>,
    /// Required tools (not consumed)
    pub required_tools: Vec<ItemId>,
    /// Energy cost
    pub energy_cost: i32,
    /// Time required in minutes
    pub time_cost: i32,
}

/// Types of item interactions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InteractionType {
    /// Simple combining of items
    Combination,
    /// Crafting new items from materials
    Crafting,
    /// Enhancing existing items
    Enhancement,
    /// Repairing damaged items
    Repair,
    /// Transmutation (changing item properties)
    Transmutation,
    /// Research-based synthesis
    Synthesis,
    /// Ritual or magical combination
    Ritual,
}

/// Result of an item interaction attempt
#[derive(Debug, Clone)]
pub struct InteractionResult {
    /// Whether interaction succeeded
    pub success: bool,
    /// Items produced
    pub outputs: Vec<Item>,
    /// Items consumed
    pub consumed_items: Vec<ItemId>,
    /// Experience gained
    pub experience_gained: i32,
    /// Description of what happened
    pub description: String,
    /// Side effects
    pub side_effects: Vec<InteractionSideEffect>,
}

/// Side effects that can occur during interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionSideEffect {
    /// Damage to tools or equipment
    ToolDamage { item_id: ItemId, damage: i32 },
    /// Attribute boost or penalty
    AttributeChange { attribute: String, change: i32 },
    /// Energy cost or restoration
    EnergyChange(i32),
    /// Fatigue change
    FatigueChange(i32),
    /// Theory understanding change
    TheoryChange { theory_id: String, change: f32 },
}

/// Rules for combining specific items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombinationRule {
    /// Items that can be combined
    pub combinable_items: Vec<ItemId>,
    /// Result of combination
    pub result: CombinationResult,
    /// Success rate
    pub base_success_rate: f32,
    /// Requirements to attempt
    pub requirements: InteractionConditions,
}

/// Result of a combination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombinationResult {
    /// Single item produced
    SingleItem(Item),
    /// Multiple possible outcomes
    MultipleOutcomes(Vec<(Item, f32)>), // (item, probability)
    /// Enhanced version of input item
    Enhancement { target_item: usize, enhancement: ItemEnhancement },
}

/// Enhancement that can be applied to items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemEnhancement {
    /// Durability change
    pub durability_change: Option<i32>,
    /// Value multiplier
    pub value_multiplier: Option<f32>,
    /// New effects added
    pub added_effects: Vec<ItemEffect>,
    /// Property changes
    pub property_changes: HashMap<String, String>,
}

impl ItemInteraction {
    /// Create a simple combination interaction
    pub fn new_combination(
        id: String,
        input_items: Vec<(ItemId, i32)>,
        output_item: Item,
        success_chance: f32,
    ) -> Self {
        let inputs = input_items
            .into_iter()
            .map(|(item_id, qty)| InteractionInput {
                item_requirement: ItemRequirement::SpecificItem(item_id),
                quantity: qty,
                consumed: true,
            })
            .collect();

        let outputs = vec![InteractionOutput {
            item: output_item,
            quantity: 1,
            success_chance,
        }];

        Self {
            id,
            inputs,
            outputs,
            conditions: InteractionConditions::default(),
            interaction_type: InteractionType::Combination,
        }
    }

    /// Create a crafting interaction
    pub fn new_crafting(
        id: String,
        materials: Vec<(ItemType, i32)>,
        output_item: Item,
        required_theories: Vec<String>,
        energy_cost: i32,
    ) -> Self {
        let inputs = materials
            .into_iter()
            .map(|(item_type, qty)| InteractionInput {
                item_requirement: ItemRequirement::ItemType(item_type),
                quantity: qty,
                consumed: true,
            })
            .collect();

        let outputs = vec![InteractionOutput {
            item: output_item,
            quantity: 1,
            success_chance: 0.8, // 80% base success for crafting
        }];

        let mut conditions = InteractionConditions::default();
        conditions.required_theories = required_theories;
        conditions.energy_cost = energy_cost;

        Self {
            id,
            inputs,
            outputs,
            conditions,
            interaction_type: InteractionType::Crafting,
        }
    }

    /// Create an enhancement interaction
    pub fn new_enhancement(
        id: String,
        target_item: ItemId,
        catalyst_items: Vec<(ItemId, i32)>,
        enhancement: ItemEnhancement,
        success_chance: f32,
    ) -> Self {
        let mut inputs = vec![InteractionInput {
            item_requirement: ItemRequirement::SpecificItem(target_item),
            quantity: 1,
            consumed: false, // Target item is modified, not consumed
        }];

        inputs.extend(catalyst_items.into_iter().map(|(item_id, qty)| InteractionInput {
            item_requirement: ItemRequirement::SpecificItem(item_id),
            quantity: qty,
            consumed: true,
        }));

        // Enhancement doesn't produce new items but modifies existing ones
        let outputs = vec![];

        Self {
            id,
            inputs,
            outputs,
            conditions: InteractionConditions::default(),
            interaction_type: InteractionType::Enhancement,
        }
    }

    /// Check if interaction can be performed with available items
    pub fn can_perform(
        &self,
        available_items: &HashMap<ItemId, (Item, i32)>,
        player_attributes: &HashMap<String, i32>,
        player_theories: &[String],
    ) -> bool {
        // Check input requirements
        for input in &self.inputs {
            if !self.check_input_availability(input, available_items) {
                return false;
            }
        }

        // Check conditions
        self.conditions.are_met(player_attributes, player_theories)
    }

    /// Perform the interaction
    pub fn perform(
        &self,
        available_items: &mut HashMap<ItemId, (Item, i32)>,
        player_attributes: &HashMap<String, i32>,
        player_theories: &[String],
    ) -> GameResult<InteractionResult> {
        if !self.can_perform(available_items, player_attributes, player_theories) {
            return Err(crate::GameError::InvalidInput(
                "Cannot perform interaction: requirements not met".to_string()
            ).into());
        }

        let mut result = InteractionResult {
            success: false,
            outputs: Vec::new(),
            consumed_items: Vec::new(),
            experience_gained: 0,
            description: String::new(),
            side_effects: Vec::new(),
        };

        // Calculate success probability
        let base_success = self.calculate_success_probability(player_attributes, player_theories);
        let random_roll = rand::random::<f32>();

        result.success = random_roll <= base_success;

        if result.success {
            // Consume input items
            for input in &self.inputs {
                if input.consumed {
                    self.consume_input(input, available_items, &mut result)?;
                }
            }

            // Produce output items
            for output in &self.outputs {
                if rand::random::<f32>() <= output.success_chance {
                    for _ in 0..output.quantity {
                        result.outputs.push(output.item.clone());
                    }
                }
            }

            // Calculate experience
            result.experience_gained = self.calculate_experience_reward();
            result.description = format!("Successfully performed {}", self.id);
        } else {
            result.description = format!("Failed to perform {}", self.id);

            // Partial consumption on failure for some interaction types
            if matches!(self.interaction_type, InteractionType::Synthesis | InteractionType::Ritual) {
                for input in &self.inputs {
                    if input.consumed && rand::random::<f32>() < 0.3 {
                        // 30% chance to consume materials even on failure
                        self.consume_input(input, available_items, &mut result)?;
                    }
                }
            }
        }

        Ok(result)
    }

    fn check_input_availability(
        &self,
        input: &InteractionInput,
        available_items: &HashMap<ItemId, (Item, i32)>,
    ) -> bool {
        match &input.item_requirement {
            ItemRequirement::SpecificItem(item_id) => {
                if let Some((_, quantity)) = available_items.get(item_id) {
                    *quantity >= input.quantity
                } else {
                    false
                }
            }
            ItemRequirement::ItemType(item_type) => {
                let mut total_found = 0;
                for (_, (item, quantity)) in available_items {
                    if std::mem::discriminant(&item.item_type) == std::mem::discriminant(item_type) {
                        total_found += quantity;
                        if total_found >= input.quantity {
                            return true;
                        }
                    }
                }
                false
            }
            ItemRequirement::ItemWithProperties { .. } => {
                // Simplified property matching
                true // Would implement detailed property checking
            }
            ItemRequirement::CrystalWithProperties { .. } => {
                // Simplified crystal property matching
                true // Would implement detailed crystal property checking
            }
        }
    }

    fn consume_input(
        &self,
        input: &InteractionInput,
        available_items: &mut HashMap<ItemId, (Item, i32)>,
        result: &mut InteractionResult,
    ) -> GameResult<()> {
        match &input.item_requirement {
            ItemRequirement::SpecificItem(item_id) => {
                if let Some((item, quantity)) = available_items.get_mut(item_id) {
                    if *quantity >= input.quantity {
                        *quantity -= input.quantity;
                        result.consumed_items.push(item_id.clone());

                        if *quantity == 0 {
                            available_items.remove(item_id);
                        }
                    }
                }
            }
            _ => {
                // Would implement consumption for other requirement types
            }
        }
        Ok(())
    }

    fn calculate_success_probability(
        &self,
        player_attributes: &HashMap<String, i32>,
        player_theories: &[String],
    ) -> f32 {
        let mut base_prob = 0.5; // 50% base success rate

        // Adjust based on player attributes
        if let Some(&mental_acuity) = player_attributes.get("mental_acuity") {
            base_prob += (mental_acuity as f32 - 50.0) * 0.005; // +0.5% per point above 50
        }

        // Adjust based on known theories
        let theory_bonus = self.conditions.required_theories.len() as f32 * 0.1;
        base_prob += theory_bonus;

        // Clamp between 0.1 and 0.95
        base_prob.max(0.1).min(0.95)
    }

    fn calculate_experience_reward(&self) -> i32 {
        match self.interaction_type {
            InteractionType::Combination => 10,
            InteractionType::Crafting => 25,
            InteractionType::Enhancement => 15,
            InteractionType::Repair => 5,
            InteractionType::Transmutation => 50,
            InteractionType::Synthesis => 75,
            InteractionType::Ritual => 100,
        }
    }
}

impl InteractionConditions {
    /// Create default conditions (no requirements)
    pub fn default() -> Self {
        Self {
            min_attributes: HashMap::new(),
            required_theories: Vec::new(),
            required_environment: None,
            required_tools: Vec::new(),
            energy_cost: 0,
            time_cost: 0,
        }
    }

    /// Check if conditions are met
    pub fn are_met(
        &self,
        player_attributes: &HashMap<String, i32>,
        player_theories: &[String],
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

        true
    }
}

/// Factory for common item interactions
pub struct InteractionFactory;

impl InteractionFactory {
    /// Create crystal enhancement interaction
    pub fn crystal_enhancement() -> ItemInteraction {
        ItemInteraction::new_enhancement(
            "crystal_enhancement".to_string(),
            "crystal_target".to_string(),
            vec![
                ("purity_reagent".to_string(), 1),
                ("stabilizing_compound".to_string(), 1),
            ],
            ItemEnhancement {
                durability_change: Some(10),
                value_multiplier: Some(1.5),
                added_effects: vec![],
                property_changes: HashMap::new(),
            },
            0.7,
        )
    }

    /// Create basic tool crafting
    pub fn tool_crafting() -> ItemInteraction {
        ItemInteraction::new_crafting(
            "basic_tool_craft".to_string(),
            vec![
                (ItemType::Material { material_type: "metal".to_string(), quality: 0.7 }, 2),
                (ItemType::Material { material_type: "wood".to_string(), quality: 0.5 }, 1),
            ],
            Item::new_tool(
                "Basic Measuring Tool".to_string(),
                "A simple resonance measuring device".to_string(),
                "resonance_measurement".to_string(),
            ),
            vec!["harmonic_fundamentals".to_string()],
            20,
        )
    }

    /// Create potion brewing interaction
    pub fn potion_brewing() -> ItemInteraction {
        ItemInteraction::new_combination(
            "energy_potion_brew".to_string(),
            vec![
                ("herb_energicum".to_string(), 2),
                ("crystal_dust".to_string(), 1),
                ("pure_water".to_string(), 1),
            ],
            Item::new_consumable(
                "Energy Restoration Potion".to_string(),
                "Restores mental energy".to_string(),
                ItemEffect::RestoreEnergy(30),
                3,
            ),
            0.8,
        )
    }

    /// Create research apparatus synthesis
    pub fn research_apparatus_synthesis() -> ItemInteraction {
        let mut interaction = ItemInteraction::new_crafting(
            "research_apparatus".to_string(),
            vec![
                (ItemType::Material { material_type: "precision_crystal".to_string(), quality: 0.9 }, 1),
                (ItemType::Material { material_type: "rare_metal".to_string(), quality: 0.8 }, 3),
                (ItemType::Book { theory_id: "advanced_resonance".to_string() }, 1),
            ],
            Item::new_basic(
                "Advanced Research Apparatus".to_string(),
                "Enables cutting-edge magical research".to_string(),
                ItemType::Educational(
                    super::educational::EducationalItem::new_research_tool(
                        "Advanced Research Apparatus".to_string(),
                        "advanced_laboratory".to_string(),
                        "resonance_amplification".to_string(),
                        0.8,
                        1.5,
                    )
                ),
            ),
            vec![
                "harmonic_fundamentals".to_string(),
                "crystal_structures".to_string(),
                "resonance_amplification".to_string(),
            ],
            50,
        );

        interaction.conditions.min_attributes.insert("mental_acuity".to_string(), 70);
        interaction.conditions.time_cost = 240; // 4 hours
        interaction.interaction_type = InteractionType::Synthesis;

        interaction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_simple_combination() {
        let interaction = InteractionFactory::potion_brewing();

        let mut available_items = HashMap::new();
        available_items.insert(
            "herb_energicum".to_string(),
            (Item::new_basic("Herb".to_string(), "Energy herb".to_string(), ItemType::Mundane), 3)
        );
        available_items.insert(
            "crystal_dust".to_string(),
            (Item::new_basic("Dust".to_string(), "Crystal dust".to_string(), ItemType::Mundane), 1)
        );
        available_items.insert(
            "pure_water".to_string(),
            (Item::new_basic("Water".to_string(), "Pure water".to_string(), ItemType::Mundane), 1)
        );

        let player_attributes = HashMap::new();
        let player_theories = vec![];

        assert!(interaction.can_perform(&available_items, &player_attributes, &player_theories));
    }

    #[test]
    fn test_crafting_requirements() {
        let interaction = InteractionFactory::tool_crafting();

        let player_attributes = HashMap::new();
        let player_theories = vec!["harmonic_fundamentals".to_string()];

        // Should require theory knowledge
        let empty_theories = vec![];
        let available_items = HashMap::new();

        assert!(!interaction.can_perform(&available_items, &player_attributes, &empty_theories));
        assert!(interaction.conditions.are_met(&player_attributes, &player_theories));
    }

    #[test]
    fn test_enhancement_interaction() {
        let interaction = InteractionFactory::crystal_enhancement();

        assert_eq!(interaction.interaction_type, InteractionType::Enhancement);
        assert_eq!(interaction.inputs.len(), 3); // target + 2 catalysts
        assert_eq!(interaction.outputs.len(), 0); // Enhancement doesn't produce new items
    }

    #[test]
    fn test_interaction_conditions() {
        let mut conditions = InteractionConditions::default();
        conditions.min_attributes.insert("mental_acuity".to_string(), 50);
        conditions.required_theories.push("test_theory".to_string());

        let mut player_attributes = HashMap::new();
        player_attributes.insert("mental_acuity".to_string(), 60);
        let player_theories = vec!["test_theory".to_string()];

        assert!(conditions.are_met(&player_attributes, &player_theories));

        // Fail attribute check
        player_attributes.insert("mental_acuity".to_string(), 40);
        assert!(!conditions.are_met(&player_attributes, &player_theories));

        // Fail theory check
        player_attributes.insert("mental_acuity".to_string(), 60);
        let empty_theories = vec![];
        assert!(!conditions.are_met(&player_attributes, &empty_theories));
    }
}