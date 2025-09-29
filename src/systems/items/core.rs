//! Core item definitions and properties
//!
//! This module defines the fundamental item structures including:
//! - Base Item struct with comprehensive properties
//! - Item types and categories
//! - Item effects and magical properties
//! - Rarity system and value calculations

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Unique identifier for items
pub type ItemId = String;

/// Core item structure with comprehensive properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    /// Unique identifier
    pub id: ItemId,
    /// Core properties
    pub properties: ItemProperties,
    /// Item type and specific functionality
    pub item_type: ItemType,
    /// Magical properties if any
    pub magical_properties: Option<MagicalProperties>,
}

/// Essential item properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemProperties {
    /// Display name
    pub name: String,
    /// Detailed description
    pub description: String,
    /// Weight in kilograms
    pub weight: f32,
    /// Value in silver pieces
    pub value: i32,
    /// Current durability
    pub durability: i32,
    /// Maximum durability
    pub max_durability: i32,
    /// Item rarity level
    pub rarity: ItemRarity,
    /// Additional custom properties
    pub custom_properties: HashMap<String, String>,
}

/// Item rarity classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ItemRarity {
    /// Common everyday items
    Common,
    /// Uncommon but not rare
    Uncommon,
    /// Rare and valuable
    Rare,
    /// Very rare, exceptional items
    Epic,
    /// Legendary artifacts with unique properties
    Legendary,
}

/// Item type determining functionality and usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemType {
    /// Basic item with no special functionality
    Mundane,

    /// Consumable items with limited uses
    Consumable {
        /// Effect when consumed
        effect: ItemEffect,
        /// Remaining uses
        uses_remaining: i32,
    },

    /// Equipment that can be worn/wielded
    Equipment(super::equipment::Equipment),

    /// Tools with specific functions
    Tool {
        /// Tool's primary function
        tool_function: String,
    },

    /// Books containing theory knowledge
    Book {
        /// Theory this book teaches
        theory_id: String,
    },

    /// Artifacts with unique magical properties
    Artifact {
        /// Special properties description
        properties: String,
    },

    /// Quest-specific items
    QuestItem {
        /// Associated quest ID
        quest_id: String,
        /// Whether this is a key item that cannot be discarded
        is_key_item: bool,
    },

    /// Educational items with learning benefits
    Educational(super::educational::EducationalItem),

    /// Currency and trade items
    Currency {
        /// Currency type (silver, gold, etc.)
        currency_type: String,
        /// Amount
        amount: i32,
    },

    /// Raw materials for crafting
    Material {
        /// Material type
        material_type: String,
        /// Quality grade
        quality: f32,
    },
}

/// Effects that items can have when used
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemEffect {
    /// Restore mental energy
    RestoreEnergy(i32),

    /// Reduce fatigue
    ReduceFatigue(i32),

    /// Temporary attribute boost
    TemporaryAttributeBoost {
        attribute: String,
        amount: i32,
        duration: i32, // in minutes
    },

    /// Teach or enhance theory understanding
    LearnTheory {
        theory_id: String,
        understanding_boost: f32,
    },

    /// Heal physical damage (for future health system)
    HealDamage(i32),

    /// Grant temporary magical ability
    TemporarySpell {
        spell_type: String,
        duration: i32,
    },

    /// Improve crystal properties
    EnhanceCrystal {
        property: String,
        amount: f32,
    },

    /// Multiple effects
    Multiple(Vec<ItemEffect>),
}

/// Magical properties for enchanted items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagicalProperties {
    /// Magical aura strength
    pub aura_strength: f32,
    /// Resonance frequency if applicable
    pub resonance_frequency: Option<i32>,
    /// Magical school/type
    pub magic_school: String,
    /// Enchantment effects
    pub enchantments: Vec<MagicalEnchantment>,
    /// Magical energy stored in item
    pub stored_energy: i32,
    /// Maximum energy capacity
    pub max_energy: i32,
}

/// Individual magical enchantment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagicalEnchantment {
    /// Enchantment type
    pub enchantment_type: String,
    /// Effect strength (0.0-1.0)
    pub strength: f32,
    /// Duration in minutes (-1 for permanent)
    pub duration: i32,
    /// Conditions for activation
    pub activation_conditions: Vec<String>,
}

impl Item {
    /// Create a new basic item
    pub fn new_basic(name: String, description: String, item_type: ItemType) -> Self {
        let id = format!("item_{}", Uuid::new_v4().to_string()[..8].to_lowercase());

        let (weight, value, durability) = match &item_type {
            ItemType::Mundane => (0.5, 1, 100),
            ItemType::Consumable { .. } => (0.2, 10, 1),
            ItemType::Equipment(_) => (2.0, 50, 200),
            ItemType::Tool { .. } => (1.0, 25, 150),
            ItemType::Book { .. } => (0.8, 100, 50),
            ItemType::Artifact { .. } => (1.5, 500, 300),
            ItemType::QuestItem { .. } => (0.5, 0, 100),
            ItemType::Educational(_) => (1.2, 200, 100),
            ItemType::Currency { .. } => (0.01, 1, 1),
            ItemType::Material { .. } => (0.3, 5, 50),
        };

        Self {
            id,
            properties: ItemProperties {
                name,
                description,
                weight,
                value,
                durability,
                max_durability: durability,
                rarity: ItemRarity::Common,
                custom_properties: HashMap::new(),
            },
            item_type,
            magical_properties: None,
        }
    }

