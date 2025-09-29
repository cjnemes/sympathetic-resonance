//! Natural language processing for command input
//!
//! This module handles tokenization and intent recognition for player commands

use regex::Regex;
use std::collections::HashMap;

/// Tokenizes raw input into meaningful components
pub struct InputTokenizer {
    /// Patterns for recognizing different token types
    token_patterns: Vec<TokenPattern>,
    /// Common synonyms and abbreviations
    synonyms: HashMap<String, String>,
}

/// Pattern for recognizing specific token types
struct TokenPattern {
    pattern: Regex,
    token_type: TokenType,
}

/// Types of tokens that can be extracted from input
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Verb,
    Object,
    Direction,
    MagicKeyword,
    Preposition,
    Article,
    Adjective,
}

/// A recognized token from input
#[derive(Debug, Clone)]
pub struct Token {
    pub text: String,
    pub token_type: TokenType,
    pub position: usize,
}

/// High-level intent recognized from tokens
#[derive(Debug, Clone)]
pub enum CommandIntent {
    Movement { direction: String },
    Examination { target: Option<String> },
    Magic {
        spell_type: String,
        crystal: Option<String>,
        target: Option<String>
    },
    Social { action: String, target: String },
    Inventory { action: String },
    System { command: String },
    Help { topic: Option<String> },
    Unknown { original_input: String },
}

impl InputTokenizer {
    /// Create a new tokenizer with default patterns
    pub fn new() -> Self {
        let mut tokenizer = Self {
            token_patterns: Vec::new(),
            synonyms: HashMap::new(),
        };

        tokenizer.initialize_patterns();
        tokenizer.initialize_synonyms();
        tokenizer
    }

    /// Set up token recognition patterns
    fn initialize_patterns(&mut self) {
        // Movement verbs
        self.add_pattern(r"\b(go|move|walk|travel|head|proceed)\b", TokenType::Verb);

        // Examination verbs
        self.add_pattern(r"\b(look|examine|inspect|study|observe|check|analyze)\b", TokenType::Verb);

        // Magic verbs
        self.add_pattern(r"\b(cast|channel|focus|resonate|attune|use)\b", TokenType::Verb);

        // Social verbs
        self.add_pattern(r"\b(talk|speak|ask|tell|say|greet|converse)\b", TokenType::Verb);

        // System verbs
        self.add_pattern(r"\b(save|load|quit|exit|help|status|inventory|quest|quests)\b", TokenType::Verb);

        // Directions
        self.add_pattern(r"\b(north|south|east|west|northeast|northwest|southeast|southwest|up|down|in|out|n|s|e|w|ne|nw|se|sw|u|d)\b", TokenType::Direction);

        // Magic keywords
        self.add_pattern(r"\b(using|with|through|via|crystal|magic|spell|energy|resonance)\b", TokenType::MagicKeyword);

        // Prepositions
        self.add_pattern(r"\b(to|at|on|in|with|using|through|about|for|from)\b", TokenType::Preposition);

        // Articles
        self.add_pattern(r"\b(the|a|an)\b", TokenType::Article);

        // Common adjectives
        self.add_pattern(r"\b(small|large|bright|dark|magical|ancient|broken|crystal|quartz|amethyst|obsidian|garnet)\b", TokenType::Adjective);
    }

    /// Add a pattern to the tokenizer
    fn add_pattern(&mut self, pattern: &str, token_type: TokenType) {
        if let Ok(regex) = Regex::new(&format!("(?i){}", pattern)) {
            self.token_patterns.push(TokenPattern {
                pattern: regex,
                token_type,
            });
        }
    }

