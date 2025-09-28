# Balance Implementation Reference Tables
*Quick lookup values for developers*

## Magic System Constants

### Mental Energy Calculations
```rust
// Core formulas for implementation
fn calculate_mental_energy(mental_acuity: u8) -> u8 {
    ((mental_acuity as f32 * 1.5).ceil() as u8).min(150)
}

fn calculate_effective_energy(current_energy: u8, fatigue: u8) -> u8 {
    current_energy.saturating_sub((fatigue as f32 * 0.6) as u8)
}

// Fatigue costs by spell complexity
const SPELL_FATIGUE_COSTS: [(u8, u8); 5] = [
    (8, 12),   // Cantrip
    (15, 22),  // Lesser
    (25, 35),  // Moderate
    (40, 55),  // Greater
    (60, 80),  // Master
];

// Crystal mismatch penalties
const CRYSTAL_MISMATCH_PENALTY: [u8; 5] = [0, 3, 8, 8, 15]; // resonance difference 0,1,2,3,4+
```

### Success Rate Calculations
```rust
// Base success rates by spell complexity
const BASE_SUCCESS_RATES: [u8; 5] = [75, 60, 45, 30, 15];

fn calculate_success_rate(
    base_rate: u8,
    resonance_sensitivity: u8,
    mental_acuity: u8,
    is_complex_spell: bool,
    knowledge_bonus: u8,
    equipment_bonus: u8,
    circumstance_penalty: u8
) -> u8 {
    let mut rate = base_rate as i16;
    rate += (resonance_sensitivity / 4) as i16;
    if is_complex_spell {
        rate += (mental_acuity / 8) as i16;
    }
    rate += knowledge_bonus as i16;
    rate += equipment_bonus as i16;
    rate -= circumstance_penalty as i16;
    (rate.max(5).min(95)) as u8  // 5-95% bounds
}
```

### Crystal Degradation
```rust
// Crystal degradation per use
const CRYSTAL_DEGRADATION_BASE: [f32; 4] = [0.8, 0.6, 0.7, 0.4]; // Quartz, Amethyst, Obsidian, Rare
const COMPLEXITY_MULTIPLIERS: [f32; 5] = [0.5, 1.0, 1.5, 2.2, 3.0];

fn calculate_degradation(
    crystal_type: CrystalType,
    spell_complexity: SpellComplexity,
    purity: u8
) -> f32 {
    let base_rate = CRYSTAL_DEGRADATION_BASE[crystal_type as usize];
    let complexity_mult = COMPLEXITY_MULTIPLIERS[spell_complexity as usize];
    let purity_protection = match purity {
        0..=30 => 1.8,
        31..=60 => 1.4,
        61..=85 => 1.0,
        86..=95 => 0.7,
        96..=100 => 0.4,
        _ => 1.0,
    };
    base_rate * complexity_mult * purity_protection
}
```

## Progression Constants

### Experience Point Calculations
```rust
fn xp_to_next_level(current_level: u8) -> u16 {
    (current_level as u16 * 25) + 50
}

// XP rewards by activity
const XP_REWARDS: [(ActivityType, (u8, u8)); 7] = [
    (SpellCasting, (2, 8)),     // based on complexity
    (FailedAttempt, (1, 3)),    // learning from failure
    (StudySession, (5, 15)),    // based on material quality
    (PuzzleSolving, (8, 20)),   // based on difficulty
    (Discovery, (25, 50)),      // major breakthrough
    (Teaching, (3, 10)),        // helping others
    (Experimentation, (8, 15)), // trying new approaches
];
```

### Faction Reputation
```rust
// Reputation change calculation
fn calculate_reputation_change(
    base_value: i8,
    faction_alignment: f32,  // -2.0 to 1.5
    publicity: f32,          // 0.5 to 2.5
    current_standing: i8     // -100 to 100
) -> i8 {
    let standing_modifier = match current_standing {
        -100..=-21 => 2.0,  // Enemy to neutral is harder
        -20..=20 => 1.0,    // Neutral range
        21..=80 => 1.0,     // Ally range
        81..=100 => 0.5,    // Inner circle has diminishing returns
        _ => 1.0,
    };

    ((base_value as f32 * faction_alignment * publicity * standing_modifier) as i8)
        .max(-50).min(50)  // Cap single action impact
}

// Standard reputation values
const REPUTATION_ACTIONS: [(ActionType, i8); 5] = [
    (MinorAligned, 3),      // +2 to +5 range
    (MajorAligned, 12),     // +8 to +15 range
    (FactionMission, 22),   // +15 to +30 range
    (MinorOpposed, -5),     // -3 to -8 range
    (MajorBetrayal, -35),   // -20 to -50 range
];
```

## Economic Constants

