# Contributing to Sympathetic Resonance

Thank you for your interest in contributing to Sympathetic Resonance! This document provides guidelines for contributing to the project.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Development Workflow](#development-workflow)
3. [Code Style](#code-style)
4. [Testing](#testing)
5. [Documentation](#documentation)
6. [Content Creation](#content-creation)
7. [Submitting Changes](#submitting-changes)

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Git
- SQLite (for database management)
- A text editor or IDE with Rust support

### Development Setup

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/yourusername/sympathetic-resonance.git
   cd sympathetic-resonance
   ```
3. Set up the upstream remote:
   ```bash
   git remote add upstream https://github.com/originalowner/sympathetic-resonance.git
   ```
4. Install dependencies and run tests:
   ```bash
   cargo build
   cargo test
   ```
5. Initialize the development database:
   ```bash
   cargo run -- --init-db
   ```

### Project Structure

```
sympathetic-resonance/
├── src/
│   ├── core/          # Core game engine
│   ├── systems/       # Game systems (magic, factions, etc.)
│   ├── input/         # Command parsing
│   ├── content/       # Content management
│   ├── persistence/   # Save/load and database
│   └── ui/           # User interface
├── content/          # Game content and database
├── tests/           # Test suite
├── docs/           # Documentation
└── assets/         # Configuration and saves
```

## Development Workflow

### Branch Naming

- `feature/description` - New features
- `bugfix/description` - Bug fixes
- `content/description` - Content additions
- `docs/description` - Documentation updates

### Development Process

1. **Create a branch** for your work:
   ```bash
   git checkout -b feature/magic-system-enhancement
   ```

2. **Make your changes** following the guidelines below

3. **Test your changes**:
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```

4. **Commit your changes** with clear, descriptive messages:
   ```bash
   git commit -m "Add crystal resonance frequency validation"
   ```

5. **Keep your branch up to date**:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

6. **Push your branch** and create a pull request

## Code Style

### Rust Guidelines

We follow standard Rust conventions with these specific requirements:

1. **Use `cargo fmt`** to format all code
2. **Use `cargo clippy`** and address all warnings
3. **Write documentation** for all public APIs
4. **Add unit tests** for new functionality

### Naming Conventions

- **Modules**: `snake_case` (e.g., `magic_system`)
- **Functions**: `snake_case` (e.g., `calculate_resonance`)
- **Types**: `PascalCase` (e.g., `MagicAttempt`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `MAX_CRYSTAL_INTEGRITY`)

### Documentation Style

```rust
/// Calculates the success probability for a magical attempt
///
/// This function takes into account the caster's abilities, crystal properties,
/// and environmental factors to determine the likelihood of success.
///
/// # Arguments
///
/// * `caster` - The player attempting the magic
/// * `spell` - The magical effect being attempted
/// * `environment` - Current environmental conditions
///
/// # Returns
///
/// A value between 0.0 and 1.0 representing success probability
///
/// # Examples
///
/// ```
/// let probability = calculate_success(&player, &healing_spell, &forest_clearing);
/// assert!(probability > 0.0 && probability <= 1.0);
/// ```
pub fn calculate_success(
    caster: &Player,
    spell: &MagicAttempt,
    environment: &Environment,
) -> f64 {
    // Implementation...
}
```

### Error Handling

- Use `anyhow::Result<T>` for functions that can fail
- Use `thiserror` for custom error types
- Provide clear, actionable error messages
- Log errors at appropriate levels

```rust
use anyhow::{Context, Result};

pub fn load_crystal_data(path: &str) -> Result<CrystalData> {
    std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read crystal data from {}", path))?
        .parse()
        .context("Invalid crystal data format")
}
```

## Testing

### Test Requirements

All contributions must include appropriate tests:

1. **Unit tests** for individual functions and methods
2. **Integration tests** for system interactions
3. **Property tests** for complex calculations (using `proptest`)
4. **Narrative tests** for story content validation

### Test Organization

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_crystal_degradation_calculation() {
        let crystal = Crystal::new(CrystalType::Quartz, 1.0, 0.8);
        let degradation = calculate_degradation(&crystal, 5);
        assert_eq!(degradation, 0.02); // 2% degradation expected
    }

    #[test]
    fn test_faction_reputation_bounds() {
        let mut faction_system = FactionSystem::new();
        faction_system.modify_reputation(FactionId::MagistersCouncil, 150);

        let reputation = faction_system.get_reputation(FactionId::MagistersCouncil);
        assert_eq!(reputation, 100); // Should be clamped to maximum
    }
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test module
cargo test magic_system

# Run with output visible
cargo test -- --nocapture

# Run tests with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

## Documentation

### Code Documentation

- All public APIs must have documentation comments
- Include examples for complex functions
- Document error conditions and edge cases
- Keep documentation up to date with code changes

### User Documentation

- Update README.md for user-facing changes
- Add entries to GAME_DESIGN_DOCUMENT.md for new systems
- Create guides for new features in the `docs/` directory
- Include command examples and screenshots where helpful

### Documentation Standards

- Use clear, concise language
- Provide practical examples
- Include troubleshooting information
- Keep documentation close to the code it describes

## Content Creation

### Database Content

Game content is stored in SQLite databases. When adding content:

1. **Follow the existing schema** defined in `src/persistence/database.rs`
2. **Use the content validation tools** to check for errors
3. **Write tests** for new content types
4. **Document content relationships** and dependencies

### Content Guidelines

- **Consistency**: New content should fit the established world and tone
- **Quality**: Proofread all text and ensure proper grammar
- **Balance**: Consider game balance implications of new content
- **Accessibility**: Use clear, inclusive language

### Content Types

- **Locations**: New areas to explore with magical signatures
- **NPCs**: Characters with faction affiliations and dialogue
- **Magic Theories**: New branches of magical knowledge
- **Items**: Crystals, artifacts, and mundane objects
- **Events**: Dynamic occurrences based on player actions

## Submitting Changes

### Pull Request Process

1. **Update documentation** for any user-facing changes
2. **Add tests** for new functionality
3. **Run the full test suite** and ensure all tests pass
4. **Update CHANGELOG.md** with a description of your changes
5. **Create a detailed pull request** description

### Pull Request Template

```markdown
## Description
Brief description of the changes made.

## Type of Change
- [ ] Bug fix (non-breaking change that fixes an issue)
- [ ] New feature (non-breaking change that adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## Testing
- [ ] All existing tests pass
- [ ] New tests added for new functionality
- [ ] Manual testing completed

## Checklist
- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No new compiler warnings
- [ ] Considered backward compatibility
```

### Review Process

1. **Automated checks** must pass (tests, linting, formatting)
2. **Code review** by at least one maintainer
3. **Testing** of new functionality
4. **Documentation review** for clarity and completeness
5. **Final approval** and merge

## Getting Help

### Communication Channels

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: General questions and design discussions
- **Discord**: Real-time development chat (link in README)
- **Email**: Direct contact with maintainers

### Common Questions

**Q: How do I add a new magic spell type?**
A: See the Magic System Guide in `docs/MAGIC_SYSTEM.md` for detailed instructions.

**Q: Can I modify the core game mechanics?**
A: Yes, but major changes should be discussed in GitHub Discussions first.

**Q: How do I test my changes with different save files?**
A: Use the `--save` flag to load specific save files for testing.

### Issue Reporting

When reporting bugs, include:
- Steps to reproduce the issue
- Expected vs actual behavior
- Your system information (OS, Rust version)
- Relevant error messages or logs
- Save file if the issue is save-specific

## Recognition

Contributors will be recognized in:
- The project's AUTHORS file
- Release notes for significant contributions
- Special recognition for major features or fixes

Thank you for contributing to Sympathetic Resonance! Your efforts help create a richer, more engaging game experience for everyone.