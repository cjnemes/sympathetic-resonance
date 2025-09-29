//! Inventory management system with constraints and organization
//!
//! This module provides:
//! - Weight and space-based inventory limits
//! - Item organization and sorting
//! - Stack management for similar items
//! - Search and filtering capabilities

use super::core::{Item, ItemId, ItemCategory};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::GameResult;

/// Comprehensive inventory management system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryManager {
    /// All items stored by ID
    pub items: HashMap<ItemId, Item>,
    /// Item stacks for similar items
    pub stacks: HashMap<ItemId, i32>,
    /// Inventory constraints
    pub constraints: InventoryConstraints,
    /// Organization settings
    pub organization: InventoryOrganization,
}

/// Inventory size and weight constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryConstraints {
    /// Maximum weight in kilograms
    pub max_weight: f32,
    /// Maximum number of item stacks
    pub max_slots: i32,
    /// Category-specific limits
    pub category_limits: HashMap<ItemCategory, i32>,
    /// Special restrictions
    pub restrictions: Vec<InventoryRestriction>,
}

/// Inventory organization and sorting preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryOrganization {
    /// How items are sorted
    pub sort_method: SortMethod,
    /// Whether to group by category
    pub group_by_category: bool,
    /// Favorite items (pinned to top)
    pub favorites: Vec<ItemId>,
    /// Hidden categories
    pub hidden_categories: Vec<ItemCategory>,
}

/// Methods for sorting inventory items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortMethod {
    /// Sort by item name alphabetically
    Name,
    /// Sort by item type/category
    Category,
    /// Sort by item value
    Value,
    /// Sort by item weight
    Weight,
    /// Sort by rarity
    Rarity,
    /// Sort by recently acquired
    Recent,
    /// Custom sort order
    Custom(Vec<ItemId>),
}

/// Inventory restrictions for special items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InventoryRestriction {
    /// Unique items (only one allowed)
    UniqueItems(Vec<String>),
    /// Faction-restricted items
    FactionRestricted { faction: String, min_reputation: i32 },
    /// Level-restricted items
    LevelRestricted { min_level: i32 },
    /// Theory-restricted items
    TheoryRestricted { required_theories: Vec<String> },
}

/// Errors that can occur during inventory operations
#[derive(Debug, Clone)]
pub enum InventoryError {
    /// Inventory is full (weight or slots)
    InsufficientSpace(String),
    /// Item not found
    ItemNotFound(ItemId),
    /// Cannot stack item
    CannotStack(String),
    /// Restriction violated
    RestrictionViolated(String),
    /// Invalid operation
    InvalidOperation(String),
}

impl std::fmt::Display for InventoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InventoryError::InsufficientSpace(msg) => write!(f, "Insufficient space: {}", msg),
            InventoryError::ItemNotFound(id) => write!(f, "Item not found: {}", id),
            InventoryError::CannotStack(msg) => write!(f, "Cannot stack: {}", msg),
            InventoryError::RestrictionViolated(msg) => write!(f, "Restriction violated: {}", msg),
            InventoryError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
        }
    }
}

impl std::error::Error for InventoryError {}