### Crystal Pricing
```rust
// Base prices in silver per gram
const CRYSTAL_BASE_PRICES: [u16; 4] = [15, 35, 28, 120]; // Quartz, Amethyst, Obsidian, Rare

// Size multipliers and energy capacity
const SIZE_DATA: [(f32, u16); 4] = [
    (1.0, 25),   // Tiny: 1-5g, max 25 energy
    (1.2, 60),   // Small: 6-15g, max 60 energy
    (1.5, 120),  // Medium: 16-40g, max 120 energy
    (2.0, 250),  // Large: 41-100g, max 250 energy
];

// Purity multipliers
fn purity_multiplier(purity: u8) -> f32 {
    match purity {
        0..=40 => 0.3,
        41..=70 => 0.8,
        71..=85 => 1.0,
        86..=95 => 2.2,
        96..=100 => 4.5,
        _ => 1.0,
    }
}

fn calculate_crystal_price(
    crystal_type: CrystalType,
    size_category: SizeCategory,
    purity: u8,
    weight_grams: u8,
    market_fluctuation: f32  // 0.85 to 1.15
) -> u16 {
    let base_price = CRYSTAL_BASE_PRICES[crystal_type as usize];
    let (size_mult, _) = SIZE_DATA[size_category as usize];
    let purity_mult = purity_multiplier(purity);

    ((base_price as f32 * weight_grams as f32 * size_mult * purity_mult * market_fluctuation) as u16)
}
```

### Session Income Targets
```rust
// Income ranges by character progression (silver per session)
const INCOME_RANGES: [(u16, u16); 4] = [
    (13, 52),   // Novice: 5-15 + 8-25 + 0-12 average
    (38, 162),  // Apprentice: wider range, higher base
    (75, 325),  // Adept: professional rates
    (150, 500), // Master: expert consultation
];

// Essential expense ranges (silver per session)
const EXPENSE_RANGES: [(u16, u16); 4] = [
    (28, 115),  // Novice: crystal + living + study
    (35, 145),  // Apprentice: higher quality needs
    (50, 200),  // Adept: significant investments
    (80, 350),  // Master: expensive materials
];
```

## Difficulty Scaling

### Challenge Targets by Level
```rust
// Target success rates for engaging gameplay
const TARGET_SUCCESS_RANGES: [(u8, u8); 4] = [
    (60, 80),  // Novice: forgiving
    (45, 70),  // Apprentice: moderate challenge
    (35, 60),  // Adept: significant challenge
    (25, 50),  // Master: high risk/reward
];

// Fatigue management becomes critical at these thresholds
const FATIGUE_CRITICALITY: [u8; 4] = [80, 65, 50, 35]; // Max fatigue before severe penalties

// Crystal replacement frequency (sessions)
const CRYSTAL_REPLACEMENT_FREQUENCY: [(u8, u8); 3] = [
    (15, 25),  // High quality
    (8, 12),   // Medium quality
    (4, 6),    // Low quality
];
```

## Session Pacing Targets

```rust
// Time allocations for 60-minute session (minutes)
struct SessionStructure {
    status_review: (u8, u8),        // (3, 5)
    investigation_loops: u8,         // 3-4 loops
    loop_duration: (u8, u8),        // (12, 15) per loop
    story_progression: (u8, u8),    // (8, 12)
    wrap_up: (u8, u8),             // (3, 5)
}

// Magical action frequency targets (per hour)
const MAGIC_FREQUENCY: [(u8, u8); 5] = [
    (8, 12),   // Cantrips
    (4, 6),    // Lesser spells
    (2, 3),    // Moderate spells
    (1, 2),    // Greater spells
    (0, 1),    // Master spells
];

// Progression milestone timing (hours of gameplay)
const PROGRESSION_MILESTONES: [(u8, u8); 4] = [
    (8, 12),   // Novice completion
    (15, 20),  // Apprentice completion
    (25, 35),  // Adept completion
    (40, 60),  // Master achievement
];
```

## Validation Thresholds

```rust
// Performance indicators requiring immediate attention
struct BalanceWarnings {
    completion_rate_min: f32,        // 0.6 (60%)
    session_length_min: u8,          // 45 minutes
    session_length_max: u8,          // 90 minutes
    economic_stall_threshold: f32,   // 0.5 (50% of players)
    strategy_dominance_max: f32,     // 0.8 (80% using same strategy)
    retention_drop_max: f32,         // 0.3 (30% drop at progression point)
}

// Adjustment triggers
const DIFFICULTY_ADJUSTMENT_THRESHOLDS: [(f32, f32); 2] = [
    (0.8, 0.15),  // >80% success rate: increase difficulty 15%
    (0.4, -0.1),  // <40% success rate: decrease difficulty 10%
];
```

## Implementation Notes

### Critical Success/Failure Boundaries
- **Minimum Success Rate**: 5% (always some chance)
- **Maximum Success Rate**: 95% (always some risk)
- **Crystal Critical Failure**: <20% integrity = 5% destruction chance
- **Fatigue Danger Zone**: >80 fatigue = severe penalties

### Economic Balance Points
- **Break-even Point**: ~12 hours gameplay
- **Comfortable Cushion**: ~30 hours gameplay
- **High-end Access**: ~50 hours gameplay
- **Market Fluctuation Range**: ±15% weekly

### Progression Gates
- **Theory Prerequisites**: Hard gates, must be earned
- **Attribute Minimums**: Soft gates, can be bypassed with risk
- **Social Standing**: Contextual gates, multiple solutions
- **Economic Barriers**: Temporary gates, can be worked around

This reference provides the exact numerical values needed for immediate implementation while maintaining the balance philosophy outlined in the comprehensive framework.