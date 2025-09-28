# Sympathetic Resonance: Balance Framework
*Implementation-Ready Numerical Values and Formulas*

## Executive Summary

This document provides precise numerical values, formulas, and balance curves for all game systems in Sympathetic Resonance. All values are designed to create meaningful choices, progressive challenge, and sustainable engagement patterns.

**Target Session Length:** 45-90 minutes
**Core Loop Duration:** 8-12 minutes per magical investigation cycle
**Progression Velocity:** 15-20 hours for major theory mastery

---

## 1. Magic System Balance

### 1.1 Mental Energy & Fatigue System

**Base Mental Energy Pool:**
```
Mental Energy = Mental Acuity × 1.5 (rounded up)
- Novice (Acuity 0-25): 0-38 energy
- Apprentice (Acuity 26-50): 39-75 energy
- Adept (Acuity 51-75): 77-113 energy
- Master (Acuity 76-100): 114-150 energy
```

**Fatigue Accumulation Rates (per magical action):**
```
Base Fatigue Cost = Spell Complexity + Crystal Mismatch Penalty - Efficiency Bonuses

Spell Complexity Tiers:
- Cantrip (detect magic, simple sensing): 8-12 fatigue
- Lesser (object location, minor healing): 15-22 fatigue
- Moderate (structural analysis, communication): 25-35 fatigue
- Greater (major healing, long-range effects): 40-55 fatigue
- Master (reality alteration, complex bindings): 60-80 fatigue

Crystal Mismatch Penalty:
- Perfect Match (resonance ±0): +0 fatigue
- Close Match (resonance ±1): +3 fatigue
- Poor Match (resonance ±2-3): +8 fatigue
- Terrible Match (resonance ±4+): +15 fatigue

Efficiency Bonuses:
- High Resonance Sensitivity (75+): -3 fatigue
- Optimal crystal purity (90%+): -2 fatigue
- Familiar spell (10+ successful casts): -2 fatigue
- Prepared components/setup: -3 fatigue
```

**Recovery Rates:**
```
Rest (passive, per hour): -8 fatigue
Active Rest (meditation): -15 fatigue per hour (max 2 hours/day)
Sleep (8 hours): -45 fatigue + mental energy restored to full
Magical Stimulants: -20 fatigue instantly (addiction risk after 3 uses/week)
```

**Effective Energy Calculation:**
```
Effective Energy = Current Mental Energy - (Current Fatigue × 0.6)

Performance Modifiers by Effective Energy:
- 90-100%: +15% success rate, -10% fatigue costs
- 70-89%: +5% success rate, normal costs
- 50-69%: Normal performance
- 30-49%: -10% success rate, +20% fatigue costs
- 10-29%: -25% success rate, +40% fatigue costs
- 0-9%: -50% success rate, +100% fatigue costs, risk of backlash
```

### 1.2 Crystal System Balance

**Crystal Degradation Formula:**
```
Degradation per Use = Base Rate × Complexity Multiplier × Purity Protection

Base Rates by Crystal Type:
- Quartz (Resonance 1-3): 0.8% per use
- Amethyst (Resonance 4-6): 0.6% per use
- Obsidian (Resonance 7-9): 0.7% per use
- Rare variants (Resonance 10): 0.4% per use

Complexity Multipliers:
- Cantrip: ×0.5
- Lesser: ×1.0
- Moderate: ×1.5
- Greater: ×2.2
- Master: ×3.0

Purity Protection:
- 0-30% purity: ×1.8 degradation
- 31-60% purity: ×1.4 degradation
- 61-85% purity: ×1.0 degradation
- 86-95% purity: ×0.7 degradation
- 96-100% purity: ×0.4 degradation

Critical Failure Threshold:
If structural integrity drops below 20%, each use has:
- 5% chance of complete crystal destruction
- 15% chance of spell failure with full fatigue cost
- 25% chance of unpredictable magical effects
```

