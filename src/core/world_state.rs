//! World state management and location tracking
//!
//! This module handles:
//! - Current game world state and environmental conditions
//! - Location management and connections
//! - Environmental storytelling through magical signatures
//! - Time tracking and world events

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::GameResult;

/// Complete world state including location, environment, and time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldState {
    /// Current time in game (minutes since start)
    pub game_time_minutes: i32,
    /// Current location ID
    pub current_location: String,
    /// All locations with their current state
    pub locations: HashMap<String, Location>,
    /// Global environmental conditions
    pub environment: EnvironmentState,
    /// Active world events and their states
    pub events: HashMap<String, WorldEvent>,
}

/// A single location in the game world
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    /// Unique identifier for this location
    pub id: String,
    /// Display name shown to player
    pub name: String,
    /// Rich description including magical signatures
    pub description: String,
    /// Available exits and their destinations
    #[serde(
        serialize_with = "crate::systems::serde_helpers::serialize_direction_map",
        deserialize_with = "crate::systems::serde_helpers::deserialize_direction_map"
    )]
    pub exits: HashMap<Direction, String>,
    /// NPCs currently in this location
    pub npcs: Vec<String>,
    /// Items available in this location
    pub items: Vec<String>,
    /// Magical properties of this location
    pub magical_properties: MagicalProperties,
    /// Faction presence and control level
    pub faction_presence: HashMap<String, FactionPresence>,
    /// Whether this location has been visited by the player
    pub visited: bool,
}

/// Cardinal and special directions for movement
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Northeast,
    Northwest,
    Southeast,
    Southwest,
    Up,
    Down,
    In,
    Out,
    Enter(String), // Enter specific location like "enter building"
}

/// Magical properties that affect spellcasting in this location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagicalProperties {
    /// Ambient magical energy level (0.0-2.0, 1.0 is normal)
    pub ambient_energy: f32,
    /// Dominant resonance frequency in this area (1-10)
    pub dominant_frequency: Option<i32>,
    /// Magical interference level (0.0-1.0, higher = more interference)
    pub interference: f32,
    /// Recent magical activity signatures
    pub recent_activity: Vec<MagicalSignature>,
    /// Permanent magical phenomena in this location
    pub phenomena: Vec<String>,
}

/// Signature left by recent magical activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagicalSignature {
    /// Type of magic that was performed
    pub magic_type: String,
    /// Strength of the signature (0.0-1.0)
    pub strength: f32,
    /// How many game minutes ago this occurred
    pub age_minutes: i32,
    /// Resonance frequency used
    pub frequency: i32,
}

/// Faction presence in a location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionPresence {
    /// Influence level (0-100)
    pub influence: i32,
    /// Visibility of presence (Hidden, Subtle, Open, Dominant)
    pub visibility: PresenceVisibility,
    /// Number of faction members present
    pub member_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PresenceVisibility {
    Hidden,    // Faction presence not obvious to casual observation
    Subtle,    // Careful observation reveals faction influence
    Open,      // Faction presence is clear and acknowledged
    Dominant,  // Faction clearly controls this location
}