impl InventoryManager {
    /// Create new inventory manager with default constraints
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            stacks: HashMap::new(),
            constraints: InventoryConstraints::default(),
            organization: InventoryOrganization::default(),
        }
    }

    /// Create inventory manager with custom constraints
    pub fn with_constraints(constraints: InventoryConstraints) -> Self {
        Self {
            items: HashMap::new(),
            stacks: HashMap::new(),
            constraints,
            organization: InventoryOrganization::default(),
        }
    }

    /// Add an item to inventory
    pub fn add_item(&mut self, item: Item) -> GameResult<()> {
        // Check if item can be added
        self.validate_addition(&item)?;

        let item_id = item.id.clone();

        // Try to stack with existing item if possible
        if let Some(existing_id) = self.find_stackable_item(&item) {
            let current_stack = self.stacks.get(&existing_id).copied().unwrap_or(1);
            self.stacks.insert(existing_id, current_stack + 1);
            return Ok(());
        }

        // Add as new item
        self.items.insert(item_id.clone(), item);
        self.stacks.insert(item_id, 1);

        Ok(())
    }

    /// Remove an item from inventory
    pub fn remove_item(&mut self, item_id: &ItemId) -> GameResult<Option<Item>> {
        if let Some(stack_size) = self.stacks.get(item_id).copied() {
            if stack_size > 1 {
                // Reduce stack size
                self.stacks.insert(item_id.clone(), stack_size - 1);
                // Return a clone of the item
                if let Some(item) = self.items.get(item_id) {
                    Ok(Some(item.clone()))
                } else {
                    Ok(None)
                }
            } else {
                // Remove completely
                self.stacks.remove(item_id);
                Ok(self.items.remove(item_id))
            }
        } else {
            Err(InventoryError::ItemNotFound(item_id.clone()).into())
        }
    }

    /// Remove multiple items of the same type
    pub fn remove_items(&mut self, item_id: &ItemId, count: i32) -> GameResult<Vec<Item>> {
        let current_stack = self.stacks.get(item_id).copied().unwrap_or(0);

        if current_stack < count {
            return Err(InventoryError::InvalidOperation(
                format!("Not enough items: have {}, need {}", current_stack, count)
            ).into());
        }

        let mut removed_items = Vec::new();

        if let Some(item) = self.items.get(item_id) {
            for _ in 0..count {
                removed_items.push(item.clone());
            }
        }

        let new_stack = current_stack - count;
        if new_stack <= 0 {
            self.stacks.remove(item_id);
            self.items.remove(item_id);
        } else {
            self.stacks.insert(item_id.clone(), new_stack);
        }

        Ok(removed_items)
    }

    /// Get an item by ID
    pub fn get_item(&self, item_id: &ItemId) -> Option<&Item> {
        self.items.get(item_id)
    }

    /// Get all items in inventory
    pub fn get_all_items(&self) -> Vec<&Item> {
        self.items.values().collect()
    }

    /// Get items by category
    pub fn get_items_by_category(&self, category: ItemCategory) -> Vec<&Item> {
        self.items
            .values()
            .filter(|item| item.category() == category)
            .collect()
    }

    /// Check if inventory contains an item
    pub fn has_item(&self, item_id: &ItemId) -> bool {
        self.items.contains_key(item_id)
    }

    /// Get stack size for an item
    pub fn get_stack_size(&self, item_id: &ItemId) -> i32 {
        self.stacks.get(item_id).copied().unwrap_or(0)
    }

    /// Calculate current total weight
    pub fn current_weight(&self) -> f32 {
        self.items
            .iter()
            .map(|(id, item)| {
                let stack_size = self.stacks.get(id).copied().unwrap_or(1) as f32;
                item.properties.weight * stack_size
            })
            .sum()
    }

    /// Calculate current slot usage
    pub fn current_slots(&self) -> i32 {
        self.stacks.len() as i32
    }

    /// Get remaining weight capacity
    pub fn remaining_weight(&self) -> f32 {
        self.constraints.max_weight - self.current_weight()
    }

    /// Get remaining slot capacity
    pub fn remaining_slots(&self) -> i32 {
        self.constraints.max_slots - self.current_slots()
    }

    /// Validate if an item can be added
    pub fn validate_addition(&self, item: &Item) -> Result<(), InventoryError> {
        // Check weight constraint
        if self.current_weight() + item.properties.weight > self.constraints.max_weight {
            return Err(InventoryError::InsufficientSpace(
                format!("Weight limit exceeded: {} + {} > {}",
                    self.current_weight(), item.properties.weight, self.constraints.max_weight)
            ));
        }

        // Check slot constraint (only if item can't be stacked)
        if self.find_stackable_item(item).is_none() {
            if self.current_slots() >= self.constraints.max_slots {
                return Err(InventoryError::InsufficientSpace(
                    "No more inventory slots available".to_string()
                ));
            }
        }

        // Check category limits
        let category = item.category();
        if let Some(&limit) = self.constraints.category_limits.get(&category) {
            let current_count = self.get_items_by_category(category.clone()).len() as i32;
            if current_count >= limit {
                return Err(InventoryError::InsufficientSpace(
                    format!("Category limit reached for {:?}: {}/{}", category, current_count, limit)
                ));
            }
        }

        // Check restrictions
        for restriction in &self.constraints.restrictions {
            self.check_restriction(item, restriction)?;
        }

        Ok(())
    }

    /// Find a stackable item of the same type
    fn find_stackable_item(&self, item: &Item) -> Option<ItemId> {
        for (id, existing_item) in &self.items {
            if item.can_stack_with(existing_item) {
                return Some(id.clone());
            }
        }
        None
    }

    /// Check if item violates a restriction
    fn check_restriction(&self, item: &Item, restriction: &InventoryRestriction) -> Result<(), InventoryError> {
        match restriction {
            InventoryRestriction::UniqueItems(unique_names) => {
                if unique_names.contains(&item.properties.name) {
                    // Check if we already have this unique item
                    for existing_item in self.items.values() {
                        if existing_item.properties.name == item.properties.name {
                            return Err(InventoryError::RestrictionViolated(
                                format!("Unique item already in inventory: {}", item.properties.name)
                            ));
                        }
                    }
                }
            }
            InventoryRestriction::FactionRestricted { .. } => {
                // Would need player faction data to check
                // For now, assume restriction is met
            }
            InventoryRestriction::LevelRestricted { .. } => {
                // Would need player level data to check
                // For now, assume restriction is met
            }
            InventoryRestriction::TheoryRestricted { .. } => {
                // Would need player theory data to check
                // For now, assume restriction is met
            }
        }
        Ok(())
    }

    /// Search for items by name
    pub fn search_by_name(&self, query: &str) -> Vec<&Item> {
        let query_lower = query.to_lowercase();
        self.items
            .values()
            .filter(|item| item.properties.name.to_lowercase().contains(&query_lower))
            .collect()
    }

    /// Search for items by type or description
    pub fn search_by_description(&self, query: &str) -> Vec<&Item> {
        let query_lower = query.to_lowercase();
        self.items
            .values()
            .filter(|item| {
                item.properties.description.to_lowercase().contains(&query_lower) ||
                format!("{:?}", item.item_type).to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Get sorted item list according to organization settings
    pub fn get_sorted_items(&self) -> Vec<(&Item, i32)> {
        let mut items: Vec<(&Item, i32)> = self.items
            .iter()
            .map(|(id, item)| (item, self.stacks.get(id).copied().unwrap_or(1)))
            .collect();

        // Apply sorting
        match &self.organization.sort_method {
            SortMethod::Name => {
                items.sort_by(|a, b| a.0.properties.name.cmp(&b.0.properties.name));
            }
            SortMethod::Category => {
                items.sort_by(|a, b| {
                    let cat_a = a.0.category();
                    let cat_b = b.0.category();
                    cat_a.partial_cmp(&cat_b).unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            SortMethod::Value => {
                items.sort_by(|a, b| b.0.properties.value.cmp(&a.0.properties.value));
            }
            SortMethod::Weight => {
                items.sort_by(|a, b| a.0.properties.weight.partial_cmp(&b.0.properties.weight).unwrap_or(std::cmp::Ordering::Equal));
            }
            SortMethod::Rarity => {
                items.sort_by(|a, b| {
                    let rarity_order = |r: &super::core::ItemRarity| match r {
                        super::core::ItemRarity::Common => 0,
                        super::core::ItemRarity::Uncommon => 1,
                        super::core::ItemRarity::Rare => 2,
                        super::core::ItemRarity::Epic => 3,
                        super::core::ItemRarity::Legendary => 4,
                    };
                    rarity_order(&b.0.properties.rarity).cmp(&rarity_order(&a.0.properties.rarity))
                });
            }
            SortMethod::Recent => {
                // Would need timestamp tracking for true recent sorting
                // For now, maintain insertion order
            }
            SortMethod::Custom(_) => {
                // Would implement custom sorting based on the provided order
            }
        }

        items
    }

    /// Reduce uses of a consumable item
    pub fn reduce_item_uses(&mut self, item_id: &ItemId) -> GameResult<()> {
        if let Some(item) = self.items.get_mut(item_id) {
            if let super::core::ItemType::Consumable { uses_remaining, .. } = &mut item.item_type {
                *uses_remaining -= 1;

                if *uses_remaining <= 0 {
                    // Remove item when depleted
                    self.remove_item(item_id)?;
                }
            }
        }
        Ok(())
    }

    /// Get inventory summary string
    pub fn get_summary(&self) -> String {
        let mut summary = String::new();
        summary.push_str(&format!("Inventory: {}/{} slots, {:.1}/{:.1} kg\n",
            self.current_slots(), self.constraints.max_slots,
            self.current_weight(), self.constraints.max_weight));

        if self.items.is_empty() {
            summary.push_str("Empty");
            return summary;
        }

        let sorted_items = self.get_sorted_items();
        for (item, stack_size) in sorted_items {
            let stack_info = if stack_size > 1 {
                format!(" ({})", stack_size)
            } else {
                String::new()
            };

            summary.push_str(&format!("  - {}{}\n", item.properties.name, stack_info));
        }

        summary
    }

    /// Get detailed inventory report
    pub fn get_detailed_report(&self) -> String {
        let mut report = String::new();
        report.push_str(&format!("=== Inventory Report ===\n"));
        report.push_str(&format!("Capacity: {}/{} slots, {:.1}/{:.1} kg\n\n",
            self.current_slots(), self.constraints.max_slots,
            self.current_weight(), self.constraints.max_weight));

        // Group by category if enabled
        if self.organization.group_by_category {
            let mut categories: HashMap<ItemCategory, Vec<(&Item, i32)>> = HashMap::new();

            for (item, stack_size) in self.get_sorted_items() {
                let category = item.category();
                categories.entry(category).or_insert_with(Vec::new).push((item, stack_size));
            }

            for (category, items) in categories {
                if self.organization.hidden_categories.contains(&category) {
                    continue;
                }

                report.push_str(&format!("{:?}:\n", category));
                for (item, stack_size) in items {
                    let stack_info = if stack_size > 1 { format!(" ({})", stack_size) } else { String::new() };
                    report.push_str(&format!("  - {} [{}g, {}s]{}\n",
                        item.properties.name,
                        item.properties.weight,
                        item.properties.value,
                        stack_info));
                }
                report.push('\n');
            }
        } else {
            for (item, stack_size) in self.get_sorted_items() {
                let stack_info = if stack_size > 1 { format!(" ({})", stack_size) } else { String::new() };
                report.push_str(&format!("- {} [{}g, {}s, {:?}]{}\n",
                    item.properties.name,
                    item.properties.weight,
                    item.properties.value,
                    item.properties.rarity,
                    stack_info));
            }
        }

        report
    }
}

impl InventoryConstraints {
    /// Create default constraints
    pub fn default() -> Self {
        Self {
            max_weight: 50.0, // 50 kg default
            max_slots: 30,    // 30 item stacks default
            category_limits: HashMap::new(),
            restrictions: Vec::new(),
        }
    }

    /// Create constraints for a scholar character
    pub fn scholar() -> Self {
        let mut constraints = Self::default();
        constraints.max_weight = 40.0; // Scholars carry less physical items
        constraints.max_slots = 40;    // But more variety of items

        // Allow more books and educational items
        constraints.category_limits.insert(ItemCategory::Books, 20);
        constraints.category_limits.insert(ItemCategory::Educational, 15);

        constraints
    }

    /// Create constraints for an explorer character
    pub fn explorer() -> Self {
        let mut constraints = Self::default();
        constraints.max_weight = 70.0; // Explorers can carry more
        constraints.max_slots = 25;    // But fewer types of items

        // Limit books but allow more tools
        constraints.category_limits.insert(ItemCategory::Books, 5);
        constraints.category_limits.insert(ItemCategory::Tools, 15);

        constraints
    }
}

impl InventoryOrganization {
    /// Create default organization settings
    pub fn default() -> Self {
        Self {
            sort_method: SortMethod::Category,
            group_by_category: true,
            favorites: Vec::new(),
            hidden_categories: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::items::core::{Item, ItemType};

    #[test]
    fn test_inventory_basic_operations() {
        let mut inventory = InventoryManager::new();

        let item1 = Item::new_basic(
            "Test Item 1".to_string(),
            "A test item".to_string(),
            ItemType::Mundane,
        );

        let item_id = item1.id.clone();

        // Add item
        inventory.add_item(item1).unwrap();
        assert!(inventory.has_item(&item_id));
        assert_eq!(inventory.current_slots(), 1);

        // Remove item
        let removed = inventory.remove_item(&item_id).unwrap();
        assert!(removed.is_some());
        assert!(!inventory.has_item(&item_id));
        assert_eq!(inventory.current_slots(), 0);
    }

    #[test]
    fn test_inventory_stacking() {
        let mut inventory = InventoryManager::new();

        let item1 = Item::new_basic(
            "Stackable Item".to_string(),
            "Can be stacked".to_string(),
            ItemType::Mundane,
        );

        let item2 = Item::new_basic(
            "Stackable Item".to_string(),
            "Can be stacked".to_string(),
            ItemType::Mundane,
        );

        let item1_id = item1.id.clone();

        // Add first item
        inventory.add_item(item1).unwrap();
        assert_eq!(inventory.current_slots(), 1);
        assert_eq!(inventory.get_stack_size(&item1_id), 1);

        // Add second identical item (should stack)
        inventory.add_item(item2).unwrap();
        assert_eq!(inventory.current_slots(), 1); // Still only one slot used
        assert_eq!(inventory.get_stack_size(&item1_id), 2);
    }

    #[test]
    fn test_inventory_weight_limits() {
        let mut constraints = InventoryConstraints::default();
        constraints.max_weight = 5.0; // Very low weight limit

        let mut inventory = InventoryManager::with_constraints(constraints);

        let mut heavy_item = Item::new_basic(
            "Heavy Item".to_string(),
            "Very heavy".to_string(),
            ItemType::Equipment(super::super::equipment::Equipment::new_basic(
                super::super::equipment::EquipmentSlot::MainHand
            )),
        );
        heavy_item.properties.weight = 10.0; // Exceeds weight limit of 5.0

        // This should fail due to weight limit
        let result = inventory.add_item(heavy_item);
        assert!(result.is_err());
    }

    #[test]
    fn test_inventory_search() {
        let mut inventory = InventoryManager::new();

        let item1 = Item::new_basic(
            "Magic Sword".to_string(),
            "A magical weapon".to_string(),
            ItemType::Mundane,
        );

        let item2 = Item::new_basic(
            "Magic Potion".to_string(),
            "A magical consumable".to_string(),
            ItemType::Mundane,
        );

        inventory.add_item(item1).unwrap();
        inventory.add_item(item2).unwrap();

        // Search by name
        let magic_items = inventory.search_by_name("Magic");
        assert_eq!(magic_items.len(), 2);

        let sword_items = inventory.search_by_name("Sword");
        assert_eq!(sword_items.len(), 1);

        // Search by description
        let weapon_items = inventory.search_by_description("weapon");
        assert_eq!(weapon_items.len(), 1);
    }

    #[test]
    fn test_inventory_organization() {
        let mut inventory = InventoryManager::new();
        inventory.organization.sort_method = SortMethod::Name;

        let item_z = Item::new_basic("Z Item".to_string(), "Last".to_string(), ItemType::Mundane);
        let item_a = Item::new_basic("A Item".to_string(), "First".to_string(), ItemType::Mundane);

        inventory.add_item(item_z).unwrap();
        inventory.add_item(item_a).unwrap();

        let sorted_items = inventory.get_sorted_items();
        assert_eq!(sorted_items[0].0.properties.name, "A Item");
        assert_eq!(sorted_items[1].0.properties.name, "Z Item");
    }
}