**Crystal Market Values:**
```
Base Price Formula:
Price = Base Cost × Size Multiplier × Purity Multiplier × Rarity Multiplier × Market Fluctuation

Base Costs by Type:
- Common Quartz: 15 silver per gram
- Amethyst: 35 silver per gram
- Obsidian: 28 silver per gram
- Rare Resonance Crystals: 120 silver per gram

Size Categories & Multipliers:
- Tiny (1-5g): ×1.0, max output: 25 energy
- Small (6-15g): ×1.2, max output: 60 energy
- Medium (16-40g): ×1.5, max output: 120 energy
- Large (41-100g): ×2.0, max output: 250 energy

Purity Multipliers:
- 0-40%: ×0.3
- 41-70%: ×0.8
- 71-85%: ×1.0
- 86-95%: ×2.2
- 96-100%: ×4.5

Market Fluctuation: ±15% weekly variation based on:
- Recent magical disasters (increased demand)
- New crystal discoveries (decreased prices)
- Faction conflicts (supply disruptions)
- Seasonal mining conditions
```

### 1.3 Success Probability Calculations

**Base Success Formula:**
```
Success Rate = Base Chance + Attribute Bonuses + Knowledge Bonuses + Equipment Bonuses - Circumstance Penalties

Base Chances by Spell Complexity:
- Cantrip: 75%
- Lesser: 60%
- Moderate: 45%
- Greater: 30%
- Master: 15%

Attribute Bonuses:
- Resonance Sensitivity: +(Sensitivity ÷ 4)%
- Mental Acuity: +(Acuity ÷ 8)% for complex spells only

Knowledge Bonuses:
- Relevant Theory Mastery: +5% per mastery level (max +25%)
- Cross-Discipline Knowledge: +2% per related theory
- Experimental Success History: +1% per 5 successful similar attempts (max +10%)

Equipment Bonuses:
- Perfect Crystal Match: +15%
- High Purity Crystal (90%+): +8%
- Prepared Materials: +5%
- Optimal Environment: +5%

Circumstance Penalties:
- Combat Stress: -20%
- Time Pressure: -15%
- Hostile Environment: -10%
- Injured/Exhausted: -5% per condition level
- Crystal Damage (below 50% integrity): -15%
```

---

## 2. Character Progression Balance

### 2.1 Mental Acuity Advancement

**Progression Rates (experience points required):**
```
XP to Next Level = (Current Level × 25) + 50

XP Gain Sources:
- Successful Spell Casting: 2-8 XP (based on complexity)
- Failed Attempts (learning): 1-3 XP
- Study Sessions: 5-15 XP (based on material quality)
- Puzzle Solving: 8-20 XP
- Discovery/Breakthrough: 25-50 XP
- Teaching Others: 3-10 XP

Time Investment:
- Level 0→25 (Novice): ~8-12 hours gameplay
- Level 25→50 (Apprentice): ~15-20 hours
- Level 50→75 (Adept): ~25-35 hours
- Level 75→100 (Master): ~40-60 hours

Diminishing Returns:
- Repeated identical spells: XP × 0.5 after 20 uses
- Study same material: XP × 0.3 after mastery
- Comfort zone penalty: -25% XP if no challenging actions in 2 hours
```

### 2.2 Resonance Sensitivity Curves

**Advancement Formula:**
```
Sensitivity XP = Base Activity Reward × Novelty Multiplier × Risk Multiplier

Base Activity Rewards:
- Crystal attunement practice: 3 XP
- New crystal type experimentation: 8 XP
- Complex resonance matching: 5 XP
- Environmental sensing: 4 XP
- Failed attempts (learning): 2 XP

Novelty Multiplier:
- First time with crystal type: ×3.0
- Rare environmental conditions: ×2.0
- Cross-resonance experiments: ×1.5
- Routine practice: ×1.0
- Mastered techniques: ×0.4

Risk Multiplier:
- Dangerous experiments: ×1.8
- Public demonstrations: ×1.3
- Safe practice: ×1.0
- Supervised learning: ×0.8

Progression Velocity:
- 0→25: 6-10 hours of magical practice
- 25→50: 12-18 hours
- 50→75: 20-30 hours
- 75→100: 35-50 hours
```

