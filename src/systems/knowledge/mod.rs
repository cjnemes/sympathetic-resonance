//! Knowledge and theory progression system

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// System for managing magical knowledge and theory progression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeSystem {
    /// Available theories and their properties
    theories: HashMap<String, TheoryDefinition>,
}

/// Definition of a magic theory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheoryDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub prerequisites: Vec<String>,
    pub complexity_level: i32,
    pub learning_time_base: i32,
    pub applications: Vec<String>,
}

impl KnowledgeSystem {
    /// Create a new knowledge system
    pub fn new() -> Self {
        Self {
            theories: HashMap::new(),
        }
    }

    /// Add a theory definition
    pub fn add_theory(&mut self, theory: TheoryDefinition) {
        self.theories.insert(theory.id.clone(), theory);
    }

    /// Get theory by ID
    pub fn get_theory(&self, id: &str) -> Option<&TheoryDefinition> {
        self.theories.get(id)
    }

    /// Get all available theories
    pub fn get_all_theories(&self) -> &HashMap<String, TheoryDefinition> {
        &self.theories
    }
}