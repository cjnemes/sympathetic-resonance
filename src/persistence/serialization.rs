//! Game state serialization and deserialization
//!
//! This module handles converting game state to/from storage format

use crate::core::{Player, WorldState};
use crate::systems::quests::QuestSystem;
use crate::GameResult;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Complete serializable game state
#[derive(Debug, Serialize, Deserialize)]
pub struct GameStateData {
    /// Format version for migration compatibility
    pub version: u32,
    /// When this save was created
    pub timestamp: DateTime<Utc>,
    /// Player character state
    pub player: Player,
    /// World state and location data
    pub world: WorldState,
    /// Quest system state and progress
    pub quest_system: QuestSystem,
    /// Save metadata
    pub metadata: SaveMetadata,
}

/// Metadata about the save file
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveMetadata {
    /// Player-provided save name
    pub save_name: String,
    /// Total playtime when saved
    pub playtime_minutes: i32,
    /// Current location name for display
    pub location_name: String,
    /// Game version when saved
    pub game_version: String,
}

/// Current save format version
const SAVE_FORMAT_VERSION: u32 = 1;

/// Serialize complete game state to JSON
pub fn serialize_game_state(
    player: &Player,
    world: &WorldState,
    quest_system: &QuestSystem,
    save_name: Option<String>,
) -> GameResult<String> {
    let location_name = world.current_location()
        .map(|loc| loc.name.clone())
        .unwrap_or_else(|| "Unknown Location".to_string());

    let save_name = save_name.unwrap_or_else(|| {
        format!("Save {}", chrono::Utc::now().format("%Y-%m-%d %H:%M"))
    });

    let game_state = GameStateData {
        version: SAVE_FORMAT_VERSION,
        timestamp: Utc::now(),
        player: player.clone(),
        world: world.clone(),
        quest_system: quest_system.clone(),
        metadata: SaveMetadata {
            save_name,
            playtime_minutes: player.playtime_minutes,
            location_name,
            game_version: crate::VERSION.to_string(),
        },
    };

    serde_json::to_string_pretty(&game_state)
        .map_err(|e| crate::GameError::SaveLoadError(format!("Serialization failed: {}", e)).into())
}

/// Deserialize game state from JSON
pub fn deserialize_game_state(data: &str) -> GameResult<(Player, WorldState, QuestSystem)> {
    let game_state: GameStateData = serde_json::from_str(data)
        .map_err(|e| crate::GameError::SaveLoadError(format!("Deserialization failed: {}", e)))?;

    // Validate version compatibility
    if game_state.version > SAVE_FORMAT_VERSION {
        return Err(crate::GameError::SaveLoadError(
            format!("Save file version {} is newer than supported version {}",
                   game_state.version, SAVE_FORMAT_VERSION)
        ).into());
    }

    // Perform any necessary migrations
    let migrated_state = migrate_save_data(game_state)?;

    Ok((migrated_state.player, migrated_state.world, migrated_state.quest_system))
}

/// Migrate save data between versions
fn migrate_save_data(mut state: GameStateData) -> GameResult<GameStateData> {
    match state.version {
        1 => {
            // Current version, no migration needed
            Ok(state)
        }
        0 => {
            // Hypothetical migration from version 0 to 1
            state.version = 1;
            // Add any necessary data transformations here
            Ok(state)
        }
        _ => {
            Err(crate::GameError::SaveLoadError(
                format!("Unknown save version: {}", state.version)
            ).into())
        }
    }
}

/// Validate game state integrity
pub fn validate_game_state(state: &GameStateData) -> GameResult<()> {
    // Check player data integrity
    if state.player.name.is_empty() {
        return Err(crate::GameError::SaveLoadError("Invalid player name".to_string()).into());
    }

    // Check attribute bounds
    if state.player.attributes.mental_acuity < 0 || state.player.attributes.mental_acuity > 100 {
        return Err(crate::GameError::SaveLoadError("Invalid mental acuity value".to_string()).into());
    }

    if state.player.attributes.resonance_sensitivity < 0 || state.player.attributes.resonance_sensitivity > 100 {
        return Err(crate::GameError::SaveLoadError("Invalid resonance sensitivity value".to_string()).into());
    }

    // Check mental state bounds
    if state.player.mental_state.fatigue < 0 || state.player.mental_state.fatigue > 100 {
        return Err(crate::GameError::SaveLoadError("Invalid fatigue value".to_string()).into());
    }

    // Check crystal integrity
    for crystal in &state.player.inventory.crystals {
        if crystal.integrity < 0.0 || crystal.integrity > 100.0 {
            return Err(crate::GameError::SaveLoadError("Invalid crystal integrity".to_string()).into());
        }
        if crystal.purity < 0.0 || crystal.purity > 1.0 {
            return Err(crate::GameError::SaveLoadError("Invalid crystal purity".to_string()).into());
        }
    }

    // Check faction reputation bounds
    for &reputation in state.player.faction_standings.values() {
        if !(-100..=100).contains(&reputation) {
            return Err(crate::GameError::SaveLoadError("Invalid faction reputation".to_string()).into());
        }
    }

    // Check world state
    if state.world.current_location.is_empty() {
        return Err(crate::GameError::SaveLoadError("Invalid current location".to_string()).into());
    }

    Ok(())
}

