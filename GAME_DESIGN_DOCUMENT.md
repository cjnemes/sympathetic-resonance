# Sympathetic Resonance: Game Design Document

## Table of Contents
1. [Game Overview](#game-overview)
2. [World & Lore](#world--lore)
3. [Magic System](#magic-system)
4. [Character Progression](#character-progression)
5. [Gameplay Mechanics](#gameplay-mechanics)
6. [Faction System](#faction-system)
7. [Technical Architecture](#technical-architecture)
8. [Development Roadmap](#development-roadmap)

## Game Overview

**Genre:** Text Adventure / Interactive Fiction
**Setting:** Low Fantasy with Science-Based Magic
**Target Audience:** Players who enjoy deep narrative systems, resource management, and exploration
**Platform:** Cross-platform terminal/console application

### Core Pillars
- **Scientific Magic:** Magic system based on consistent, observable scientific principles
- **Meaningful Choices:** Player decisions have lasting consequences across multiple systems
- **Resource Management:** Balance mental energy, crystal degradation, and faction relationships
- **Rich Narrative:** Deep worldbuilding with environmental storytelling and faction politics

### Unique Selling Points
- First text adventure to feature scientifically grounded magic with measurable resource costs
- Complex faction system where every choice affects multiple relationships
- Theory-based progression where players learn magic through experimentation and study
- Environmental storytelling through magical resonance signatures

## World & Lore

### The World of Resonance

**Historical Context:**
150 years ago, the phenomenon of Sympathetic Resonance was discovered when scholars noticed that certain crystalline formations could amplify human neural energy to create measurable electromagnetic effects. This discovery has fundamentally altered society, politics, and daily life.

### The Magic System: Sympathetic Resonance

**Core Principle:**
Magic operates through the scientific principle of sympathetic resonance, where neural energy from trained practitioners is amplified through crystalline matrices to create measurable electromagnetic effects that can influence matter and energy.

**Scientific Foundation:**
- **Energy Source:** Human neural electrical activity (measurable in microvolts)
- **Amplification Medium:** Naturally occurring crystal lattices with specific resonance frequencies
- **Physical Law:** Conservation of energy applies—greater effects require more neural energy
- **Observable Effects:** Electromagnetic signatures detectable with proper instruments

**Limitations:**
- Mental fatigue accumulates with use, requiring rest to recover
- Crystals degrade over time and with use, becoming less efficient
- Stronger effects require stronger sympathetic connections between caster and target
- Maximum output limited by practitioner's neural capacity and crystal quality

### Key Locations

**Resonance Prime:** The capital city where magic was first discovered. Home to the Magisters' Council and major magical institutions.

**The Crystal Wastes:** Dangerous badlands where unstable magical phenomena occur naturally. Source of rare crystals but highly hazardous.

**Port Harmony:** Coastal trading city where different factions maintain uneasy cooperation. Major hub for crystal trade.

**The Underground:** Hidden network of tunnels and safe houses used by unregulated magical practitioners.

**The Sanctum:** Mountain monastery where the Order of Natural Harmony studies magic's integration with traditional ways of life.

### Historical Events

**The First Resonance (Year 0):** Scholar Mira Tenlan accidentally activates a crystal formation while studying geological samples, creating the first recorded magical effect.

**The Crystal Wars (Years 45-52):** Military conflicts over control of major crystal deposits, leading to current regulatory framework.

**The Great Reformation (Year 89):** Political restructuring that established the five major factions and their spheres of influence.

**The Resonance Plague (Year 134):** Magical disaster caused by uncontrolled experimentation, leading to stricter magical regulations.

## Magic System

### Sympathetic Resonance Mechanics

**Resonance Frequency Matching:**
Different crystals resonate at different frequencies (1-10), and spells have optimal frequency ranges. Perfect matches provide bonuses, while mismatches create penalties.

**Sympathetic Connection Strength:**
- **Strong Connections:** Personal items, family members (+20 success bonus)
- **Moderate Connections:** Recently touched objects, acquaintances (+10 bonus)
- **Weak Connections:** Observed items, strangers (no bonus)
- **Forced Connections:** Unrelated targets (-30 penalty, requires advanced training)

**Crystal Properties:**
- **Type:** Quartz, Amethyst, Obsidian, etc. (determines resonance frequency)
- **Purity:** 0-100% (affects efficiency and degradation resistance)
- **Size:** Tiny/Small/Medium/Large (affects maximum power output)
- **Structural Integrity:** 0-100% (decreases with use, affects reliability)

**Mental Energy & Fatigue:**
- **Current Mental Energy:** 0-100 based on character's Mental Acuity stat
- **Mental Fatigue:** 0-100 accumulated through magical use
- **Effective Energy:** Current Energy minus (Fatigue × 0.5)
- **Recovery:** Rest (-10/hour), Sleep (-30/night), Meditation (-15/hour)

### Magic Applications

**Sympathetic Healing:**
Use personal connection to patient (hair, belongings) to accelerate natural healing processes.

**Object Location:**
Create resonance with lost object through items that were previously in contact.

**Structural Analysis:**
Analyze material composition and stress points in buildings or objects.

**Communication Enhancement:**
Amplify voice or create sympathetic connections for distant communication.

**Environmental Sensing:**
Detect magical signatures, recent magical activity, or hidden magical items.

## Character Progression

### Core Attributes

**Mental Acuity (0-100):**
- Determines maximum mental energy pool (Mental Acuity × 2)
- Affects learning speed for new magical theories
- Reduces mental fatigue accumulation rate
- Progression: Study, successful experiments, puzzle solving

**Resonance Sensitivity (0-100):**
- Determines success rates for magical actions
- Allows detection of subtle magical phenomena
- Affects crystal efficiency (reduces degradation per use)
- Progression: Magical practice, exposure to different crystal types

**Social Standing (0-100 per faction):**
- Influences available opportunities and information access
- Affects pricing and availability of crystals/services
- Determines political power and influence
- Progression: Faction-aligned actions, completing missions

### Knowledge System

Instead of traditional skill trees, players develop **Theoretical Understanding** in branching knowledge areas:

**Harmonic Fundamentals (Required for all magic):**
- Basic resonance principles
- Energy conservation understanding
- Simple frequency matching

**Crystal Lattice Theory:**
- Understanding crystal structures
- Predicting degradation patterns
- Crystal enhancement techniques

**Neural Amplification:**
- Efficient mental energy usage
- Fatigue resistance techniques
- Enhanced focus methods

**Sympathetic Binding:**
- Creating connections between disparate objects
- Long-distance magical effects
- Persistent magical links

**Resonance Disruption:**
- Defensive magic techniques
- Countering other magical effects
- Protective barriers and shields

### Progression Methods

**Experimentation:**
Players can attempt to discover new applications by combining known theories with creative approaches. Success depends on theoretical knowledge and approach quality.

**Study:**
Reading texts, attending lectures, and formal education increase theoretical understanding and unlock new capabilities.

**Practice:**
Repeated use of magic improves efficiency and reduces failure rates, but also degrades crystals and causes fatigue.

**Discovery:**
Finding ancient artifacts, unusual natural phenomena, or revolutionary insights can unlock entirely new theoretical branches.

## Gameplay Mechanics

### Core Gameplay Loop

**Minute-to-Minute:**
- Observe environment for magical signatures and clues
- Experiment with magical interactions using available crystals
- Manage mental fatigue and crystal degradation
- Navigate social/political situations considering faction relationships

**Session-to-Session:**
- Investigate larger mysteries (new phenomena, political plots)
- Develop magical theories through experimentation and study
- Build relationships with factions and key NPCs
- Acquire rare crystals and expand magical capabilities

### Resource Management

**Mental Energy Management:**
Players must balance magical usage with mental energy reserves, planning rest periods and managing fatigue accumulation.

**Crystal Economy:**
Crystals are finite resources that degrade with use. Players must balance immediate magical needs with long-term resource preservation.

**Faction Reputation:**
Actions affect standing with multiple factions simultaneously, requiring careful consideration of long-term political consequences.

### Conflict Resolution

**Magical Combat:**
Turn-based system where players use magical effects strategically, managing energy costs and crystal efficiency while countering opponent actions.

**Social Encounters:**
Dialogue trees modified by faction reputation and character knowledge, where magical displays can help or harm depending on context.

**Environmental Challenges:**
Puzzles requiring creative magical applications, often with multiple valid solutions based on player's theoretical knowledge.

### Choice & Consequence System

**Immediate Consequences:** Success/failure of magical attempts, NPC reactions
**Short-term Consequences:** Faction reputation changes, access to areas/information
**Long-term Consequences:** Major faction conflicts, availability of advanced theories, personal rivalries/alliances

## Faction System

### The Five Major Factions

**The Magisters' Council (Academic/Regulatory):**
- **Philosophy:** Controlled study and regulation of magical practice
- **Benefits:** Research access, legal protection, formal training opportunities
- **Conflicts:** Opposes unregulated practice and underground research

**The Order of Natural Harmony (Conservative/Religious):**
- **Philosophy:** Magic should supplement, not replace, the natural order
- **Benefits:** Community support, healing knowledge, moral authority
- **Conflicts:** Opposes industrial magic applications and radical experimentation

**The Industrial Consortium (Commercial/Progressive):**
- **Philosophy:** Magic as a tool for economic progress and technological advancement
- **Benefits:** Funding opportunities, cutting-edge crystals, economic networks
- **Conflicts:** Clashes with traditionalists and those opposing magical monopolies

**The Underground Network (Libertarian/Revolutionary):**
- **Philosophy:** Magical knowledge should be free and unregulated by authority
- **Benefits:** Access to forbidden knowledge, black market connections, personal freedom
- **Conflicts:** Directly opposes government regulation and conservative restrictions

**The Neutral Scholars (Academic/Independent):**
- **Philosophy:** Pure research and knowledge sharing without political agenda
- **Benefits:** Unbiased information, cross-faction cooperation opportunities
- **Conflicts:** Pressure from other factions demanding exclusive loyalty

### Reputation Mechanics

**Standing Scale (-100 to +100 per faction):**
- +81 to +100: Inner Circle (access to secrets, leadership roles)
- +51 to +80: Trusted Ally (special missions, advanced resources)
- +21 to +50: Member (regular access, basic support)
- -20 to +20: Neutral (standard interactions)
- -50 to -21: Suspected (restricted access, surveillance)
- -80 to -51: Enemy (refused service, active opposition)
- -100 to -81: Marked for Elimination (hunted, attacked on sight)

**Reputation Changes:**
Actions affect multiple factions simultaneously. Helping one faction often harms standing with their rivals, creating complex political calculations.

## Technical Architecture

### Technology Stack
- **Language:** Rust (memory safety, performance, cross-platform)
- **Database:** SQLite (embedded, reliable, SQL queries for complex content)
- **Serialization:** Serde (JSON/binary save files)
- **UI:** Ratatui (rich terminal interface)
- **Testing:** Built-in Rust testing framework

### Key Systems

**Command Parser:**
Three-layer system: tokenization → intent recognition → command execution. Supports natural language input with fallback to explicit commands.

**Magic Calculation Engine:**
Modular system for computing magical success rates, fatigue accumulation, and crystal degradation based on character stats and environmental factors.

**World State Manager:**
Persistent game world with complex NPC relationships, faction politics, and dynamic events based on player choices.

**Content Management:**
Database-driven content system allowing easy addition of locations, dialogues, and magical theories without code changes.

### Project Structure
```
adventure_game/
├── src/
│   ├── core/          # Core game engine
│   ├── systems/       # Game systems (magic, factions, etc.)
│   ├── input/         # Command parsing and input handling
│   ├── content/       # Content loading and management
│   ├── persistence/   # Save/load and database operations
│   └── ui/           # User interface and display
├── content/          # Game content database and assets
├── tests/           # Comprehensive testing suite
└── assets/          # Save files and configuration
```

## Development Roadmap

### Phase 1: Foundation (Weeks 1-4)
- [ ] Set up Rust project with core dependencies
- [ ] Implement basic data structures (Player, World, Magic)
- [ ] Create SQLite schema and content loading system
- [ ] Build simple command parsing for core verbs
- [ ] Develop basic magic calculation engine with tests

### Phase 2: Core Systems (Weeks 5-8)
- [ ] Implement full magic system with all calculation types
- [ ] Create faction reputation system
- [ ] Build character progression and knowledge trees
- [ ] Develop save/load system for complex game state
- [ ] Add basic combat and social interaction systems

### Phase 3: Content & Polish (Weeks 9-12)
- [ ] Create rich content database with multiple locations
- [ ] Implement full dialogue and narrative systems
- [ ] Add comprehensive testing for all game systems
- [ ] Create detailed documentation and user guides
- [ ] Polish user interface and command parsing

### Phase 4: Enhancement (Weeks 13-16)
- [ ] Add advanced features (crystal crafting, theory research)
- [ ] Implement complex multi-session story arcs
- [ ] Create mod support and content creation tools
- [ ] Performance optimization and bug fixing
- [ ] Beta testing and community feedback integration

### Success Metrics
- **Technical:** All major systems implemented with comprehensive test coverage
- **Gameplay:** Rich, interconnected systems that create emergent storytelling
- **Content:** Multiple faction questlines with meaningful choice consequences
- **Polish:** Intuitive command interface with helpful feedback and error handling

## Appendices

### Glossary of Terms
- **Sympathetic Resonance:** The scientific principle underlying the magic system
- **Neural Amplification:** Using mental energy to power magical effects
- **Crystal Matrix:** The internal structure of crystals that enables magical amplification
- **Resonance Frequency:** The specific vibrational rate at which crystals operate
- **Theoretical Understanding:** Knowledge-based progression system replacing traditional skills

### References and Inspiration
- Brandon Sanderson's Cosmere magic systems (particularly Allomancy and Surgebinding)
- Classic text adventures: Zork, Adventure, Trinity
- Modern interactive fiction: 80 Days, Sunless Sea, Disco Elysium
- Scientific principles: Electromagnetic theory, resonance physics, conservation of energy