    /// Set up common synonyms and abbreviations
    fn initialize_synonyms(&mut self) {
        // Movement synonyms
        self.synonyms.insert("n".to_string(), "north".to_string());
        self.synonyms.insert("s".to_string(), "south".to_string());
        self.synonyms.insert("e".to_string(), "east".to_string());
        self.synonyms.insert("w".to_string(), "west".to_string());
        self.synonyms.insert("ne".to_string(), "northeast".to_string());
        self.synonyms.insert("nw".to_string(), "northwest".to_string());
        self.synonyms.insert("se".to_string(), "southeast".to_string());
        self.synonyms.insert("sw".to_string(), "southwest".to_string());
        self.synonyms.insert("u".to_string(), "up".to_string());
        self.synonyms.insert("d".to_string(), "down".to_string());

        // Examination synonyms
        self.synonyms.insert("l".to_string(), "look".to_string());
        self.synonyms.insert("ex".to_string(), "examine".to_string());
        self.synonyms.insert("x".to_string(), "examine".to_string());

        // Inventory synonyms
        self.synonyms.insert("inv".to_string(), "inventory".to_string());
        self.synonyms.insert("i".to_string(), "inventory".to_string());

        // System synonyms
        self.synonyms.insert("q".to_string(), "quit".to_string());
        self.synonyms.insert("h".to_string(), "help".to_string());
        self.synonyms.insert("stats".to_string(), "status".to_string());
    }

    /// Tokenize input string into meaningful components
    pub fn tokenize(&self, input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let normalized_input = self.normalize_input(input);
        let words: Vec<&str> = normalized_input.split_whitespace().collect();

        for (position, word) in words.iter().enumerate() {
            // Expand synonyms first
            let expanded_word = self.synonyms.get(*word).unwrap_or(&word.to_string()).clone();

            // Find matching token type
            let mut token_type = None;
            for pattern in &self.token_patterns {
                if pattern.pattern.is_match(&expanded_word) {
                    token_type = Some(pattern.token_type.clone());
                    break;
                }
            }

            // If no specific type found, treat as object
            let final_type = token_type.unwrap_or(TokenType::Object);

            tokens.push(Token {
                text: expanded_word,
                token_type: final_type,
                position,
            });
        }

        tokens
    }

    /// Normalize input for better parsing
    fn normalize_input(&self, input: &str) -> String {
        input.to_lowercase().trim().to_string()
    }

    /// Recognize high-level intent from tokens
    pub fn recognize_intent(&self, tokens: &[Token]) -> CommandIntent {
        if tokens.is_empty() {
            return CommandIntent::Unknown {
                original_input: "".to_string()
            };
        }

        // Look for primary verb
        let verb_token = tokens.iter().find(|t| t.token_type == TokenType::Verb);

        match verb_token {
            Some(verb) => {
                match verb.text.as_str() {
                    // Movement commands
                    "go" | "move" | "walk" | "travel" | "head" | "proceed" => {
                        self.parse_movement_intent(tokens)
                    }

                    // Direct direction commands
                    "north" | "south" | "east" | "west" | "northeast" | "northwest" |
                    "southeast" | "southwest" | "up" | "down" | "in" | "out" => {
                        CommandIntent::Movement {
                            direction: verb.text.clone()
                        }
                    }

                    // Examination commands
                    "look" | "examine" | "inspect" | "study" | "observe" | "check" | "analyze" => {
                        self.parse_examination_intent(tokens)
                    }

                    // Magic commands
                    "cast" | "channel" | "focus" | "resonate" | "attune" | "use" => {
                        self.parse_magic_intent(tokens)
                    }

                    // Social commands
                    "talk" | "speak" | "ask" | "tell" | "say" | "greet" | "converse" => {
                        self.parse_social_intent(tokens)
                    }

                    // Inventory commands
                    "inventory" => {
                        CommandIntent::Inventory { action: "show".to_string() }
                    }

                    // System commands
                    "save" | "load" | "quit" | "exit" | "status" | "quest" | "quests" => {
                        CommandIntent::System { command: self.build_system_command(tokens) }
                    }

                    "help" => {
                        self.parse_help_intent(tokens)
                    }

                    _ => CommandIntent::Unknown {
                        original_input: tokens.iter()
                            .map(|t| t.text.as_str())
                            .collect::<Vec<_>>()
                            .join(" ")
                    }
                }
            }

            // Check if first token is a direction (implicit movement)
            None => {
                if let Some(first_token) = tokens.first() {
                    if first_token.token_type == TokenType::Direction {
                        return CommandIntent::Movement {
                            direction: first_token.text.clone()
                        };
                    }
                }

                CommandIntent::Unknown {
                    original_input: tokens.iter()
                        .map(|t| t.text.as_str())
                        .collect::<Vec<_>>()
                        .join(" ")
                }
            }
        }
    }

