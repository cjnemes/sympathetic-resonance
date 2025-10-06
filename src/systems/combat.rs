//! Combat system integrating with magic for turn-based magical encounters
//!
//! This module provides:
//! - Enemy definitions and management
//! - Turn-based combat loop
//! - Damage calculations using magic system
//! - Enemy AI and decision making
//! - Combat rewards and consequences

use crate::core::{Player, WorldState};
use crate::systems::magic::{MagicSystem, MagicResult};
use crate::systems::factions::FactionId;
use crate::GameResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rand::Rng;

/// Difficulty tier for enemies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DifficultyTier {
    Beginner,
    Intermediate,
    Advanced,
    Boss,
}

impl DifficultyTier {
    fn hp_multiplier(&self) -> i32 {
        match self {
            DifficultyTier::Beginner => 50,
            DifficultyTier::Intermediate => 100,
            DifficultyTier::Advanced => 150,
            DifficultyTier::Boss => 250,
        }
    }

    fn experience_multiplier(&self) -> i32 {
        match self {
            DifficultyTier::Beginner => 50,
            DifficultyTier::Intermediate => 100,
            DifficultyTier::Advanced => 200,
            DifficultyTier::Boss => 500,
        }
    }
}

/// Loot drop definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootDrop {
    pub item_id: String,
    pub drop_chance: f32,
    pub quantity_range: (i32, i32),
}

/// Enemy definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enemy {
    pub id: String,
    pub name: String,
    pub description: String,
    pub health: i32,
    pub max_health: i32,
    /// Magical resistance by spell type (0.0 = no resistance, 1.0 = immune)
    pub magical_resistance: HashMap<String, f32>,
    pub difficulty_tier: DifficultyTier,
    pub loot_table: Vec<LootDrop>,
    pub experience_reward: i32,
    pub faction_affiliation: Option<FactionId>,
    /// Vulnerable frequency (takes extra damage from this crystal frequency)
    pub vulnerable_frequency: Option<u8>,
}

impl Enemy {
    /// Create a new enemy with defaults
    pub fn new(id: String, name: String, description: String, tier: DifficultyTier) -> Self {
        let max_health = tier.hp_multiplier();
        let experience_reward = tier.experience_multiplier();

        Self {
            id,
            name,
            description,
            health: max_health,
            max_health,
            magical_resistance: HashMap::new(),
            difficulty_tier: tier,
            loot_table: Vec::new(),
            experience_reward,
            faction_affiliation: None,
            vulnerable_frequency: None,
        }
    }

    /// Add resistance to a spell type
    pub fn with_resistance(mut self, spell_type: &str, resistance: f32) -> Self {
        self.magical_resistance.insert(spell_type.to_string(), resistance.clamp(0.0, 1.0));
        self
    }

    /// Add loot drop
    pub fn with_loot(mut self, item_id: &str, drop_chance: f32, quantity: (i32, i32)) -> Self {
        self.loot_table.push(LootDrop {
            item_id: item_id.to_string(),
            drop_chance: drop_chance.clamp(0.0, 1.0),
            quantity_range: quantity,
        });
        self
    }

    /// Set faction affiliation
    pub fn with_faction(mut self, faction: FactionId) -> Self {
        self.faction_affiliation = Some(faction);
        self
    }

    /// Set vulnerable frequency
    pub fn with_vulnerable_frequency(mut self, frequency: u8) -> Self {
        self.vulnerable_frequency = Some(frequency);
        self
    }

    /// Take damage
    pub fn take_damage(&mut self, amount: i32) {
        self.health = (self.health - amount).max(0);
    }

    /// Check if enemy is alive
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    /// Get health percentage
    pub fn health_percentage(&self) -> f32 {
        self.health as f32 / self.max_health as f32
    }
}

/// Defense action types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefenseType {
    Shield,      // Reduces damage 50%, costs energy
    Evade,       // 70% chance to avoid, costs fatigue
    CounterMagic, // Reflects 30% damage, requires theory knowledge
}

/// Combat action
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CombatAction {
    Attack { spell_type: String },
    Defend { defense_type: DefenseType },
    UseItem { item_id: String },
    Flee,
}

