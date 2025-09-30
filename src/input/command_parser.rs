//! Command parser that converts recognized intents into executable commands

use crate::input::natural_language::{InputTokenizer, CommandIntent};
use crate::core::world_state::Direction;
use serde::{Deserialize, Serialize};

/// Main command parser that processes user input
pub struct CommandParser {
    tokenizer: InputTokenizer,
}

/// Result of command parsing
#[derive(Debug, Clone)]
pub enum CommandResult {
    Success(ParsedCommand),
    Error(String),
    Help(String),
}

/// A successfully parsed command ready for execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParsedCommand {
    /// Move in a specific direction
    Move { direction: Direction },

    /// Look at current location or specific target
    Look { target: Option<String> },

    /// Examine something in detail
    Examine { target: String },

    /// Cast magic with optional crystal and target
    CastMagic {
        spell_type: String,
        crystal: Option<String>,
        target: Option<String>
    },

    /// Talk to an NPC
    Talk { target: String },

    /// Ask NPC about a topic
    Ask { target: String, topic: String },

    /// Show inventory
    Inventory,

    /// Show character status
    Status,

    /// Show crystal status
    CrystalStatus,

    /// Show faction standings
    FactionStatus,

    /// Save the game
    Save { slot: Option<String> },

    /// Load a saved game
    Load { slot: Option<String> },

    /// Show help
    Help { topic: Option<String> },

    /// Quit the game
    Quit,

    /// Rest to recover energy
    Rest,

    /// Meditate for faster recovery
    Meditate,

    /// Study a magic theory
    Study { theory: String },

    /// Research a new topic
    Research { topic: String },

    /// Quest-related commands
    /// Show available quests
    QuestList,

    /// Show active quests
    QuestActive,

    /// Show quest details
    QuestInfo { quest_id: String },

    /// Start a quest
    QuestStart { quest_id: String },

    /// Show quest status
    QuestStatus { quest_id: String },

    /// Get quest recommendations
    QuestRecommendations,

    /// Abandon a quest
    QuestAbandon { quest_id: String },

    /// Take an item
    Take { item: String },

    /// Drop an item
    Drop { item: String },

    /// Equip a crystal
    Equip { crystal: String },

    /// Use an item
    UseItem { item: String, target: Option<String> },

    /// Unequip an item
    UnequipItem { slot: Option<String> },

    /// Combine/craft items
    CraftItem { action: String, items: Vec<String>, recipe: Option<String> },

    /// Examine an item in detail
    ExamineItem { item: String },

    /// Give item to someone
    GiveItem { item: String, target: String },

    /// Unknown command with suggestions
    Unknown {
        original: String,
        suggestions: Vec<String>
    },
}

impl CommandParser {
    /// Create a new command parser
    pub fn new() -> Self {
        Self {
            tokenizer: InputTokenizer::new(),
        }
    }

    /// Parse raw input into a command
    pub fn parse(&self, input: &str) -> CommandResult {
        if input.trim().is_empty() {
            return CommandResult::Error("Please enter a command.".to_string());
        }

        let tokens = self.tokenizer.tokenize(input);
        let intent = self.tokenizer.recognize_intent(&tokens);

        match intent {
            CommandIntent::Movement { direction } => {
                self.parse_movement(direction)
            }

            CommandIntent::Examination { target } => {
                self.parse_examination(target)
            }

            CommandIntent::Magic { spell_type, crystal, target } => {
                CommandResult::Success(ParsedCommand::CastMagic {
                    spell_type,
                    crystal,
                    target,
                })
            }

            CommandIntent::Social { action, target } => {
                self.parse_social(action, target)
            }

            CommandIntent::Inventory { action: _ } => {
                CommandResult::Success(ParsedCommand::Inventory)
            }

            CommandIntent::Item { action, target, destination } => {
                self.parse_item_command(action, target, destination)
            }

            CommandIntent::Equipment { action, item, slot } => {
                self.parse_equipment_command(action, item, slot)
            }

            CommandIntent::Crafting { action, items, recipe } => {
                self.parse_crafting_command(action, items, recipe)
            }

            CommandIntent::System { command } => {
                self.parse_system_command(command)
            }

            CommandIntent::Help { topic } => {
                self.generate_help(topic)
            }

            CommandIntent::Unknown { original_input } => {
                self.handle_unknown_command(original_input)
            }
        }
    }

