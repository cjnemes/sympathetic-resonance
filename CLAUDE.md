# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Critical Development Principles

**⚠️ ERROR-FIXING PRIORITY:** Always fix compilation errors immediately before proceeding with additional work. Leaving errors and moving forward causes too many cascading issues. This is a strict requirement for all development work.

## Project Overview

**Sympathetic Resonance** is a text adventure game featuring science-based magic in a low fantasy world. The game uses a sophisticated magic system where neural energy is amplified through crystal matrices with measurable costs and limitations.

## Build and Test Commands

### Running the Game
```bash
# Development mode
cargo run

# Release mode (optimized)
cargo run --release

# With quiet output
cargo run --quiet
```

### Testing
```bash
# Run all tests
cargo test

# Run specific test or module
cargo test magic_system
cargo test quest_

# Run with output
cargo test -- --nocapture

# Run single test
cargo test test_name -- --exact
```

### Code Quality
```bash
# Format code
cargo fmt

# Lint and check for issues
cargo clippy

# Fast check without building
cargo check
```

### Database Management
```bash
# Initialize database (automated on first run)
cargo run -- --init-db
```

## Architecture Overview

### Module Organization

```
src/
├── core/           # Game engine coordination and fundamental structures
│   ├── game_engine.rs  # Main game loop, coordinates all systems
│   ├── player.rs       # Player state, energy, inventory
│   ├── world_state.rs  # Locations, environment, time
│   └── events.rs       # Event bus for inter-system communication
│
├── systems/        # Core gameplay mechanics
│   ├── magic/          # Magic calculations and resonance
│   │   ├── calculation_engine.rs  # Core magic mathematics
│   │   ├── resonance_system.rs    # Sympathetic resonance logic
│   │   └── crystal_management.rs  # Crystal properties/degradation
│   ├── factions/       # Reputation and political relationships
│   ├── knowledge/      # Theory learning and progression
│   ├── quests/         # Quest system with educational framework
│   ├── dialogue/       # NPC interactions
│   ├── combat/         # Magical combat resolution
│   └── items/          # Equipment, inventory, crafting
│       ├── core.rs         # Base item definitions
│       ├── equipment.rs    # Equipment slots and management
│       ├── inventory.rs    # Inventory constraints and stacking
│       ├── educational.rs  # Learning enhancement items
│       ├── interactions.rs # Take, drop, use mechanics
│       └── unlock_system.rs # Progressive item unlocking based on progression
│
├── input/          # Command parsing and natural language
│   ├── command_parser.rs      # Command tokenization
│   ├── command_handlers.rs    # Command execution
│   └── natural_language.rs    # NLP and intent recognition
│
├── persistence/    # Database and save system
│   ├── database.rs         # SQLite operations (schema v3)
│   ├── save_system.rs      # Game state persistence
│   └── serialization.rs    # JSON serialization
│
└── ui/             # Terminal interface and display
```

### Key System Interactions

**GameEngine** (`src/core/game_engine.rs`) is the central coordinator that:
- Owns all game systems (MagicSystem, FactionSystem, QuestSystem, etc.)
- Processes commands through CommandParser
- Maintains Player and WorldState
- Manages save/load through SaveManager and DatabaseManager

**Command Flow:**
1. Input → CommandParser (tokenization)
2. CommandParser → natural_language module (intent recognition)
3. execute_command() → command_handlers (system-specific handlers)
4. Handlers mutate Player/WorldState/Systems
5. Response generated and displayed

**Magic System Integration:**
- Player has mental energy and fatigue tracked in `core/player.rs`
- MagicSystem calculates costs based on theory knowledge from KnowledgeSystem
- Crystal properties affect resonance calculations
- FactionSystem reputation can affect magical research access
- Quest completion can unlock new magical theories

**Quest System:**
- QuestSystem tracks definitions and player progress
- Integrates with KnowledgeSystem for theory-based objectives
- Uses DialogueSystem for NPC-based quest triggers
- FactionSystem affects quest availability and rewards
- Commands: `quest list`, `quests`, `quest status`, `help quests`

**Item System:**
- ItemSystem manages equipment slots, inventory constraints, and crafting
- Educational items boost learning efficiency for specific theories
- Equipment provides stat bonuses that affect magical abilities
- Progressive unlock system reveals items based on theory mastery and quest completion
- Natural language commands: "take sword", "equip helmet", "drop item", "unequip chest"
- Integrates with Player inventory (backward compatible)

## Database Schema

The game uses SQLite (schema version 3) with these key tables:
- `locations` - World map with magical properties
- `npcs` - NPC definitions with dialogue trees (JSON)
- `theories` - Magic theory definitions with prerequisites
- `items` - Item definitions with type-specific properties (JSON)
- `schema_version` - Migration tracking

Database path: `content/database.db`

## Testing Strategy

**Test Organization:**
- Unit tests: inline in modules with `#[cfg(test)]`
- Integration tests: `src/integration_tests.rs`
- System-specific tests: `src/systems/quest_tests.rs`
- Standalone integration tests: `tests/` directory
  - `tests/quest_dialogue_integration.rs` - Quest-dialogue interactions
  - `tests/item_system_integration.rs` - Item system workflows
  - `tests/educational_content_integration.rs` - Educational item integration
  - `tests/take_drop_integration.rs` - Item pickup/drop mechanics
  - `tests/unequip_integration.rs` - Equipment removal workflows