### 2.3 Theory Learning Framework

**Knowledge Tree Structure:**
```
Theory Prerequisites & Learning Times:

Tier 1 (Foundation):
- Harmonic Fundamentals: 0 prerequisites, 3-5 study sessions
- Basic Crystal Theory: Harmonics required, 4-6 sessions
- Energy Conservation: Harmonics required, 3-4 sessions

Tier 2 (Specialization):
- Crystal Lattice Theory: Basic Crystal + Energy Conservation, 6-8 sessions
- Neural Amplification: Energy Conservation + practice prerequisite, 5-7 sessions
- Sympathetic Binding: Harmonics + Acuity 40+, 7-10 sessions

Tier 3 (Advanced):
- Resonance Disruption: All Tier 2 + combat experience, 10-15 sessions
- Quantum Harmonics: Crystal Lattice + Acuity 70+, 12-18 sessions
- Master's Synthesis: All theories + discovery prerequisite, 15-25 sessions

Study Session Parameters:
- Duration: 30-60 minutes real-time (compressed in game)
- Mental Energy Cost: 15-25 per session
- Prerequisites: Access to materials, teacher, or discovered knowledge
- Failure Rate: 10-25% based on theory complexity and character stats
```

### 2.4 Faction Reputation Dynamics

**Reputation Gain/Loss Rates:**
```
Action Impact Formula:
Reputation Change = Base Value × Faction Alignment × Publicity Multiplier × Standing Modifier

Standard Action Values:
- Minor aligned action: +2 to +5 reputation
- Major aligned action: +8 to +15 reputation
- Faction mission completion: +15 to +30 reputation
- Minor opposed action: -3 to -8 reputation
- Major betrayal: -20 to -50 reputation

Faction Alignment Multipliers:
- Strongly aligned action: ×1.5
- Neutral action: ×1.0
- Mildly opposed: ×-0.8
- Strongly opposed: ×-2.0

Publicity Multiplier:
- Secret action: ×0.5
- Private action: ×1.0
- Public action: ×1.5
- Highly publicized: ×2.5

Standing Modifier:
- Enemy to Neutral: ×2.0 (harder to gain trust)
- Neutral to Ally: ×1.0
- Ally to Inner Circle: ×0.5 (diminishing returns)

Cross-Faction Effects:
Actions benefiting one faction affect rivals:
- Magisters vs Underground: 100% inverse correlation
- Order vs Industrial: 80% inverse correlation
- Academic vs Commercial: 40% inverse correlation
- Neutral Scholars: 20% correlation with all sides
```

---

## 3. Content Pacing Balance

### 3.1 Session Structure & Duration

**Target Session Breakdown (60-minute session):**
```
Opening/Status Review: 3-5 minutes
- Check mental energy, crystal status, faction standings
- Review recent discoveries and ongoing investigations

Core Investigation Loop (repeat 3-4 times): 12-15 minutes each
- Environmental observation: 2-3 minutes
- Magical experimentation: 4-6 minutes
- Social/political interaction: 3-4 minutes
- Resource management decisions: 2-3 minutes

Story Progression/Resolution: 8-12 minutes
- Major discovery or plot advancement
- Faction relationship development
- Knowledge acquisition or theory breakthrough

Session Wrap-up: 3-5 minutes
- Save progress, update character sheet
- Plan next session activities
```