/// Create a quick save summary for display
pub fn create_save_summary(state: &GameStateData) -> String {
    let hours = state.metadata.playtime_minutes / 60;
    let minutes = state.metadata.playtime_minutes % 60;

    format!(
        "{}\n\
         Character: {}\n\
         Location: {}\n\
         Playtime: {}h {}m\n\
         Saved: {}",
        state.metadata.save_name,
        state.player.name,
        state.metadata.location_name,
        hours, minutes,
        state.timestamp.format("%Y-%m-%d %H:%M UTC")
    )
}

/// Extract minimal info for save file listing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveFileInfo {
    pub save_name: String,
    pub character_name: String,
    pub location_name: String,
    pub playtime_minutes: i32,
    pub timestamp: DateTime<Utc>,
    pub game_version: String,
}

impl From<&GameStateData> for SaveFileInfo {
    fn from(state: &GameStateData) -> Self {
        Self {
            save_name: state.metadata.save_name.clone(),
            character_name: state.player.name.clone(),
            location_name: state.metadata.location_name.clone(),
            playtime_minutes: state.metadata.playtime_minutes,
            timestamp: state.timestamp,
            game_version: state.metadata.game_version.clone(),
        }
    }
}

/// Compress save data for storage efficiency
pub fn compress_save_data(data: &str) -> GameResult<Vec<u8>> {
    // For now, just convert to bytes. In the future, could add compression
    Ok(data.as_bytes().to_vec())
}

/// Decompress save data from storage
pub fn decompress_save_data(data: &[u8]) -> GameResult<String> {
    String::from_utf8(data.to_vec())
        .map_err(|e| crate::GameError::SaveLoadError(format!("Invalid UTF-8 in save data: {}", e)).into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Player;
    use crate::systems::quests::QuestSystem;

    #[test]
    fn test_serialization_roundtrip() {
        let player = Player::new("Test Player".to_string());
        let world = WorldState::new();
        let quest_system = QuestSystem::new();

        let serialized = serialize_game_state(&player, &world, &quest_system, Some("Test Save".to_string())).unwrap();
        let (loaded_player, _loaded_world, _loaded_quest_system) = deserialize_game_state(&serialized).unwrap();

        assert_eq!(loaded_player.name, "Test Player");
    }

    #[test]
    fn test_validation() {
        let player = Player::new("Test Player".to_string());
        let world = WorldState::new();
        let quest_system = QuestSystem::new();

        let serialized = serialize_game_state(&player, &world, &quest_system, None).unwrap();
        let game_state_data = serde_json::from_str::<GameStateData>(&serialized).unwrap();

        assert!(validate_game_state(&game_state_data).is_ok());
    }

    #[test]
    fn test_invalid_data_validation() {
        let mut player = Player::new("Test Player".to_string());
        player.attributes.mental_acuity = 150; // Invalid value

        let world = WorldState::new();
        let quest_system = QuestSystem::new();
        let serialized = serialize_game_state(&player, &world, &quest_system, None).unwrap();
        let game_state_data = serde_json::from_str::<GameStateData>(&serialized).unwrap();

        assert!(validate_game_state(&game_state_data).is_err());
    }

    #[test]
    fn test_save_summary_creation() {
        let player = Player::new("Hero".to_string());
        let world = WorldState::new();
        let quest_system = QuestSystem::new();

        let serialized = serialize_game_state(&player, &world, &quest_system, Some("Epic Adventure".to_string())).unwrap();
        let game_state_data = serde_json::from_str::<GameStateData>(&serialized).unwrap();
        let summary = create_save_summary(&game_state_data);

        assert!(summary.contains("Epic Adventure"));
        assert!(summary.contains("Hero"));
    }

    #[test]
    fn test_compression_roundtrip() {
        let test_data = "Test save data";
        let compressed = compress_save_data(test_data).unwrap();
        let decompressed = decompress_save_data(&compressed).unwrap();

        assert_eq!(test_data, decompressed);
    }
}