    /// Create a new magical item
    pub fn new_magical(
        name: String,
        description: String,
        item_type: ItemType,
        magical_properties: MagicalProperties,
    ) -> Self {
        let mut item = Self::new_basic(name, description, item_type);
        item.magical_properties = Some(magical_properties);
        item.properties.rarity = ItemRarity::Rare; // Magical items are at least rare
        item.properties.value *= 5; // Magical items are more valuable
        item
    }

    /// Create a quest item
    pub fn new_quest_item(name: String, description: String, quest_id: String, is_key_item: bool) -> Self {
        Self::new_basic(
            name,
            description,
            ItemType::QuestItem { quest_id, is_key_item },
        )
    }

    /// Create a consumable item
    pub fn new_consumable(name: String, description: String, effect: ItemEffect, uses: i32) -> Self {
        Self::new_basic(
            name,
            description,
            ItemType::Consumable {
                effect,
                uses_remaining: uses,
            },
        )
    }

    /// Create a tool
    pub fn new_tool(name: String, description: String, function: String) -> Self {
        Self::new_basic(
            name,
            description,
            ItemType::Tool {
                tool_function: function,
            },
        )
    }

    /// Create a book
    pub fn new_book(name: String, description: String, theory_id: String) -> Self {
        Self::new_basic(
            name,
            description,
            ItemType::Book { theory_id },
        )
    }

    /// Check if item is usable (not broken)
    pub fn is_usable(&self) -> bool {
        self.properties.durability > 0
    }

    /// Check if item is magical
    pub fn is_magical(&self) -> bool {
        self.magical_properties.is_some()
    }

    /// Get effective value considering condition
    pub fn effective_value(&self) -> i32 {
        let condition_factor = self.properties.durability as f32 / self.properties.max_durability as f32;
        (self.properties.value as f32 * condition_factor) as i32
    }

    /// Reduce durability from use or damage
    pub fn damage(&mut self, amount: i32) {
        self.properties.durability = (self.properties.durability - amount).max(0);
    }

    /// Repair item
    pub fn repair(&mut self, amount: i32) {
        self.properties.durability = (self.properties.durability + amount).min(self.properties.max_durability);
    }

    /// Set custom property
    pub fn set_custom_property(&mut self, key: String, value: String) {
        self.properties.custom_properties.insert(key, value);
    }

    /// Get custom property
    pub fn get_custom_property(&self, key: &str) -> Option<&String> {
        self.properties.custom_properties.get(key)
    }

    /// Calculate weight multiplier based on size if applicable
    pub fn weight_multiplier(&self) -> f32 {
        // Could be enhanced based on size properties
        1.0
    }

    /// Check if item can stack with another
    pub fn can_stack_with(&self, other: &Item) -> bool {
        // Basic stacking rules - same type, name, and properties
        self.properties.name == other.properties.name &&
        std::mem::discriminant(&self.item_type) == std::mem::discriminant(&other.item_type) &&
        self.magical_properties.is_none() && other.magical_properties.is_none()
    }

    /// Get rarity multiplier for various calculations
    pub fn rarity_multiplier(&self) -> f32 {
        match self.properties.rarity {
            ItemRarity::Common => 1.0,
            ItemRarity::Uncommon => 1.5,
            ItemRarity::Rare => 2.5,
            ItemRarity::Epic => 5.0,
            ItemRarity::Legendary => 10.0,
        }
    }

    /// Get item category for organization
    pub fn category(&self) -> ItemCategory {
        match &self.item_type {
            ItemType::Mundane => ItemCategory::Miscellaneous,
            ItemType::Consumable { .. } => ItemCategory::Consumables,
            ItemType::Equipment(_) => ItemCategory::Equipment,
            ItemType::Tool { .. } => ItemCategory::Tools,
            ItemType::Book { .. } => ItemCategory::Books,
            ItemType::Artifact { .. } => ItemCategory::Artifacts,
            ItemType::QuestItem { .. } => ItemCategory::Quest,
            ItemType::Educational(_) => ItemCategory::Educational,
            ItemType::Currency { .. } => ItemCategory::Currency,
            ItemType::Material { .. } => ItemCategory::Materials,
        }
    }
}

/// Item categories for organization
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ItemCategory {
    Equipment,
    Consumables,
    Tools,
    Books,
    Artifacts,
    Quest,
    Educational,
    Currency,
    Materials,
    Miscellaneous,
}

