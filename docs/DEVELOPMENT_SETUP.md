# Development Setup Guide

## Prerequisites

### Required Software

**Rust Development Environment:**
- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Cargo (included with Rust)
- Git 2.20+

**Development Tools:**
- SQLite 3.35+ (for database management)
- A text editor or IDE with Rust support

**Recommended IDEs:**
- Visual Studio Code with rust-analyzer extension
- CLion with Rust plugin
- Vim/Neovim with rust tools

### System Requirements

**Minimum:**
- 4GB RAM
- 2GB free disk space
- Terminal with Unicode support

**Recommended:**
- 8GB+ RAM
- 5GB+ free disk space
- Modern terminal with 256+ color support

## Initial Setup

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/sympathetic-resonance.git
cd sympathetic-resonance
```

### 2. Install Rust Dependencies

```bash
# Update to latest stable Rust
rustup update stable

# Install additional components
rustup component add clippy rustfmt

# Install development tools
cargo install cargo-tarpaulin  # Code coverage
cargo install cargo-audit      # Security auditing
cargo install cargo-watch      # File watching for development
```

### 3. Verify Installation

```bash
# Check Rust version
rustc --version  # Should be 1.70+

# Verify project builds
cargo check

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run linting
cargo clippy
```

### 4. Initialize Database

```bash
# Create initial game database
cargo run -- --init-db

# Verify database creation
ls -la content/database.db
```

## Development Workflow

### Daily Development

**Start Development Session:**
```bash
# Update dependencies
cargo update

# Run tests to ensure clean state
cargo test

# Start file watcher for continuous compilation
cargo watch -x check -x test
```

**During Development:**
```bash
# Quick compilation check
cargo check

# Run specific test module
cargo test magic_system

# Run with debug output
cargo run -- --debug

# Format code before commits
cargo fmt

# Check for common issues
cargo clippy
```

### Code Quality Standards

**Before Committing:**
```bash
# Format all code
cargo fmt

# Check for issues
cargo clippy -- -D warnings

# Run full test suite
cargo test

# Check test coverage (if tarpaulin installed)
cargo tarpaulin --out Html
```

**Performance Profiling:**
```bash
# Build with debug symbols for profiling
cargo build --release --profile=release-with-debug

# Run performance tests
cargo test --release performance_

# Memory usage analysis
valgrind --tool=massif cargo run --release
```

## Project Structure Navigation

### Key Directories

```
sympathetic-resonance/
├── src/
│   ├── core/          # Core game engine
│   ├── systems/       # Game systems (magic, factions)
│   ├── input/         # Command parsing
│   ├── content/       # Content management
│   ├── persistence/   # Save/load functionality
│   └── ui/           # User interface
├── content/          # Game content database
├── tests/           # Integration tests
├── docs/           # Documentation
└── assets/         # Configuration and saves
```

### Important Files

- `Cargo.toml` - Project configuration and dependencies
- `src/main.rs` - Application entry point
- `src/lib.rs` - Library root with public API
- `GAME_DESIGN_DOCUMENT.md` - Complete game specification
- `BALANCE_FRAMEWORK.md` - Numerical balance reference

## Development Commands

### Common Tasks

**Building:**
```bash
cargo build                    # Debug build
cargo build --release         # Optimized build
cargo check                   # Fast compilation check
```

**Testing:**
```bash
cargo test                     # Run all tests
cargo test magic_system        # Run specific module tests
cargo test -- --nocapture     # Show test output
cargo test --release          # Test optimized build
```

**Running:**
```bash
cargo run                     # Run with debug build
cargo run --release          # Run optimized version
cargo run -- --help         # Show command line options
cargo run -- --debug        # Enable debug mode
```

**Code Quality:**
```bash
cargo fmt                     # Format code
cargo clippy                  # Lint code
cargo audit                   # Security audit
cargo doc --open             # Generate and open docs
```

### Database Management

**Development Database:**
```bash
# Initialize fresh database
cargo run -- --init-db

# Backup current database
cp content/database.db content/database.backup

# View database schema
sqlite3 content/database.db ".schema"

# Query database directly
sqlite3 content/database.db "SELECT * FROM locations LIMIT 5;"
```

### Content Development

**Adding New Content:**
1. Edit content in `content/` directory
2. Update database schema if needed
3. Run validation: `cargo test content_validation`
4. Test in game: `cargo run`

**Content Validation:**
```bash
# Validate all content consistency
cargo test content_