**Magical Action Frequency:**
```
Actions per Hour Targets:
- Cantrip-level magic: 8-12 attempts
- Lesser spells: 4-6 attempts
- Moderate spells: 2-3 attempts
- Greater spells: 1-2 attempts
- Master-level magic: 0-1 attempts

Fatigue Limiting Factors:
- Novice characters: Fatigue limit reached after 45-60 minutes
- Experienced characters: Can sustain 90+ minutes with rest management
- Recovery breaks: Required every 20-30 minutes of intensive magic use

Crystal Replacement Frequency:
- High-quality crystals: Replace every 15-25 sessions
- Medium-quality crystals: Replace every 8-12 sessions
- Low-quality crystals: Replace every 4-6 sessions
- Emergency/backup crystals: Single-use or very limited applications
```

### 3.2 Progression Milestones & Timing

**Major Advancement Timeline:**
```
Character Development Stages:

Novice Phase (0-15 hours):
- Learn basic magic principles and safety
- Establish faction relationships
- Master 2-3 cantrip-level applications
- Acquire first quality crystal set
- Complete 1-2 minor faction missions

Apprentice Phase (15-35 hours):
- Develop specialization focus (healing, analysis, etc.)
- Master Tier 2 magical theories
- Build strong reputation with 1-2 factions
- Handle moderate magical challenges independently
- Participate in significant faction activities

Adept Phase (35-65 hours):
- Access to advanced magical theories
- Lead faction missions or research projects
- Develop innovative magical applications
- Navigate complex political situations
- Mentor other characters

Master Phase (65+ hours):
- Pioneer new magical theories or techniques
- Influence major faction decisions
- Handle city-wide or region-wide threats
- Access to legendary crystals and knowledge
- Shape the political and magical landscape

Story Arc Pacing:
- Personal mystery resolution: 8-15 hours
- Faction storyline completion: 20-30 hours
- Major world event participation: 35-50 hours
- Character legacy achievement: 60+ hours
```

### 3.3 Economic Progression Balance

**Income vs. Expenses Framework:**
```
Character Income Sources:

Novice Income (per session):
- Odd jobs for factions: 5-15 silver
- Simple magic services: 8-25 silver
- Information gathering: 3-12 silver
- Study/research assistance: 10-20 silver

Experienced Income (per session):
- Professional consultations: 25-75 silver
- Faction missions: 50-150 silver
- Magical investigations: 40-100 silver
- Teaching/mentoring: 20-60 silver

Expense Categories:

Essential Costs:
- Crystal replacement: 15-80 silver per session
- Basic living expenses: 5-10 silver per session
- Study materials: 8-25 silver per session

Optional Investments:
- High-quality crystals: 200-800 silver
- Advanced training: 100-500 silver
- Faction dues/bribes: 20-100 silver
- Equipment upgrades: 50-300 silver

Economic Progression Goals:
- Break-even sustainability: Achieved around 10-15 hours play
- Comfortable resource cushion: 25-35 hours
- High-end crystal access: 45-60 hours
- Economic independence: 80+ hours

Market Disruption Events:
- Crystal mine disasters: 200-400% price increases for 3-6 sessions
- New discoveries: 30-50% price decreases for specific types
- Faction conflicts: Supply shortages, 150-300% increases
- Political changes: Tax/tariff adjustments, ±20-40% shifts
```

---

## 4. Difficulty Progression Framework

### 4.1 Challenge Scaling by Character Level

**Novice Challenges (Mental Acuity 0-25):**
```
Magical Challenges:
- Success rates: 60-80% for attempted spells
- Consequences of failure: Minor fatigue, no lasting harm
- Available spells: Cantrips and basic Lesser spells only
- Crystal requirements: Low-quality crystals sufficient

Social Challenges:
- Faction politics: Simple requests, clear motivations
- NPC interactions: Forgiving of social mistakes
- Information access: Surface-level details readily available
- Consequences: Reputation changes are small and recoverable

Resource Challenges:
- Crystal costs: Affordable with basic income
- Fatigue management: Forgiving recovery rates
- Time pressure: Generous deadlines for most tasks
```