impl ItemRarity {
    /// Get color code for display
    pub fn color_code(&self) -> &'static str {
        match self {
            ItemRarity::Common => "white",
            ItemRarity::Uncommon => "green",
            ItemRarity::Rare => "blue",
            ItemRarity::Epic => "purple",
            ItemRarity::Legendary => "orange",
        }
    }

    /// Get value multiplier
    pub fn value_multiplier(&self) -> f32 {
        match self {
            ItemRarity::Common => 1.0,
            ItemRarity::Uncommon => 2.0,
            ItemRarity::Rare => 5.0,
            ItemRarity::Epic => 15.0,
            ItemRarity::Legendary => 50.0,
        }
    }
}

impl ItemEffect {
    /// Check if effect is beneficial
    pub fn is_beneficial(&self) -> bool {
        match self {
            ItemEffect::RestoreEnergy(_) => true,
            ItemEffect::ReduceFatigue(_) => true,
            ItemEffect::TemporaryAttributeBoost { .. } => true,
            ItemEffect::LearnTheory { .. } => true,
            ItemEffect::HealDamage(_) => true,
            ItemEffect::TemporarySpell { .. } => true,
            ItemEffect::EnhanceCrystal { .. } => true,
            ItemEffect::Multiple(effects) => effects.iter().any(|e| e.is_beneficial()),
        }
    }

    /// Get effect description for display
    pub fn description(&self) -> String {
        match self {
            ItemEffect::RestoreEnergy(amount) => format!("Restores {} mental energy", amount),
            ItemEffect::ReduceFatigue(amount) => format!("Reduces fatigue by {}", amount),
            ItemEffect::TemporaryAttributeBoost { attribute, amount, duration } => {
                format!("Increases {} by {} for {} minutes", attribute, amount, duration)
            }
            ItemEffect::LearnTheory { theory_id, understanding_boost } => {
                format!("Teaches {} (understanding +{:.1}%)", theory_id, understanding_boost * 100.0)
            }
            ItemEffect::HealDamage(amount) => format!("Heals {} damage", amount),
            ItemEffect::TemporarySpell { spell_type, duration } => {
                format!("Grants {} spell for {} minutes", spell_type, duration)
            }
            ItemEffect::EnhanceCrystal { property, amount } => {
                format!("Improves crystal {} by {:.2}", property, amount)
            }
            ItemEffect::Multiple(effects) => {
                let descriptions: Vec<String> = effects.iter().map(|e| e.description()).collect();
                descriptions.join("; ")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_creation() {
        let item = Item::new_basic(
            "Test Item".to_string(),
            "A test item".to_string(),
            ItemType::Mundane,
        );

        assert_eq!(item.properties.name, "Test Item");
        assert_eq!(item.properties.rarity, ItemRarity::Common);
        assert!(item.is_usable());
        assert!(!item.is_magical());
    }

    #[test]
    fn test_consumable_creation() {
        let effect = ItemEffect::RestoreEnergy(25);
        let item = Item::new_consumable(
            "Energy Potion".to_string(),
            "Restores mental energy".to_string(),
            effect,
            3,
        );

        if let ItemType::Consumable { uses_remaining, .. } = item.item_type {
            assert_eq!(uses_remaining, 3);
        } else {
            panic!("Expected consumable item type");
        }
    }

    #[test]
    fn test_item_damage_and_repair() {
        let mut item = Item::new_basic(
            "Fragile Item".to_string(),
            "Breaks easily".to_string(),
            ItemType::Mundane,
        );

        let initial_durability = item.properties.durability;

        item.damage(50);
        assert_eq!(item.properties.durability, initial_durability - 50);
        assert!(item.is_usable());

        item.repair(25);
        assert_eq!(item.properties.durability, initial_durability - 25);

        item.damage(200); // Excessive damage
        assert_eq!(item.properties.durability, 0);
        assert!(!item.is_usable());
    }

    #[test]
    fn test_rarity_multipliers() {
        assert_eq!(ItemRarity::Common.value_multiplier(), 1.0);
        assert_eq!(ItemRarity::Legendary.value_multiplier(), 50.0);
    }

    #[test]
    fn test_effect_descriptions() {
        let energy_effect = ItemEffect::RestoreEnergy(30);
        assert!(energy_effect.description().contains("30"));
        assert!(energy_effect.is_beneficial());

        let multiple_effect = ItemEffect::Multiple(vec![
            ItemEffect::RestoreEnergy(15),
            ItemEffect::ReduceFatigue(10),
        ]);
        assert!(multiple_effect.description().contains(";"));
        assert!(multiple_effect.is_beneficial());
    }

    #[test]
    fn test_custom_properties() {
        let mut item = Item::new_basic(
            "Custom Item".to_string(),
            "Has custom properties".to_string(),
            ItemType::Mundane,
        );

        item.set_custom_property("creator".to_string(), "Test Creator".to_string());
        item.set_custom_property("magical_school".to_string(), "Evocation".to_string());

        assert_eq!(item.get_custom_property("creator"), Some(&"Test Creator".to_string()));
        assert_eq!(item.get_custom_property("magical_school"), Some(&"Evocation".to_string()));
        assert_eq!(item.get_custom_property("nonexistent"), None);
    }
}