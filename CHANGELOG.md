# Changelog

All notable changes to Sympathetic Resonance will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Complete persistence layer with all game systems (Issue #33)
  - Custom serialization helpers for HashMap with enum/tuple keys
  - Full save/load support for Combat, Faction, Knowledge, Dialogue, and Magic systems
- Periodic auto-save system (Issue #36)
  - Automatic saves every 5 minutes (configurable: 1, 5, 10, 15 minutes, or disabled)
  - Event-based triggers: quest completion, level up, major faction changes, combat end
  - Automatic cleanup keeping last 3 autosaves (configurable: 1-10)
  - Silent operation with optional debug notifications
  - Configuration API for enabling/disabling and customizing intervals
  - Status command to view autosave settings and next save time
  - 19 comprehensive tests covering all scenarios
- Command history with rustyline (Issue #35)
  - Up/down arrow navigation through previous commands
  - Ctrl+R reverse search through command history
  - Persistent history across sessions (max 1000 commands)
  - Left/right arrows for in-line command editing
  - Ctrl+C to cancel input, Ctrl+D to exit
  - Platform-specific history file storage

### Changed
- Enhanced save files to include all game systems (previously only Player, WorldState, QuestSystem)
- Updated GameStateData structure to serialize/deserialize all 8 game systems
- Applied custom serialization to 20+ HashMap fields across codebase
- Replaced basic stdin input with rustyline for improved CLI experience

### Technical
- Created `src/systems/serde_helpers.rs` for JSON-compatible serialization
- Added support for `HashMap<FactionId>`, `HashMap<LearningMethod>`, `HashMap<Direction>`, `HashMap<(FactionId, FactionId)>`, `HashMap<i32>`
- Added autosave timer and configuration to GameEngine
- Implemented cleanup logic to manage autosave file count
- Added test helper for isolated save directory testing
- Integrated rustyline 14.0 for readline functionality
- All 282 tests passing (222 unit + 60 integration)

## [0.1.0] - Planned

### Added (Milestone 1 - Foundation)
- [ ] Core architecture and data structures
- [ ] Basic magic calculation engine
- [ ] Database schema and content loading system
- [ ] Simple command parsing for core verbs
- [ ] Save/load system for basic game state
- [ ] Unit tests with >70% coverage

### Added (Milestone 2 - Core Systems)
- [ ] Complete magic system with all calculation types
- [ ] Faction reputation system
- [ ] Character progression and theory learning
- [ ] NPC dialogue system
- [ ] Crystal economy and marketplace
- [ ] Integration testing framework

### Added (Milestone 3 - Content & Polish)
- [ ] Complete narrative content
- [ ] Quest system with branching paths
- [ ] Advanced natural language processing
- [ ] Tutorial system
- [ ] Enhanced UI/UX
- [ ] Performance optimization