    /// Parse movement command intent
    fn parse_movement_intent(&self, tokens: &[Token]) -> CommandIntent {
        // Look for direction token
        if let Some(direction_token) = tokens.iter().find(|t| t.token_type == TokenType::Direction) {
            CommandIntent::Movement {
                direction: direction_token.text.clone()
            }
        } else {
            CommandIntent::Unknown {
                original_input: tokens.iter()
                    .map(|t| t.text.as_str())
                    .collect::<Vec<_>>()
                    .join(" ")
            }
        }
    }

    /// Parse examination command intent
    fn parse_examination_intent(&self, tokens: &[Token]) -> CommandIntent {
        // Look for object to examine
        let target = tokens.iter()
            .filter(|t| matches!(t.token_type, TokenType::Object | TokenType::Adjective | TokenType::MagicKeyword))
            .filter(|t| !matches!(t.text.as_str(), "look" | "examine" | "inspect" | "study" | "observe" | "check" | "analyze" | "using" | "with" | "through" | "via" | "magic" | "spell" | "energy" | "resonance"))
            .map(|t| t.text.clone())
            .collect::<Vec<_>>()
            .join(" ");

        let target = if target.is_empty() { None } else { Some(target) };

        CommandIntent::Examination { target }
    }

    /// Parse magic command intent
    fn parse_magic_intent(&self, tokens: &[Token]) -> CommandIntent {
        let mut spell_type = String::new();
        let mut crystal = None;
        let mut target = None;
        let mut using_found = false;
        let mut on_found = false;

        for token in tokens {
            match token.text.as_str() {
                "cast" | "channel" | "focus" | "resonate" | "attune" | "use" => {
                    // Skip the verb
                }
                "using" | "with" | "through" | "via" => {
                    using_found = true;
                }
                "on" | "at" => {
                    on_found = true;
                }
                "crystal" => {
                    // Skip the word "crystal" itself
                }
                _ => {
                    if using_found && crystal.is_none() {
                        // Next object after "using" is the crystal
                        crystal = Some(token.text.clone());
                        using_found = false;
                    } else if on_found && target.is_none() {
                        // Next object after "on" is the target
                        target = Some(token.text.clone());
                        on_found = false;
                    } else if spell_type.is_empty() && token.token_type == TokenType::Object {
                        // First object is likely the spell type
                        spell_type = token.text.clone();
                    }
                }
            }
        }

        if spell_type.is_empty() {
            spell_type = "light".to_string(); // Default spell
        }

        CommandIntent::Magic { spell_type, crystal, target }
    }

    /// Parse social command intent
    fn parse_social_intent(&self, tokens: &[Token]) -> CommandIntent {
        let action = tokens.iter()
            .find(|t| t.token_type == TokenType::Verb)
            .map(|t| t.text.clone())
            .unwrap_or_else(|| "talk".to_string());

        // For ask commands, we need to preserve the "about" structure
        if action == "ask" {
            // Reconstruct the original command structure: "person about topic"
            let mut target_parts = Vec::new();

            for token in tokens.iter().skip(1) { // Skip the "ask" verb
                match token.token_type {
                    TokenType::Object | TokenType::Adjective | TokenType::MagicKeyword => {
                        target_parts.push(token.text.clone());
                    }
                    TokenType::Preposition if token.text == "about" => {
                        target_parts.push(token.text.clone());
                    }
                    _ => {} // Skip other token types
                }
            }

            let target = target_parts.join(" ");
            CommandIntent::Social { action, target }
        } else {
            // For other social commands, just collect objects
            let target = tokens.iter()
                .filter(|t| matches!(t.token_type, TokenType::Object | TokenType::Adjective))
                .filter(|t| !matches!(t.text.as_str(), "to" | "with"))
                .map(|t| t.text.clone())
                .collect::<Vec<_>>()
                .join(" ");

            CommandIntent::Social { action, target }
        }
    }

    /// Parse help command intent
    fn parse_help_intent(&self, tokens: &[Token]) -> CommandIntent {
        let topic = tokens.iter()
            .skip(1) // Skip "help" verb
            .filter(|t| matches!(t.token_type, TokenType::Object | TokenType::MagicKeyword | TokenType::Verb))
            .filter(|t| !matches!(t.text.as_str(), "help"))
            .map(|t| t.text.clone())
            .collect::<Vec<_>>()
            .join(" ");

        let topic = if topic.is_empty() { None } else { Some(topic) };

        CommandIntent::Help { topic }
    }