**Apprentice Challenges (Mental Acuity 26-50):**
```
Magical Challenges:
- Success rates: 45-70% for attempted spells
- Consequences of failure: Moderate fatigue, potential crystal damage
- Available spells: Full Lesser spell access, basic Moderate spells
- Crystal requirements: Medium-quality crystals recommended

Social Challenges:
- Faction politics: Competing interests, moral ambiguity
- NPC interactions: Consequences for poor choices
- Information access: Requires investigation and relationship building
- Consequences: Reputation changes affect available opportunities

Resource Challenges:
- Crystal costs: Requires planning and budgeting
- Fatigue management: Strategic rest timing becomes important
- Time pressure: Multiple concurrent deadlines
```

**Adept Challenges (Mental Acuity 51-75):**
```
Magical Challenges:
- Success rates: 35-60% for attempted spells
- Consequences of failure: High fatigue, crystal destruction risk, backlash
- Available spells: Full Moderate access, limited Greater spells
- Crystal requirements: High-quality crystals often necessary

Social Challenges:
- Faction politics: Complex webs of alliance and rivalry
- NPC interactions: High stakes, permanent consequences possible
- Information access: Dangerous secrets, protected knowledge
- Consequences: Actions can trigger faction-wide responses

Resource Challenges:
- Crystal costs: Significant investment required
- Fatigue management: Critical for survival in dangerous situations
- Time pressure: Life-or-death deadlines, competing priorities
```

**Master Challenges (Mental Acuity 76-100):**
```
Magical Challenges:
- Success rates: 25-50% for attempted spells
- Consequences of failure: Severe fatigue, injury, environmental damage
- Available spells: Greater spells, experimental Master techniques
- Crystal requirements: Rare, expensive crystals mandatory

Social Challenges:
- Faction politics: City-shaping decisions, faction leadership
- NPC interactions: Historical significance, lasting legacy impacts
- Information access: World-altering secrets, forbidden knowledge
- Consequences: Actions reshape the political landscape

Resource Challenges:
- Crystal costs: Substantial wealth required
- Fatigue management: Life-threatening if mismanaged
- Time pressure: Regional or world-threatening crisis timelines
```

### 4.2 Dynamic Difficulty Adjustment

**Performance-Based Scaling:**
```
Success Rate Monitoring:
- If player success rate > 80% over 10 attempts: Increase challenge by 15%
- If player success rate < 40% over 10 attempts: Decrease challenge by 10%
- Target success rate range: 50-70% for engaging difficulty

Challenge Adjustment Methods:
- Environmental penalties: Hostile conditions, time pressure
- Resource scarcity: Crystal availability, increased costs
- Social complexity: Additional faction complications
- Magical interference: Competing magic users, unstable phenomena

Player Agency Preservation:
- Always provide multiple approach options
- Clearly telegraph increased difficulty
- Offer preparatory opportunities (research, ally recruitment)
- Maintain escape routes for overwhelming situations
```

---

## 5. Validation & Testing Methodology

### 5.1 Balance Validation Framework

**Mathematical Model Validation:**
```
Simulation Parameters:
- Run 1000+ iterations of each major character progression path
- Test edge cases: minimum/maximum stat builds, unusual play patterns
- Validate economic models: inflation resistance, progression sustainability
- Verify difficulty curves: challenge appropriateness at each level

Key Performance Indicators:
- Average session length: 45-90 minutes (target: 60-75 minutes)
- Player engagement: <5% idle time during active gameplay
- Progression satisfaction: Meaningful advancement every 2-3 sessions
- Economic balance: Positive resource growth with meaningful choices
- Failure recovery: <30 minutes to recover from major setbacks

Automated Testing:
- Character progression simulators
- Economic model stress testing
- Difficulty curve validation
- Resource depletion/recovery cycles
```