/// Combat outcome
#[derive(Debug, Clone)]
pub enum CombatOutcome {
    Victory {
        experience: i32,
        loot: Vec<String>,
        faction_change: Option<(FactionId, i32)>,
    },
    Defeat {
        energy_drain_percent: i32,
        fatigue_increase: i32,
        faction_penalty: Option<(FactionId, i32)>,
    },
    Fled {
        energy_cost: i32,
        fatigue_cost: i32,
        faction_penalty: Option<(FactionId, i32)>,
    },
}

/// Active combat encounter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatEncounter {
    pub enemy: Enemy,
    pub turn_count: i32,
    pub player_defending: bool,
    pub last_defense_type: Option<DefenseType>,
}

impl CombatEncounter {
    /// Create new combat encounter
    pub fn new(enemy: Enemy) -> Self {
        Self {
            enemy,
            turn_count: 0,
            player_defending: false,
            last_defense_type: None,
        }
    }
}

/// Combat system managing combat encounters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatSystem {
    active_encounter: Option<CombatEncounter>,
}

impl CombatSystem {
    /// Create a new combat system
    pub fn new() -> Self {
        Self {
            active_encounter: None,
        }
    }

    /// Start a combat encounter
    pub fn start_encounter(&mut self, enemy: Enemy) -> GameResult<String> {
        if self.active_encounter.is_some() {
            return Err(crate::GameError::InvalidCommand(
                "Already in combat!".to_string()
            ).into());
        }

        let enemy_name = enemy.name.clone();
        self.active_encounter = Some(CombatEncounter::new(enemy));

        Ok(format!(
            "Combat initiated with {}! Prepare for battle.\n\
             Use 'cast <spell>' to attack, 'defend' for protection, or 'flee' to escape.",
            enemy_name
        ))
    }

    /// Check if currently in combat
    pub fn is_in_combat(&self) -> bool {
        self.active_encounter.is_some()
    }

    /// Get current enemy (if in combat)
    pub fn current_enemy(&self) -> Option<&Enemy> {
        self.active_encounter.as_ref().map(|e| &e.enemy)
    }

    /// Execute player attack action
    pub fn player_attack(
        &mut self,
        player: &mut Player,
        world: &mut WorldState,
        magic_system: &mut MagicSystem,
        spell_type: &str,
    ) -> GameResult<String> {
        let encounter = self.active_encounter.as_mut()
            .ok_or_else(|| crate::GameError::InvalidCommand("Not in combat".to_string()))?;

        // Cast spell using magic system
        let magic_result = magic_system.attempt_magic(
            spell_type,
            player,
            world,
            Some(&encounter.enemy.name),
        )?;

        let mut output = String::new();

        // Calculate damage if spell succeeded
        if magic_result.success {
            // Get enemy data before borrowing encounter mutably
            let enemy_name = encounter.enemy.name.clone();
            let enemy_max_hp = encounter.enemy.max_health;
            let enemy_resistances = encounter.enemy.magical_resistance.clone();
            let enemy_vuln_freq = encounter.enemy.vulnerable_frequency;

            // Calculate damage (avoiding borrowing conflicts)
            let damage = Self::calculate_damage_static(
                &magic_result,
                player,
                &enemy_resistances,
                enemy_vuln_freq,
                spell_type
            );

            encounter.enemy.take_damage(damage);

            output.push_str(&format!(
                "Your {} spell strikes {}! (Damage: {}, Enemy HP: {}/{})\n",
                spell_type,
                enemy_name,
                damage,
                encounter.enemy.health,
                enemy_max_hp
            ));

            // Check if enemy defeated
            if !encounter.enemy.is_alive() {
                let outcome = self.resolve_victory(player);
                self.active_encounter = None;
                return Ok(format!("{}\n{}", output, self.format_outcome(&outcome)));
            }
        } else {
            output.push_str(&format!(
                "Your {} spell fizzled! The magic fails to manifest properly.\n",
                spell_type
            ));
        }

        // Enemy turn
        encounter.turn_count += 1;
        encounter.player_defending = false;
        let enemy_action_result = self.enemy_turn(player, magic_system, world)?;
        output.push_str(&enemy_action_result);

        Ok(output)
    }