    /// Build system command string from tokens
    fn build_system_command(&self, tokens: &[Token]) -> String {
        tokens.iter()
            .filter(|t| !matches!(t.token_type, TokenType::Article | TokenType::Preposition))
            .map(|t| t.text.clone())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenization() {
        let tokenizer = InputTokenizer::new();
        let tokens = tokenizer.tokenize("look at the crystal");

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].text, "look");
        assert_eq!(tokens[0].token_type, TokenType::Verb);
        assert_eq!(tokens[3].text, "crystal");
    }

    #[test]
    fn test_movement_intent() {
        let tokenizer = InputTokenizer::new();
        let tokens = tokenizer.tokenize("go north");
        let intent = tokenizer.recognize_intent(&tokens);

        match intent {
            CommandIntent::Movement { direction } => {
                assert_eq!(direction, "north");
            }
            _ => panic!("Expected movement intent"),
        }
    }

    #[test]
    fn test_direction_shortcut() {
        let tokenizer = InputTokenizer::new();
        let tokens = tokenizer.tokenize("n");
        let intent = tokenizer.recognize_intent(&tokens);

        match intent {
            CommandIntent::Movement { direction } => {
                assert_eq!(direction, "north");
            }
            _ => panic!("Expected movement intent"),
        }
    }

    #[test]
    fn test_magic_intent() {
        let tokenizer = InputTokenizer::new();
        let tokens = tokenizer.tokenize("cast healing using amethyst on guard");
        let intent = tokenizer.recognize_intent(&tokens);

        match intent {
            CommandIntent::Magic { spell_type, crystal, target } => {
                assert_eq!(spell_type, "healing");
                assert_eq!(crystal, Some("amethyst".to_string()));
                assert_eq!(target, Some("guard".to_string()));
            }
            _ => panic!("Expected magic intent"),
        }
    }

    #[test]
    fn test_examination_intent() {
        let tokenizer = InputTokenizer::new();
        let tokens = tokenizer.tokenize("examine the ancient crystal formation");
        let intent = tokenizer.recognize_intent(&tokens);

        match intent {
            CommandIntent::Examination { target } => {
                assert_eq!(target, Some("ancient crystal formation".to_string()));
            }
            _ => panic!("Expected examination intent"),
        }
    }

    #[test]
    fn test_synonyms() {
        let tokenizer = InputTokenizer::new();
        let tokens = tokenizer.tokenize("ex crystal");
        let intent = tokenizer.recognize_intent(&tokens);

        match intent {
            CommandIntent::Examination { target } => {
                assert_eq!(target, Some("crystal".to_string()));
            }
            _ => panic!("Expected examination intent"),
        }
    }

    #[test]
    fn test_quest_list_intent() {
        let tokenizer = InputTokenizer::new();
        let tokens = tokenizer.tokenize("quest list");
        let intent = tokenizer.recognize_intent(&tokens);

        match intent {
            CommandIntent::System { command } => {
                assert_eq!(command, "quest list");
            }
            _ => panic!("Expected system intent for quest list, got: {:?}", intent),
        }
    }

    #[test]
    fn test_quests_command_intent() {
        let tokenizer = InputTokenizer::new();
        let tokens = tokenizer.tokenize("quests");
        let intent = tokenizer.recognize_intent(&tokens);

        match intent {
            CommandIntent::System { command } => {
                assert_eq!(command, "quests");
            }
            _ => panic!("Expected system intent for quests command, got: {:?}", intent),
        }
    }

    #[test]
    fn test_quest_start_intent() {
        let tokenizer = InputTokenizer::new();
        let tokens = tokenizer.tokenize("quest start resonance_foundation");
        let intent = tokenizer.recognize_intent(&tokens);

        match intent {
            CommandIntent::System { command } => {
                assert_eq!(command, "quest start resonance_foundation");
            }
            _ => panic!("Expected system intent for quest start, got: {:?}", intent),
        }
    }
}