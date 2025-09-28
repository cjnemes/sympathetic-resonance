//! Database management and content loading system
//!
//! This module handles:
//! - SQLite database schema creation and management
//! - Content loading from database
//! - Database migration and versioning

use rusqlite::{Connection, Result as SqlResult, params};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::core::world_state::{Location, Direction, MagicalProperties, FactionPresence, PresenceVisibility};
use crate::GameResult;

/// Database schema version for migration management
const SCHEMA_VERSION: i32 = 1;

/// Manager for all database operations
pub struct DatabaseManager {
    connection: Connection,
}

/// NPC definition from database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub faction_id: Option<String>,
    pub dialogue_tree: String, // JSON string
    pub current_location: String,
}

/// Magic theory definition from database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheoryData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub prerequisites: Vec<String>, // Theory IDs that must be known first
    pub complexity_level: i32,
    pub learning_time_base: i32, // Base time in minutes to learn
    pub applications: Vec<String>, // What this theory enables
}

/// Item definition from database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub item_type: String,
    pub properties: String, // JSON string for type-specific properties
}

impl DatabaseManager {
    /// Create a new database manager and open connection
    pub fn new(database_path: &str) -> GameResult<Self> {
        let connection = Connection::open(database_path)
            .map_err(|e| crate::GameError::DatabaseError(format!("Failed to open database: {}", e)))?;

        Ok(Self { connection })
    }

    /// Initialize database schema
    pub fn initialize_schema(&self) -> GameResult<()> {
        // Create version table first
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS schema_version (
                version INTEGER PRIMARY KEY
            )",
            [],
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to create version table: {}", e)))?;

        // Check current version
        let current_version: Option<i32> = self.connection
            .query_row(
                "SELECT version FROM schema_version ORDER BY version DESC LIMIT 1",
                [],
                |row| row.get(0),
            )
            .optional()
            .map_err(|e| crate::GameError::DatabaseError(format!("Failed to check version: {}", e)))?;

        if current_version.is_none() || current_version.unwrap() < SCHEMA_VERSION {
            self.create_tables()?;
            self.update_schema_version()?;
        }

        Ok(())
    }

    /// Create all database tables
    fn create_tables(&self) -> GameResult<()> {
        // Locations table
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS locations (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                ambient_energy REAL DEFAULT 1.0,
                dominant_frequency INTEGER,
                interference REAL DEFAULT 0.0,
                phenomena TEXT, -- JSON array
                visited BOOLEAN DEFAULT FALSE
            )",
            [],
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to create locations table: {}", e)))?;

        // Location exits (separate table for flexibility)
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS location_exits (
                location_id TEXT NOT NULL,
                direction TEXT NOT NULL,
                destination_id TEXT NOT NULL,
                FOREIGN KEY(location_id) REFERENCES locations(id),
                FOREIGN KEY(destination_id) REFERENCES locations(id),
                PRIMARY KEY(location_id, direction)
            )",
            [],
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to create exits table: {}", e)))?;

        // NPCs table
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS npcs (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                faction_id TEXT,
                dialogue_tree TEXT, -- JSON
                current_location TEXT,
                FOREIGN KEY(current_location) REFERENCES locations(id)
            )",
            [],
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to create npcs table: {}", e)))?;

        // Magic theories table
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS magic_theories (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                prerequisites TEXT, -- JSON array of theory IDs
                complexity_level INTEGER NOT NULL,
                learning_time_base INTEGER NOT NULL,
                applications TEXT -- JSON array of applications
            )",
            [],
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to create theories table: {}", e)))?;

        // Items table
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS items (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                item_type TEXT NOT NULL,
                properties TEXT -- JSON for type-specific properties
            )",
            [],
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to create items table: {}", e)))?;

        // Faction presence in locations
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS faction_presence (
                location_id TEXT NOT NULL,
                faction_id TEXT NOT NULL,
                influence INTEGER NOT NULL,
                visibility TEXT NOT NULL,
                member_count INTEGER DEFAULT 0,
                FOREIGN KEY(location_id) REFERENCES locations(id),
                PRIMARY KEY(location_id, faction_id)
            )",
            [],
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to create faction presence table: {}", e)))?;

        // Create indexes for performance
        self.create_indexes()?;

        Ok(())
    }