    /// Calculate damage from magic attack (static version to avoid borrowing conflicts)
    fn calculate_damage_static(
        magic_result: &MagicResult,
        player: &Player,
        enemy_resistances: &HashMap<String, f32>,
        enemy_vuln_freq: Option<u8>,
        spell_type: &str,
    ) -> i32 {
        // Base damage from magic power level
        let base_damage = (magic_result.power_level * 10.0) as i32;

        // Theory bonus (from player's magic system integration)
        let theory_bonus = player.calculate_spell_type_bonus(spell_type);
        let damage_multiplier = 1.0 + theory_bonus;

        // Enemy resistance
        let resistance = enemy_resistances.get(spell_type).unwrap_or(&0.0);
        let resistance_multiplier = 1.0 - resistance;

        // Frequency vulnerability bonus
        let vulnerability_bonus = if let (Some(crystal), Some(vuln_freq)) =
            (player.active_crystal(), enemy_vuln_freq)
        {
            if crystal.frequency == vuln_freq as i32 {
                1.5 // +50% damage for frequency match
            } else {
                1.0
            }
        } else {
            1.0
        };

        // Calculate final damage
        let final_damage = (base_damage as f32 * damage_multiplier * resistance_multiplier * vulnerability_bonus) as i32;

        final_damage.max(1) // Minimum 1 damage
    }

    /// Calculate damage from magic attack (convenience wrapper)
    fn calculate_damage(
        &self,
        magic_result: &MagicResult,
        player: &Player,
        enemy: &Enemy,
        spell_type: &str,
    ) -> i32 {
        Self::calculate_damage_static(
            magic_result,
            player,
            &enemy.magical_resistance,
            enemy.vulnerable_frequency,
            spell_type
        )
    }

    /// Execute player defense action
    pub fn player_defend(
        &mut self,
        player: &mut Player,
        defense_type: DefenseType,
    ) -> GameResult<String> {
        let encounter = self.active_encounter.as_mut()
            .ok_or_else(|| crate::GameError::InvalidCommand("Not in combat".to_string()))?;

        // Apply defense costs
        let (energy_cost, fatigue_cost) = match defense_type {
            DefenseType::Shield => (15, 5),
            DefenseType::Evade => (10, 15),
            DefenseType::CounterMagic => {
                // Requires mental resonance theory
                if player.theory_understanding("mental_resonance") < 0.5 {
                    return Err(crate::GameError::InsufficientResources(
                        "Counter-magic requires Mental Resonance Theory (0.5)".to_string()
                    ).into());
                }
                (25, 10)
            }
        };

        player.use_mental_energy(energy_cost, fatigue_cost)?;

        encounter.player_defending = true;
        encounter.last_defense_type = Some(defense_type);

        let defense_name = match defense_type {
            DefenseType::Shield => "shield",
            DefenseType::Evade => "evasive stance",
            DefenseType::CounterMagic => "counter-magic ward",
        };

        Ok(format!("You adopt a defensive {} position.", defense_name))
    }

    /// Player attempts to flee
    pub fn player_flee(
        &mut self,
        player: &mut Player,
    ) -> GameResult<String> {
        let encounter = self.active_encounter.take()
            .ok_or_else(|| crate::GameError::InvalidCommand("Not in combat".to_string()))?;

        // Apply flee costs
        player.use_mental_energy(20, 20)?;

        // Faction penalty if enemy has affiliation
        let faction_penalty = encounter.enemy.faction_affiliation.map(|faction| (faction, -5));

        if let Some((faction_id, penalty)) = faction_penalty {
            // Apply faction penalty (would need faction_system integration)
            let faction_name = format!("{:?}", faction_id);
            return Ok(format!(
                "You flee from combat with {}!\n\
                 Energy Cost: 20, Fatigue Cost: 20\n\
                 Faction Penalty: {} {}",
                encounter.enemy.name,
                faction_name,
                penalty
            ));
        }

        Ok(format!(
            "You flee from combat with {}!\n\
             Energy Cost: 20, Fatigue Cost: 20",
            encounter.enemy.name
        ))
    }

