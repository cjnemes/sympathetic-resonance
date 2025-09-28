use crate::core::{Player, WorldState};
use crate::GameResult;
use std::io::{self, Write};

pub struct GameUI;

impl GameUI {
    pub fn new() -> Self {
        Self
    }

    pub fn display_welcome(&self) -> GameResult<()> {
        println!("Welcome to Sympathetic Resonance!");
        println!("A text adventure game of science-based magic.");
        println!();
        Ok(())
    }

    pub fn display_prompt(&self) -> GameResult<()> {
        print!("> ");
        io::stdout().flush().map_err(|e| crate::GameError::IoError(e.to_string()))?;
        Ok(())
    }

    pub fn display_response(&self, response: &str) -> GameResult<()> {
        println!("{}", response);
        println!();
        Ok(())
    }

    pub fn display_status(&self, player: &Player, _world: &WorldState) -> GameResult<()> {
        println!("=== Status ===");
        println!("Mental Energy: {}/{}", player.mental_state.current_energy, player.mental_state.max_energy);
        println!("Fatigue: {}", player.mental_state.fatigue);
        println!("Location: {}", player.current_location);
        println!();
        Ok(())
    }
}

impl Default for GameUI {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Player, WorldState};

    #[test]
    fn test_game_ui_creation() {
        let ui = GameUI::new();
        // Just verify we can create a UI instance
        assert_eq!(std::mem::size_of_val(&ui), std::mem::size_of::<GameUI>());
    }

    #[test]
    fn test_display_welcome() {
        let ui = GameUI::new();
        let result = ui.display_welcome();
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_prompt() {
        let ui = GameUI::new();
        let result = ui.display_prompt();
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_response() {
        let ui = GameUI::new();
        let test_response = "This is a test response";
        let result = ui.display_response(test_response);
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_response_empty() {
        let ui = GameUI::new();
        let result = ui.display_response("");
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_status() {
        let ui = GameUI::new();
        let player = Player::new("Test Player".to_string());
        let world = WorldState::new();

        let result = ui.display_status(&player, &world);
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_status_with_modified_player() {
        let ui = GameUI::new();
        let mut player = Player::new("Test Player".to_string());
        let world = WorldState::new();

        // Modify player state to test different display scenarios
        player.mental_state.current_energy = 50;
        player.mental_state.max_energy = 100;
        player.mental_state.fatigue = 25;

        let result = ui.display_status(&player, &world);
        assert!(result.is_ok());
    }

    #[test]
    fn test_default_implementation() {
        let ui1 = GameUI::new();
        let ui2 = GameUI::default();

        // Both should be equivalent (empty structs)
        assert_eq!(std::mem::size_of_val(&ui1), std::mem::size_of_val(&ui2));
    }
}