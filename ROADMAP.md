# Sympathetic Resonance - Development Roadmap

**Last Updated:** 2025-09-30
**Current Version:** v0.4.0
**Status:** Core Systems Complete, Content Expansion Phase

---

## 🎯 Project Vision

A text adventure game featuring science-based magic in a low fantasy world, where players learn through experimentation, manage finite resources, and navigate complex faction politics.

---

## ✅ Completed Features (v0.1.0 - v0.4.0)

### Core Engine
- [x] Main game loop and command processing
- [x] Natural language command parser with advanced intent recognition
- [x] Player state management (attributes, inventory, knowledge)
- [x] World state and location system
- [x] Save/load functionality with platform-specific directories
- [x] Database-driven content management (SQLite)

### Magic System
- [x] Sympathetic resonance calculations
- [x] Crystal management and degradation
- [x] 6 spell types (light, detection, healing, shield, growth, minor_telekinesis)
- [x] Theory-based magic bonuses
- [x] Resource costs (mental energy, fatigue, time)
- [x] Success probability calculations

### Knowledge & Learning
- [x] 9 magic theories across 3 tiers
- [x] 6 learning methods (study, experimentation, observation, teaching, research, mentorship)
- [x] Theory prerequisite system
- [x] Learning efficiency modifiers
- [x] Progress tracking and mastery system

### Faction System
- [x] 5 major factions with distinct philosophies
- [x] Reputation tracking with cross-faction effects
- [x] Political relationship modeling
- [x] Faction-specific dialogue options

### Quest System
- [x] Quest definition and tracking framework
- [x] 6 educational quest templates
- [x] Objective types (theory, dialogue, location, item, faction)
- [x] Quest recommendations based on player state
- [x] Integration with dialogue, knowledge, and faction systems

### Item System
- [x] Core item framework with types and properties
- [x] Equipment system with slot management
- [x] Inventory constraints (weight, space, categories)
- [x] Educational items that enhance learning
- [x] Item stacking and organization
- [x] Integration with magic and knowledge systems

### User Interface
- [x] Comprehensive help system
- [x] Rich command feedback and descriptions
- [x] Status displays (player, crystals, factions, quests)
- [x] Natural language error suggestions

---

## 📊 Feature Status Legend

| Status | Meaning |
|--------|---------|
| **✅ Implemented** | Feature is complete, tested, and in production |
| **🚧 In Progress** | Actively being developed |
| **📋 Planned** | Scheduled for implementation, scope defined |
| **💡 Proposed** | Under consideration, not yet committed |
| **❌ Out of Scope** | Not planned for current project vision |

---

## 🚧 Currently Unimplemented Features

### Combat System - ❌ **Out of Scope** (MVP)

**Status:** Stub implementation exists
**Location:** `src/systems/combat.rs`
**Decision:** Combat is not essential to the core educational experience

**Rationale:**
- Game focuses on learning, exploration, and political navigation
- Magic system already provides conflict resolution through spells
- Combat would require significant balancing and complexity
- Can be added post-MVP if player feedback demands it

**If Implemented (Future):**
- Turn-based magical combat
- Integration with spell system
- Strategic use of theories and crystals
- Faction-aware encounter generation

---

### Item Interactions - 📋 **Planned** (Post-MVP)

#### Take/Drop Items - 📋 **Planned** (v0.5.0)
**Current State:** Placeholder messages
**Location:** `src/input/command_handlers.rs:833-838`

**Scope:**
- Basic item pickup from locations
- Drop items to current location
- Integration with existing ItemSystem
- Inventory weight/space validation

**Implementation Estimate:** 2-3 days
**Dependencies:** None (ItemSystem ready)

---

#### Crafting System - 💡 **Proposed** (v0.6.0+)

**Current State:** Command parsed, returns "not yet implemented"
**Location:** `src/input/command_handlers.rs:176`