/// Global environmental conditions affecting magic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentState {
    /// Weather conditions affecting outdoor magic
    pub weather: Weather,
    /// Time of day affecting certain magical phenomena
    pub time_of_day: TimeOfDay,
    /// Seasonal effects on magic
    pub season: Season,
    /// Global magical disturbances
    pub disturbances: Vec<GlobalDisturbance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Weather {
    Clear,      // No weather effects
    Cloudy,     // Minor reduction in solar-based magic
    Rainy,      // Water magic enhanced, fire magic reduced
    Stormy,     // Electrical interference with all magic
    Foggy,      // Scrying and detection magic impaired
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeOfDay {
    Dawn,       // Transition magic enhanced
    Morning,    // Light magic enhanced
    Midday,     // Solar magic at peak
    Afternoon,  // Stable conditions
    Evening,    // Transition magic enhanced
    Night,      // Shadow magic enhanced
    Midnight,   // Dark magic at peak
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Season {
    Spring,     // Growth magic enhanced
    Summer,     // Fire and light magic enhanced
    Autumn,     // Decay and change magic enhanced
    Winter,     // Ice and preservation magic enhanced
}

/// Major world events that affect multiple locations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldEvent {
    /// Unique event identifier
    pub id: String,
    /// Event name and description
    pub name: String,
    /// Current progress or state of the event (0.0-1.0)
    pub progress: f32,
    /// Locations affected by this event
    pub affected_locations: Vec<String>,
    /// How this event modifies magical properties
    pub magical_effects: HashMap<String, f32>,
    /// Whether this event is currently active
    pub active: bool,
}

/// Global magical disturbance affecting wide areas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalDisturbance {
    /// Type of disturbance
    pub disturbance_type: DisturbanceType,
    /// Strength of the disturbance (0.0-1.0)
    pub strength: f32,
    /// Duration in game minutes
    pub duration_minutes: i32,
    /// Time when disturbance started
    pub start_time: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DisturbanceType {
    MagicalStorm,      // Random interference with all magic
    ResonanceFlux,     // Shifting resonance frequencies
    EnergyDrain,       // Reduced magical effectiveness
    Amplification,     // Enhanced magical effects (dangerous)
}

impl Default for WorldState {
    fn default() -> Self {
        Self::new()
    }
}

impl WorldState {
    /// Create a new world state with default values
    pub fn new() -> Self {
        Self {
            game_time_minutes: 0,
            current_location: "tutorial_chamber".to_string(),
            locations: HashMap::new(),
            environment: EnvironmentState {
                weather: Weather::Clear,
                time_of_day: TimeOfDay::Morning,
                season: Season::Spring,
                disturbances: Vec::new(),
            },
            events: HashMap::new(),
        }
    }

    /// Get the current location
    pub fn current_location(&self) -> Option<&Location> {
        self.locations.get(&self.current_location)
    }

    /// Get current location mutably
    pub fn current_location_mut(&mut self) -> Option<&mut Location> {
        self.locations.get_mut(&self.current_location)
    }

    /// Move to a new location if possible
    pub fn move_to_location(&mut self, direction: Direction) -> GameResult<String> {
        // Get destination before any mutable operations
        let destination = {
            let current_location = self.current_location()
                .ok_or_else(|| crate::GameError::ContentNotFound(
                    format!("Current location '{}' not found", self.current_location)
                ))?;

            current_location.exits.get(&direction)
                .ok_or_else(|| crate::GameError::InvalidCommand(
                    "You can't go that way".to_string()
                ))?
                .clone()
        };

        if !self.locations.contains_key(&destination) {
            return Err(crate::GameError::ContentNotFound(
                format!("Destination '{}' not found", destination)
            ).into());
        }

        // Mark new location as visited
        if let Some(location) = self.locations.get_mut(&destination) {
            location.visited = true;
        }

        self.current_location = destination.clone();
        Ok(destination)
    }

    /// Add a location to the world
    pub fn add_location(&mut self, location: Location) {
        self.locations.insert(location.id.clone(), location);
    }

    /// Advance game time and update world state
    pub fn advance_time(&mut self, minutes: i32) {
        self.game_time_minutes += minutes;

        // Update time of day
        let hour_of_day = (self.game_time_minutes / 60) % 24;
        self.environment.time_of_day = match hour_of_day {
            5..=6 => TimeOfDay::Dawn,
            7..=11 => TimeOfDay::Morning,
            12..=13 => TimeOfDay::Midday,
            14..=17 => TimeOfDay::Afternoon,
            18..=19 => TimeOfDay::Evening,
            20..=23 => TimeOfDay::Night,
            _ => TimeOfDay::Midnight,
        };

        // Age magical signatures
        for location in self.locations.values_mut() {
            for signature in &mut location.magical_properties.recent_activity {
                signature.age_minutes += minutes;
            }
            // Remove old signatures (older than 2 hours)
            location.magical_properties.recent_activity
                .retain(|sig| sig.age_minutes < 120);
        }

        // Update global disturbances
        self.environment.disturbances.retain_mut(|disturbance| {
            let elapsed = self.game_time_minutes - disturbance.start_time;
            elapsed < disturbance.duration_minutes
        });
    }

    /// Add a magical signature to current location
    pub fn add_magical_signature(&mut self, magic_type: String, strength: f32, frequency: i32) {
        if let Some(location) = self.current_location_mut() {
            location.magical_properties.recent_activity.push(MagicalSignature {
                magic_type,
                strength,
                age_minutes: 0,
                frequency,
            });
        }
    }

    /// Calculate magical modifier for current location and environment
    pub fn calculate_magical_modifier(&self, spell_frequency: i32) -> f32 {
        let mut modifier = 1.0;

        if let Some(location) = self.current_location() {
            // Ambient energy affects all magic
            modifier *= location.magical_properties.ambient_energy;

            // Frequency resonance with location
            if let Some(dominant_freq) = location.magical_properties.dominant_frequency {
                let frequency_diff = (spell_frequency - dominant_freq).abs();
                if frequency_diff == 0 {
                    modifier *= 1.2; // Perfect resonance bonus
                } else if frequency_diff <= 2 {
                    modifier *= 1.1; // Good resonance
                } else if frequency_diff >= 5 {
                    modifier *= 0.9; // Poor resonance
                }
            }

            // Interference reduces effectiveness
            modifier *= 1.0 - location.magical_properties.interference;
        }

        // Weather effects
        match self.environment.weather {
            Weather::Clear => {}, // No effect
            Weather::Cloudy => modifier *= 0.95,
            Weather::Rainy => modifier *= 0.9,
            Weather::Stormy => modifier *= 0.8,
            Weather::Foggy => modifier *= 0.9,
        }

        // Time of day effects
        match self.environment.time_of_day {
            TimeOfDay::Dawn | TimeOfDay::Evening => modifier *= 1.05, // Transition times
            TimeOfDay::Midday => modifier *= 1.1, // Peak energy
            TimeOfDay::Midnight => modifier *= 1.05, // Mystical hour
            _ => {}, // No special effect
        }

        // Global disturbances
        for disturbance in &self.environment.disturbances {
            match disturbance.disturbance_type {
                DisturbanceType::MagicalStorm => {
                    modifier *= 1.0 - (disturbance.strength * 0.3);
                }
                DisturbanceType::ResonanceFlux => {
                    modifier *= 1.0 - (disturbance.strength * 0.2);
                }
                DisturbanceType::EnergyDrain => {
                    modifier *= 1.0 - (disturbance.strength * 0.4);
                }
                DisturbanceType::Amplification => {
                    modifier *= 1.0 + (disturbance.strength * 0.3);
                }
            }
        }

        modifier.max(0.1) // Minimum 10% effectiveness
    }

    /// Get available exits from current location
    pub fn available_exits(&self) -> Vec<(Direction, String)> {
        if let Some(location) = self.current_location() {
            location.exits.iter()
                .map(|(dir, dest)| {
                    let dest_name = self.locations.get(dest)
                        .map(|loc| loc.name.clone())
                        .unwrap_or_else(|| dest.clone());
                    (dir.clone(), dest_name)
                })
                .collect()
        } else {
            Vec::new()
        }
    }
}

impl Location {
    /// Create a new location
    pub fn new(id: String, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
            exits: HashMap::new(),
            npcs: Vec::new(),
            items: Vec::new(),
            magical_properties: MagicalProperties {
                ambient_energy: 1.0,
                dominant_frequency: None,
                interference: 0.0,
                recent_activity: Vec::new(),
                phenomena: Vec::new(),
            },
            faction_presence: HashMap::new(),
            visited: false,
        }
    }

    /// Add an exit to another location
    pub fn add_exit(&mut self, direction: Direction, destination: String) {
        self.exits.insert(direction, destination);
    }

    /// Check if location has significant faction presence
    pub fn dominant_faction(&self) -> Option<(&String, &FactionPresence)> {
        self.faction_presence.iter()
            .max_by_key(|(_, presence)| presence.influence)
            .filter(|(_, presence)| presence.influence >= 50)
    }
}