    /// Enemy takes their turn
    fn enemy_turn(
        &mut self,
        player: &mut Player,
        _magic_system: &mut MagicSystem,
        _world: &mut WorldState,
    ) -> GameResult<String> {
        let encounter = self.active_encounter.as_mut()
            .ok_or_else(|| crate::GameError::InvalidCommand("Not in combat".to_string()))?;

        // Simple AI: attack aggressively when player is low on energy
        let _action = if player.mental_state.current_energy < 30 {
            "aggressive_attack"
        } else if encounter.enemy.health_percentage() < 0.3 {
            // Flee if low health
            if rand::thread_rng().gen_bool(0.5) {
                return self.enemy_flees();
            }
            "desperate_attack"
        } else {
            "normal_attack"
        };

        // Enemy attacks with a basic spell
        // Get difficulty tier to avoid borrowing issues
        let difficulty_tier = encounter.enemy.difficulty_tier;
        let spell_type = Self::enemy_spell_for_tier(difficulty_tier);

        // Simplified enemy attack (doesn't use full magic system to avoid player cost application)
        let base_damage = match encounter.enemy.difficulty_tier {
            DifficultyTier::Beginner => rand::thread_rng().gen_range(10..=20),
            DifficultyTier::Intermediate => rand::thread_rng().gen_range(25..=40),
            DifficultyTier::Advanced => rand::thread_rng().gen_range(40..=60),
            DifficultyTier::Boss => rand::thread_rng().gen_range(60..=90),
        };

        // Apply defense reductions
        let final_damage = if encounter.player_defending {
            match encounter.last_defense_type {
                Some(DefenseType::Shield) => base_damage / 2, // 50% reduction
                Some(DefenseType::Evade) => {
                    if rand::thread_rng().gen_bool(0.7) {
                        0 // 70% chance to dodge completely
                    } else {
                        base_damage
                    }
                }
                Some(DefenseType::CounterMagic) => {
                    // Reflect 30% damage back to enemy
                    let reflected = (base_damage as f32 * 0.3) as i32;
                    encounter.enemy.take_damage(reflected);
                    base_damage - reflected
                }
                None => base_damage,
            }
        } else {
            base_damage
        };

        // Apply damage to player by reducing energy
        let actual_damage = final_damage.min(player.mental_state.current_energy);
        player.mental_state.current_energy = (player.mental_state.current_energy - actual_damage).max(0);

        let mut output = format!(
            "\n{} attacks with {}! (Damage: {})\n",
            encounter.enemy.name,
            spell_type,
            actual_damage
        );

        // Check if player is defeated (energy depleted)
        if player.mental_state.current_energy == 0 {
            let outcome = self.resolve_defeat(player);
            self.active_encounter = None;
            output.push_str(&format!("\n{}", self.format_outcome(&outcome)));
        }

        Ok(output)
    }

    /// Select appropriate spell for enemy tier
    fn enemy_spell_for_tier(tier: DifficultyTier) -> String {
        match tier {
            DifficultyTier::Beginner => "energy_blast",
            DifficultyTier::Intermediate => "focused_strike",
            DifficultyTier::Advanced => "resonant_pulse",
            DifficultyTier::Boss => "devastating_wave",
        }.to_string()
    }

    /// Enemy flees from combat
    fn enemy_flees(&mut self) -> GameResult<String> {
        let encounter = self.active_encounter.take()
            .ok_or_else(|| crate::GameError::InvalidCommand("Not in combat".to_string()))?;

        Ok(format!(
            "\n{} flees from combat!",
            encounter.enemy.name
        ))
    }

