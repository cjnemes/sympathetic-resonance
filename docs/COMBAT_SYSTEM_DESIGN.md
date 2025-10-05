# Combat System Design Document
**Sympathetic Resonance - Magical Combat System**

## Overview

The combat system integrates seamlessly with the existing magic system to provide turn-based magical combat encounters. Combat emphasizes strategic spell selection, resource management (energy, fatigue, crystals), and theory knowledge.

## Core Principles

1. **Magic System Integration** - Reuse all existing magic calculations and costs
2. **Resource Management** - Energy, fatigue, and crystal degradation matter
3. **Strategic Depth** - Theory knowledge and spell selection create tactical choices
4. **Balanced Risk/Reward** - Combat provides XP and loot but consumes resources
5. **Narrative Integration** - Combat supports quest objectives and faction conflicts

## Architecture

### Data Structures

```rust
// Enemy Definition
pub struct Enemy {
    pub id: String,
    pub name: String,
    pub description: String,
    pub health: i32,
    pub max_health: i32,
    pub magical_resistance: HashMap<String, f32>,  // spell_type -> resistance (0.0-1.0)
    pub difficulty_tier: DifficultyTier,
    pub loot_table: Vec<LootDrop>,
    pub experience_reward: i32,
    pub faction_affiliation: Option<FactionId>,
}

// Combat State
pub struct CombatEncounter {
    pub enemy: Enemy,
    pub turn_count: i32,
    pub player_actions: Vec<CombatAction>,
    pub enemy_actions: Vec<CombatAction>,
    pub status_effects: Vec<StatusEffect>,
}

// Combat Actions
pub enum CombatAction {
    Attack { spell_type: String, target: String },
    Defend { defense_type: DefenseType },
    UseItem { item_id: String },
    Flee,
}

pub enum DefenseType {
    Shield,      // Reduces damage 50%, costs energy
    Evade,       // 70% chance to avoid, costs fatigue
    CounterMagic, // Reflects 30% damage, requires theory knowledge
}

// Combat Results
pub enum CombatOutcome {
    Victory { experience: i32, loot: Vec<String>, faction_change: Option<(FactionId, i32)> },
    Defeat { consequences: DefeatConsequences },
    Fled { energy_cost: i32, faction_penalty: Option<(FactionId, i32)> },
}
```

### Turn Resolution

```
1. Check combat active status
2. Player selects action
3. Enemy AI selects action
4. Resolve initiative (player Mental Acuity vs enemy Speed)
5. Execute faster action first
6. Apply damage/effects
7. Check win/lose conditions
8. Advance to next turn or end combat
```

## Damage Calculations

### Offensive Magic Damage

```rust
// Base damage from magic system power level
let base_damage = magic_result.power_level * 10;

// Theory bonus (up to +100%)
let theory_bonus = player.theory_bonus_for_spell(spell_type);
let damage_multiplier = 1.0 + theory_bonus;

// Enemy resistance
let resistance = enemy.magical_resistance.get(spell_type).unwrap_or(&0.0);
let resistance_multiplier = 1.0 - resistance;

// Final damage
let final_damage = (base_damage * damage_multiplier * resistance_multiplier) as i32;
```

### Critical Hits

- **Frequency Resonance Match**: If crystal frequency matches enemy vulnerability: +50% damage
- **Theory Mastery**: If theory understanding >0.8: 20% crit chance (+100% damage)
- **Environmental Bonus**: Favorable conditions: +25% damage

## Enemy AI

### Decision Matrix

```rust
fn select_action(enemy: &Enemy, player: &Player, turn: i32) -> EnemyAction {
    // Early game: Aggressive attacks
    if turn <= 3 {
        return EnemyAction::Attack(select_best_attack());
    }

    // Low health: Defensive or flee
    if enemy.health < enemy.max_health / 3 {
        if random() < 0.5 {
            return EnemyAction::Defend;
        } else {
            return EnemyAction::Flee;
        }
    }

    // Player low energy: Aggressive
    if player.mental_energy < 30 {
        return EnemyAction::Attack(select_powerful_attack());
    }

    // Default: Balanced strategy
    weighted_random_choice(&[
        (EnemyAction::Attack(normal_attack()), 0.6),
        (EnemyAction::Defend, 0.3),
        (EnemyAction::Special, 0.1),
    ])
}
```

## Enemy Types

### Tier 1: Beginner (Tutorial)
- **Corrupted Crystal Shard** (50 HP)
  - Low magical resistance across all types
  - Teaches basic combat mechanics
  - Rewards: Damaged crystal, 50 XP

### Tier 2: Intermediate (Mid-game)
- **Rogue Practitioner** (100 HP)
  - Moderate resistance to basic spells
  - Uses healing magic
  - Faction: Underground Network
  - Rewards: Research notes, crystal fragment, 100 XP, -10 Underground reputation

### Tier 3: Advanced (Late-game)
- **Resonance Anomaly** (150 HP)
  - High resistance to unmatched frequencies
  - Vulnerable to theoretical synthesis
  - Environmental hazard
  - Rewards: Rare crystal, 200 XP, theory insight

