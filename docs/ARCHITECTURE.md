# Technical Architecture

## Overview

Sympathetic Resonance is built using a modular architecture in Rust, designed for maintainability, performance, and extensibility. This document provides technical details for developers working on the project.

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Game Engine Core                        │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Command   │  │   World     │  │   Player    │         │
│  │   Parser    │  │   State     │  │   Manager   │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Magic     │  │  Faction    │  │ Knowledge   │         │
│  │   System    │  │  System     │  │   System    │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │  Content    │  │ Persistence │  │   Event     │         │
│  │  Manager    │  │   Layer     │  │   System    │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
└─────────────────────────────────────────────────────────────┘
```

## Module Structure

### Core Modules (`src/core/`)

**game_engine.rs**
- Main game loop and state management
- Coordinates between all other systems
- Handles user input processing and response generation

**player.rs**
- Player character state and attributes
- Mental energy and fatigue tracking
- Inventory and equipment management

**world_state.rs**
- Current location and world state
- Environmental conditions and magical signatures
- Time and event tracking

**events.rs**
- Event system for loosely coupled communication
- Publish/subscribe pattern for system notifications
- Game-wide state change propagation

### System Modules (`src/systems/`)

**magic/**
- `calculation_engine.rs` - Core magic mathematics
- `resonance_system.rs` - Sympathetic resonance logic
- `crystal_management.rs` - Crystal properties and degradation

**factions/**
- `reputation.rs` - Reputation tracking and calculations
- `politics.rs` - Inter-faction relationships and conflicts

**knowledge/**
- `theory_trees.rs` - Magic theory prerequisites and progression
- `progression.rs` - Character learning and advancement

**combat/**
- `magical_combat.rs` - Turn-based magical conflict resolution

### Input/Output Modules

**input/** - Command parsing and natural language processing
**ui/** - Terminal interface and rich display formatting
**content/** - Content loading and narrative management
**persistence/** - Save/load and database operations

## Key Design Patterns

### Command Pattern
All player actions are implemented as commands with execute/undo capability:

```rust
trait Command {
    fn execute(&self, world: &mut World, player: &mut Player) -> CommandResult;
    fn can_execute(&self, world: &World, player: &Player) -> bool;
    fn get_help_text(&self) -> &str;
}
```

### Observer Pattern
Event system allows loose coupling between systems:

```rust
struct EventBus {
    listeners: HashMap<EventType, Vec<Box<dyn EventListener>>>,
}

impl EventBus {
    fn publish(&mut self, event: Event) {
        // Notify all registered listeners
    }
}
```

### Strategy Pattern
Different magic calculations use pluggable strategies:

```rust
trait MagicCalculation {
    fn calculate_success(&self, context: &MagicContext) -> f64;
    fn calculate_cost(&self, context: &MagicContext) -> i32;
}
```

## Data Flow

### Command Processing Flow
1. **Input** → Command Parser → Command Recognition
2. **Validation** → Check preconditions and resource availability
3. **Execution** → Modify game state and trigger events
4. **Response** → Generate feedback and update displays

### Magic System Flow
1. **Intent** → Player specifies magical action and target
2. **Resonance Analysis** → Calculate sympathetic connections
3. **Resource Check** → Verify mental energy and crystal availability
4. **Calculation** → Determine success probability and costs
5. **Resolution** → Apply effects and update resources
6. **Feedback** → Describe results and consequences

### Persistence Flow
1. **State Serialization** → Convert game state to JSON
2. **Validation** → Ensure data integrity and version compatibility
3. **Storage** → Write to file with backup creation
4. **Loading** → Read, validate, and deserialize save data
5. **Migration** → Handle version updates and data migration

## Performance Considerations

### Memory Management
- Use `Rc<RefCell<T>>` for shared mutable state
- Implement object pooling for frequently created/destroyed objects
- Cache frequently accessed calculations

### Computation Optimization
- Pre-compute magic success probabilities for common scenarios
- Lazy loading for content not immediately needed
- Efficient database queries with proper indexing

### Response Time Targets
- Command recognition: <50ms
- Simple actions: <100ms
- Complex magic calculations: <200ms
- Save/load operations: <500ms

## Testing Strategy

### Unit Testing
Every module has comprehensive unit tests covering:
- Normal operation scenarios
- Edge cases and boundary conditions
- Error handling and recovery

### Integration Testing
System interaction testing for:
- Magic system + faction reputation effects
- Save/load with complex game states
- Command parsing with all system integrations

### Performance Testing
Automated benchmarks for:
- Magic calculation performance
- Memory usage during extended sessions
- Database query optimization

## Dependencies

### Core Dependencies
- **serde**: Serialization for save files and configuration
- **rusqlite**: Embedded database for content storage
- **tokio**: Async runtime for potential future features
- **regex**: Command parsing and text processing

### UI Dependencies
- **ratatui**: Rich terminal user interface
- **crossterm**: Cross-platform terminal manipulation

### Development Dependencies
- **pretty_assertions**: Enhanced test output
- **tempfile**: Temporary file creation for tests

## Build Configuration

### Development Profile
```toml
[profile.dev]
opt-level = 0      # Fast compilation
debug = true       # Full debug info
```

### Release Profile
```toml
[profile.release]
opt-level = 3      # Maximum optimization
lto = true         # Link-time optimization
codegen-units = 1  # Better optimization
panic = "abort"    # Smaller binary size
```

## Security Considerations

### Input Validation
- All user input validated and sanitized
- SQL injection prevention through parameterized queries
- File system access restrictions

### Save File Security
- Save file integrity verification
- Protection against save file tampering
- Graceful handling of corrupted saves

## Deployment

### Platform Support
- Windows (x86_64)
- macOS (x86_64, ARM64)
- Linux (x86_64)

### Distribution
- Static binary with embedded content database
- No external dependencies required
- Single-file distribution for easy installation

## Monitoring and Debugging

### Logging
Structured logging with different levels:
- `ERROR`: Critical failures
- `WARN`: Recoverable issues
- `INFO`: General operation info
- `DEBUG`: Detailed debugging info
- `TRACE`: Verbose execution tracing

### Debugging Tools
- Custom debug commands for internal state inspection
- Performance profiling integration
- Memory usage tracking and leak detection

This architecture provides a solid foundation for the complex interactions required by Sympathetic Resonance while maintaining code clarity and extensibility.