    /// Parse movement commands
    fn parse_movement(&self, direction_str: String) -> CommandResult {
        match Direction::from_string(&direction_str) {
            Some(direction) => CommandResult::Success(ParsedCommand::Move { direction }),
            None => CommandResult::Error(format!("'{}' is not a valid direction.", direction_str)),
        }
    }

    /// Parse examination commands
    fn parse_examination(&self, target: Option<String>) -> CommandResult {
        match target {
            Some(target_str) => {
                // Check for special examination targets
                match target_str.as_str() {
                    "around" | "room" | "here" => {
                        CommandResult::Success(ParsedCommand::Look { target: None })
                    }
                    "crystals" | "crystal" => {
                        CommandResult::Success(ParsedCommand::CrystalStatus)
                    }
                    "status" | "self" | "me" => {
                        CommandResult::Success(ParsedCommand::Status)
                    }
                    _ => {
                        CommandResult::Success(ParsedCommand::Examine { target: target_str })
                    }
                }
            }
            None => {
                CommandResult::Success(ParsedCommand::Look { target: None })
            }
        }
    }

    /// Parse social interaction commands
    fn parse_social(&self, action: String, target: String) -> CommandResult {
        if target.is_empty() {
            return CommandResult::Error("Who do you want to talk to?".to_string());
        }

        match action.as_str() {
            "ask" => {
                // Try to parse "ask person about topic" format
                if let Some(about_pos) = target.find(" about ") {
                    let person = target[..about_pos].to_string();
                    let topic = target[about_pos + 7..].to_string();
                    CommandResult::Success(ParsedCommand::Ask { target: person, topic })
                } else {
                    CommandResult::Error("What do you want to ask about? Use: ask <person> about <topic>".to_string())
                }
            }
            _ => {
                CommandResult::Success(ParsedCommand::Talk { target })
            }
        }
    }

    /// Parse system commands
    fn parse_system_command(&self, command: String) -> CommandResult {
        let parts: Vec<&str> = command.split_whitespace().collect();

        match parts.as_slice() {
            ["save"] => CommandResult::Success(ParsedCommand::Save { slot: None }),
            ["load"] => CommandResult::Success(ParsedCommand::Load { slot: None }),
            ["status"] => CommandResult::Success(ParsedCommand::Status),
            ["quit"] | ["exit"] => CommandResult::Success(ParsedCommand::Quit),

            // Quest commands
            ["quest", "list"] | ["quests"] => CommandResult::Success(ParsedCommand::QuestList),
            ["quest", "active"] => CommandResult::Success(ParsedCommand::QuestActive),
            ["quest", "recommendations"] => CommandResult::Success(ParsedCommand::QuestRecommendations),
            ["quest", "info", quest_id] => CommandResult::Success(ParsedCommand::QuestInfo { quest_id: quest_id.to_string() }),
            ["quest", "status", quest_id] => CommandResult::Success(ParsedCommand::QuestStatus { quest_id: quest_id.to_string() }),
            ["quest", "start", quest_id] => CommandResult::Success(ParsedCommand::QuestStart { quest_id: quest_id.to_string() }),
            ["quest", "abandon", quest_id] => CommandResult::Success(ParsedCommand::QuestAbandon { quest_id: quest_id.to_string() }),

            _ => CommandResult::Error(format!("Unknown system command: {}", command)),
        }
    }