### Boss: Endgame
- **Council Enforcer** (250 HP)
  - Adaptive resistance (learns from player spells)
  - Multi-spell attacks
  - Faction: Magisters' Council
  - Rewards: Unique artifact, 500 XP, major faction consequences

## Combat Commands

```bash
# Initiate combat
attack <enemy> with <spell>
engage <enemy>

# During combat
cast <spell> [on <target>]
defend [shield|evade|counter]
use <item>
flee

# Information
examine enemy
status
```

## Resource Costs (Balanced for Combat)

- **Offensive Spell**: Normal magic system costs
- **Shield Defense**: 15 energy, 5 fatigue
- **Evade**: 10 energy, 15 fatigue
- **Counter-Magic**: 25 energy, 10 fatigue (requires mental_resonance ≥0.5)
- **Flee**: 20 energy, 20 fatigue, -5 faction reputation (if enemy has affiliation)

## Rewards & Progression

### Experience Scaling
- Base: enemy_tier * 50 XP
- First-time bonus: +50%
- Flawless victory (no damage taken): +25%
- Combat efficiency (turns < 5): +10%

### Loot System
```rust
pub struct LootDrop {
    pub item_id: String,
    pub drop_chance: f32,  // 0.0-1.0
    pub quantity_range: (i32, i32),
}

// Example:
LootDrop {
    item_id: "damaged_amethyst",
    drop_chance: 0.6,
    quantity_range: (1, 3),
}
```

### Faction Consequences
- Attacking faction-affiliated enemies: Reputation penalty
- Defeating faction enemies grants opposite faction bonus
- Sparing enemies (flee before killing): Reduced penalties

## Integration with Quest System

### Quest Objectives
```rust
QuestObjective::DefeatEnemy {
    enemy_id: String,
    quantity: i32,
}

QuestObjective::WinCombatWithCondition {
    enemy_id: String,
    condition: CombatCondition,  // e.g., "no damage taken", "using only light magic"
}
```

### Example Combat Quest

**"Containment Protocol"** (Magisters' Council)
- Objective: Defeat 3 Corrupted Crystal Shards
- Condition: Use detection magic to identify weak points
- Reward: Crystal analysis kit, +15 Magisters' Council reputation
- Theory requirement: harmonic_fundamentals ≥0.3

## Balance Considerations

### Difficulty Tuning
- Enemy HP scaling: player_level * tier_multiplier * 25
- Damage scaling: 10-30 per hit (tier 1), 30-60 per hit (tier 3)
- Player health: Not tracked (defeat = flee with consequences, not death)

### Defeat Consequences
- Energy drain: Set to 10%
- Fatigue increase: +40
- Item loss: 10% chance to drop equipped crystal
- Faction penalty: -10 if enemy has affiliation
- Quest failure: Combat objectives marked as failed

### Combat Frequency
- Optional encounters: Can flee most combats
- Required encounters: 3-5 per major questline
- Random encounters: Low probability (10% when traveling)

## Implementation Phases

### Phase 1: Core Combat (Week 1)
- [x] Enemy struct and definitions
- [x] CombatEncounter state management
- [x] Turn-based loop implementation
- [x] Basic damage calculations
- [x] Win/lose condition checking

### Phase 2: Magic Integration (Week 1)
- [x] Integrate MagicSystem for damage
- [x] Apply energy/fatigue/crystal costs
- [x] Theory bonuses in combat
- [x] Enemy resistance system

### Phase 3: AI & Actions (Week 1-2)
- [x] Enemy AI decision making
- [x] Defense actions (shield, evade, counter)
- [x] Item usage in combat
- [x] Flee mechanic

### Phase 4: Rewards & Integration (Week 2)
- [x] Experience calculation and awarding
- [x] Loot drop system
- [x] Faction reputation changes
- [x] Quest objective integration

### Phase 5: Content & Testing (Week 2-3)
- [x] 5 enemy definitions (tiers 1-3)
- [x] 3 combat-based quests
- [x] 20+ combat tests
- [x] Integration tests with magic system

## Testing Strategy

### Unit Tests
- Enemy creation and configuration
- Damage calculation accuracy
- Defense action mechanics
- Loot drop probability
- AI decision logic

### Integration Tests
- Full combat encounter (player victory)
- Full combat encounter (player defeat)
- Flee from combat
- Combat with quest integration
- Combat with faction consequences

### Performance Tests
- Combat loop <50ms per turn
- Enemy AI <10ms decision time

## Success Metrics

- **Engagement**: 80% of players engage in at least 3 combats
- **Balance**: Average combat duration 3-7 turns
- **Difficulty**: 60% victory rate on tier-appropriate enemies
- **Resource Management**: Players maintain >30% energy after combat
- **Variety**: Players use 3+ different spell types in combat

---

*Design Version: 1.0*
*Implementation Start: October 5, 2025*
*Target Completion: October 19, 2025 (2 weeks)*