    /// Resolve combat victory
    fn resolve_victory(&self, _player: &mut Player) -> CombatOutcome {
        let encounter = self.active_encounter.as_ref().unwrap();

        // Calculate experience
        let base_exp = encounter.enemy.experience_reward;
        let efficiency_bonus = if encounter.turn_count < 5 { 1.1 } else { 1.0 };
        let total_exp = (base_exp as f32 * efficiency_bonus) as i32;

        // Roll for loot
        let mut loot = Vec::new();
        let mut rng = rand::thread_rng();
        for drop in &encounter.enemy.loot_table {
            if rng.gen::<f32>() < drop.drop_chance {
                let quantity = rng.gen_range(drop.quantity_range.0..=drop.quantity_range.1);
                for _ in 0..quantity {
                    loot.push(drop.item_id.clone());
                }
            }
        }

        // Faction consequences (defeating enemy gives penalty with their faction)
        let faction_change = encounter.enemy.faction_affiliation.map(|faction| (faction, -10));

        CombatOutcome::Victory {
            experience: total_exp,
            loot,
            faction_change,
        }
    }

    /// Resolve combat defeat
    fn resolve_defeat(&self, player: &mut Player) -> CombatOutcome {
        let encounter = self.active_encounter.as_ref().unwrap();

        // Drain energy to 10% and add fatigue
        player.mental_state.current_energy = (player.mental_state.max_energy as f32 * 0.1) as i32;
        player.mental_state.fatigue = (player.mental_state.fatigue + 40).min(100);

        let faction_penalty = encounter.enemy.faction_affiliation.map(|faction| (faction, -10));

        CombatOutcome::Defeat {
            energy_drain_percent: 90,
            fatigue_increase: 40,
            faction_penalty,
        }
    }

    /// Format combat outcome for display
    fn format_outcome(&self, outcome: &CombatOutcome) -> String {
        match outcome {
            CombatOutcome::Victory { experience, loot, faction_change } => {
                let mut output = format!("\n=== VICTORY ===\nYou have defeated the enemy!\n");
                output.push_str(&format!("Experience Gained: {}\n", experience));

                if !loot.is_empty() {
                    output.push_str(&format!("Loot Acquired: {}\n", loot.join(", ")));
                }

                if let Some((faction, change)) = faction_change {
                    output.push_str(&format!("Faction Change: {:?} {}\n", faction, change));
                }

                output
            }
            CombatOutcome::Defeat { energy_drain_percent, fatigue_increase, faction_penalty } => {
                let mut output = format!(
                    "\n=== DEFEAT ===\nYou have been overwhelmed!\n\
                     Energy Drained: {}%, Fatigue Increased: {}\n",
                    energy_drain_percent, fatigue_increase
                );

                if let Some((faction, penalty)) = faction_penalty {
                    output.push_str(&format!("Faction Penalty: {:?} {}\n", faction, penalty));
                }

                output
            }
            CombatOutcome::Fled { energy_cost, fatigue_cost, faction_penalty } => {
                let mut output = format!(
                    "You fled from combat.\nEnergy Cost: {}, Fatigue Cost: {}\n",
                    energy_cost, fatigue_cost
                );

                if let Some((faction, penalty)) = faction_penalty {
                    output.push_str(&format!("Faction Penalty: {:?} {}\n", faction, penalty));
                }

                output
            }
        }
    }

    /// Get current combat status
    pub fn get_status(&self) -> Option<String> {
        self.active_encounter.as_ref().map(|encounter| {
            format!(
                "=== COMBAT STATUS ===\n\
                 Enemy: {} (HP: {}/{})\n\
                 Turn: {}\n\
                 Your Defense: {}",
                encounter.enemy.name,
                encounter.enemy.health,
                encounter.enemy.max_health,
                encounter.turn_count,
                if encounter.player_defending { "Active" } else { "None" }
            )
        })
    }

    /// Legacy method for compatibility
    pub fn handle_combat(
        &self,
        _player: &mut Player,
        _world: &mut WorldState,
        _target: &str,
    ) -> GameResult<String> {
        Ok("Use 'attack <enemy> with <spell>' to initiate combat.".to_string())
    }
}