    /// Generate help text
    fn generate_help(&self, topic: Option<String>) -> CommandResult {
        let help_text = match topic.as_deref() {
            Some("movement") | Some("move") => {
                "Movement Commands:\n\
                 • north, south, east, west (or n, s, e, w)\n\
                 • up, down, in, out\n\
                 • go <direction>\n\n\
                 Examples:\n\
                 • north\n\
                 • go east\n\
                 • up"
            }

            Some("magic") => {
                "Magic Commands:\n\
                 • cast <spell> using <crystal> on <target>\n\
                 • examine <crystal>\n\
                 • study <theory>\n\
                 • research <topic>\n\n\
                 Examples:\n\
                 • cast healing using amethyst on guard\n\
                 • cast light using quartz\n\
                 • examine my crystals\n\
                 • study harmonic fundamentals"
            }

            Some("social") => {
                "Social Commands:\n\
                 • talk to <person>\n\
                 • ask <person> about <topic>\n\
                 • faction status\n\n\
                 Examples:\n\
                 • talk to scholar\n\
                 • ask merchant about crystals\n\
                 • faction status"
            }

            Some("system") => {
                "System Commands:\n\
                 • save [slot] - Save your game\n\
                 • load [slot] - Load a saved game\n\
                 • status - Show character information\n\
                 • inventory - Show your items\n\
                 • quit - Exit the game\n\n\
                 Examples:\n\
                 • save\n\
                 • load game1\n\
                 • status"
            }
            Some("quests") | Some("quest") => {
                "Quest Commands:\n\
                 • quest list - Show all available quests\n\
                 • quest active - Show your active quests\n\
                 • quest info <id> - Show detailed quest information\n\
                 • quest status <id> - Show quest progress\n\
                 • quest start <id> - Start a quest\n\
                 • quest abandon <id> - Abandon a quest\n\
                 • quest recommendations - Get quest suggestions\n\n\
                 Examples:\n\
                 • quest list\n\
                 • quest start resonance_foundation\n\
                 • quest status crystal_analysis\n\
                 • quest recommendations"
            }

            Some("examination") | Some("look") => {
                "Examination Commands:\n\
                 • look - Look around current location\n\
                 • examine <target> - Examine something closely\n\
                 • analyze <target> - Magical analysis\n\n\
                 Examples:\n\
                 • look\n\
                 • examine crystal formation\n\
                 • analyze magical signature"
            }

            None => {
                "Available Commands:\n\n\
                 Movement: north, south, east, west, up, down\n\
                 Examination: look, examine <target>\n\
                 Magic: cast <spell> using <crystal>\n\
                 Social: talk to <person>, ask <person> about <topic>\n\
                 Quests: quest list, quest start <id>, quest status <id>\n\
                 System: save, load, status, inventory, quit\n\n\
                 For detailed help on a topic, type: help <topic>\n\
                 Available topics: movement, magic, social, system, examination, quests"
            }

            Some(unknown) => {
                &format!("No help available for '{}'. Available topics: movement, magic, social, system, examination, quests", unknown)
            }
        };

        CommandResult::Help(help_text.to_string())
    }

    /// Handle unknown commands with suggestions
    fn handle_unknown_command(&self, original: String) -> CommandResult {
        let suggestions = self.generate_suggestions(&original);

        CommandResult::Success(ParsedCommand::Unknown {
            original: original.clone(),
            suggestions
        })
    }

    /// Generate suggestions for unknown commands
    fn generate_suggestions(&self, input: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        let lower_input = input.to_lowercase();

        // Common command suggestions based on partial matches
        if lower_input.contains("mov") || lower_input.contains("go") {
            suggestions.push("Try: north, south, east, west".to_string());
        }

        if lower_input.contains("look") || lower_input.contains("see") || lower_input.contains("exam") {
            suggestions.push("Try: look, examine <target>".to_string());
        }

        if lower_input.contains("magic") || lower_input.contains("cast") || lower_input.contains("spell") {
            suggestions.push("Try: cast <spell> using <crystal>".to_string());
        }

        if lower_input.contains("talk") || lower_input.contains("speak") || lower_input.contains("say") {
            suggestions.push("Try: talk to <person>".to_string());
        }

        if lower_input.contains("inv") || lower_input.contains("item") {
            suggestions.push("Try: inventory, status".to_string());
        }

        if lower_input.contains("help") {
            suggestions.push("Try: help, help <topic>".to_string());
        }

        // If no specific suggestions, provide general help
        if suggestions.is_empty() {
            suggestions.push("Type 'help' for available commands".to_string());
            suggestions.push("Common commands: look, north, inventory, status".to_string());
        }

        suggestions
    }