# Check specific content type
cargo test location_validation
cargo test dialogue_validation
```

## IDE Configuration

### Visual Studio Code

**Recommended Extensions:**
- rust-analyzer (official Rust support)
- Error Lens (inline error display)
- GitLens (Git integration)
- SQLite Viewer (database inspection)

**Settings (`.vscode/settings.json`):**
```json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.cargo.features": "all",
    "editor.formatOnSave": true,
    "files.associations": {
        "*.md": "markdown"
    }
}
```

**Tasks (`.vscode/tasks.json`):**
```json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "cargo check",
            "type": "cargo",
            "command": "check",
            "group": "build"
        },
        {
            "label": "cargo test",
            "type": "cargo",
            "command": "test",
            "group": "test"
        }
    ]
}
```

### CLion Configuration

**Rust Plugin Settings:**
- Enable "Use cargo check"
- Set "Run clippy on external changes"
- Enable "Format with rustfmt on save"

## Debugging

### Debug Builds

```bash
# Build with debug symbols
cargo build

# Run with environment logging
RUST_LOG=debug cargo run

# Run specific log level
RUST_LOG=sympathetic_resonance::magic=trace cargo run
```

### Common Issues

**Compilation Errors:**
- Check Rust version: `rustc --version`
- Update dependencies: `cargo update`
- Clean build: `cargo clean && cargo build`

**Database Issues:**
- Reinitialize: `rm content/database.db && cargo run -- --init-db`
- Check permissions: `ls -la content/`
- Verify SQLite installation: `sqlite3 --version`

**Performance Issues:**
- Profile with: `cargo build --release && perf record cargo run --release`
- Memory analysis: `valgrind cargo run`
- Check debug build: Debug builds are significantly slower

## Testing

### Test Categories

**Unit Tests:**
```bash
cargo test tests::unit          # All unit tests
cargo test magic::tests         # Magic system unit tests
cargo test faction::tests       # Faction system tests
```

**Integration Tests:**
```bash
cargo test tests::integration   # All integration tests
cargo test magic_faction_integration  # Specific integration
```

**Performance Tests:**
```bash
cargo test --release tests::performance
```

### Writing Tests

**Test Structure:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_magic_calculation() {
        // Arrange
        let player = create_test_player();
        let crystal = Crystal::new(CrystalType::Quartz, 0.8);

        // Act
        let result = calculate_magic_success(&player, &crystal);

        // Assert
        assert!(result > 0.0 && result <= 1.0);
    }
}
```

### Test Data

**Creating Test Data:**
```rust
// Use builder pattern for complex test objects
let player = PlayerBuilder::new()
    .mental_acuity(50)
    .resonance_sensitivity(30)
    .build();

// Use fixtures for consistent test data
let test_location = load_test_location("market_square");
```

## Continuous Integration

### GitHub Actions Workflow

The repository includes automated CI that runs:
- Code formatting check (`cargo fmt --check`)
- Linting (`cargo clippy`)
- Test suite (`cargo test`)
- Security audit (`cargo audit`)

**Local CI Simulation:**
```bash
# Run all CI checks locally
./scripts/ci-check.sh

# Or manually:
cargo fmt --check && \
cargo clippy -- -D warnings && \
cargo test && \
cargo audit
```

## Contributing

### Before Submitting Changes

1. **Code Quality:**
   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   cargo test
   ```

2. **Documentation:**
   - Update relevant documentation
   - Add docstrings for new public functions
   - Update CHANGELOG.md if applicable

3. **Testing:**
   - Add tests for new functionality
   - Ensure existing tests still pass
   - Test edge cases and error conditions

### Performance Requirements

**Response Time Targets:**
- Command recognition: <50ms
- Simple actions: <100ms
- Complex calculations: <200ms
- Save/load: <500ms

**Memory Usage:**
- Typical session: <50MB
- Extended session: <100MB
- Maximum acceptable: <200MB

### Code Review Checklist

- [ ] Code follows Rust conventions and project style
- [ ] All public functions have documentation
- [ ] Tests cover new functionality and edge cases
- [ ] No clippy warnings or errors
- [ ] Performance targets met for new features
- [ ] Database migrations (if any) are reversible
- [ ] Error handling follows project patterns

This setup guide ensures all developers can quickly get started and maintain consistent development practices across the project.