use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentLoader {
    data_path: String,
}

impl ContentLoader {
    pub fn new(data_path: String) -> Self {
        Self { data_path }
    }

    pub fn load_all_content(&self) -> crate::GameResult<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameContent {
    pub locations: HashMap<String, LocationContent>,
    pub items: HashMap<String, ItemContent>,
    pub npcs: HashMap<String, NpcContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationContent {
    pub name: String,
    pub description: String,
    pub exits: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemContent {
    pub name: String,
    pub description: String,
    pub value: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcContent {
    pub name: String,
    pub description: String,
    pub dialogue: Vec<String>,
}