    /// Parse item interaction commands
    fn parse_item_command(&self, action: String, target: Option<String>, destination: Option<String>) -> CommandResult {
        match action.as_str() {
            "take" => {
                if let Some(item) = target {
                    CommandResult::Success(ParsedCommand::Take { item })
                } else {
                    CommandResult::Error("What do you want to take?".to_string())
                }
            }
            "drop" => {
                if let Some(item) = target {
                    CommandResult::Success(ParsedCommand::Drop { item })
                } else {
                    CommandResult::Error("What do you want to drop?".to_string())
                }
            }
            "use" => {
                if let Some(item) = target {
                    CommandResult::Success(ParsedCommand::UseItem { item, target: destination })
                } else {
                    CommandResult::Error("What do you want to use?".to_string())
                }
            }
            "give" => {
                if let Some(item) = target {
                    if let Some(recipient) = destination {
                        CommandResult::Success(ParsedCommand::GiveItem { item, target: recipient })
                    } else {
                        CommandResult::Error("Give the item to whom?".to_string())
                    }
                } else {
                    CommandResult::Error("What do you want to give?".to_string())
                }
            }
            "consume" => {
                if let Some(item) = target {
                    CommandResult::Success(ParsedCommand::UseItem { item, target: None })
                } else {
                    CommandResult::Error("What do you want to consume?".to_string())
                }
            }
            _ => CommandResult::Error(format!("Unknown item action: {}", action))
        }
    }

    /// Parse equipment commands
    fn parse_equipment_command(&self, action: String, item: Option<String>, slot: Option<String>) -> CommandResult {
        match action.as_str() {
            "equip" => {
                if let Some(item_name) = item {
                    CommandResult::Success(ParsedCommand::Equip { crystal: item_name })
                } else {
                    CommandResult::Error("What do you want to equip?".to_string())
                }
            }
            "unequip" => {
                CommandResult::Success(ParsedCommand::UnequipItem { slot })
            }
            _ => CommandResult::Error(format!("Unknown equipment action: {}", action))
        }
    }

    /// Parse crafting commands
    fn parse_crafting_command(&self, action: String, items: Vec<String>, recipe: Option<String>) -> CommandResult {
        if items.is_empty() {
            return CommandResult::Error("What items do you want to craft with?".to_string());
        }

        CommandResult::Success(ParsedCommand::CraftItem { action, items, recipe })
    }