**Player Testing Protocols:**
```
Alpha Testing Phase:
- 6-8 dedicated testers, 20+ hours each
- Focus on system comprehension and basic balance
- Track completion rates, confusion points, abandoned sessions
- Measure time-to-competency for core mechanics

Beta Testing Phase:
- 20-30 diverse testers, varied gaming backgrounds
- Focus on long-term progression and replayability
- Monitor economic patterns, strategy emergence
- Identify dominant strategies or broken combinations

Live Monitoring:
- Session length distribution analysis
- Progress point achievement timing
- Resource usage pattern tracking
- Player retention at key difficulty spikes
```

### 5.2 Balance Adjustment Protocols

**Response Triggers:**
```
Immediate Adjustments Required:
- Average session completion rate < 60%
- Economic progression stalled for >50% of players
- Single strategy dominates >80% of optimal play
- Player retention drops >30% at specific progression points

Gradual Adjustment Indicators:
- Session length consistently outside 45-90 minute range
- Resource abundance/scarcity affecting >25% of players
- Challenge difficulty rated "too easy" or "too hard" by >40% of players
- Progression feels "grindy" to >30% of players

Adjustment Implementation:
- Mathematical model updates with version control
- A/B testing for major changes
- Gradual rollout with player feedback monitoring
- Documentation of all changes with rationale
```

**Version Control for Balance:**
```
Balance Versioning System:
- Major versions: Fundamental system changes
- Minor versions: Numerical adjustments to existing systems
- Patch versions: Bug fixes and edge case handling

Change Documentation Requirements:
- Problem identification with supporting data
- Proposed solution with mathematical justification
- Expected impact on player experience
- Success metrics for validation
- Rollback plan if changes prove problematic

Community Communication:
- Clear explanation of changes and reasoning
- Timeline for evaluation period
- Feedback collection mechanisms
- Regular updates on balance health
```

---

## 6. Implementation Checklist

### 6.1 Core Systems Implementation Priority

**Phase 1: Foundation (Weeks 1-2)**
- [ ] Mental Energy and Fatigue calculation engine
- [ ] Basic crystal degradation system
- [ ] Simple success rate calculations
- [ ] Character attribute progression tracking

**Phase 2: Complexity (Weeks 3-4)**
- [ ] Full magical success formula implementation
- [ ] Crystal market price fluctuation system
- [ ] Faction reputation cross-effects
- [ ] Theory learning prerequisite system

**Phase 3: Polish (Weeks 5-6)**
- [ ] Dynamic difficulty adjustment algorithms
- [ ] Economic balance monitoring tools
- [ ] Player performance tracking systems
- [ ] Automated balance validation testing

### 6.2 Data Collection Requirements

**Essential Metrics to Track:**
```
Player Performance:
- Success rates by spell type and character level
- Session length distribution
- Resource usage patterns
- Progression milestone timing

Economic Health:
- Crystal price stability
- Income/expense ratios by character level
- Resource hoarding vs. consumption patterns
- Market disruption recovery times

Social Dynamics:
- Faction reputation distribution
- Political choice consequences
- NPC relationship development patterns
- Story branch selection frequency
```

---

## Conclusion

This balance framework provides concrete, implementable values designed to create the target experience for Sympathetic Resonance. The mathematical models ensure:

1. **Meaningful Choices**: Resource costs create genuine decision points without punishing experimentation
2. **Progressive Challenge**: Difficulty scales appropriately with character advancement
3. **Multiple Strategies**: Various approaches remain viable throughout progression
4. **Failure Recovery**: Setbacks provide learning opportunities without breaking progression
5. **Pacing Control**: Players experience steady advancement with periodic major breakthroughs

All values are designed to be easily adjustable based on testing feedback while maintaining system coherence. The validation methodology ensures the balance remains healthy throughout the game's lifecycle.

**Next Steps:**
1. Implement core mathematical systems with these values
2. Build automated testing framework for balance validation
3. Begin alpha testing with focus on progression curves
4. Iterate based on player data and feedback
5. Establish ongoing balance monitoring protocols

This framework should provide the precise numerical foundation needed for successful implementation of Sympathetic Resonance's interconnected game systems.