- Performance benchmarks: `src/performance_tests.rs`

**Current Status:** 279/279 tests passing (219 unit + 60 integration)

**Quest Choice Test Coverage:**
- 16 comprehensive tests for quest choice system covering:
  - Choice selection validation (valid/invalid IDs, prerequisites)
  - Requirement validation (theory, faction, item requirements)
  - Outcome application (faction changes, theory insights, experience)
  - Edge cases and error conditions
  - Content unlocks and NPC reactions

**When Adding Features:**
1. Add unit tests for new functions
2. Add integration tests for cross-system interactions
3. Add standalone integration tests in `tests/` for complex workflows
4. Ensure backward compatibility with save files
5. Update natural language parser if adding new commands
6. Add command handlers in `input/command_handlers.rs`

## Command System Architecture

**Adding New Commands:**
1. Add command handler in `src/input/command_handlers.rs`
2. Update system verbs in `src/input/natural_language.rs` if needed
3. Add intent matching logic in command parser
4. Add help text to help system
5. Add integration tests for command parsing

**Natural Language Parser:**
- Tokenizes input into verbs, objects, prepositions
- Recognizes system verbs: "cast", "study", "quest", "talk", "examine", "go", etc.
- Handles synonyms and variations
- Two pathways: `parse()` for simple commands, `parse_advanced()` for complex

## Save System

**Save Format:** JSON serialization of game state (compressed)
**Save Location:** Platform-specific (uses `dirs` crate)
- macOS: `~/Library/Application Support/SympatheticResonance/saves/`
- Linux: `~/.local/share/sympathetic-resonance/saves/`
- Windows: `%LOCALAPPDATA%\SympatheticResonance\saves\`

**Slot System:** Named save slots with path traversal protection
- Slot names sanitized to alphanumeric, underscore, hyphen only
- Maximum 50 characters
- Default to "autosave" if not specified
- Automatic backups created before overwriting

**Commands:**
- `save` or `save <slot_name>` - Save current game state
- `load` or `load <slot_name>` - Load saved game state

**Save includes:**
- Player state (attributes, inventory, knowledge)
- World state (location, time, conditions, events)
- Quest progress and active quests
- Faction reputation
- Theory understanding levels
- Learning history and research progress

## Magic System Details

**Sympathetic Resonance:**
- Magic requires neural energy + crystal amplification
- Crystals have frequency (1-10) and degradation level
- Success depends on: theory knowledge, crystal quality, mental fatigue, environmental factors
- Failed spells consume 50% resources (prevents exploitation)
- Time always advances (prevents temporal manipulation)

**Theory System:**
- 9 theories across 3 tiers with prerequisites
- 6 learning methods: study, experimentation, observation, teaching, research, mentorship
- Theory understanding affects magic success rates and unlocks new spells

## Performance Targets

- Database operations: <200ms
- Command processing: <35ms
- Magic calculations: <35ms
- Save/load operations: <500ms

## Security Considerations

**Path Traversal Protection:**
- Save slot names are sanitized (see `src/persistence/save_system.rs`)
- Malicious names like `../../../etc/passwd` are neutralized

**Database Safety:**
- All queries use parameterized statements
- No dynamic SQL construction from user input

**Magic System Balance:**
- Failed spells consume resources (no free attempts)
- Crystal degradation prevents infinite power
- Mental fatigue requires rest cycles

## Development Philosophy

**Code Style:**
- Follow standard Rust conventions
- Use `anyhow::Result` for error handling (aliased as `GameResult`)
- Prefer composition over inheritance
- Use event system for loose coupling between systems

**Design Patterns:**
- Command Pattern: All player actions are commands
- Observer Pattern: EventBus for inter-system communication
- Strategy Pattern: Pluggable magic calculation strategies

**Modularity:**
- Systems should be loosely coupled
- Use GameEngine as coordination layer
- Avoid direct cross-system dependencies (use events instead)

## Content Expansion

**Adding New Magic Theories:**
1. Add to database `theories` table
2. Update KnowledgeSystem initialization
3. Add calculation modifiers in MagicSystem
4. Create quests that teach the theory

**Adding New Quests:**
1. Define in `src/systems/quest_examples.rs` or database
2. Specify objectives (theory requirements, dialogue triggers, etc.)
3. Link to faction reputation changes
4. Add to QuestSystem during initialization

**Adding New Items:**
1. Add to database `items` table with type-specific properties
2. Update item type enum if needed
3. Add integration with relevant systems (equipment, educational, etc.)

## Key Files Reference

- `src/core/game_engine.rs:43-89` - GameEngine initialization and system setup
- `src/input/natural_language.rs` - System verb recognition and intent parsing
- `src/input/command_handlers.rs` - All command execution logic
- `src/persistence/database.rs:55-90` - Database schema and version management
- `src/systems/magic/calculation_engine.rs` - Core magic mathematics
- `src/systems/quests.rs` - Quest state machine and progression logic
- `src/systems/items/mod.rs` - Item system public API