    /// Parse advanced commands with multiple parameters
    pub fn parse_advanced(&self, input: &str) -> CommandResult {
        let trimmed = input.trim().to_lowercase();

        // Handle complex multi-word commands
        if trimmed.starts_with("save ") {
            let slot = trimmed[5..].trim().to_string();
            return CommandResult::Success(ParsedCommand::Save {
                slot: if slot.is_empty() { None } else { Some(slot) }
            });
        }

        if trimmed.starts_with("load ") {
            let slot = trimmed[5..].trim().to_string();
            return CommandResult::Success(ParsedCommand::Load {
                slot: if slot.is_empty() { None } else { Some(slot) }
            });
        }

        if trimmed.starts_with("study ") {
            let theory = trimmed[6..].trim().to_string();
            if theory.is_empty() {
                return CommandResult::Error("What theory do you want to study?".to_string());
            }
            return CommandResult::Success(ParsedCommand::Study { theory });
        }

        if trimmed.starts_with("research ") {
            let topic = trimmed[9..].trim().to_string();
            if topic.is_empty() {
                return CommandResult::Error("What do you want to research?".to_string());
            }
            return CommandResult::Success(ParsedCommand::Research { topic });
        }

        if trimmed.starts_with("take ") {
            let item = trimmed[5..].trim().to_string();
            if item.is_empty() {
                return CommandResult::Error("What do you want to take?".to_string());
            }
            return CommandResult::Success(ParsedCommand::Take { item });
        }

        if trimmed.starts_with("drop ") {
            let item = trimmed[5..].trim().to_string();
            if item.is_empty() {
                return CommandResult::Error("What do you want to drop?".to_string());
            }
            return CommandResult::Success(ParsedCommand::Drop { item });
        }

        if trimmed.starts_with("equip ") {
            let crystal = trimmed[6..].trim().to_string();
            if crystal.is_empty() {
                return CommandResult::Error("What crystal do you want to equip?".to_string());
            }
            return CommandResult::Success(ParsedCommand::Equip { crystal });
        }

        // Handle single-word advanced commands
        match trimmed.as_str() {
            "rest" => CommandResult::Success(ParsedCommand::Rest),
            "meditate" => CommandResult::Success(ParsedCommand::Meditate),
            "faction status" | "factions" => CommandResult::Success(ParsedCommand::FactionStatus),
            "crystal status" | "crystals" => CommandResult::Success(ParsedCommand::CrystalStatus),
            _ => self.parse(input), // Fall back to normal parsing
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movement_parsing() {
        let parser = CommandParser::new();
        let result = parser.parse("go north");

        match result {
            CommandResult::Success(ParsedCommand::Move { direction }) => {
                assert_eq!(direction, Direction::North);
            }
            _ => panic!("Expected successful movement command"),
        }
    }

    #[test]
    fn test_magic_parsing() {
        let parser = CommandParser::new();
        let result = parser.parse("cast healing using amethyst on guard");

        match result {
            CommandResult::Success(ParsedCommand::CastMagic { spell_type, crystal, target }) => {
                assert_eq!(spell_type, "healing");
                assert_eq!(crystal, Some("amethyst".to_string()));
                assert_eq!(target, Some("guard".to_string()));
            }
            _ => panic!("Expected successful magic command"),
        }
    }

    #[test]
    fn test_examination_parsing() {
        let parser = CommandParser::new();
        let result = parser.parse("examine crystal formation");

        match result {
            CommandResult::Success(ParsedCommand::Examine { target }) => {
                assert_eq!(target, "crystal formation");
            }
            _ => panic!("Expected successful examination command"),
        }
    }

    #[test]
    fn test_help_parsing() {
        let parser = CommandParser::new();
        let result = parser.parse("help magic");

        match result {
            CommandResult::Help(text) => {
                assert!(text.contains("Magic Commands"));
            }
            _ => panic!("Expected help result"),
        }
    }

    #[test]
    fn test_advanced_parsing() {
        let parser = CommandParser::new();
        let result = parser.parse_advanced("save game1");

        match result {
            CommandResult::Success(ParsedCommand::Save { slot }) => {
                assert_eq!(slot, Some("game1".to_string()));
            }
            _ => panic!("Expected successful save command"),
        }
    }

    #[test]
    fn test_unknown_command_suggestions() {
        let parser = CommandParser::new();
        let result = parser.parse("blahblah");

        match result {
            CommandResult::Success(ParsedCommand::Unknown { original, suggestions }) => {
                assert_eq!(original, "blahblah");
                assert!(!suggestions.is_empty());
            }
            _ => panic!("Expected unknown command with suggestions"),
        }
    }

    #[test]
    fn test_social_parsing() {
        let parser = CommandParser::new();
        let result = parser.parse("ask merchant about crystals");

        match result {
            CommandResult::Success(ParsedCommand::Ask { target, topic }) => {
                assert_eq!(target, "merchant");
                assert_eq!(topic, "crystals");
            }
            _ => panic!("Expected successful ask command"),
        }
    }

    #[test]
    fn test_quest_list_parsing() {
        let parser = CommandParser::new();
        let result = parser.parse("quest list");

        match result {
            CommandResult::Success(ParsedCommand::QuestList) => {
                // Success
            }
            other => panic!("Expected successful quest list command, got: {:?}", other),
        }
    }

    #[test]
    fn test_quests_parsing() {
        let parser = CommandParser::new();
        let result = parser.parse("quests");

        match result {
            CommandResult::Success(ParsedCommand::QuestList) => {
                // Success
            }
            other => panic!("Expected successful quests command, got: {:?}", other),
        }
    }

    #[test]
    fn test_quest_start_parsing() {
        let parser = CommandParser::new();
        let result = parser.parse("quest start resonance_foundation");

        match result {
            CommandResult::Success(ParsedCommand::QuestStart { quest_id }) => {
                assert_eq!(quest_id, "resonance_foundation");
            }
            other => panic!("Expected successful quest start command, got: {:?}", other),
        }
    }

    #[test]
    fn test_quest_parsing_via_parse_advanced() {
        let parser = CommandParser::new();
        let result = parser.parse_advanced("quest list");

        match result {
            CommandResult::Success(ParsedCommand::QuestList) => {
                // Success
            }
            other => panic!("Expected successful quest list via parse_advanced, got: {:?}", other),
        }
    }
}