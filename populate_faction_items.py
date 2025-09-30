#!/usr/bin/env python3
"""
Script to populate the database with faction-specific educational items.
This demonstrates how to insert the designed faction items into the game database.
"""

import sqlite3
import json
import os

# Define all faction items with their properties
FACTION_ITEMS = [
    # MAGISTERS' COUNCIL ITEMS
    {
        "id": "council_scholars_circlet",
        "name": "Council Scholar's Circlet",
        "description": "An elegant circlet worn by senior academics, inscribed with formulas that enhance systematic learning while discouraging reckless experimentation.",
        "item_type": "Equipment",
        "properties": json.dumps({
            "weight": 0.5,
            "value": 750,
            "rarity": "Rare",
            "durability": 100,
            "max_durability": 100,
            "magical": True,
            "equipment_slot": "Head",
            "bonuses": [
                {"type": "LearningEfficiency", "method": "Study", "bonus": 0.4},
                {"type": "LearningEfficiency", "method": "Experimentation", "bonus": -0.2},
                {"type": "FactionBonus", "faction": "MagistersCouncil", "bonus": 2}
            ],
            "requirements": {
                "mental_acuity": 60,
                "faction_rep": {"MagistersCouncil": 75},
                "theories": ["harmonic_fundamentals", "crystal_structures", "mental_resonance", "bio_resonance", "detection_arrays"]
            },
            "abilities": [{"name": "Academic Network", "cooldown": 720}]
        })
    },
    {
        "id": "codified_theory_compendium",
        "name": "Codified Theory Compendium",
        "description": "A comprehensive academic reference containing cross-indexed theories with detailed annotations from Council scholars.",
        "item_type": "Educational",
        "properties": json.dumps({
            "weight": 3.0,
            "value": 300,
            "rarity": "Uncommon",
            "durability": 80,
            "max_durability": 100,
            "magical": True,
            "educational_function": "KnowledgeArchive",
            "learning_bonus": 0.3,
            "applicable_theories": ["harmonic_fundamentals", "crystal_structures", "mental_resonance", "bio_resonance", "detection_arrays"],
            "requirements": {
                "faction_rep": {"MagistersCouncil": 25}
            }
        })
    },
    {
        "id": "academy_research_laboratory",
        "name": "Academy Research Laboratory",
        "description": "A complete controlled experimental facility that guarantees safe, precise magical research with zero risk of catastrophic failure.",
        "item_type": "Tool",
        "properties": json.dumps({
            "weight": 100.0,
            "value": 5000,
            "rarity": "Legendary",
            "durability": 100,
            "max_durability": 100,
            "magical": True,
            "tool_function": "controlled_experimentation",
            "precision_bonus": 1.0,
            "requirements": {
                "mental_acuity": 80,
                "faction_rep": {"MagistersCouncil": 100},
                "theories": ["harmonic_fundamentals", "detection_arrays"]
            }
        })
    },
    {
        "id": "magistrates_seal_ring",
        "name": "Magistrate's Seal Ring",
        "description": "A gold ring bearing the official seal of the Magisters' Council, granting diplomatic privileges and teaching bonuses.",
        "item_type": "Equipment",
        "properties": json.dumps({
            "weight": 0.1,
            "value": 400,
            "rarity": "Uncommon",
            "durability": 100,
            "max_durability": 100,
            "magical": True,
            "equipment_slot": "Ring",
            "bonuses": [
                {"type": "LearningEfficiency", "method": "Teaching", "bonus": 0.2},
                {"type": "FactionBonus", "faction": "MagistersCouncil", "bonus": 1}
            ],
            "requirements": {
                "faction_rep": {"MagistersCouncil": 25}
            },
            "abilities": [{"name": "Diplomatic Immunity", "type": "passive"}]
        })
    },

    # ORDER OF NATURAL HARMONY ITEMS
    {
        "id": "harmony_meditation_stone",
        "name": "Harmony Meditation Stone",
        "description": "A smooth river stone that resonates with natural energy, enhancing focus during dawn and dusk meditation sessions.",
        "item_type": "Equipment",
        "properties": json.dumps({
            "weight": 0.8,
            "value": 250,
            "rarity": "Uncommon",
            "durability": 100,
            "max_durability": 100,
            "magical": True,
            "equipment_slot": "Neck",
            "bonuses": [
                {"type": "LearningEfficiency", "method": "Study", "bonus": 0.35, "environment": "natural", "time_bonus": "dawn_dusk"},
                {"type": "EnergyCostReduction", "bonus": 0.25}
            ],
            "requirements": {
                "resonance_sensitivity": 40,
                "faction_rep": {"OrderOfNaturalHarmony": 25}
            }
        })
    },
    {
        "id": "living_crystal_garden",
        "name": "Living Crystal Garden",
        "description": "A symbiotic collection of crystals that grow stronger as your understanding deepens, providing enhanced bio-resonance research capabilities.",
        "item_type": "Tool",
        "properties": json.dumps({
            "weight": 50.0,
            "value": 1500,
            "rarity": "Rare",
            "durability": 100,
            "max_durability": 100,
            "magical": True,
            "tool_function": "symbiotic_research",
            "precision_bonus": 0.4,
            "theory_focus": "bio_resonance",
            "requirements": {
                "resonance_sensitivity": 50,
                "faction_rep": {"OrderOfNaturalHarmony": 75},
                "theories": ["bio_resonance"]
            }
        })
    },
    {
        "id": "natures_wisdom_tome",
        "name": "Nature's Wisdom Tome",
        "description": "An ancient book written on living bark that changes its teachings with the seasons, revealing different aspects of natural magic.",
        "item_type": "Educational",
        "properties": json.dumps({
            "weight": 2.5,
            "value": 800,
            "rarity": "Rare",
            "durability": 90,
            "max_durability": 100,
            "magical": True,
            "educational_function": "KnowledgeArchive",
            "learning_bonus": 0.45,
            "applicable_theories": ["bio_resonance", "detection_arrays"],
            "seasonal_bonus": True,
            "requirements": {
                "faction_rep": {"OrderOfNaturalHarmony": 75}
            }
        })
    },
    {
        "id": "spiritual_balance_amulet",
        "name": "Spiritual Balance Amulet",
        "description": "A wooden amulet carved from sacred grove trees, providing protection against magical corruption and mental fatigue.",
        "item_type": "Equipment",
        "properties": json.dumps({
            "weight": 0.3,
            "value": 350,
            "rarity": "Uncommon",
            "durability": 100,
            "max_durability": 100,
            "magical": True,
            "equipment_slot": "Neck",
            "bonuses": [
                {"type": "LearningEfficiency", "method": "Observation", "bonus": 0.3},
                {"type": "FatigueResistance", "bonus": 0.5}
            ],
            "requirements": {
                "faction_rep": {"OrderOfNaturalHarmony": 25}
            },
            "abilities": [{"name": "Inner Peace", "type": "passive", "effect": "stress_immunity"}]
        })
    },

    # INDUSTRIAL CONSORTIUM ITEMS
    {
        "id": "efficiency_optimizer_goggles",
        "name": "Efficiency Optimizer Goggles",
        "description": "Advanced optical enhancement devices that analyze magical processes and suggest optimization pathways for maximum efficiency.",
        "item_type": "Equipment",
        "properties": json.dumps({
            "weight": 1.2,
            "value": 400,
            "rarity": "Uncommon",
            "durability": 80,
            "max_durability": 100,
            "magical": True,
            "equipment_slot": "Head",
            "bonuses": [
                {"type": "LearningEfficiency", "method": "Experimentation", "bonus": 0.25},
                {"type": "LearningEfficiency", "method": "Research", "bonus": 0.35, "environment": "workshop"}
            ],
            "requirements": {
                "mental_acuity": 45,
                "faction_rep": {"IndustrialConsortium": 25}
            },
            "abilities": [{"name": "Process Analysis", "cooldown": 240}]
        })
    },
    {
        "id": "advanced_experimental_apparatus",
        "name": "Advanced Experimental Apparatus",
        "description": "Cutting-edge magical research equipment that enables rapid prototyping and parallel experimentation, with built-in safety protocols.",
        "item_type": "Tool",
        "properties": json.dumps({
            "weight": 75.0,
            "value": 2500,
            "rarity": "Rare",
            "durability": 90,
            "max_durability": 100,
            "magical": True,
            "tool_function": "rapid_prototyping",
            "precision_bonus": 0.6,
            "risk_reward": {"breakthrough_chance": 0.05, "failure_chance": 0.05},
            "requirements": {
                "mental_acuity": 65,
                "faction_rep": {"IndustrialConsortium": 75},
                "theories": ["resonance_amplification"]
            }
        })
    },
    {
        "id": "innovation_database",
        "name": "Innovation Database",
        "description": "A crystalline storage device containing thousands of proprietary magical techniques and commercial applications developed by Consortium researchers.",
        "item_type": "Educational",
        "properties": json.dumps({
            "weight": 1.0,
            "value": 600,
            "rarity": "Uncommon",
            "durability": 100,
            "max_durability": 100,
            "magical": True,
            "educational_function": "KnowledgeArchive",
            "learning_bonus": 0.3,
            "applicable_theories": ["light_manipulation", "resonance_amplification"],
            "commercial_value": True,
            "requirements": {
                "faction_rep": {"IndustrialConsortium": 25}
            }
        })
    },
    {
        "id": "productivity_enhancement_suite",
        "name": "Productivity Enhancement Suite",
        "description": "An integrated system of efficiency-boosting magical devices worn as a vest, optimizing workflow and enabling parallel learning processes.",
        "item_type": "Equipment",
        "properties": json.dumps({
            "weight": 4.0,
            "value": 1200,
            "rarity": "Rare",
            "durability": 85,
            "max_durability": 100,
            "magical": True,
            "equipment_slot": "Chest",
            "bonuses": [
                {"type": "LearningEfficiency", "method": "Study", "bonus": 0.2},
                {"type": "LearningEfficiency", "method": "Research", "bonus": 0.2},
                {"type": "LearningEfficiency", "method": "Experimentation", "bonus": 0.2},
                {"type": "EnergyCostReduction", "bonus": 0.4}
            ],
            "requirements": {
                "mental_acuity": 65,
                "faction_rep": {"IndustrialConsortium": 75}
            },
            "abilities": [{"name": "Workflow Optimization", "cooldown": 1440, "effect": "parallel_learning"}]
        })
    },

    # UNDERGROUND NETWORK ITEMS
    {
        "id": "forbidden_knowledge_cache",
        "name": "Forbidden Knowledge Cache",
        "description": "A concealed data crystal containing dangerous magical theories censored by authorities. Use with extreme caution.",
        "item_type": "Educational",
        "properties": json.dumps({
            "weight": 0.5,
            "value": 800,
            "rarity": "Rare",
            "durability": 100,
            "max_durability": 100,
            "magical": True,
            "educational_function": "KnowledgeArchive",
            "learning_bonus": 0.5,
            "applicable_theories": ["sympathetic_networks", "theoretical_synthesis"],
            "dangerous": True,
            "detection_risk": 0.1,
            "requirements": {
                "faction_rep": {"UndergroundNetwork": 25},
                "environment": "hidden"
            }
        })
    },
    {
        "id": "experimental_risk_amplifier",
        "name": "Experimental Risk Amplifier",
        "description": "An unstable magical device that dramatically increases experimental potential while risking catastrophic magical backlash.",
        "item_type": "Equipment",
        "properties": json.dumps({
            "weight": 2.0,
            "value": 1500,
            "rarity": "Rare",
            "durability": 60,
            "max_durability": 100,
            "magical": True,
            "equipment_slot": "MainHand",
            "bonuses": [
                {"type": "LearningEfficiency", "method": "Experimentation", "bonus": 0.8}
            ],
            "requirements": {
                "resonance_sensitivity": 70,
                "faction_rep": {"UndergroundNetwork": 75},
                "theories": ["sympathetic_networks"]
            },
            "abilities": [
                {
                    "name": "Dangerous Insights",
                    "type": "triggered",
                    "trigger": "experimentation",
                    "breakthrough_chance": 0.15,
                    "backlash_chance": 0.15
                }
            ]
        })
    },
    {
        "id": "revolutionaries_cloak",
        "name": "Revolutionary's Cloak",
        "description": "A dark cloak woven with concealment enchantments, allowing discrete magical research and communication with other revolutionaries.",
        "item_type": "Equipment",
        "properties": json.dumps({
            "weight": 1.5,
            "value": 450,
            "rarity": "Uncommon",
            "durability": 90,
            "max_durability": 100,
            "magical": True,
            "equipment_slot": "Back",
            "bonuses": [
                {"type": "LearningEfficiency", "method": "Study", "bonus": 0.25},
                {"type": "LearningEfficiency", "method": "Research", "bonus": 0.25},
                {"type": "LearningEfficiency", "method": "Experimentation", "bonus": 0.25}
            ],
            "requirements": {
                "faction_rep": {"UndergroundNetwork": 25}
            },
            "abilities": [
                {
                    "name": "Underground Network",
                    "cooldown": 360,
                    "effect": "knowledge_sharing"
                }
            ],
            "concealment": True
        })
    },
    {
        "id": "black_market_research_tools",
        "name": "Black Market Research Tools",
        "description": "A collection of illegal research instruments of varying quality, enabling banned magical procedures with unpredictable results.",
        "item_type": "Tool",
        "properties": json.dumps({
            "weight": 10.0,
            "value": 900,
            "rarity": "Uncommon",
            "durability": 70,
            "max_durability": 100,
            "magical": True,
            "tool_function": "illegal_research",
            "precision_bonus": 0.45,
            "variable_quality": True,
            "legal_risk": True,
            "requirements": {
                "faction_rep": {"UndergroundNetwork": 75}
            }
        })
    },

    # NEUTRAL SCHOLARS ITEMS
    {
        "id": "diplomatic_synthesis_lens",
        "name": "Diplomatic Synthesis Lens",
        "description": "A crystalline monocle that reveals the underlying connections between different schools of magical thought.",
        "item_type": "Equipment",
        "properties": json.dumps({
            "weight": 0.3,
            "value": 350,
            "rarity": "Uncommon",
            "durability": 100,
            "max_durability": 100,
            "magical": True,
            "equipment_slot": "Head",
            "bonuses": [
                {"type": "LearningEfficiency", "method": "Study", "bonus": 0.3, "cross_faction": True},
                {"type": "LearningEfficiency", "method": "Research", "bonus": 0.3, "cross_faction": True}
            ],
            "requirements": {
                "faction_rep": {"NeutralScholars": 25}
            },
            "abilities": [
                {
                    "name": "Cross-Cultural Analysis",
                    "type": "passive",
                    "effect": "conflict_reduction"
                }
            ]
        })
    },
    {
        "id": "universal_theory_framework",
        "name": "Universal Theory Framework",
        "description": "A comprehensive theoretical model that demonstrates the fundamental connections between all schools of magical thought.",
        "item_type": "Educational",
        "properties": json.dumps({
            "weight": 5.0,
            "value": 1200,
            "rarity": "Rare",
            "durability": 100,
            "max_durability": 100,
            "magical": True,
            "educational_function": "KnowledgeArchive",
            "learning_bonus": 0.35,
            "applicable_theories": [
                "harmonic_fundamentals", "crystal_structures", "mental_resonance",
                "bio_resonance", "detection_arrays", "light_manipulation",
                "resonance_amplification", "sympathetic_networks", "theoretical_synthesis"
            ],
            "synthesis_bonus": 0.45,
            "requirements": {
                "faction_rep": {"NeutralScholars": 75},
                "theories": ["harmonic_fundamentals", "bio_resonance", "light_manipulation", "sympathetic_networks"]
            }
        })
    },
    {
        "id": "scholars_neutrality_medallion",
        "name": "Scholar's Neutrality Medallion",
        "description": "A perfectly balanced medallion that allows safe interaction with opposing faction items and ideologies.",
        "item_type": "Equipment",
        "properties": json.dumps({
            "weight": 0.2,
            "value": 800,
            "rarity": "Rare",
            "durability": 100,
            "max_durability": 100,
            "magical": True,
            "equipment_slot": "Neck",
            "bonuses": [
                {"type": "LearningEfficiency", "method": "Study", "bonus": 0.2},
                {"type": "LearningEfficiency", "method": "Research", "bonus": 0.2},
                {"type": "LearningEfficiency", "method": "Experimentation", "bonus": 0.2},
                {"type": "LearningEfficiency", "method": "Teaching", "bonus": 0.2},
                {"type": "LearningEfficiency", "method": "Observation", "bonus": 0.2}
            ],
            "requirements": {
                "faction_rep": {"NeutralScholars": 75}
            },
            "abilities": [
                {
                    "name": "Diplomatic Immunity",
                    "type": "passive",
                    "effect": "faction_item_immunity"
                }
            ]
        })
    },
    {
        "id": "synthesis_masters_archive",
        "name": "Synthesis Master's Archive",
        "description": "The ultimate repository of cross-faction magical knowledge, enabling the creation of entirely new magical disciplines through grand synthesis.",
        "item_type": "Educational",
        "properties": json.dumps({
            "weight": 8.0,
            "value": 5000,
            "rarity": "Legendary",
            "durability": 100,
            "max_durability": 100,
            "magical": True,
            "educational_function": "KnowledgeArchive",
            "learning_bonus": 0.6,
            "applicable_theories": [
                "theoretical_synthesis", "sympathetic_networks",
                "resonance_amplification", "light_manipulation"
            ],
            "grand_synthesis": True,
            "requirements": {
                "mental_acuity": 90,
                "resonance_sensitivity": 80,
                "faction_rep": {"NeutralScholars": 100},
                "theories": [
                    "harmonic_fundamentals", "crystal_structures", "mental_resonance",
                    "bio_resonance", "detection_arrays", "light_manipulation",
                    "resonance_amplification", "sympathetic_networks"
                ]
            }
        })
    }
]

