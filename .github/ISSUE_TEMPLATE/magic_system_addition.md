---
name: Magic System Addition
about: New magical applications, spells, or theoretical frameworks
title: "[MAGIC] New magical application: [brief description]"
labels: ["content:magic", "type:feature", "needs-balance-review"]
assignees: ''

---

## Magic System Addition

### **Magical Application Overview**

**Application Name:**
<!-- The name of the new magical ability/spell -->

**Application Type:**
- [ ] Cantrip (Simple, low-cost effects)
- [ ] Lesser Magic (Basic applications with moderate costs)
- [ ] Moderate Magic (Complex effects requiring skill)
- [ ] Greater Magic (Advanced applications with high costs)
- [ ] Master Magic (Experimental or legendary techniques)

**Primary Magic Category:**
- [ ] Light Generation & Manipulation
- [ ] Sympathetic Healing & Bio-resonance
- [ ] Detection & Magical Sensing
- [ ] Physical Manipulation & Force
- [ ] Communication & Information Transfer
- [ ] New Category (specify): ________________

### **Scientific Foundation**

**Physical Principle:**
<!-- Describe the real-world scientific principle this magic is based on -->

**Crystal Resonance Requirements:**
**Optimal Crystal Type:**
- [ ] Quartz (Frequency 4) - Basic applications
- [ ] Amethyst (Frequency 7) - Healing & bio-resonance
- [ ] Obsidian (Frequency 2) - Force & manipulation
- [ ] Garnet (Frequency 6) - Detection & analysis
- [ ] Rare Crystal (Frequency 10) - Advanced applications

**Alternative Crystal Compatibility:**
<!-- List other crystals that can be used with penalties -->

**Energy Requirements:**
**Base Mental Energy Cost:** _____ (Reference: Light=8, Healing=15, Detection=12, Manipulation=20)
**Base Fatigue Cost:** _____ (Reference: Light=5, Healing=12, Detection=8, Manipulation=15)
**Time to Cast:** _____ minutes

### **Sympathetic Connection Requirements**

**Connection Strength Needed:**
- [ ] None (Direct effects on crystal/caster)
- [ ] Weak (Recently observed objects)
- [ ] Moderate (Previously touched items)
- [ ] Strong (Personal belongings, close relationships)
- [ ] Very Strong (Family members, long-owned items)

**Range Limitations:**
- [ ] Touch range only
- [ ] Line of sight (same room)
- [ ] Short range (same building)
- [ ] Medium range (same city district)
- [ ] Long range (sympathetic connection strength dependent)

### **Game Balance Considerations**

**Success Rate Calculation:**
<!-- Describe how this integrates with the existing success rate formula -->
**Base Success Rate:** ____% (Reference: Cantrip=75%, Lesser=60%, Moderate=45%, Greater=30%, Master=15%)

**Modifiers to Standard Formula:**
- Frequency matching bonus/penalty: [Explain any deviations]
- Special skill requirements: [Any prerequisite theories needed]
- Environmental factors: [Specific conditions that help/hinder]

**Crystal Degradation:**
**Degradation per Use:** ____% (Reference: Light=0.5%, Healing=1.2%, Detection=0.8%, Manipulation=2.0%)

**Power Level Output:** ____ (0.0-1.0+ scale, Reference: Light=0.6, Healing=0.8, Detection=0.7, Manipulation=1.0)

### **Theory Prerequisites**

**Required Theoretical Knowledge:**
<!-- List theories the player must know to attempt this magic -->
- [ ] Harmonic Fundamentals (Basic resonance principles)
- [ ] Crystal Lattice Theory (Understanding crystal structures)
- [ ] Neural Amplification (Efficient energy usage)
- [ ] Sympathetic Binding (Creating connections)
- [ ] Resonance Disruption (Defensive techniques)
- [ ] New Theory Required: ________________

**Learning Method:**
- [ ] Study (Available in books/lectures)
- [ ] Experimentation (Can be discovered through trial)
- [ ] Mentorship (Requires teacher from specific faction)
- [ ] Discovery (Found through exploration/artifacts)

### **Narrative Integration**

**Faction Associations:**
**Favored by:**
- [ ] Magisters' Council (Academic regulation)
- [ ] Order of Natural Harmony (Traditional integration)
- [ ] Industrial Consortium (Commercial applications)
- [ ] Underground Network (Unregulated practice)
- [ ] Neutral Scholars (Pure research)

**Opposed by:**
<!-- Which factions would view this magic negatively and why -->

**Historical Context:**
<!-- How does this magic fit into the world's 150-year magical history? -->

### **Practical Applications**

**Common Uses:**
<!-- List everyday applications of this magic -->
1.
2.
3.

**Combat Applications:**
<!-- How this magic works in conflict situations -->

**Economic Applications:**
<!-- How this magic affects commerce and industry -->

**Social Implications:**
<!-- How this magic affects interpersonal relationships and society -->

### **Failure Conditions & Consequences**

**Minor Failure (Common):**
<!-- What happens with typical magical failures -->

**Major Failure (Rare):**
<!-- Consequences of significant magical mishaps -->

**Catastrophic Failure (Very Rare):**
<!-- Worst-case scenario outcomes -->

**Backlash Effects:**
<!-- Specific consequences for this type of magic -->

### **Implementation Details**

**Code Module Location:**
<!-- Where in the codebase this should be implemented -->
- [ ] New calculator in `src/systems/magic/calculation_engine.rs`
- [ ] Extension to existing calculator
- [ ] New module in `src/systems/magic/`

**Database Schema Changes:**
<!-- Any new tables or columns needed -->

**User Interface Impact:**
<!-- How this appears to players -->

**Help System Integration:**
<!-- What documentation/tutorial content is needed -->

### **Testing Requirements**

**Mathematical Validation:**
- [ ] Success rate calculations produce expected curves
- [ ] Energy/fatigue costs balanced against other magic types
- [ ] Crystal degradation rates sustainable for gameplay

**Integration Testing:**
- [ ] Works correctly with faction reputation system
- [ ] Integrates properly with character progression
- [ ] Doesn't break existing magical applications

**Playtesting Focus:**
- [ ] Validates fun factor and strategic depth
- [ ] Confirms educational value about scientific principles
- [ ] Ensures balanced risk/reward compared to existing magic

### **Success Criteria**

**Technical Acceptance:**
- [ ] All unit tests pass for new calculations
- [ ] Integration tests verify system interactions
- [ ] Performance impact <5ms per magic attempt
- [ ] Save/load compatibility maintained

**Design Acceptance:**
- [ ] Balances well against existing magical applications
- [ ] Enhances rather than replaces existing strategies
- [ ] Provides meaningful choices for players
- [ ] Supports educational gameplay goals

**Content Acceptance:**
- [ ] Fits naturally into existing narrative
- [ ] Enhances faction political dynamics
- [ ] Provides interesting sympathetic connection challenges
- [ ] Includes appropriate scientific explanations

### **Additional Context**
<!-- Inspiration, references, or examples that help explain this magic -->

**Scientific References:**
<!-- Real-world scientific papers or phenomena this is based on -->

**Fantasy Inspiration:**
<!-- Other fictional magic systems that influenced this design -->

**Player Use Cases:**
<!-- Specific scenarios where players would want to use this magic -->