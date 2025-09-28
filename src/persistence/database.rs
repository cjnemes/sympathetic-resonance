//! Database management and content loading system
//!
//! This module handles:
//! - SQLite database schema creation and management
//! - Content loading from database
//! - Database migration and versioning

use rusqlite::{Connection, params, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::core::world_state::{Location, Direction, MagicalProperties, FactionPresence, PresenceVisibility};
use crate::GameResult;

/// Database schema version for migration management
const SCHEMA_VERSION: i32 = 3;

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

        // Magic theories table (enhanced for comprehensive learning system)
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS magic_theories (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                tier INTEGER NOT NULL, -- 1=Foundation, 2=Application, 3=Advanced
                category TEXT NOT NULL, -- Theory category for organization
                prerequisites TEXT, -- JSON array of theory IDs
                complexity_level INTEGER NOT NULL,
                learning_time_base INTEGER NOT NULL,
                scientific_concepts TEXT, -- JSON array of scientific concepts
                applications TEXT, -- JSON array of applications
                available_methods TEXT, -- JSON array of available learning methods
                method_multipliers TEXT -- JSON object of method efficiency multipliers
            )",
            [],
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to create theories table: {}", e)))?;

        // Player theory progress tracking
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS player_theory_progress (
                player_id TEXT NOT NULL,
                theory_id TEXT NOT NULL,
                understanding_level REAL NOT NULL DEFAULT 0.0,
                experience_points INTEGER NOT NULL DEFAULT 0,
                learning_history TEXT, -- JSON object of method contributions
                time_invested INTEGER NOT NULL DEFAULT 0,
                discovered_at INTEGER NOT NULL,
                mastered_at INTEGER,
                is_active_research BOOLEAN DEFAULT FALSE,
                research_progress REAL DEFAULT 0.0,
                PRIMARY KEY(player_id, theory_id),
                FOREIGN KEY(theory_id) REFERENCES magic_theories(id)
            )",
            [],
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to create theory progress table: {}", e)))?;

        // Learning activity log for detailed tracking
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS learning_activities (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                player_id TEXT NOT NULL,
                theory_id TEXT NOT NULL,
                method TEXT NOT NULL,
                duration INTEGER NOT NULL,
                success_rate REAL NOT NULL,
                experience_gained INTEGER NOT NULL,
                understanding_gained REAL NOT NULL,
                resources_used TEXT, -- JSON object of resources consumed
                side_effects TEXT, -- JSON array of side effects
                timestamp INTEGER NOT NULL,
                FOREIGN KEY(theory_id) REFERENCES magic_theories(id)
            )",
            [],
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to create learning activities table: {}", e)))?;

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

        // Quest definitions table
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS quest_definitions (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT NOT NULL,
                category TEXT NOT NULL,
                difficulty TEXT NOT NULL,
                requirements TEXT NOT NULL, -- JSON
                objectives TEXT NOT NULL, -- JSON array
                rewards TEXT NOT NULL, -- JSON
                faction_effects TEXT, -- JSON
                educational_focus TEXT, -- JSON
                branching_paths TEXT, -- JSON
                involved_npcs TEXT, -- JSON array
                locations TEXT, -- JSON array
                estimated_duration INTEGER NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to create quest definitions table: {}", e)))?;

        // Player quest progress tracking
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS player_quest_progress (
                player_id TEXT NOT NULL,
                quest_id TEXT NOT NULL,
                status TEXT NOT NULL,
                started_at INTEGER NOT NULL,
                completed_at INTEGER,
                objective_progress TEXT NOT NULL, -- JSON
                chosen_branch TEXT,
                player_choices TEXT, -- JSON
                time_invested INTEGER NOT NULL DEFAULT 0,
                quest_variables TEXT, -- JSON
                learning_progress TEXT, -- JSON
                PRIMARY KEY(player_id, quest_id),
                FOREIGN KEY(quest_id) REFERENCES quest_definitions(id)
            )",
            [],
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to create player quest progress table: {}", e)))?;

        // Quest objective completion log for detailed tracking
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS quest_objective_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                player_id TEXT NOT NULL,
                quest_id TEXT NOT NULL,
                objective_id TEXT NOT NULL,
                completed_at INTEGER NOT NULL,
                progress_value REAL NOT NULL,
                completion_method TEXT,
                learning_data TEXT, -- JSON
                FOREIGN KEY(quest_id) REFERENCES quest_definitions(id)
            )",
            [],
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to create quest objective log table: {}", e)))?;

        // Quest rewards awarded to players
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS quest_rewards_awarded (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                player_id TEXT NOT NULL,
                quest_id TEXT NOT NULL,
                reward_type TEXT NOT NULL,
                reward_data TEXT NOT NULL, -- JSON
                awarded_at INTEGER NOT NULL,
                FOREIGN KEY(quest_id) REFERENCES quest_definitions(id)
            )",
            [],
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to create quest rewards table: {}", e)))?;

        // Global quest state and unlocks
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS quest_global_state (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL, -- JSON
                updated_at INTEGER NOT NULL
            )",
            [],
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to create quest global state table: {}", e)))?;

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
            "CREATE INDEX IF NOT EXISTS idx_theory_progress_player ON player_theory_progress(player_id)",
            "CREATE INDEX IF NOT EXISTS idx_theory_progress_theory ON player_theory_progress(theory_id)",
            "CREATE INDEX IF NOT EXISTS idx_learning_activities_player ON learning_activities(player_id)",
            "CREATE INDEX IF NOT EXISTS idx_learning_activities_theory ON learning_activities(theory_id)",
            "CREATE INDEX IF NOT EXISTS idx_learning_activities_timestamp ON learning_activities(timestamp)",
            "CREATE INDEX IF NOT EXISTS idx_theories_tier ON magic_theories(tier)",
            "CREATE INDEX IF NOT EXISTS idx_theories_category ON magic_theories(category)",
            // Quest system indexes
            "CREATE INDEX IF NOT EXISTS idx_quest_definitions_category ON quest_definitions(category)",
            "CREATE INDEX IF NOT EXISTS idx_quest_definitions_difficulty ON quest_definitions(difficulty)",
            "CREATE INDEX IF NOT EXISTS idx_player_quest_progress_player ON player_quest_progress(player_id)",
            "CREATE INDEX IF NOT EXISTS idx_player_quest_progress_status ON player_quest_progress(status)",
            "CREATE INDEX IF NOT EXISTS idx_quest_objective_log_player ON quest_objective_log(player_id)",
            "CREATE INDEX IF NOT EXISTS idx_quest_objective_log_quest ON quest_objective_log(quest_id)",
            "CREATE INDEX IF NOT EXISTS idx_quest_objective_log_completed ON quest_objective_log(completed_at)",
            "CREATE INDEX IF NOT EXISTS idx_quest_rewards_player ON quest_rewards_awarded(player_id)",
            "CREATE INDEX IF NOT EXISTS idx_quest_rewards_quest ON quest_rewards_awarded(quest_id)",
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
        // Use transaction for batch operations
        let transaction = self.connection.unchecked_transaction()?;

        // Load all locations first
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

        // New expanded locations for richer gameplay
        self.insert_location(
            "resonance_observatory",
            "Resonance Observatory",
            "A crystalline dome atop the city's highest tower, where massive detection arrays monitor magical phenomena across the region. The curved walls are embedded with thousands of tiny crystals that create a living map of magical activity, their gentle chiming providing an auditory representation of the city's magical pulse.",
            1.5, // Enhanced for detection work
            Some(8), // Sapphire frequency
            0.2, // Slight interference from overlapping signals
            &["harmonic_visualization".to_string(), "long_range_detection".to_string(), "magical_weather_sensing".to_string()],
        )?;

        self.insert_location(
            "crystal_garden_lab",
            "Crystal Garden Laboratory",
            "An indoor botanical garden where crystals and living plants create symbiotic magical relationships. Terraced growing beds showcase how different crystal frequencies enhance plant growth, while gentle streams carry crystalline water that amplifies healing properties. The air shimmers with beneficial harmonics.",
            1.3, // Moderate enhancement supporting growth
            Some(6), // Emerald frequency
            0.05, // Very low interference
            &["healing_amplification".to_string(), "growth_acceleration".to_string(), "purification_fields".to_string()],
        )?;

        self.insert_location(
            "harmonic_testing_chambers",
            "Harmonic Testing Chambers",
            "A series of acoustically isolated underground chambers where dangerous resonance experiments can be conducted safely. Each chamber can be tuned to specific frequencies, with thick crystal walls that contain magical feedback. Warning runes glow when resonance approaches dangerous levels.",
            0.8, // Reduced to prevent cascading effects
            None, // Chambers can be tuned to any frequency
            0.4, // High interference from containment systems
            &["resonance_containment".to_string(), "frequency_isolation".to_string(), "safety_monitoring".to_string()],
        )?;

        self.insert_location(
            "faction_diplomacy_hall",
            "Faction Diplomacy Hall",
            "A neutral meeting space designed to facilitate communication between opposing factions. The circular chamber features five distinct sections, each attuned to different magical philosophies, with a central area where all frequencies harmonize. Ancient peace treaties are etched in crystal along the walls.",
            1.1, // Slightly enhanced to support communication magic
            Some(7), // Amethyst frequency
            0.15, // Minimal interference to prevent eavesdropping
            &["truth_resonance".to_string(), "emotion_stabilization".to_string(), "communication_enhancement".to_string()],
        )?;

        self.insert_location(
            "crystalline_archives",
            "Crystalline Archives",
            "A vast library where knowledge is stored within crystal matrices rather than books. Towering crystal shelves pulse with different colors representing various fields of study, while reading alcoves allow scholars to interface directly with crystalline knowledge through careful resonance matching. The whispered echoes of a thousand conversations about magic theory fill the air.",
            1.4, // Enhanced for memory and learning
            Some(9), // Lapis Lazuli frequency
            0.1, // Low interference to preserve data integrity
            &["memory_enhancement".to_string(), "knowledge_resonance".to_string(), "research_acceleration".to_string()],
        )?;

        self.insert_location(
            "unstable_resonance_site",
            "Unstable Resonance Site",
            "A dangerous research area on the city's outskirts where a previous magical experiment created permanent instability in local space-time. Reality flickers here, with objects occasionally phasing between dimensions and time flowing at inconsistent rates. Only the most experienced practitioners dare to study the chaotic magical phenomena, protected by multiple layers of containment barriers.",
            1.8, // Dangerously high and fluctuating
            Some(11), // Diamond frequency
            0.5, // Maximum safe interference levels
            &["reality_distortion".to_string(), "temporal_fluctuation".to_string(), "dimensional_instability".to_string(), "magical_overflow".to_string()],
        )?;

        // Now insert exits after all locations exist
        // Tutorial progression path
        self.insert_exit("tutorial_chamber", "north", "practice_hall")?;
        self.insert_exit("practice_hall", "south", "tutorial_chamber")?;
        self.insert_exit("practice_hall", "east", "crystal_garden_lab")?;
        self.insert_exit("crystal_garden_lab", "west", "practice_hall")?;

        // Advanced learning areas
        self.insert_exit("crystal_garden_lab", "north", "crystalline_archives")?;
        self.insert_exit("crystalline_archives", "south", "crystal_garden_lab")?;
        self.insert_exit("practice_hall", "down", "harmonic_testing_chambers")?;
        self.insert_exit("harmonic_testing_chambers", "up", "practice_hall")?;

        // Observatory and political areas
        self.insert_exit("crystalline_archives", "up", "resonance_observatory")?;
        self.insert_exit("resonance_observatory", "down", "crystalline_archives")?;
        self.insert_exit("crystalline_archives", "east", "faction_diplomacy_hall")?;
        self.insert_exit("faction_diplomacy_hall", "west", "crystalline_archives")?;

        // Dangerous area (requires advanced access)
        self.insert_exit("harmonic_testing_chambers", "north", "unstable_resonance_site")?;
        self.insert_exit("unstable_resonance_site", "south", "harmonic_testing_chambers")?;
        self.insert_exit("faction_diplomacy_hall", "northeast", "unstable_resonance_site")?;
        self.insert_exit("unstable_resonance_site", "southwest", "faction_diplomacy_hall")?;

        // Load comprehensive magic theory hierarchy
        self.load_foundational_theories()?;
        self.load_application_theories()?;
        self.load_advanced_theories()?;

        // Load NPCs for all locations
        self.load_default_npcs()?;

        transaction.commit()?;
        Ok(())
    }

    /// Load Tier 1 Foundation theories
    fn load_foundational_theories(&self) -> GameResult<()> {
        // Harmonic Fundamentals - Core resonance principles
        let mut methods = std::collections::HashMap::new();
        methods.insert("Study".to_string(), 1.0);
        methods.insert("Observation".to_string(), 0.9);
        methods.insert("Experimentation".to_string(), 1.2);

        self.insert_comprehensive_theory(
            "harmonic_fundamentals",
            "Harmonic Fundamentals",
            "The foundational principles of sympathetic resonance, exploring how magical energy behaves as waves and oscillations. This theory encompasses the fundamental laws of energy conservation in magical systems and introduces the concept of resonant frequency matching.",
            1, // Tier 1: Foundation
            "HarmonicFundamentals",
            &[], // No prerequisites
            2,   // Complexity level
            45,  // Learning time in minutes
            &["Wave Physics".to_string(), "Harmonic Oscillation".to_string(), "Energy Conservation".to_string()],
            &["Basic magical resonance".to_string(), "Energy efficiency calculations".to_string(), "Frequency matching techniques".to_string()],
            &["Study".to_string(), "Observation".to_string(), "Experimentation".to_string()],
            &methods,
        )?;

        // Crystal Structures - Physical foundations
        methods.clear();
        methods.insert("Study".to_string(), 1.0);
        methods.insert("Observation".to_string(), 1.3);
        methods.insert("Experimentation".to_string(), 1.5);

        self.insert_comprehensive_theory(
            "crystal_structures",
            "Crystal Lattice Theory",
            "Understanding the atomic and molecular structure of magical crystals, including how lattice formations amplify, focus, and modulate magical frequencies. Covers crystal growth patterns, defect analysis, and purity effects on magical conductivity.",
            1, // Tier 1: Foundation
            "CrystalStructures",
            &["harmonic_fundamentals".to_string()],
            3,   // Complexity level
            60,  // Learning time
            &["Crystallography".to_string(), "Solid State Physics".to_string(), "Materials Science".to_string()],
            &["Crystal efficiency optimization".to_string(), "Degradation prediction".to_string(), "Quality assessment".to_string()],
            &["Study".to_string(), "Observation".to_string(), "Experimentation".to_string()],
            &methods,
        )?;

        // Mental Resonance - Consciousness-matter interaction
        methods.clear();
        methods.insert("Study".to_string(), 1.0);
        methods.insert("Observation".to_string(), 0.8);
        methods.insert("Experimentation".to_string(), 1.1);
        methods.insert("Teaching".to_string(), 1.4);

        self.insert_comprehensive_theory(
            "mental_resonance",
            "Mental Resonance Theory",
            "The study of how consciousness interacts with magical fields, including the role of mental acuity in magical manipulation and the neurological basis of resonance sensitivity. Explores the feedback loops between mind and magical energy.",
            1, // Tier 1: Foundation
            "MentalResonance",
            &["harmonic_fundamentals".to_string()],
            3,   // Complexity level
            75,  // Learning time
            &["Neuroscience".to_string(), "Psychology".to_string(), "Biophysics".to_string()],
            &["Mental efficiency improvement".to_string(), "Fatigue reduction techniques".to_string(), "Consciousness-energy interfacing".to_string()],
            &["Study".to_string(), "Observation".to_string(), "Experimentation".to_string(), "Teaching".to_string()],
            &methods,
        )?;

        Ok(())
    }

    /// Load Tier 2 Application theories
    fn load_application_theories(&self) -> GameResult<()> {
        // Light Manipulation - Electromagnetic applications
        let mut methods = std::collections::HashMap::new();
        methods.insert("Study".to_string(), 1.0);
        methods.insert("Observation".to_string(), 1.2);
        methods.insert("Experimentation".to_string(), 1.8);
        methods.insert("Teaching".to_string(), 1.3);

        self.insert_comprehensive_theory(
            "light_manipulation",
            "Electromagnetic Spectrum Control",
            "Application of harmonic principles to manipulate light and other electromagnetic phenomena. Covers wavelength shifting, intensity modulation, and coherent light generation through magical resonance.",
            2, // Tier 2: Application
            "LightManipulation",
            &["harmonic_fundamentals".to_string(), "crystal_structures".to_string()],
            5,   // Complexity level
            90,  // Learning time
            &["Electromagnetic Theory".to_string(), "Optics".to_string(), "Photonics".to_string()],
            &["Illumination spells".to_string(), "Light-based communication".to_string(), "Optical illusions".to_string(), "Laser-like effects".to_string()],
            &["Study".to_string(), "Observation".to_string(), "Experimentation".to_string(), "Teaching".to_string()],
            &methods,
        )?;

        // Bio-resonance - Healing applications
        methods.clear();
        methods.insert("Study".to_string(), 1.0);
        methods.insert("Observation".to_string(), 1.1);
        methods.insert("Experimentation".to_string(), 1.4);
        methods.insert("Teaching".to_string(), 1.6);

        self.insert_comprehensive_theory(
            "bio_resonance",
            "Biological Sympathetic Healing",
            "The application of sympathetic frequencies to biological systems for healing and restoration. Explores cellular resonance, tissue regeneration through frequency matching, and the bioelectric basis of magical healing.",
            2, // Tier 2: Application
            "BioResonance",
            &["harmonic_fundamentals".to_string(), "mental_resonance".to_string()],
            6,   // Complexity level
            120, // Learning time
            &["Biology".to_string(), "Physiology".to_string(), "Biochemistry".to_string(), "Medical Physics".to_string()],
            &["Healing spells".to_string(), "Pain relief techniques".to_string(), "Tissue regeneration".to_string(), "Disease diagnosis".to_string()],
            &["Study".to_string(), "Observation".to_string(), "Experimentation".to_string(), "Teaching".to_string()],
            &methods,
        )?;

        // Detection Arrays - Sensing applications
        methods.clear();
        methods.insert("Study".to_string(), 1.0);
        methods.insert("Observation".to_string(), 1.5);
        methods.insert("Experimentation".to_string(), 1.3);
        methods.insert("Teaching".to_string(), 1.2);

        self.insert_comprehensive_theory(
            "detection_arrays",
            "Magical Signature Analysis",
            "Techniques for detecting, analyzing, and interpreting magical signatures and energy patterns. Covers the construction of detection networks, signal processing of magical emanations, and identification of magical sources.",
            2, // Tier 2: Application
            "DetectionArrays",
            &["crystal_structures".to_string(), "mental_resonance".to_string()],
            5,   // Complexity level
            105, // Learning time
            &["Signal Processing".to_string(), "Pattern Recognition".to_string(), "Sensor Networks".to_string()],
            &["Magic detection spells".to_string(), "Signature identification".to_string(), "Tracking techniques".to_string(), "Security systems".to_string()],
            &["Study".to_string(), "Observation".to_string(), "Experimentation".to_string(), "Teaching".to_string()],
            &methods,
        )?;

        Ok(())
    }

    /// Load Tier 3 Advanced theories
    fn load_advanced_theories(&self) -> GameResult<()> {
        // Sympathetic Networks - Long-distance connections
        let mut methods = std::collections::HashMap::new();
        methods.insert("Study".to_string(), 1.0);
        methods.insert("Observation".to_string(), 1.1);
        methods.insert("Experimentation".to_string(), 1.6);
        methods.insert("Teaching".to_string(), 1.8);
        methods.insert("Research".to_string(), 2.2);

        self.insert_comprehensive_theory(
            "sympathetic_networks",
            "Long-Distance Sympathetic Connections",
            "Advanced techniques for establishing and maintaining magical connections across vast distances. Explores quantum entanglement principles in magical systems, network topology for magical communication, and the infrastructure requirements for stable long-range connections.",
            3, // Tier 3: Advanced
            "SympatheticNetworks",
            &["light_manipulation".to_string(), "detection_arrays".to_string()],
            8,   // Complexity level
            180, // Learning time
            &["Quantum Mechanics".to_string(), "Network Theory".to_string(), "Information Theory".to_string()],
            &["Long-distance communication".to_string(), "Remote sensing".to_string(), "Teleportation preparation".to_string(), "Magical internet protocols".to_string()],
            &["Study".to_string(), "Observation".to_string(), "Experimentation".to_string(), "Teaching".to_string(), "Research".to_string()],
            &methods,
        )?;

        // Resonance Amplification - Power multiplication
        methods.clear();
        methods.insert("Study".to_string(), 1.0);
        methods.insert("Observation".to_string(), 1.0);
        methods.insert("Experimentation".to_string(), 2.0);
        methods.insert("Teaching".to_string(), 1.5);
        methods.insert("Research".to_string(), 2.5);

        self.insert_comprehensive_theory(
            "resonance_amplification",
            "Power Multiplication Systems",
            "Techniques for amplifying magical power through resonance cascades and harmonic multiplication. Covers the construction of amplification arrays, power efficiency optimization, and safety protocols for high-energy magical systems.",
            3, // Tier 3: Advanced
            "ResonanceAmplification",
            &["crystal_structures".to_string(), "bio_resonance".to_string()],
            9,   // Complexity level
            210, // Learning time
            &["Electrical Engineering".to_string(), "Power Systems".to_string(), "Control Theory".to_string()],
            &["Power amplification spells".to_string(), "Energy storage systems".to_string(), "Magical power grids".to_string(), "High-energy applications".to_string()],
            &["Study".to_string(), "Observation".to_string(), "Experimentation".to_string(), "Teaching".to_string(), "Research".to_string()],
            &methods,
        )?;

        // Theoretical Synthesis - Creating new approaches
        methods.clear();
        methods.insert("Study".to_string(), 1.0);
        methods.insert("Observation".to_string(), 0.9);
        methods.insert("Experimentation".to_string(), 1.8);
        methods.insert("Teaching".to_string(), 2.0);
        methods.insert("Research".to_string(), 3.0);

        self.insert_comprehensive_theory(
            "theoretical_synthesis",
            "Unified Magical Theory Development",
            "The pinnacle of magical education: synthesizing knowledge from all fields to develop entirely new theoretical frameworks and magical applications. Includes methodology for magical research, theory validation, and the creation of novel magical effects.",
            3, // Tier 3: Advanced
            "TheoreticalSynthesis",
            &["sympathetic_networks".to_string(), "resonance_amplification".to_string()],
            10,  // Complexity level
            300, // Learning time
            &["Systems Theory".to_string(), "Mathematical Modeling".to_string(), "Research Methodology".to_string(), "Innovation Theory".to_string()],
            &["Novel spell creation".to_string(), "Theory development".to_string(), "Magical innovation".to_string(), "Research leadership".to_string()],
            &["Study".to_string(), "Observation".to_string(), "Experimentation".to_string(), "Teaching".to_string(), "Research".to_string()],
            &methods,
        )?;

        Ok(())
    }

    /// Load default NPCs for all locations
    fn load_default_npcs(&self) -> GameResult<()> {
        use std::collections::HashMap;

        // Helper function to create dialogue trees with proper theory requirements
        let create_dialogue_tree = |greeting_texts: Vec<&str>, topics: Vec<(&str, Vec<&str>, Option<&str>)>| -> String {
            let mut topic_map = HashMap::new();

            for (topic_id, topic_texts, theory_req) in topics {
                let mut requirements = HashMap::new();
                if let Some(theory) = theory_req {
                    requirements.insert("theory_requirements".to_string(), serde_json::json!([theory]));
                }

                topic_map.insert(topic_id.to_string(), serde_json::json!({
                    "text_templates": topic_texts,
                    "responses": [],
                    "requirements": requirements
                }));
            }

            let dialogue_tree = serde_json::json!({
                "greeting": {
                    "text_templates": greeting_texts,
                    "responses": [],
                    "requirements": {}
                },
                "topics": topic_map,
                "faction_specific": {}
            });

            dialogue_tree.to_string()
        };

        // 1. Resonance Observatory NPCs
        self.insert_npc(
            "observer_lyra",
            "Observer Lyra Nightwatch",
            "A keen-eyed detection specialist manning the observatory's surveillance arrays. Her expression carries the weight of moral conflicts about the balance between security and privacy.",
            Some("magisters_council"),
            &create_dialogue_tree(
                vec!["Welcome to the Observatory. I monitor the city's magical pulse.", "The detection arrays show interesting patterns today."],
                vec![
                    ("detection_theory", vec!["Detection magic relies on resonance pattern recognition.", "Each magical signature has unique characteristics."], Some("detection_arrays")),
                    ("surveillance_ethics", vec!["Sometimes I wonder if we observe too much.", "Knowledge is power, but at what cost?"], None),
                ]
            ),
            "resonance_observatory"
        )?;

        self.insert_npc(
            "technician_marcus",
            "Technician Marcus Clearview",
            "An equipment engineer focused on the commercial applications of detection technology. His workshop tools are always immaculately organized.",
            Some("industrial_consortium"),
            &create_dialogue_tree(
                vec!["These detection arrays represent cutting-edge magical engineering.", "I'm working on efficiency improvements for commercial deployment."],
                vec![
                    ("crystal_engineering", vec!["Sapphire crystals provide excellent detection clarity.", "Proper crystal tuning is essential for accurate readings."], Some("crystal_structures")),
                    ("commercial_applications", vec!["Detection magic has enormous market potential.", "Businesses need magical security solutions."], None),
                ]
            ),
            "resonance_observatory"
        )?;

        // 2. Crystal Garden Laboratory NPCs
        self.insert_npc(
            "healer_seraphina",
            "Healer Seraphina Bloomheart",
            "A bio-resonance researcher who bridges traditional healing wisdom with modern magical theory. Plants seem to flourish in her presence.",
            Some("order_of_harmony"),
            &create_dialogue_tree(
                vec!["Welcome to the Garden. Here, life and magic dance in harmony.", "The plants here respond to healing frequencies with remarkable vigor."],
                vec![
                    ("healing_theory", vec!["Biological systems have natural resonant frequencies.", "Healing magic works by restoring harmonic balance."], Some("bio_resonance")),
                    ("plant_magic", vec!["Plants are excellent teachers of natural magic.", "They show us how to work with life energy, not against it."], None),
                ]
            ),
            "crystal_garden_lab"
        )?;

        self.insert_npc(
            "dr_felix",
            "Dr. Felix Verdant",
            "An independent researcher studying the intricate relationships between life and magic. His notebooks are filled with detailed observations of bio-magical phenomena.",
            Some("neutral_scholars"),
            &create_dialogue_tree(
                vec!["Fascinating! The bio-magical interactions here provide endless research opportunities.", "I'm documenting how crystal frequencies affect living systems."],
                vec![
                    ("research_methods", vec!["Careful observation reveals the deepest truths.", "Scientific method applies to magical research as well."], None),
                    ("life_magic_theory", vec!["Life force has its own unique resonance patterns.", "Understanding bio-magic requires patience and empathy."], Some("bio_resonance")),
                ]
            ),
            "crystal_garden_lab"
        )?;

        // 3. Harmonic Testing Chambers NPCs
        self.insert_npc(
            "warden_gareth",
            "Safety Warden Gareth Ironshield",
            "A safety officer haunted by past magical disasters. His vigilance has prevented countless accidents, but the weight of responsibility shows in his weathered face.",
            Some("magisters_council"),
            &create_dialogue_tree(
                vec!["Safety first in the testing chambers. I've seen what happens when protocols are ignored.", "These containment systems have saved more lives than most realize."],
                vec![
                    ("safety_protocols", vec!["Every safety rule is written in someone's pain.", "Proper containment prevents magical cascade failures."], None),
                    ("magical_disasters", vec!["The Unstable Site reminds us of magic's dangers.", "Prevention is always better than cleanup."], Some("sympathetic_networks")),
                ]
            ),
            "harmonic_testing_chambers"
        )?;

        self.insert_npc(
            "mage_kira",
            "Experimental Mage Kira Stormwright",
            "A rogue researcher pushing the boundaries of safe magical experimentation. Her eyes gleam with dangerous curiosity about forbidden techniques.",
            Some("underground_network"),
            &create_dialogue_tree(
                vec!["True discovery requires risk. The Establishment fears what we might learn.", "These chambers contain my experiments, but not my ambition."],
                vec![
                    ("experimental_magic", vec!["Innovation requires breaking a few rules.", "The most interesting magic happens at the edges of safety."], Some("sympathetic_networks")),
                    ("forbidden_research", vec!["They call it 'forbidden' because it threatens their control.", "Knowledge should be free, regardless of risk."], None),
                ]
            ),
            "harmonic_testing_chambers"
        )?;

        // 4. Faction Diplomacy Hall NPCs
        self.insert_npc(
            "ambassador_cordelia",
            "Ambassador Cordelia Bridgeweaver",
            "A diplomatic coordinator working tirelessly to maintain peace between the factions. Her patient demeanor conceals the stress of constant mediation.",
            Some("neutral_scholars"),
            &create_dialogue_tree(
                vec!["Welcome to the Diplomacy Hall. Here, we seek understanding across factional divides.", "Peaceful dialogue requires patience and mutual respect."],
                vec![
                    ("diplomatic_theory", vec!["Communication magic helps bridge emotional gaps.", "True diplomacy requires understanding all perspectives."], Some("mental_resonance")),
                    ("faction_politics", vec!["Each faction has valid concerns and blind spots.", "Progress comes through compromise, not dominance."], None),
                ]
            ),
            "faction_diplomacy_hall"
        )?;

        self.insert_npc(
            "secretary_malik",
            "Secretary Malik Neutralspace",
            "An independent records keeper with no faction affiliations. His meticulous notes capture the nuances of every political negotiation.",
            None,
            &create_dialogue_tree(
                vec!["I maintain neutral records of all diplomatic proceedings.", "Accurate documentation prevents future misunderstandings."],
                vec![
                    ("record_keeping", vec!["Every word matters in diplomatic negotiations.", "Historical context informs current discussions."], None),
                    ("neutral_perspective", vec!["Independence allows me to see all sides clearly.", "Faction loyalty can blind people to important truths."], None),
                ]
            ),
            "faction_diplomacy_hall"
        )?;

        // 5. Crystalline Archives NPCs
        self.insert_npc(
            "sage_meridian",
            "Sage Meridian Crystalscribe",
            "The chief archivist guarding the library's vast crystalline knowledge stores. Her deep understanding of magical theory is matched only by her protective instincts about dangerous information.",
            Some("neutral_scholars"),
            &create_dialogue_tree(
                vec!["Welcome to the Archives. Knowledge illuminates, but it must be shared responsibly.", "These crystals contain the accumulated wisdom of generations."],
                vec![
                    ("knowledge_theory", vec!["Information stored in crystal matrices never degrades.", "Proper resonance allows direct knowledge transfer."], Some("crystalline_archives")),
                    ("forbidden_knowledge", vec!["Some knowledge requires wisdom to handle safely.", "I decide what information students are ready to access."], Some("sympathetic_networks")),
                ]
            ),
            "crystalline_archives"
        )?;

        self.insert_npc(
            "assistant_thomas",
            "Assistant Thomas Indexwell",
            "A young librarian innovating new methods for organizing and accessing crystalline knowledge. His enthusiasm for information systems is infectious.",
            Some("neutral_scholars"),
            &create_dialogue_tree(
                vec!["I'm developing new indexing systems for crystal-stored knowledge!", "The organization of information is an art form."],
                vec![
                    ("information_systems", vec!["Better indexing means faster knowledge access.", "I'm working on resonance-based search algorithms."], Some("detection_arrays")),
                    ("learning_efficiency", vec!["The right information at the right time accelerates learning.", "Knowledge organization should serve the student's needs."], None),
                ]
            ),
            "crystalline_archives"
        )?;

        // 6. Unstable Resonance Site NPCs
        self.insert_npc(
            "captain_vera",
            "Captain Vera Stormward",
            "A military disaster containment commander responsible for maintaining the barriers around the unstable site. Her tactical mind constantly assesses magical threats.",
            Some("magisters_council"),
            &create_dialogue_tree(
                vec!["This site remains dangerous despite our containment efforts.", "My job is to ensure this disaster never spreads beyond these barriers."],
                vec![
                    ("disaster_containment", vec!["Magical disasters require constant vigilance.", "These barriers prevent reality distortions from expanding."], Some("sympathetic_networks")),
                    ("military_tactics", vec!["Magical threats require specialized defensive strategies.", "We train for scenarios most can't imagine."], None),
                ]
            ),
            "unstable_resonance_site"
        )?;

        self.insert_npc(
            "echo_voidwalker",
            "Echo Voidwalker",
            "A dangerous scavenger studying unstable magic despite official warnings. Their identity remains partially obscured by reality distortions.",
            Some("underground_network"),
            &create_dialogue_tree(
                vec!["This place whispers secrets the Council wants buried.", "Reality bends here in ways that reveal magic's true nature."],
                vec![
                    ("unstable_magic", vec!["Chaos teaches lessons that order cannot.", "The boundaries between dimensions grow thin here."], Some("sympathetic_networks")),
                    ("forbidden_research", vec!["True understanding requires embracing danger.", "They fear what we might discover in the chaos."], Some("energy_manipulation")),
                ]
            ),
            "unstable_resonance_site"
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

    /// Insert a comprehensive magic theory with all learning metadata
    pub fn insert_comprehensive_theory(
        &self,
        id: &str,
        name: &str,
        description: &str,
        tier: i32,
        category: &str,
        prerequisites: &[String],
        complexity: i32,
        learning_time: i32,
        scientific_concepts: &[String],
        applications: &[String],
        available_methods: &[String],
        method_multipliers: &std::collections::HashMap<String, f32>,
    ) -> GameResult<()> {
        let prereq_json = serde_json::to_string(prerequisites)
            .map_err(|e| crate::GameError::DatabaseError(format!("Failed to serialize prerequisites: {}", e)))?;
        let apps_json = serde_json::to_string(applications)
            .map_err(|e| crate::GameError::DatabaseError(format!("Failed to serialize applications: {}", e)))?;
        let concepts_json = serde_json::to_string(scientific_concepts)
            .map_err(|e| crate::GameError::DatabaseError(format!("Failed to serialize scientific concepts: {}", e)))?;
        let methods_json = serde_json::to_string(available_methods)
            .map_err(|e| crate::GameError::DatabaseError(format!("Failed to serialize methods: {}", e)))?;
        let multipliers_json = serde_json::to_string(method_multipliers)
            .map_err(|e| crate::GameError::DatabaseError(format!("Failed to serialize multipliers: {}", e)))?;

        self.connection.execute(
            "INSERT OR REPLACE INTO magic_theories
             (id, name, description, tier, category, prerequisites, complexity_level, learning_time_base,
              scientific_concepts, applications, available_methods, method_multipliers)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![id, name, description, tier, category, prereq_json, complexity, learning_time,
                   concepts_json, apps_json, methods_json, multipliers_json],
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to insert theory: {}", e)))?;

        Ok(())
    }

    /// Insert a magic theory (legacy method for backward compatibility)
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
        // Determine tier from complexity
        let tier = match complexity {
            1..=3 => 1, // Foundation
            4..=6 => 2, // Application
            _ => 3,     // Advanced
        };

        // Default category
        let category = "HarmonicFundamentals";

        // Default scientific concepts
        let scientific_concepts = vec!["General Magic Theory".to_string()];

        // Default available methods
        let available_methods = vec!["Study".to_string(), "Observation".to_string()];

        // Default method multipliers
        let mut method_multipliers = std::collections::HashMap::new();
        method_multipliers.insert("Study".to_string(), 1.0);
        method_multipliers.insert("Observation".to_string(), 0.8);

        self.insert_comprehensive_theory(
            id, name, description, tier, category, prerequisites, complexity, learning_time,
            &scientific_concepts, applications, &available_methods, &method_multipliers
        )
    }

    /// Insert or update player theory progress
    pub fn save_theory_progress(
        &self,
        player_id: &str,
        theory_id: &str,
        understanding_level: f32,
        experience_points: i32,
        learning_history: &std::collections::HashMap<String, i32>,
        time_invested: i32,
        discovered_at: i64,
        mastered_at: Option<i64>,
        is_active_research: bool,
        research_progress: f32,
    ) -> GameResult<()> {
        let history_json = serde_json::to_string(learning_history)
            .map_err(|e| crate::GameError::DatabaseError(format!("Failed to serialize learning history: {}", e)))?;

        self.connection.execute(
            "INSERT OR REPLACE INTO player_theory_progress
             (player_id, theory_id, understanding_level, experience_points, learning_history,
              time_invested, discovered_at, mastered_at, is_active_research, research_progress)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![player_id, theory_id, understanding_level, experience_points, history_json,
                   time_invested, discovered_at, mastered_at, is_active_research, research_progress],
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to save theory progress: {}", e)))?;

        Ok(())
    }

    /// Log a learning activity
    pub fn log_learning_activity(
        &self,
        player_id: &str,
        theory_id: &str,
        method: &str,
        duration: i32,
        success_rate: f32,
        experience_gained: i32,
        understanding_gained: f32,
        resources_used: &std::collections::HashMap<String, i32>,
        side_effects: &[String],
        timestamp: i64,
    ) -> GameResult<()> {
        let resources_json = serde_json::to_string(resources_used)
            .map_err(|e| crate::GameError::DatabaseError(format!("Failed to serialize resources: {}", e)))?;
        let effects_json = serde_json::to_string(side_effects)
            .map_err(|e| crate::GameError::DatabaseError(format!("Failed to serialize side effects: {}", e)))?;

        self.connection.execute(
            "INSERT INTO learning_activities
             (player_id, theory_id, method, duration, success_rate, experience_gained,
              understanding_gained, resources_used, side_effects, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![player_id, theory_id, method, duration, success_rate, experience_gained,
                   understanding_gained, resources_json, effects_json, timestamp],
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to log learning activity: {}", e)))?;

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

    /// Load all magic theories from database with comprehensive data
    pub fn load_theories(&self) -> GameResult<HashMap<String, TheoryData>> {
        let mut theories = HashMap::new();

        // Try new schema first, fall back to old for compatibility
        let query = if self.has_enhanced_theory_schema()? {
            "SELECT id, name, description, tier, category, prerequisites, complexity_level,
             learning_time_base, scientific_concepts, applications, available_methods, method_multipliers
             FROM magic_theories"
        } else {
            "SELECT id, name, description, prerequisites, complexity_level, learning_time_base, applications
             FROM magic_theories"
        };

        let mut stmt = self.connection.prepare(query)
            .map_err(|e| crate::GameError::DatabaseError(format!("Failed to prepare theories query: {}", e)))?;

        let theory_rows = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let description: String = row.get(2)?;

            // Handle both old and new schema by checking column count
            let prerequisites_json: String = if self.has_enhanced_theory_schema().unwrap_or(false) {
                let _tier: i32 = row.get(3)?; // Skip for now, handled in knowledge system
                let _category: String = row.get(4)?; // Skip for now
                row.get(5)?
            } else {
                row.get(3)?
            };

            let (complexity_level, learning_time_base, applications_json): (i32, i32, String) =
                if self.has_enhanced_theory_schema().unwrap_or(false) {
                    let complexity_level: i32 = row.get(6)?;
                    let learning_time_base: i32 = row.get(7)?;
                    let _scientific_concepts_json: String = row.get(8)?; // Skip for now
                    let applications_json: String = row.get(9)?;
                    let _available_methods_json: String = row.get(10)?; // Skip for now
                    let _method_multipliers_json: String = row.get(11)?; // Skip for now
                    (complexity_level, learning_time_base, applications_json)
                } else {
                    let complexity_level: i32 = row.get(4)?;
                    let learning_time_base: i32 = row.get(5)?;
                    let applications_json: String = row.get(6)?;
                    (complexity_level, learning_time_base, applications_json)
                };

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

    /// Check if the database has the enhanced theory schema
    fn has_enhanced_theory_schema(&self) -> GameResult<bool> {
        let column_exists = self.connection
            .prepare("SELECT tier FROM magic_theories LIMIT 1")
            .is_ok();
        Ok(column_exists)
    }

    /// Load player theory progress for a specific player
    pub fn load_player_theory_progress(&self, player_id: &str) -> GameResult<HashMap<String, (f32, i32, std::collections::HashMap<String, i32>, i32, i64, Option<i64>, bool, f32)>> {
        let mut progress = HashMap::new();

        let mut stmt = self.connection.prepare(
            "SELECT theory_id, understanding_level, experience_points, learning_history,
             time_invested, discovered_at, mastered_at, is_active_research, research_progress
             FROM player_theory_progress WHERE player_id = ?1"
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to prepare progress query: {}", e)))?;

        let progress_rows = stmt.query_map([player_id], |row| {
            let theory_id: String = row.get(0)?;
            let understanding_level: f32 = row.get(1)?;
            let experience_points: i32 = row.get(2)?;
            let learning_history_json: String = row.get(3)?;
            let time_invested: i32 = row.get(4)?;
            let discovered_at: i64 = row.get(5)?;
            let mastered_at: Option<i64> = row.get(6)?;
            let is_active_research: bool = row.get(7)?;
            let research_progress: f32 = row.get(8)?;

            let learning_history: std::collections::HashMap<String, i32> = serde_json::from_str(&learning_history_json)
                .unwrap_or_else(|_| std::collections::HashMap::new());

            Ok((theory_id, (understanding_level, experience_points, learning_history, time_invested,
                           discovered_at, mastered_at, is_active_research, research_progress)))
        }).map_err(|e| crate::GameError::DatabaseError(format!("Failed to query progress: {}", e)))?;

        for progress_result in progress_rows {
            let (theory_id, progress_data) = progress_result
                .map_err(|e| crate::GameError::DatabaseError(format!("Failed to parse progress: {}", e)))?;
            progress.insert(theory_id, progress_data);
        }

        Ok(progress)
    }

    /// Load learning activities for a player and theory
    pub fn load_learning_activities(&self, player_id: &str, theory_id: Option<&str>, limit: Option<i32>) -> GameResult<Vec<(String, String, i32, f32, i32, f32, std::collections::HashMap<String, i32>, Vec<String>, i64)>> {
        let mut activities = Vec::new();

        let query = if theory_id.is_some() {
            "SELECT theory_id, method, duration, success_rate, experience_gained,
             understanding_gained, resources_used, side_effects, timestamp
             FROM learning_activities WHERE player_id = ?1 AND theory_id = ?2
             ORDER BY timestamp DESC"
        } else {
            "SELECT theory_id, method, duration, success_rate, experience_gained,
             understanding_gained, resources_used, side_effects, timestamp
             FROM learning_activities WHERE player_id = ?1
             ORDER BY timestamp DESC"
        };

        let query_with_limit = if let Some(limit_val) = limit {
            format!("{} LIMIT {}", query, limit_val)
        } else {
            query.to_string()
        };

        let mut stmt = self.connection.prepare(&query_with_limit)
            .map_err(|e| crate::GameError::DatabaseError(format!("Failed to prepare activities query: {}", e)))?;

        let parse_row = |row: &rusqlite::Row| -> rusqlite::Result<(String, String, i32, f32, i32, f32, std::collections::HashMap<String, i32>, Vec<String>, i64)> {
            let theory_id: String = row.get(0)?;
            let method: String = row.get(1)?;
            let duration: i32 = row.get(2)?;
            let success_rate: f32 = row.get(3)?;
            let experience_gained: i32 = row.get(4)?;
            let understanding_gained: f32 = row.get(5)?;
            let resources_used_json: String = row.get(6)?;
            let side_effects_json: String = row.get(7)?;
            let timestamp: i64 = row.get(8)?;

            let resources_used: std::collections::HashMap<String, i32> = serde_json::from_str(&resources_used_json)
                .unwrap_or_else(|_| std::collections::HashMap::new());
            let side_effects: Vec<String> = serde_json::from_str(&side_effects_json)
                .unwrap_or_else(|_| Vec::new());

            Ok((theory_id, method, duration, success_rate, experience_gained,
               understanding_gained, resources_used, side_effects, timestamp))
        };

        let activity_rows = if let Some(theory_id_val) = theory_id {
            stmt.query_map([player_id, theory_id_val], parse_row)
                .map_err(|e| crate::GameError::DatabaseError(format!("Failed to query activities: {}", e)))?
        } else {
            stmt.query_map([player_id], parse_row)
                .map_err(|e| crate::GameError::DatabaseError(format!("Failed to query activities: {}", e)))?
        };

        for activity_result in activity_rows {
            let activity_data = activity_result
                .map_err(|e| crate::GameError::DatabaseError(format!("Failed to parse activity: {}", e)))?;
            activities.push(activity_data);
        }

        Ok(activities)
    }

    /// Insert an NPC into the database
    pub fn insert_npc(
        &self,
        id: &str,
        name: &str,
        description: &str,
        faction_id: Option<&str>,
        dialogue_tree_json: &str,
        current_location: &str,
    ) -> GameResult<()> {
        self.connection.execute(
            "INSERT OR REPLACE INTO npcs
             (id, name, description, faction_id, dialogue_tree, current_location)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![id, name, description, faction_id, dialogue_tree_json, current_location],
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to insert NPC: {}", e)))?;

        Ok(())
    }

    /// Load all NPCs from the database
    pub fn load_npcs(&self) -> GameResult<Vec<crate::systems::dialogue::NPC>> {
        let mut stmt = self.connection.prepare(
            "SELECT id, name, description, faction_id, dialogue_tree FROM npcs"
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to prepare NPC query: {}", e)))?;

        let npc_rows = stmt.query_map([], |row| {
            let faction_str: Option<String> = row.get(3)?;
            let faction_id = faction_str.as_ref().map(|s| match s.as_str() {
                "magisters_council" => crate::systems::factions::FactionId::MagistersCouncil,
                "underground_network" => crate::systems::factions::FactionId::UndergroundNetwork,
                "order_of_harmony" => crate::systems::factions::FactionId::OrderOfHarmony,
                "industrial_consortium" => crate::systems::factions::FactionId::IndustrialConsortium,
                "neutral_scholars" => crate::systems::factions::FactionId::NeutralScholars,
                _ => crate::systems::factions::FactionId::NeutralScholars, // Default fallback
            });

            let dialogue_tree_json: String = row.get(4)?;
            let dialogue_tree: crate::systems::dialogue::DialogueTree =
                serde_json::from_str(&dialogue_tree_json)
                    .map_err(|_| rusqlite::Error::InvalidColumnType(4, "Invalid JSON".to_string(), rusqlite::types::Type::Text))?;

            Ok(crate::systems::dialogue::NPC {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                faction_affiliation: faction_id,
                dialogue_tree,
                current_disposition: 0, // Default neutral disposition
            })
        }).map_err(|e| crate::GameError::DatabaseError(format!("Failed to query NPCs: {}", e)))?;

        let mut npcs = Vec::new();
        for npc_result in npc_rows {
            let npc = npc_result
                .map_err(|e| crate::GameError::DatabaseError(format!("Failed to parse NPC: {}", e)))?;
            npcs.push(npc);
        }

        Ok(npcs)
    }

    /// Insert a quest definition into the database
    pub fn insert_quest_definition(&self, quest: &crate::systems::quests::QuestDefinition) -> GameResult<()> {
        let requirements_json = serde_json::to_string(&quest.requirements)
            .map_err(|e| crate::GameError::DatabaseError(format!("Failed to serialize requirements: {}", e)))?;
        let objectives_json = serde_json::to_string(&quest.objectives)
            .map_err(|e| crate::GameError::DatabaseError(format!("Failed to serialize objectives: {}", e)))?;
        let rewards_json = serde_json::to_string(&quest.rewards)
            .map_err(|e| crate::GameError::DatabaseError(format!("Failed to serialize rewards: {}", e)))?;
        let faction_effects_json = serde_json::to_string(&quest.faction_effects)
            .map_err(|e| crate::GameError::DatabaseError(format!("Failed to serialize faction effects: {}", e)))?;
        let educational_focus_json = serde_json::to_string(&quest.educational_focus)
            .map_err(|e| crate::GameError::DatabaseError(format!("Failed to serialize educational focus: {}", e)))?;
        let branching_paths_json = serde_json::to_string(&quest.branching_paths)
            .map_err(|e| crate::GameError::DatabaseError(format!("Failed to serialize branching paths: {}", e)))?;
        let involved_npcs_json = serde_json::to_string(&quest.involved_npcs)
            .map_err(|e| crate::GameError::DatabaseError(format!("Failed to serialize involved NPCs: {}", e)))?;
        let locations_json = serde_json::to_string(&quest.locations)
            .map_err(|e| crate::GameError::DatabaseError(format!("Failed to serialize locations: {}", e)))?;

        let category_str = match quest.category {
            crate::systems::quests::QuestCategory::Tutorial => "Tutorial",
            crate::systems::quests::QuestCategory::Research => "Research",
            crate::systems::quests::QuestCategory::Political => "Political",
            crate::systems::quests::QuestCategory::Practical => "Practical",
            crate::systems::quests::QuestCategory::Social => "Social",
            crate::systems::quests::QuestCategory::Experimental => "Experimental",
            crate::systems::quests::QuestCategory::Narrative => "Narrative",
        };

        let difficulty_str = match quest.difficulty {
            crate::systems::quests::QuestDifficulty::Beginner => "Beginner",
            crate::systems::quests::QuestDifficulty::Intermediate => "Intermediate",
            crate::systems::quests::QuestDifficulty::Advanced => "Advanced",
            crate::systems::quests::QuestDifficulty::Expert => "Expert",
            crate::systems::quests::QuestDifficulty::Master => "Master",
        };

        let now = chrono::Utc::now().timestamp();

        self.connection.execute(
            "INSERT OR REPLACE INTO quest_definitions
             (id, title, description, category, difficulty, requirements, objectives, rewards,
              faction_effects, educational_focus, branching_paths, involved_npcs, locations,
              estimated_duration, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
            params![
                quest.id, quest.title, quest.description, category_str, difficulty_str,
                requirements_json, objectives_json, rewards_json, faction_effects_json,
                educational_focus_json, branching_paths_json, involved_npcs_json, locations_json,
                quest.estimated_duration, now, now
            ],
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to insert quest definition: {}", e)))?;

        Ok(())
    }

    /// Load all quest definitions from the database
    pub fn load_quest_definitions(&self) -> GameResult<std::collections::HashMap<String, crate::systems::quests::QuestDefinition>> {
        let mut quests = std::collections::HashMap::new();

        let mut stmt = self.connection.prepare(
            "SELECT id, title, description, category, difficulty, requirements, objectives, rewards,
             faction_effects, educational_focus, branching_paths, involved_npcs, locations, estimated_duration
             FROM quest_definitions"
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to prepare quest query: {}", e)))?;

        let quest_rows = stmt.query_map([], |row| {
            let id: String = row.get(0)?;
            let title: String = row.get(1)?;
            let description: String = row.get(2)?;
            let category_str: String = row.get(3)?;
            let difficulty_str: String = row.get(4)?;
            let requirements_json: String = row.get(5)?;
            let objectives_json: String = row.get(6)?;
            let rewards_json: String = row.get(7)?;
            let faction_effects_json: String = row.get(8)?;
            let educational_focus_json: String = row.get(9)?;
            let branching_paths_json: String = row.get(10)?;
            let involved_npcs_json: String = row.get(11)?;
            let locations_json: String = row.get(12)?;
            let estimated_duration: i32 = row.get(13)?;

            let category = match category_str.as_str() {
                "Tutorial" => crate::systems::quests::QuestCategory::Tutorial,
                "Research" => crate::systems::quests::QuestCategory::Research,
                "Political" => crate::systems::quests::QuestCategory::Political,
                "Practical" => crate::systems::quests::QuestCategory::Practical,
                "Social" => crate::systems::quests::QuestCategory::Social,
                "Experimental" => crate::systems::quests::QuestCategory::Experimental,
                "Narrative" => crate::systems::quests::QuestCategory::Narrative,
                _ => crate::systems::quests::QuestCategory::Tutorial,
            };

            let difficulty = match difficulty_str.as_str() {
                "Beginner" => crate::systems::quests::QuestDifficulty::Beginner,
                "Intermediate" => crate::systems::quests::QuestDifficulty::Intermediate,
                "Advanced" => crate::systems::quests::QuestDifficulty::Advanced,
                "Expert" => crate::systems::quests::QuestDifficulty::Expert,
                "Master" => crate::systems::quests::QuestDifficulty::Master,
                _ => crate::systems::quests::QuestDifficulty::Beginner,
            };

            let requirements: crate::systems::quests::QuestRequirements = serde_json::from_str(&requirements_json)
                .map_err(|_| rusqlite::Error::InvalidColumnType(5, "Invalid requirements JSON".to_string(), rusqlite::types::Type::Text))?;
            let objectives: Vec<crate::systems::quests::QuestObjective> = serde_json::from_str(&objectives_json)
                .map_err(|_| rusqlite::Error::InvalidColumnType(6, "Invalid objectives JSON".to_string(), rusqlite::types::Type::Text))?;
            let rewards: crate::systems::quests::QuestRewards = serde_json::from_str(&rewards_json)
                .map_err(|_| rusqlite::Error::InvalidColumnType(7, "Invalid rewards JSON".to_string(), rusqlite::types::Type::Text))?;
            let faction_effects: std::collections::HashMap<crate::systems::factions::FactionId, i32> = serde_json::from_str(&faction_effects_json)
                .unwrap_or_else(|_| std::collections::HashMap::new());
            let educational_focus: crate::systems::quests::EducationalObjectives = serde_json::from_str(&educational_focus_json)
                .map_err(|_| rusqlite::Error::InvalidColumnType(9, "Invalid educational focus JSON".to_string(), rusqlite::types::Type::Text))?;
            let branching_paths: std::collections::HashMap<String, crate::systems::quests::QuestBranch> = serde_json::from_str(&branching_paths_json)
                .unwrap_or_else(|_| std::collections::HashMap::new());
            let involved_npcs: Vec<String> = serde_json::from_str(&involved_npcs_json)
                .unwrap_or_else(|_| Vec::new());
            let locations: Vec<String> = serde_json::from_str(&locations_json)
                .unwrap_or_else(|_| Vec::new());

            Ok((id.clone(), crate::systems::quests::QuestDefinition {
                id,
                title,
                description,
                category,
                difficulty,
                requirements,
                objectives,
                rewards,
                faction_effects,
                educational_focus,
                branching_paths,
                involved_npcs,
                locations,
                estimated_duration,
            }))
        }).map_err(|e| crate::GameError::DatabaseError(format!("Failed to query quest definitions: {}", e)))?;

        for quest_result in quest_rows {
            let (id, quest) = quest_result
                .map_err(|e| crate::GameError::DatabaseError(format!("Failed to parse quest definition: {}", e)))?;
            quests.insert(id, quest);
        }

        Ok(quests)
    }

    /// Save player quest progress to database
    pub fn save_quest_progress(&self, player_id: &str, progress: &crate::systems::quests::QuestProgress) -> GameResult<()> {
        let objective_progress_json = serde_json::to_string(&progress.objective_progress)
            .map_err(|e| crate::GameError::DatabaseError(format!("Failed to serialize objective progress: {}", e)))?;
        let player_choices_json = serde_json::to_string(&progress.player_choices)
            .map_err(|e| crate::GameError::DatabaseError(format!("Failed to serialize player choices: {}", e)))?;
        let quest_variables_json = serde_json::to_string(&progress.quest_variables)
            .map_err(|e| crate::GameError::DatabaseError(format!("Failed to serialize quest variables: {}", e)))?;
        let learning_progress_json = serde_json::to_string(&progress.learning_progress)
            .map_err(|e| crate::GameError::DatabaseError(format!("Failed to serialize learning progress: {}", e)))?;

        let status_str = match progress.status {
            crate::systems::quests::QuestStatus::Available => "Available",
            crate::systems::quests::QuestStatus::NotAvailable => "NotAvailable",
            crate::systems::quests::QuestStatus::InProgress => "InProgress",
            crate::systems::quests::QuestStatus::Completed => "Completed",
            crate::systems::quests::QuestStatus::Failed => "Failed",
            crate::systems::quests::QuestStatus::Abandoned => "Abandoned",
        };

        self.connection.execute(
            "INSERT OR REPLACE INTO player_quest_progress
             (player_id, quest_id, status, started_at, completed_at, objective_progress,
              chosen_branch, player_choices, time_invested, quest_variables, learning_progress)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                player_id, progress.quest_id, status_str, progress.started_at.timestamp(),
                progress.completed_at.map(|dt| dt.timestamp()), objective_progress_json,
                progress.chosen_branch, player_choices_json, progress.time_invested,
                quest_variables_json, learning_progress_json
            ],
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to save quest progress: {}", e)))?;

        Ok(())
    }

    /// Load player quest progress from database
    pub fn load_quest_progress(&self, player_id: &str) -> GameResult<std::collections::HashMap<String, crate::systems::quests::QuestProgress>> {
        let mut progress_map = std::collections::HashMap::new();

        let mut stmt = self.connection.prepare(
            "SELECT quest_id, status, started_at, completed_at, objective_progress,
             chosen_branch, player_choices, time_invested, quest_variables, learning_progress
             FROM player_quest_progress WHERE player_id = ?1"
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to prepare quest progress query: {}", e)))?;

        let progress_rows = stmt.query_map([player_id], |row| {
            let quest_id: String = row.get(0)?;
            let status_str: String = row.get(1)?;
            let started_at_timestamp: i64 = row.get(2)?;
            let completed_at_timestamp: Option<i64> = row.get(3)?;
            let objective_progress_json: String = row.get(4)?;
            let chosen_branch: Option<String> = row.get(5)?;
            let player_choices_json: String = row.get(6)?;
            let time_invested: i32 = row.get(7)?;
            let quest_variables_json: String = row.get(8)?;
            let learning_progress_json: String = row.get(9)?;

            let status = match status_str.as_str() {
                "Available" => crate::systems::quests::QuestStatus::Available,
                "NotAvailable" => crate::systems::quests::QuestStatus::NotAvailable,
                "InProgress" => crate::systems::quests::QuestStatus::InProgress,
                "Completed" => crate::systems::quests::QuestStatus::Completed,
                "Failed" => crate::systems::quests::QuestStatus::Failed,
                "Abandoned" => crate::systems::quests::QuestStatus::Abandoned,
                _ => crate::systems::quests::QuestStatus::NotAvailable,
            };

            let started_at = chrono::DateTime::from_timestamp(started_at_timestamp, 0)
                .unwrap_or_else(|| chrono::Utc::now());
            let completed_at = completed_at_timestamp.and_then(|ts| chrono::DateTime::from_timestamp(ts, 0));

            let objective_progress: std::collections::HashMap<String, crate::systems::quests::ObjectiveProgress> =
                serde_json::from_str(&objective_progress_json).unwrap_or_else(|_| std::collections::HashMap::new());
            let player_choices: std::collections::HashMap<String, String> =
                serde_json::from_str(&player_choices_json).unwrap_or_else(|_| std::collections::HashMap::new());
            let quest_variables: std::collections::HashMap<String, String> =
                serde_json::from_str(&quest_variables_json).unwrap_or_else(|_| std::collections::HashMap::new());
            let learning_progress: crate::systems::quests::QuestLearningProgress =
                serde_json::from_str(&learning_progress_json).unwrap_or_else(|_| crate::systems::quests::QuestLearningProgress {
                    mastered_concepts: Vec::new(),
                    demonstrated_methods: Vec::new(),
                    assessment_scores: std::collections::HashMap::new(),
                    learning_metrics: crate::systems::quests::LearningMetrics {
                        completion_efficiency: 0.0,
                        first_attempt_success_rate: 0.0,
                        help_requests: 0,
                        application_accuracy: 0.0,
                    },
                });

            Ok((quest_id.clone(), crate::systems::quests::QuestProgress {
                quest_id,
                status,
                started_at,
                completed_at,
                objective_progress,
                chosen_branch,
                player_choices,
                time_invested,
                quest_variables,
                learning_progress,
            }))
        }).map_err(|e| crate::GameError::DatabaseError(format!("Failed to query quest progress: {}", e)))?;

        for progress_result in progress_rows {
            let (quest_id, progress) = progress_result
                .map_err(|e| crate::GameError::DatabaseError(format!("Failed to parse quest progress: {}", e)))?;
            progress_map.insert(quest_id, progress);
        }

        Ok(progress_map)
    }

    /// Log quest objective completion
    pub fn log_quest_objective_completion(
        &self,
        player_id: &str,
        quest_id: &str,
        objective_id: &str,
        progress_value: f32,
        completion_method: Option<&str>,
        learning_data: &std::collections::HashMap<String, String>,
    ) -> GameResult<()> {
        let learning_data_json = serde_json::to_string(learning_data)
            .map_err(|e| crate::GameError::DatabaseError(format!("Failed to serialize learning data: {}", e)))?;

        self.connection.execute(
            "INSERT INTO quest_objective_log
             (player_id, quest_id, objective_id, completed_at, progress_value, completion_method, learning_data)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                player_id, quest_id, objective_id, chrono::Utc::now().timestamp(),
                progress_value, completion_method, learning_data_json
            ],
        ).map_err(|e| crate::GameError::DatabaseError(format!("Failed to log quest objective completion: {}", e)))?;

        Ok(())
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

    fn create_test_db() -> (DatabaseManager, NamedTempFile) {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let db = DatabaseManager::new(db_path).unwrap();
        db.initialize_schema().unwrap();
        (db, temp_file)
    }

    #[test]
    fn test_database_creation() {
        let (_db, _temp_file) = create_test_db();
        // If we get here without panic, database creation worked
    }

    #[test]
    fn test_location_insertion_and_loading() {
        let (db, _temp_file) = create_test_db();

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
        let (db, _temp_file) = create_test_db();

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
        let (db, _temp_file) = create_test_db();

        db.insert_location("room1", "Room 1", "First room", 1.0, None, 0.0, &[]).unwrap();
        db.insert_location("room2", "Room 2", "Second room", 1.0, None, 0.0, &[]).unwrap();
        db.insert_exit("room1", "north", "room2").unwrap();

        let locations = db.load_locations().unwrap();
        let room1 = &locations["room1"];

        assert!(room1.exits.contains_key(&Direction::North));
        assert_eq!(room1.exits[&Direction::North], "room2");
    }
}