def populate_database(db_path):
    """Populate the database with faction-specific educational items."""
    if not os.path.exists(db_path):
        print(f"Error: Database file {db_path} not found!")
        return False

    try:
        conn = sqlite3.connect(db_path)
        cursor = conn.cursor()

        # Clear existing items first
        cursor.execute("DELETE FROM items")

        # Insert new faction items
        items_inserted = 0
        for item in FACTION_ITEMS:
            cursor.execute('''
                INSERT INTO items (id, name, description, item_type, properties)
                VALUES (?, ?, ?, ?, ?)
            ''', (
                item['id'],
                item['name'],
                item['description'],
                item['item_type'],
                item['properties']
            ))
            items_inserted += 1

        conn.commit()
        conn.close()

        print(f"✅ Successfully populated database with {items_inserted} faction-specific educational items!")
        return True

    except sqlite3.Error as e:
        print(f"❌ Database error: {e}")
        return False
    except Exception as e:
        print(f"❌ Unexpected error: {e}")
        return False

def validate_balance():
    """Calculate and display faction balance statistics."""
    print("\n=== FACTION BALANCE ANALYSIS ===")

    faction_counts = {}
    faction_power = {}

    for item in FACTION_ITEMS:
        # Determine faction from item ID
        item_id = item['id']
        if any(x in item_id for x in ['council', 'academy', 'magistrates']):
            faction = 'Magisters Council'
        elif any(x in item_id for x in ['harmony', 'living', 'natures', 'spiritual']):
            faction = 'Order of Natural Harmony'
        elif any(x in item_id for x in ['efficiency', 'advanced', 'innovation', 'productivity']):
            faction = 'Industrial Consortium'
        elif any(x in item_id for x in ['forbidden', 'experimental', 'revolutionaries', 'black_market']):
            faction = 'Underground Network'
        elif any(x in item_id for x in ['diplomatic', 'universal', 'scholars', 'synthesis']):
            faction = 'Neutral Scholars'
        else:
            faction = 'Unknown'

        # Count items per faction
        faction_counts[faction] = faction_counts.get(faction, 0) + 1

        # Calculate power score
        props = json.loads(item['properties'])
        power_score = 0

        # Base power from rarity
        rarity_power = {
            'Common': 1, 'Uncommon': 2, 'Rare': 4, 'Epic': 8, 'Legendary': 16
        }
        power_score += rarity_power.get(props.get('rarity', 'Common'), 1)

        # Additional power from bonuses
        if 'bonuses' in props:
            power_score += len(props['bonuses']) * 2
        if 'abilities' in props:
            power_score += len(props['abilities']) * 3
        if 'learning_bonus' in props:
            power_score += props['learning_bonus'] * 10

        faction_power[faction] = faction_power.get(faction, 0) + power_score

    print("Items per Faction:")
    for faction, count in sorted(faction_counts.items()):
        print(f"  {faction}: {count} items")

    print("\nPower Scores per Faction:")
    powers = []
    for faction, power in sorted(faction_power.items()):
        print(f"  {faction}: {power:.1f}")
        powers.append(power)

    # Balance analysis
    if powers:
        avg_power = sum(powers) / len(powers)
        max_power = max(powers)
        min_power = min(powers)
        balance_ratio = (max_power - min_power) / avg_power if avg_power > 0 else 0

        print(f"\nBalance Statistics:")
        print(f"  Average Power: {avg_power:.1f}")
        print(f"  Power Range: {min_power:.1f} - {max_power:.1f}")
        print(f"  Balance Ratio: {balance_ratio:.2f} ({'✅ BALANCED' if balance_ratio < 0.25 else '⚠️  NEEDS BALANCING'})")

    print("================================")

def main():
    """Main function to run the database population script."""
    print("🎮 Sympathetic Resonance - Faction Items Database Population")
    print("=" * 60)

    db_path = "./content/database.db"

    # First validate the balance of our design
    validate_balance()

    # Ask for confirmation
    print(f"\nAbout to populate database: {db_path}")
    print(f"This will replace any existing items with {len(FACTION_ITEMS)} new faction-specific educational items.")

    confirm = input("Proceed? (y/N): ").lower().strip()
    if confirm != 'y':
        print("Operation cancelled.")
        return

    # Populate the database
    success = populate_database(db_path)

    if success:
        print(f"\n✅ Database population completed successfully!")
        print(f"   Total items added: {len(FACTION_ITEMS)}")
        print(f"   Database location: {os.path.abspath(db_path)}")
        print("\n🎯 Next steps:")
        print("   1. Run the game to test the new faction items")
        print("   2. Check that item bonuses work correctly")
        print("   3. Validate faction reputation requirements")
        print("   4. Test item synergies and conflicts")
    else:
        print("\n❌ Database population failed. Please check the error messages above.")

if __name__ == "__main__":
    main()