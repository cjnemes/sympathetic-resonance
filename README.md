# Sympathetic Resonance

A text adventure game featuring science-based magic in a unique low fantasy world.

## Overview

Experience a world where magic operates according to scientific principles, where crystal-amplified neural energy creates measurable electromagnetic effects. Navigate complex faction politics, manage finite magical resources, and uncover the mysteries of Sympathetic Resonance in this rich, choice-driven narrative adventure.

## Key Features

- **Science-Based Magic System**: Magic follows consistent physical laws with measurable costs and limitations
- **Complex Character Progression**: Advance through theoretical understanding rather than traditional skills
- **Dynamic Faction System**: Five major factions with interconnected politics and reputation consequences
- **Rich Resource Management**: Balance mental energy, crystal degradation, and political relationships
- **Meaningful Choices**: Decisions have lasting consequences across multiple game systems
- **Quest System**: Structured educational progression with faction-aware storylines
- **Environmental Storytelling**: Discover lore through magical resonance signatures and world details

## Quick Start

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Git

### Installation

```bash
git clone https://github.com/yourusername/sympathetic-resonance.git
cd sympathetic-resonance
cargo build --release
```

### Running the Game

```bash
cargo run --release
```

### Development Build

```bash
cargo run
```

## Project Structure

```
sympathetic-resonance/
├── src/
│   ├── core/          # Core game engine (world state, player, events)
│   ├── systems/       # Game systems (magic, factions, knowledge, quests, combat)
│   ├── input/         # Command parsing and natural language processing
│   ├── content/       # Content loading and narrative engine
│   ├── persistence/   # Save/load system and database operations
│   └── ui/           # Terminal interface and display systems
├── content/          # Game content database and narrative assets
├── tests/           # Comprehensive testing suite
├── assets/          # Save files and configuration
└── docs/           # Additional documentation
```

## Magic System Overview

The game's magic system is based on **Sympathetic Resonance**, where:

- Neural energy from practitioners is amplified through crystal matrices
- Different crystals resonate at specific frequencies (1-10)
- Magic requires sympathetic connections between caster and target
- Mental fatigue accumulates with use and requires rest to recover
- Crystals degrade over time and with use, becoming less efficient

### Example Magic Use

```
> examine crystal formation
You notice an amethyst cluster resonating at frequency 7, with high purity but
showing minor stress fractures. The formation hums softly with residual magical energy.

> cast sympathetic healing using amethyst on wounded guard
You focus your mental energy through the amethyst, creating a resonance link with
the guard's leather bracelet. The sympathetic connection strengthens the body's
natural healing processes.

[Mental Energy: 65→50, Amethyst degradation: 2%, Healing successful]
```

## Faction System

Five major factions shape the political landscape:

- **Magisters' Council**: Academic regulation and controlled study
- **Order of Natural Harmony**: Traditional values with magical integration
- **Industrial Consortium**: Commercial magic applications and progress
- **Underground Network**: Free magical knowledge and unregulated practice
- **Neutral Scholars**: Independent research and cross-faction cooperation

Actions affect reputation with multiple factions simultaneously, creating complex political considerations for every choice.

## Quest System

The quest system provides structured educational progression through the game's magic theory learning framework. Quests guide players through scientific discovery while respecting player autonomy and exploration preferences.

### Quest Types

- **Theory Foundation Quests**: Introduction to magical principles and scientific methodology
- **Practical Application Quests**: Hands-on spellcasting with theoretical reinforcement
- **Faction Political Quests**: Navigate complex relationships and diplomatic challenges
- **Advanced Research Quests**: Multi-theory synthesis and advanced magical techniques
- **Collaborative Quests**: Cross-faction cooperation and mentorship opportunities

### Educational Framework

- **Competency-Based Progression**: Demonstrate understanding, not just completion
- **Multiple Learning Pathways**: Study, experimentation, observation, teaching, research, mentorship
- **Scientific Thinking**: Hypothesis formation, testing, and iterative improvement
- **Systems Integration**: Understand connections between magic, politics, and society

### Example Quest Progression

```
> quest status
Active Quests:
- "Resonance Foundation": Learn basic theory of sympathetic resonance (2/3 objectives)
  ✓ Visit the Resonance Observatory
  ✓ Talk to Scholar Elena about frequency theory
  ○ Study harmonic_resonance theory to level 1.0

> study harmonic_resonance
You spend time analyzing the relationship between crystal frequencies and neural
resonance patterns. The mathematical principles become clearer as you work through
the theoretical framework.

[Understanding gained: 0.8 → 1.2, Quest objective completed!]

Quest "Resonance Foundation" completed!
Unlocked: "Crystalline Applications" quest chain
```

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test magic_system

# Run with output
cargo test -- --nocapture
```

### Code Style

This project follows standard Rust conventions:

```bash
# Format code
cargo fmt

# Check for issues
cargo clippy

# Check without building
cargo check
```

### Database Schema

The game uses SQLite for content storage. Initialize the database:

```bash
# Create initial database (automated on first run)
cargo run -- --init-db
```

### Content Creation

Game content is stored in SQLite and can be modified through:

1. Direct SQL editing of `content/database.db`
2. JSON import/export tools (planned)
3. Content creation utilities (planned)

## Documentation

- [Game Design Document](GAME_DESIGN_DOCUMENT.md) - Complete design specification
- [Technical Architecture](docs/ARCHITECTURE.md) - Code organization and patterns
- [Magic System Guide](docs/MAGIC_SYSTEM.md) - Detailed mechanics documentation
- [Faction Guide](docs/FACTIONS.md) - Political system and reputation mechanics
- [Contributing Guide](CONTRIBUTING.md) - Development workflow and guidelines

## Commands Reference

### Basic Commands
- `look` / `examine <target>` - Observe your surroundings or specific objects
- `go <direction>` / `north/south/east/west` - Move between locations
- `inventory` / `inv` - Check your possessions
- `status` - View character stats and current state

### Magic Commands
- `cast <spell> using <crystal>` - Perform magical actions
- `analyze <target>` - Examine magical properties
- `resonate <crystal>` - Test crystal frequency and condition
- `study <theory>` - Learn magical principles

### Social Commands
- `talk to <person>` - Engage in conversation
- `ask <person> about <topic>` - Request specific information
- `faction status` - Check standing with all factions

### Quest Commands
- `quest status` - View active and completed quests
- `quest start <quest_name>` - Begin a new quest (if requirements met)
- `quest help` - Show quest system overview

### System Commands
- `save` - Save current game state
- `load` - Load saved game
- `help` - Show available commands
- `quit` - Exit the game

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Areas for Contribution

- **Content Creation**: New locations, dialogues, magical theories
- **System Enhancement**: Additional magic applications, faction interactions
- **UI/UX Improvements**: Better command parsing, help systems
- **Testing**: Comprehensive test coverage for all systems
- **Documentation**: Guides, tutorials, and API documentation

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Brandon Sanderson's Cosmere for inspiration on hard magic systems
- Classic text adventures (Zork, Adventure) for foundational design
- The Rust game development community for excellent libraries and tools

## Support

- Report bugs and request features through [GitHub Issues](https://github.com/yourusername/sympathetic-resonance/issues)
- Join discussions in [GitHub Discussions](https://github.com/yourusername/sympathetic-resonance/discussions)
- Contact the development team at [email@example.com](mailto:email@example.com)

---

*"In the resonance between mind and crystal, between theory and practice, between choice and consequence, lies the true magic of discovery."*