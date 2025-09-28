//! Save and load system for game state persistence
//!
//! This module provides file-based save/load functionality with
//! multiple save slots and backup management

use crate::core::{Player, WorldState};
use crate::persistence::serialization::{
    SaveFileInfo, serialize_game_state, deserialize_game_state,
    validate_game_state, compress_save_data, decompress_save_data
};
use crate::GameResult;
use std::fs;
use std::path::{Path, PathBuf};

/// Manages save file operations
pub struct SaveManager {
    /// Directory for save files
    save_directory: PathBuf,
    /// Maximum number of auto-save backups to keep
    max_backups: usize,
}

/// Information about an available save slot
#[derive(Debug, Clone)]
pub struct SaveSlot {
    pub slot_name: String,
    pub file_path: PathBuf,
    pub info: Option<SaveFileInfo>,
    pub exists: bool,
}

impl SaveManager {
    /// Create a new save manager
    pub fn new() -> GameResult<Self> {
        let save_directory = Self::get_save_directory()?;

        // Create save directory if it doesn't exist
        if !save_directory.exists() {
            fs::create_dir_all(&save_directory)
                .map_err(|e| crate::GameError::SaveLoadError(format!("Failed to create save directory: {}", e)))?;
        }

        Ok(Self {
            save_directory,
            max_backups: 5,
        })
    }

    /// Get the platform-appropriate save directory
    fn get_save_directory() -> GameResult<PathBuf> {
        let mut path = dirs::home_dir()
            .ok_or_else(|| crate::GameError::SaveLoadError("Cannot find home directory".to_string()))?;

        // Platform-specific save locations
        #[cfg(target_os = "windows")]
        {
            path.push("AppData");
            path.push("Local");
            path.push("SympatheticResonance");
            path.push("saves");
        }

        #[cfg(target_os = "macos")]
        {
            path.push("Library");
            path.push("Application Support");
            path.push("SympatheticResonance");
            path.push("saves");
        }

        #[cfg(target_os = "linux")]
        {
            path.push(".local");
            path.push("share");
            path.push("sympathetic-resonance");
            path.push("saves");
        }

        // Fallback for other platforms
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            path.push(".sympathetic-resonance");
            path.push("saves");
        }