impl Direction {
    /// Parse direction from player input
    pub fn from_string(input: &str) -> Option<Self> {
        match input.to_lowercase().as_str() {
            "north" | "n" => Some(Direction::North),
            "south" | "s" => Some(Direction::South),
            "east" | "e" => Some(Direction::East),
            "west" | "w" => Some(Direction::West),
            "northeast" | "ne" => Some(Direction::Northeast),
            "northwest" | "nw" => Some(Direction::Northwest),
            "southeast" | "se" => Some(Direction::Southeast),
            "southwest" | "sw" => Some(Direction::Southwest),
            "up" | "u" => Some(Direction::Up),
            "down" | "d" => Some(Direction::Down),
            "in" => Some(Direction::In),
            "out" => Some(Direction::Out),
            _ => None,
        }
    }

    /// Get display name for direction
    pub fn display_name(&self) -> &str {
        match self {
            Direction::North => "north",
            Direction::South => "south",
            Direction::East => "east",
            Direction::West => "west",
            Direction::Northeast => "northeast",
            Direction::Northwest => "northwest",
            Direction::Southeast => "southeast",
            Direction::Southwest => "southwest",
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::In => "in",
            Direction::Out => "out",
            Direction::Enter(name) => name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_state_creation() {
        let world = WorldState::new();
        assert_eq!(world.game_time_minutes, 0);
        assert_eq!(world.current_location, "tutorial_chamber");
    }

    #[test]
    fn test_location_creation() {
        let location = Location::new(
            "test_room".to_string(),
            "Test Room".to_string(),
            "A simple test room.".to_string(),
        );
        assert_eq!(location.id, "test_room");
        assert_eq!(location.name, "Test Room");
        assert!(!location.visited);
    }

    #[test]
    fn test_movement() {
        let mut world = WorldState::new();

        // Create test locations
        let mut start = Location::new(
            "start".to_string(),
            "Starting Room".to_string(),
            "The beginning.".to_string(),
        );
        start.add_exit(Direction::North, "end".to_string());

        let end = Location::new(
            "end".to_string(),
            "End Room".to_string(),
            "The destination.".to_string(),
        );

        world.add_location(start);
        world.add_location(end);
        world.current_location = "start".to_string();

        let result = world.move_to_location(Direction::North);
        assert!(result.is_ok());
        assert_eq!(world.current_location, "end");
    }

    #[test]
    fn test_magical_modifier_calculation() {
        let mut world = WorldState::new();

        let mut location = Location::new(
            "magic_room".to_string(),
            "Magic Room".to_string(),
            "A room with strong magical properties.".to_string(),
        );
        location.magical_properties.ambient_energy = 1.5;
        location.magical_properties.dominant_frequency = Some(4);

        world.add_location(location);
        world.current_location = "magic_room".to_string();

        let modifier = world.calculate_magical_modifier(4); // Perfect frequency match
        assert!(modifier > 1.0); // Should be enhanced
    }

    #[test]
    fn test_direction_parsing() {
        assert_eq!(Direction::from_string("north"), Some(Direction::North));
        assert_eq!(Direction::from_string("n"), Some(Direction::North));
        assert_eq!(Direction::from_string("invalid"), None);
    }
}