**Proposed Scope:**
- Item combination rules
- Recipe discovery through theory knowledge
- Crystal modification and enhancement
- Educational tool creation

**Considerations:**
- Should crafting be theory-driven?
- Balance complexity vs. educational value
- Integration with faction workshops?

**Implementation Estimate:** 1-2 weeks
**Dependencies:** Take/Drop items, expanded item types

---

#### Give Items to NPCs - 💡 **Proposed** (v0.6.0+)

**Current State:** Command parsed, returns "not yet implemented"
**Location:** `src/input/command_handlers.rs:195`

**Proposed Scope:**
- Gift items to NPCs for reputation
- Quest-required item delivery
- Trading system (optional)

**Implementation Estimate:** 3-5 days
**Dependencies:** NPC relationship tracking

---

#### Unequip Items - 📋 **Planned** (v0.5.0)

**Current State:** Placeholder message
**Location:** `src/input/command_handlers.rs:167-169`

**Scope:**
- Remove equipped items
- Return to inventory
- Stat modifier removal
- Equipment slot management

**Implementation Estimate:** 1 day
**Dependencies:** None (EquipmentManager ready)

---

### Enhanced Examination - 💡 **Proposed** (v0.6.0+)

#### Detailed Object Examination
**Current State:** Generic messages
**Location:** `src/input/command_handlers.rs:244`

**Proposed Scope:**
- Object-specific descriptions
- Hidden details revealed by high resonance sensitivity
- Lore discovery system
- Integration with quest objectives

---

#### Magical Analysis
**Current State:** Placeholder message
**Location:** `src/input/command_handlers.rs:278`

**Proposed Scope:**
- Detect magical properties of objects
- Reveal resonance frequencies
- Theory requirements for advanced analysis
- Crystal compatibility detection

---

### Quest System Enhancements - 📋 **Planned** (v0.5.0-v0.6.0)

#### Quest Abandonment
**Current State:** Returns "not yet implemented"
**Location:** `src/input/command_handlers.rs:1072`

**Scope:**
- Allow players to abandon active quests
- Faction reputation penalties (if appropriate)
- Resource refund logic
- Re-availability conditions

**Implementation Estimate:** 2 days

---

#### Quest Content Expansion - 🚧 **In Progress**
**Priority:** **HIGH** - Maximum player value

**Phase 1 (v0.5.0) - Narrative Depth:**
- Expand 6 quest templates with rich dialogue
- Add faction-specific variations
- Create branching outcomes
- Estimated: 2-3 weeks

**Phase 2 (v0.6.0) - Advanced Quests:**
- Multi-step quest chains
- Cross-faction collaboration quests
- Theory synthesis challenges
- Estimated: 3-4 weeks

---

### Health System - 💡 **Proposed** (Requires Combat Decision)

**Current State:** Item effects reference non-existent health
**Location:** `src/systems/items/mod.rs:281`

**Dependencies:** Combat system decision
**Status:** Blocked until combat scope determined

**If Implemented:**
- Health points and damage tracking
- Healing spells and items
- Fatigue effects on health
- Injury system

---

## 📅 Roadmap by Version

### v0.5.0 - "Interaction Expansion" (Est. 2-3 weeks)

**Focus:** Complete item interactions and quest content

**Features:**
- ✅ Take/Drop items
- ✅ Unequip functionality
- ✅ Quest abandonment
- ✅ Quest Phase 1 narrative expansion
- ✅ NPC dialogue expansion
- ✅ Location description enrichment

**Success Metrics:**
- Players can fully interact with item system
- At least 15 rich quest experiences available
- Average quest completion time: 15-20 minutes

---

### v0.6.0 - "Depth & Discovery" (Est. 4-6 weeks)

**Focus:** Advanced features and content depth

**Features:**
- 📋 Crafting system
- 📋 Enhanced examination with lore discovery
- 📋 Magical analysis capabilities
- 📋 Quest Phase 2 advanced chains
- 📋 Give items to NPCs
- 💡 Secret discovery system