        Ok(path)
    }

    /// Save game state to specified slot
    pub fn save_game(
        &self,
        player: &Player,
        world: &WorldState,
        slot_name: Option<String>,
        save_name: Option<String>,
    ) -> GameResult<String> {
        let slot = slot_name.unwrap_or_else(|| "quicksave".to_string());
        let file_path = self.get_save_file_path(&slot);

        // Create backup if file exists
        if file_path.exists() {
            self.create_backup(&file_path)?;
        }

        // Serialize game state
        let serialized_data = serialize_game_state(player, world, save_name)?;

        // Compress data
        let compressed_data = compress_save_data(&serialized_data)?;

        // Write to file
        fs::write(&file_path, compressed_data)
            .map_err(|e| crate::GameError::SaveLoadError(format!("Failed to write save file: {}", e)))?;

        Ok(format!("Game saved to slot '{}'", slot))
    }

    /// Load game state from specified slot
    pub fn load_game(&self, slot_name: &str) -> GameResult<(Player, WorldState)> {
        let file_path = self.get_save_file_path(slot_name);

        if !file_path.exists() {
            return Err(crate::GameError::SaveLoadError(
                format!("Save file '{}' does not exist", slot_name)
            ).into());
        }

        // Read compressed data
        let compressed_data = fs::read(&file_path)
            .map_err(|e| crate::GameError::SaveLoadError(format!("Failed to read save file: {}", e)))?;

        // Decompress data
        let serialized_data = decompress_save_data(&compressed_data)?;

        // Deserialize game state
        let game_state = deserialize_game_state(&serialized_data)?;

        // Validate integrity
        validate_game_state(&game_state)?;

        Ok((game_state.player, game_state.world))
    }

    /// Get information about a save slot without loading the full game
    pub fn get_save_info(&self, slot_name: &str) -> GameResult<Option<SaveFileInfo>> {
        let file_path = self.get_save_file_path(slot_name);

        if !file_path.exists() {
            return Ok(None);
        }

        // Read just enough data to get metadata
        let compressed_data = fs::read(&file_path)
            .map_err(|e| crate::GameError::SaveLoadError(format!("Failed to read save file: {}", e)))?;

        let serialized_data = decompress_save_data(&compressed_data)?;
        let game_state = deserialize_game_state(&serialized_data)?;

        Ok(Some(SaveFileInfo::from(&game_state)))
    }

    /// List all available save slots
    pub fn list_save_slots(&self) -> GameResult<Vec<SaveSlot>> {
        let mut slots = Vec::new();

        // Read save directory
        let entries = fs::read_dir(&self.save_directory)
            .map_err(|e| crate::GameError::SaveLoadError(format!("Failed to read save directory: {}", e)))?;

        for entry in entries {
            let entry = entry
                .map_err(|e| crate::GameError::SaveLoadError(format!("Failed to read directory entry: {}", e)))?;

            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("save") {
                if let Some(slot_name) = path.file_stem().and_then(|s| s.to_str()) {
                    let info = self.get_save_info(slot_name).unwrap_or(None);

                    slots.push(SaveSlot {
                        slot_name: slot_name.to_string(),
                        file_path: path,
                        info,
                        exists: true,
                    });
                }
            }
        }

        // Sort by modification time (newest first)
        slots.sort_by(|a, b| {
            let time_a = a.info.as_ref().map(|info| info.timestamp);
            let time_b = b.info.as_ref().map(|info| info.timestamp);
            time_b.cmp(&time_a)
        });

        Ok(slots)
    }

    /// Delete a save slot
    pub fn delete_save(&self, slot_name: &str) -> GameResult<()> {
        let file_path = self.get_save_file_path(slot_name);

        if file_path.exists() {
            fs::remove_file(&file_path)
                .map_err(|e| crate::GameError::SaveLoadError(format!("Failed to delete save file: {}", e)))?;
        }

        Ok(())
    }

    /// Create automatic backup
    fn create_backup(&self, original_path: &Path) -> GameResult<()> {
        let backup_dir = self.save_directory.join("backups");
        if !backup_dir.exists() {
            fs::create_dir_all(&backup_dir)
                .map_err(|e| crate::GameError::SaveLoadError(format!("Failed to create backup directory: {}", e)))?;
        }

        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let backup_name = format!("{}_{}.backup",
                                 original_path.file_stem().unwrap().to_str().unwrap(),
                                 timestamp);
        let backup_path = backup_dir.join(backup_name);

        fs::copy(original_path, &backup_path)
            .map_err(|e| crate::GameError::SaveLoadError(format!("Failed to create backup: {}", e)))?;

        // Clean up old backups
        self.cleanup_old_backups(&backup_dir)?;

        Ok(())
    }

    /// Remove old backup files to save space
    fn cleanup_old_backups(&self, backup_dir: &Path) -> GameResult<()> {
        let mut backups = Vec::new();

        if let Ok(entries) = fs::read_dir(backup_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("backup") {
                    if let Ok(metadata) = fs::metadata(&path) {
                        if let Ok(modified) = metadata.modified() {
                            backups.push((path, modified));
                        }
                    }
                }
            }
        }

        // Sort by modification time (newest first)
        backups.sort_by(|a, b| b.1.cmp(&a.1));

        // Remove excess backups
        for (path, _) in backups.into_iter().skip(self.max_backups) {
            let _ = fs::remove_file(path); // Ignore errors for backup cleanup
        }

        Ok(())
    }

    /// Get file path for a save slot
    fn get_save_file_path(&self, slot_name: &str) -> PathBuf {
        self.save_directory.join(format!("{}.save", slot_name))
    }

    /// Quick save to default slot
    pub fn quick_save(&self, player: &Player, world: &WorldState) -> GameResult<String> {
        self.save_game(player, world, Some("quicksave".to_string()), None)
    }

    /// Auto-save (typically called periodically)
    pub fn auto_save(&self, player: &Player, world: &WorldState) -> GameResult<String> {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M");
        let slot_name = format!("autosave_{}", timestamp);
        self.save_game(player, world, Some(slot_name), Some("Auto Save".to_string()))
    }

    /// Import save file from external location
    pub fn import_save(&self, source_path: &Path, slot_name: &str) -> GameResult<()> {
        if !source_path.exists() {
            return Err(crate::GameError::SaveLoadError("Source file does not exist".to_string()).into());
        }

        // Validate the save file first
        let data = fs::read(source_path)
            .map_err(|e| crate::GameError::SaveLoadError(format!("Failed to read source file: {}", e)))?;

        let serialized_data = decompress_save_data(&data)?;
        let game_state = deserialize_game_state(&serialized_data)?;
        validate_game_state(&game_state)?;

        // Copy to save directory
        let target_path = self.get_save_file_path(slot_name);
        fs::copy(source_path, target_path)
            .map_err(|e| crate::GameError::SaveLoadError(format!("Failed to copy save file: {}", e)))?;

        Ok(())
    }

    /// Export save file to external location
    pub fn export_save(&self, slot_name: &str, target_path: &Path) -> GameResult<()> {
        let source_path = self.get_save_file_path(slot_name);

        if !source_path.exists() {
            return Err(crate::GameError::SaveLoadError("Save slot does not exist".to_string()).into());
        }

        fs::copy(source_path, target_path)
            .map_err(|e| crate::GameError::SaveLoadError(format!("Failed to export save file: {}", e)))?;

        Ok(())
    }

    /// Get save directory path for user reference
    pub fn get_save_directory_path(&self) -> &Path {
        &self.save_directory
    }

    /// Check available disk space for saves
    pub fn check_disk_space(&self) -> GameResult<u64> {
        // This is a simplified implementation
        // In practice, you'd want to check actual available space
        Ok(1024 * 1024 * 100) // Return 100MB as placeholder
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_save_manager() -> (SaveManager, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = SaveManager::new().unwrap();
        manager.save_directory = temp_dir.path().to_path_buf();
        (manager, temp_dir)
    }

    #[test]
    fn test_save_and_load() {
        let (manager, _temp_dir) = create_test_save_manager();
        let player = Player::new("Test Player".to_string());
        let world = WorldState::new();

        // Save game
        let save_result = manager.save_game(&player, &world, Some("test".to_string()), Some("Test Save".to_string()));
        assert!(save_result.is_ok());

        // Load game
        let load_result = manager.load_game("test");
        assert!(load_result.is_ok());

        let (loaded_player, _loaded_world) = load_result.unwrap();
        assert_eq!(loaded_player.name, "Test Player");
    }

    #[test]
    fn test_save_info() {
        let (manager, _temp_dir) = create_test_save_manager();
        let player = Player::new("Info Test".to_string());
        let world = WorldState::new();

        manager.save_game(&player, &world, Some("info_test".to_string()), Some("Info Test Save".to_string())).unwrap();

        let info = manager.get_save_info("info_test").unwrap();
        assert!(info.is_some());

        let info = info.unwrap();
        assert_eq!(info.character_name, "Info Test");
        assert_eq!(info.save_name, "Info Test Save");
    }

    #[test]
    fn test_list_saves() {
        let (manager, _temp_dir) = create_test_save_manager();
        let player = Player::new("List Test".to_string());
        let world = WorldState::new();

        // Create multiple saves
        manager.save_game(&player, &world, Some("save1".to_string()), None).unwrap();
        manager.save_game(&player, &world, Some("save2".to_string()), None).unwrap();

        let slots = manager.list_save_slots().unwrap();
        assert_eq!(slots.len(), 2);
        assert!(slots.iter().any(|slot| slot.slot_name == "save1"));
        assert!(slots.iter().any(|slot| slot.slot_name == "save2"));
    }

    #[test]
    fn test_delete_save() {
        let (manager, _temp_dir) = create_test_save_manager();
        let player = Player::new("Delete Test".to_string());
        let world = WorldState::new();

        manager.save_game(&player, &world, Some("delete_test".to_string()), None).unwrap();
        assert!(manager.get_save_info("delete_test").unwrap().is_some());

        manager.delete_save("delete_test").unwrap();
        assert!(manager.get_save_info("delete_test").unwrap().is_none());
    }

    #[test]
    fn test_quick_save() {
        let (manager, _temp_dir) = create_test_save_manager();
        let player = Player::new("Quick Test".to_string());
        let world = WorldState::new();

        let result = manager.quick_save(&player, &world);
        assert!(result.is_ok());

        // Should be able to load quicksave
        let load_result = manager.load_game("quicksave");
        assert!(load_result.is_ok());
    }
}