    /// Create database indexes for performance
    fn create_indexes(&self) -> GameResult<()> {
        let indexes = vec![
            "CREATE INDEX IF NOT EXISTS idx_location_exits_location ON location_exits(location_id)",
            "CREATE INDEX IF NOT EXISTS idx_npcs_location ON npcs(current_location)",
            "CREATE INDEX IF NOT EXISTS idx_npcs_faction ON npcs(faction_id)",
            "CREATE INDEX IF NOT EXISTS idx_faction_presence_location ON faction_presence(location_id)",
        ];

        for index_sql in indexes {
            self.connection.execute(index_sql, [])
                .map_err(|e| crate::GameError::DatabaseError(format!("Failed to create index: {}", e)))?;
        }

        Ok(())
    }

    /// Update schema version
    fn update_schema_version(&self) -> GameResult<()> {
        self.connection.execute(
            "INSERT OR REPLACE INTO schema_version (version) VALUES (?1)",
            params![SCHEMA_VERSION],
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to update schema version: {}", e)))?;

        Ok(())
    }

    /// Load default content into the database
    pub fn load_default_content(&self) -> GameResult<()> {
        // Load tutorial chamber
        self.insert_location(
            "tutorial_chamber",
            "Tutorial Chamber",
            "A simple stone chamber with crystalline formations embedded in the walls. \
             Soft light emanates from the crystals, creating an atmosphere of focused learning. \
             The air hums with gentle magical energy, perfect for beginning practitioners.",
            1.2, // Enhanced ambient energy for learning
            Some(4), // Quartz frequency
            0.0, // No interference
            &[],
        )?;

        // Load basic location connections
        self.insert_exit("tutorial_chamber", "north", "practice_hall")?;

        self.insert_location(
            "practice_hall",
            "Practice Hall",
            "A larger chamber designed for magical experimentation. Scorch marks and \
             crystal fragments scattered across the floor tell stories of countless \
             magical attempts. Protective barriers shimmer along the walls.",
            1.0, // Normal ambient energy
            None, // No dominant frequency
            0.1, // Slight interference from residual magic
            &["Protection barriers active".to_string()],
        )?;

        self.insert_exit("practice_hall", "south", "tutorial_chamber")?;

        // Load basic magic theories
        self.insert_theory(
            "harmonic_fundamentals",
            "Harmonic Fundamentals",
            "The basic principles of sympathetic resonance and magical energy conservation.",
            &[],
            1,
            30,
            &["Basic resonance understanding".to_string(), "Energy conservation".to_string()],
        )?;

        self.insert_theory(
            "crystal_lattice_basics",
            "Crystal Lattice Theory",
            "Understanding how crystal structures amplify and focus magical energy.",
            &["harmonic_fundamentals".to_string()],
            2,
            45,
            &["Crystal efficiency improvement".to_string(), "Degradation prediction".to_string()],
        )?;

        Ok(())
    }

    /// Insert a location into the database
    pub fn insert_location(
        &self,
        id: &str,
        name: &str,
        description: &str,
        ambient_energy: f32,
        dominant_frequency: Option<i32>,
        interference: f32,
        phenomena: &[String],
    ) -> GameResult<()> {
        let phenomena_json = serde_json::to_string(phenomena)
            .map_err(|e| crate::GameError::DatabaseError(format!("Failed to serialize phenomena: {}", e)))?;

        self.connection.execute(
            "INSERT OR REPLACE INTO locations
             (id, name, description, ambient_energy, dominant_frequency, interference, phenomena, visited)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, FALSE)",
            params![id, name, description, ambient_energy, dominant_frequency, interference, phenomena_json],
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to insert location: {}", e)))?;

        Ok(())
    }

    /// Insert an exit between locations
    pub fn insert_exit(&self, from_location: &str, direction: &str, to_location: &str) -> GameResult<()> {
        self.connection.execute(
            "INSERT OR REPLACE INTO location_exits (location_id, direction, destination_id) VALUES (?1, ?2, ?3)",
            params![from_location, direction, to_location],
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to insert exit: {}", e)))?;

        Ok(())
    }

    /// Insert a magic theory
    pub fn insert_theory(
        &self,
        id: &str,
        name: &str,
        description: &str,
        prerequisites: &[String],
        complexity: i32,
        learning_time: i32,
        applications: &[String],
    ) -> GameResult<()> {
        let prereq_json = serde_json::to_string(prerequisites)
            .map_err(|e| crate::GameError::DatabaseError(format!("Failed to serialize prerequisites: {}", e)))?;
        let apps_json = serde_json::to_string(applications)
            .map_err(|e| crate::GameError::DatabaseError(format!("Failed to serialize applications: {}", e)))?;

        self.connection.execute(
            "INSERT OR REPLACE INTO magic_theories
             (id, name, description, prerequisites, complexity_level, learning_time_base, applications)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![id, name, description, prereq_json, complexity, learning_time, apps_json],
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to insert theory: {}", e)))?;

        Ok(())
    }

    /// Load all locations from database
    pub fn load_locations(&self) -> GameResult<HashMap<String, Location>> {
        let mut locations = HashMap::new();

        // Load basic location data
        let mut stmt = self.connection.prepare(
            "SELECT id, name, description, ambient_energy, dominant_frequency, interference, phenomena, visited
             FROM locations"
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to prepare location query: {}", e)))?;

        let location_rows = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let description: String = row.get(2)?;
            let ambient_energy: f32 = row.get(3)?;
            let dominant_frequency: Option<i32> = row.get(4)?;
            let interference: f32 = row.get(5)?;
            let phenomena_json: String = row.get(6)?;
            let visited: bool = row.get(7)?;

            let phenomena: Vec<String> = serde_json::from_str(&phenomena_json)
                .unwrap_or_else(|_| Vec::new());

            Ok((id.clone(), Location {
                id,
                name,
                description,
                exits: HashMap::new(), // Will be populated below
                npcs: Vec::new(), // Will be populated below
                items: Vec::new(), // Will be populated below
                magical_properties: MagicalProperties {
                    ambient_energy,
                    dominant_frequency,
                    interference,
                    recent_activity: Vec::new(),
                    phenomena,
                },
                faction_presence: HashMap::new(), // Will be populated below
                visited,
            }))
        }).map_err(|e| crate::GameError::DatabaseError(format!("Failed to query locations: {}", e)))?;

        for location_result in location_rows {
            let (id, location) = location_result
                .map_err(|e| crate::GameError::DatabaseError(format!("Failed to parse location: {}", e)))?;
            locations.insert(id, location);
        }

        // Load exits
        self.load_exits(&mut locations)?;

        // Load faction presence
        self.load_faction_presence(&mut locations)?;

        Ok(locations)
    }

    /// Load exits for all locations
    fn load_exits(&self, locations: &mut HashMap<String, Location>) -> GameResult<()> {
        let mut stmt = self.connection.prepare(
            "SELECT location_id, direction, destination_id FROM location_exits"
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to prepare exits query: {}", e)))?;

        let exit_rows = stmt.query_map([], |row| {
            let location_id: String = row.get(0)?;
            let direction: String = row.get(1)?;
            let destination_id: String = row.get(2)?;
            Ok((location_id, direction, destination_id))
        }).map_err(|e| crate::GameError::DatabaseError(format!("Failed to query exits: {}", e)))?;

        for exit_result in exit_rows {
            let (location_id, direction_str, destination_id) = exit_result
                .map_err(|e| crate::GameError::DatabaseError(format!("Failed to parse exit: {}", e)))?;

            if let Some(location) = locations.get_mut(&location_id) {
                if let Some(direction) = Direction::from_string(&direction_str) {
                    location.exits.insert(direction, destination_id);
                }
            }
        }

        Ok(())
    }

    /// Load faction presence for all locations
    fn load_faction_presence(&self, locations: &mut HashMap<String, Location>) -> GameResult<()> {
        let mut stmt = self.connection.prepare(
            "SELECT location_id, faction_id, influence, visibility, member_count FROM faction_presence"
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to prepare faction presence query: {}", e)))?;

        let presence_rows = stmt.query_map([], |row| {
            let location_id: String = row.get(0)?;
            let faction_id: String = row.get(1)?;
            let influence: i32 = row.get(2)?;
            let visibility_str: String = row.get(3)?;
            let member_count: i32 = row.get(4)?;
            Ok((location_id, faction_id, influence, visibility_str, member_count))
        }).map_err(|e| crate::GameError::DatabaseError(format!("Failed to query faction presence: {}", e)))?;

        for presence_result in presence_rows {
            let (location_id, faction_id, influence, visibility_str, member_count) = presence_result
                .map_err(|e| crate::GameError::DatabaseError(format!("Failed to parse faction presence: {}", e)))?;

            let visibility = match visibility_str.as_str() {
                "Hidden" => PresenceVisibility::Hidden,
                "Subtle" => PresenceVisibility::Subtle,
                "Open" => PresenceVisibility::Open,
                "Dominant" => PresenceVisibility::Dominant,
                _ => PresenceVisibility::Hidden,
            };

            if let Some(location) = locations.get_mut(&location_id) {
                location.faction_presence.insert(faction_id, FactionPresence {
                    influence,
                    visibility,
                    member_count,
                });
            }
        }

        Ok(())
    }

    /// Load all magic theories from database
    pub fn load_theories(&self) -> GameResult<HashMap<String, TheoryData>> {
        let mut theories = HashMap::new();

        let mut stmt = self.connection.prepare(
            "SELECT id, name, description, prerequisites, complexity_level, learning_time_base, applications
             FROM magic_theories"
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to prepare theories query: {}", e)))?;

        let theory_rows = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let description: String = row.get(2)?;
            let prerequisites_json: String = row.get(3)?;
            let complexity_level: i32 = row.get(4)?;
            let learning_time_base: i32 = row.get(5)?;
            let applications_json: String = row.get(6)?;

            let prerequisites: Vec<String> = serde_json::from_str(&prerequisites_json)
                .unwrap_or_else(|_| Vec::new());
            let applications: Vec<String> = serde_json::from_str(&applications_json)
                .unwrap_or_else(|_| Vec::new());

            Ok((id.clone(), TheoryData {
                id,
                name,
                description,
                prerequisites,
                complexity_level,
                learning_time_base,
                applications,
            }))
        }).map_err(|e| crate::GameError::DatabaseError(format!("Failed to query theories: {}", e)))?;

        for theory_result in theory_rows {
            let (id, theory) = theory_result
                .map_err(|e| crate::GameError::DatabaseError(format!("Failed to parse theory: {}", e)))?;
            theories.insert(id, theory);
        }

        Ok(theories)
    }

    /// Get database connection for advanced operations
    pub fn connection(&self) -> &Connection {
        &self.connection
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn create_test_db() -> DatabaseManager {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let db = DatabaseManager::new(db_path).unwrap();
        db.initialize_schema().unwrap();
        db
    }

    #[test]
    fn test_database_creation() {
        let _db = create_test_db();
        // If we get here without panic, database creation worked
    }

    #[test]
    fn test_location_insertion_and_loading() {
        let db = create_test_db();

        db.insert_location(
            "test_room",
            "Test Room",
            "A test location",
            1.0,
            Some(5),
            0.1,
            &["Test phenomenon".to_string()],
        ).unwrap();

        let locations = db.load_locations().unwrap();
        assert!(locations.contains_key("test_room"));

        let test_room = &locations["test_room"];
        assert_eq!(test_room.name, "Test Room");
        assert_eq!(test_room.magical_properties.ambient_energy, 1.0);
        assert_eq!(test_room.magical_properties.dominant_frequency, Some(5));
    }

    #[test]
    fn test_theory_insertion_and_loading() {
        let db = create_test_db();

        db.insert_theory(
            "test_theory",
            "Test Theory",
            "A test magical theory",
            &["prereq1".to_string()],
            3,
            60,
            &["Test application".to_string()],
        ).unwrap();

        let theories = db.load_theories().unwrap();
        assert!(theories.contains_key("test_theory"));

        let test_theory = &theories["test_theory"];
        assert_eq!(test_theory.name, "Test Theory");
        assert_eq!(test_theory.complexity_level, 3);
        assert_eq!(test_theory.prerequisites, vec!["prereq1".to_string()]);
    }

    #[test]
    fn test_exits() {
        let db = create_test_db();

        db.insert_location("room1", "Room 1", "First room", 1.0, None, 0.0, &[]).unwrap();
        db.insert_location("room2", "Room 2", "Second room", 1.0, None, 0.0, &[]).unwrap();
        db.insert_exit("room1", "north", "room2").unwrap();

        let locations = db.load_locations().unwrap();
        let room1 = &locations["room1"];

        assert!(room1.exits.contains_key(&Direction::North));
        assert_eq!(room1.exits[&Direction::North], "room2");
    }
}