impl Default for CombatSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Create example enemies for testing and gameplay
pub fn create_example_enemies() -> Vec<Enemy> {
    vec![
        // Tier 1: Beginner
        Enemy::new(
            "corrupted_shard".to_string(),
            "Corrupted Crystal Shard".to_string(),
            "A fragment of crystal tainted by unstable magical energies.".to_string(),
            DifficultyTier::Beginner,
        )
        .with_resistance("shield", 0.2)
        .with_loot("damaged_crystal", 0.6, (1, 2))
        .with_vulnerable_frequency(5),

        // Tier 2: Intermediate
        Enemy::new(
            "rogue_practitioner".to_string(),
            "Rogue Practitioner".to_string(),
            "An outlaw magic user practicing forbidden techniques.".to_string(),
            DifficultyTier::Intermediate,
        )
        .with_resistance("light", 0.3)
        .with_resistance("healing", 0.5)
        .with_faction(FactionId::UndergroundNetwork)
        .with_loot("research_notes", 0.4, (1, 1))
        .with_loot("crystal_fragment", 0.5, (1, 3)),

        // Tier 3: Advanced
        Enemy::new(
            "resonance_anomaly".to_string(),
            "Resonance Anomaly".to_string(),
            "A dangerous manifestation of wild magical resonance.".to_string(),
            DifficultyTier::Advanced,
        )
        .with_resistance("detection", 0.6)
        .with_resistance("manipulation", 0.4)
        .with_loot("rare_crystal", 0.3, (1, 1))
        .with_vulnerable_frequency(7),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enemy_creation() {
        let enemy = Enemy::new(
            "test".to_string(),
            "Test Enemy".to_string(),
            "A test".to_string(),
            DifficultyTier::Beginner,
        );

        assert_eq!(enemy.health, 50);
        assert_eq!(enemy.max_health, 50);
        assert_eq!(enemy.experience_reward, 50);
    }

    #[test]
    fn test_enemy_damage() {
        let mut enemy = Enemy::new(
            "test".to_string(),
            "Test".to_string(),
            "Test".to_string(),
            DifficultyTier::Beginner,
        );

        enemy.take_damage(20);
        assert_eq!(enemy.health, 30);
        assert!(enemy.is_alive());

        enemy.take_damage(40);
        assert_eq!(enemy.health, 0);
        assert!(!enemy.is_alive());
    }

    #[test]
    fn test_enemy_health_percentage() {
        let mut enemy = Enemy::new(
            "test".to_string(),
            "Test".to_string(),
            "Test".to_string(),
            DifficultyTier::Beginner,
        );

        assert_eq!(enemy.health_percentage(), 1.0);

        enemy.take_damage(25);
        assert_eq!(enemy.health_percentage(), 0.5);
    }

    #[test]
    fn test_combat_system_creation() {
        let combat_system = CombatSystem::new();
        assert!(!combat_system.is_in_combat());
        assert!(combat_system.current_enemy().is_none());
    }

    #[test]
    fn test_start_encounter() {
        let mut combat_system = CombatSystem::new();
        let enemy = Enemy::new(
            "test".to_string(),
            "Test Enemy".to_string(),
            "A test".to_string(),
            DifficultyTier::Beginner,
        );

        let result = combat_system.start_encounter(enemy);
        assert!(result.is_ok());
        assert!(combat_system.is_in_combat());
    }

    #[test]
    fn test_cannot_start_second_encounter() {
        let mut combat_system = CombatSystem::new();
        let enemy1 = Enemy::new("test1".to_string(), "Test 1".to_string(), "Test".to_string(), DifficultyTier::Beginner);
        let enemy2 = Enemy::new("test2".to_string(), "Test 2".to_string(), "Test".to_string(), DifficultyTier::Beginner);

        combat_system.start_encounter(enemy1).unwrap();
        let result = combat_system.start_encounter(enemy2);

        assert!(result.is_err());
    }

    #[test]
    fn test_example_enemies() {
        let enemies = create_example_enemies();
        assert_eq!(enemies.len(), 3);

        // Check beginner enemy
        assert_eq!(enemies[0].difficulty_tier, DifficultyTier::Beginner);
        assert_eq!(enemies[0].health, 50);

        // Check intermediate enemy
        assert_eq!(enemies[1].difficulty_tier, DifficultyTier::Intermediate);
        assert_eq!(enemies[1].faction_affiliation, Some(FactionId::UndergroundNetwork));

        // Check advanced enemy
        assert_eq!(enemies[2].difficulty_tier, DifficultyTier::Advanced);
        assert_eq!(enemies[2].health, 150);
    }
}