**Success Metrics:**
- Crafting provides meaningful player choices
- Lore system encourages exploration
- 25+ hours of unique gameplay content

---

### v0.7.0 - "Polish & Balance" (Est. 3-4 weeks)

**Focus:** Game balance, UX refinement, content polish

**Features:**
- 📋 Theory balance pass
- 📋 Magic cost tuning
- 📋 Faction reputation rebalancing
- 📋 Tutorial improvements
- 📋 Achievement system (optional)
- 📋 Extended ending content

**Success Metrics:**
- Balanced difficulty curve
- Clear player progression path
- Satisfying conclusion experience

---

### v1.0.0 - "Release Candidate" (Est. 2-3 weeks)

**Focus:** Final polish, testing, documentation

**Deliverables:**
- 📋 Complete player guide
- 📋 Full test coverage
- 📋 Performance optimization
- 📋 Bug fixes and edge cases
- 📋 Release documentation
- 📋 Community setup (if applicable)

---

## 🚫 Explicitly Out of Scope

### For Current Project Vision:

1. **Graphical Interface**
   - Decision: Pure text adventure maintains focus
   - Alternative: Terminal UI enhancements only

2. **Multiplayer/Online Features**
   - Decision: Single-player experience is core
   - Complexity: Too high for current scope

3. **Real-Time Elements**
   - Decision: Turn-based only
   - Rationale: Supports thoughtful learning approach

4. **Extensive Combat System**
   - Decision: Educational focus over combat
   - See Combat System section above

5. **Economy/Trading System**
   - Decision: Not aligned with educational goals
   - Alternative: Gift-giving for reputation only

---

## 📈 Success Criteria by Phase

### MVP (v0.4.0 - v0.5.0)
- ✅ All core systems functional
- ✅ 10+ hours of gameplay content
- ✅ Complete learning progression (Tier 1-3 theories)
- 🚧 Full item interaction capabilities

### Release (v1.0.0)
- 📋 25+ hours unique content
- 📋 Multiple playthrough variety (faction paths)
- 📋 Complete quest chains for all factions
- 📋 Polished UX with helpful feedback
- 📋 Comprehensive documentation

---

## 🔄 Feature Request Process

### Adding New Features
1. Create GitHub issue with "enhancement" label
2. Discuss alignment with project vision
3. Classify as: Planned, Proposed, or Out of Scope
4. Update this ROADMAP.md with decision
5. Estimate and schedule if approved

### Changing Feature Status
- Changes must be documented in ROADMAP.md
- Major scope changes require issue discussion
- Update version targets when priorities shift

---

## 📝 Notes for Future Development

### When to Implement Combat:
- Player feedback strongly requests it
- Educational value can be demonstrated
- Resource allocation allows for proper balance
- Can integrate theory application meaningfully

### When to Add Crafting:
- Item interaction basics are polished
- Player has expressed need for item customization
- Theory-crafting connections are well-defined
- Balancing resources available

### Content vs. Features Priority:
**Current Philosophy:** Rich content in existing systems > new feature scaffolding
- Focus on quest narrative depth
- Expand NPC personalities and dialogues
- Deepen location descriptions
- Add lore discovery mechanics

---

## 🎓 Educational Philosophy

Features should serve the learning experience:
- ✅ **Theory-driven progression** - Understanding unlocks capabilities
- ✅ **Multiple learning pathways** - Accommodate different play styles
- ✅ **Scientific thinking** - Hypothesis, test, iterate
- ✅ **Meaningful consequences** - Choices matter
- ❌ **Grind mechanics** - No repetition without learning
- ❌ **Arbitrary gates** - Progress through understanding, not metrics

---

**Maintained by:** Development Team
**Review Cycle:** After each major version
**Last Reviewed:** 2025-09-30

*This roadmap is a living document. Features may be added, removed, or re-prioritized based on development realities and player feedback.*
