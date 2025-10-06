# Changelog

All notable changes to Sympathetic Resonance will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Complete persistence layer with all game systems (Issue #33)
- Custom serialization helpers for HashMap with enum/tuple keys
- Full save/load support for Combat, Faction, Knowledge, Dialogue, and Magic systems

### Changed
- Enhanced save files to include all game systems (previously only Player, WorldState, QuestSystem)
- Updated GameStateData structure to serialize/deserialize all 8 game systems
- Applied custom serialization to 20+ HashMap fields across codebase

### Technical
- Created `src/systems/serde_helpers.rs` for JSON-compatible serialization
- Added support for `HashMap<FactionId>`, `HashMap<LearningMethod>`, `HashMap<Direction>`, `HashMap<(FactionId, FactionId)>`, `HashMap<i32>`
- All 266 tests passing (206 unit + 60 integration)

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
- [ ] All MVP locations and NPCs
- [ ] Faction questlines with choice consequences
- [ ] Main mystery storyline
- [ ] Tutorial system
- [ ] Help system with contextual guidance
- [ ] Performance optimization

### Added (Milestone 4 - Beta & Release)
- [ ] Beta testing program
- [ ] Final bug fixes and polish
- [ ] Complete user documentation
- [ ] Release build and distribution

## Future Versions

### [1.1.0] - Extended World (Planned)
- Additional locations and faction content
- Advanced magic applications
- Enhanced NPC interactions

### [1.2.0] - Social Systems (Planned)
- Guild and organization mechanics
- Advanced reputation modeling
- Community features

### [2.0.0] - Major Expansion (Planned)
- New regions with unique magical phenomena
- Advanced character customization
- Mod support and content creation tools