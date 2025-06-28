#!/usr/bin/env python3
"""
Improved usage example for Aminus Python bindings using the cleaner API
"""

# Import the Rust module and create convenient aliases
import aminus_py

# Create aliases for cleaner API
StatTable = aminus_py.PyStatTable
ArtifactBuilder = aminus_py.PyArtifactBuilder
Formulas = aminus_py.PyFormulas

# Import utility functions
get_stat_name = aminus_py.get_stat_name
is_elemental_dmg_bonus = aminus_py.is_elemental_dmg_bonus
get_roll_quality_multiplier = aminus_py.get_roll_quality_multiplier

# Constants for easier usage
class Stats:
    BASE_HP = 0
    FLAT_HP = 1
    HP_PERCENT = 2
    BASE_ATK = 3
    FLAT_ATK = 4
    ATK_PERCENT = 5
    BASE_DEF = 6
    FLAT_DEF = 7
    DEF_PERCENT = 8
    ELEMENTAL_MASTERY = 9
    CRIT_RATE = 10
    CRIT_DMG = 11
    ENERGY_RECHARGE = 12
    DMG_BONUS = 13
    ELEMENTAL_DMG_BONUS = 14
    PYRO_DMG_BONUS = 15
    CRYO_DMG_BONUS = 16
    GEO_DMG_BONUS = 17
    DENDRO_DMG_BONUS = 18
    ELECTRO_DMG_BONUS = 19
    HYDRO_DMG_BONUS = 20
    ANEMO_DMG_BONUS = 21
    PHYSICAL_DMG_BONUS = 22
    NORMAL_ATK_DMG_BONUS = 23
    CHARGE_ATK_DMG_BONUS = 24
    PLUNGE_ATK_DMG_BONUS = 25
    SKILL_DMG_BONUS = 26
    BURST_DMG_BONUS = 27
    HEALING_BONUS = 28
    NONE = 29

class Elements:
    PYRO = 0
    HYDRO = 1
    ELECTRO = 2
    CRYO = 3
    ANEMO = 4
    GEO = 5
    DENDRO = 6
    PHYSICAL = 7

class DamageTypes:
    NORMAL = 0
    CHARGED = 1
    PLUNGING = 2
    SKILL = 3
    BURST = 4

class BaseScaling:
    ATK = 0
    DEF = 1
    HP = 2

class Amplifiers:
    NONE = 0
    FORWARD = 1
    REVERSE = 2

class RollQuality:
    LOW = 0
    MID = 1
    HIGH = 2
    MAX = 3
    AVG = 4

def main():
    print("=== Improved Aminus Python Bindings Example ===\n")
    
    # 1. Create a stat table for a character using constants
    print("1. Creating character stats...")
    character_stats = StatTable()
    
    # Add base stats using readable constants
    character_stats.add(Stats.BASE_ATK, 311)    # Character base ATK
    character_stats.add(Stats.BASE_HP, 12981)   # Character base HP
    character_stats.add(Stats.BASE_DEF, 751)    # Character base DEF
    
    print(f"Character base stats: {character_stats}")
    
    # 2. Create weapon stats
    print("\n2. Adding weapon stats...")
    character_stats.add(Stats.FLAT_ATK, 674)   # 5* weapon ATK
    character_stats.add(Stats.CRIT_DMG, 0.662) # Crit DMG substat
    
    # 3. Create artifacts using the builder with constants
    print("\n3. Building artifacts...")
    artifact_builder = ArtifactBuilder.kqm_all_5_star(
        Stats.ATK_PERCENT,     # Sands: ATK%
        Stats.PYRO_DMG_BONUS,  # Goblet: Pyro DMG
        Stats.CRIT_RATE        # Circlet: Crit Rate
    )
    
    # Add some realistic substat rolls using quality constants
    artifact_builder.roll(Stats.CRIT_DMG, RollQuality.HIGH, 5, 6)
    artifact_builder.roll(Stats.CRIT_RATE, RollQuality.MID, 5, 4)
    artifact_builder.roll(Stats.ATK_PERCENT, RollQuality.AVG, 5, 8)
    artifact_builder.roll(Stats.FLAT_ATK, RollQuality.LOW, 5, 2)
    
    artifact_stats = artifact_builder.build()
    print(f"Artifact rolls: {artifact_builder.current_rolls()}/{artifact_builder.max_rolls()}")
    print(f"Artifact stats: {artifact_stats}")
    
    # 4. Combine all stats
    print("\n4. Calculating final stats...")
    character_stats.add_table(artifact_stats)
    
    # 5. Calculate derived stats
    print("\n5. Derived calculations...")
    total_atk = Formulas.total_atk(character_stats)
    total_hp = Formulas.total_hp(character_stats)
    total_def = Formulas.total_def(character_stats)
    crit_multiplier = Formulas.avg_crit_multiplier(character_stats)
    
    print(f"Total ATK: {total_atk:.0f}")
    print(f"Total HP: {total_hp:.0f}")
    print(f"Total DEF: {total_def:.0f}")
    print(f"Average Crit Multiplier: {crit_multiplier:.2f}")
    
    # Show individual stats with readable names
    print(f"Crit Rate: {character_stats.get(Stats.CRIT_RATE):.1%}")
    print(f"Crit DMG: {character_stats.get(Stats.CRIT_DMG):.1%}")
    print(f"ATK%: {character_stats.get(Stats.ATK_PERCENT):.1%}")
    print(f"Pyro DMG%: {character_stats.get(Stats.PYRO_DMG_BONUS):.1%}")
    
    # 6. Calculate damage using constants
    print("\n6. Damage calculation example...")
    damage = Formulas.calculate_damage(
        element_id=Elements.PYRO,
        damage_type_id=DamageTypes.SKILL,
        scaling_id=BaseScaling.ATK,
        amplifier_id=Amplifiers.FORWARD,  # Vaporize
        instances=1.0,
        motion_value=2.0,  # 200% ATK scaling
        character=character_stats,
        buffs=None
    )
    print(f"Skill damage (with Vaporize): {damage:.0f}")
    
    # Test different amplifiers
    no_reaction_damage = Formulas.calculate_damage(
        element_id=Elements.PYRO,
        damage_type_id=DamageTypes.SKILL,
        scaling_id=BaseScaling.ATK,
        amplifier_id=Amplifiers.NONE,
        instances=1.0,
        motion_value=2.0,
        character=character_stats,
        buffs=None
    )
    print(f"Skill damage (no reaction): {no_reaction_damage:.0f}")
    
    vaporize_multiplier = damage / no_reaction_damage
    print(f"Vaporize multiplier: {vaporize_multiplier:.2f}x")
    
    # 7. Utility functions with constants
    print("\n7. Utility functions...")
    print(f"Crit Rate stat name: {get_stat_name(Stats.CRIT_RATE)}")
    print(f"Pyro DMG is elemental bonus: {is_elemental_dmg_bonus(Stats.PYRO_DMG_BONUS)}")
    print(f"ATK% is elemental bonus: {is_elemental_dmg_bonus(Stats.ATK_PERCENT)}")
    print(f"High roll multiplier: {get_roll_quality_multiplier(RollQuality.HIGH):.2f}")
    print(f"Max roll multiplier: {get_roll_quality_multiplier(RollQuality.MAX):.2f}")

if __name__ == "__main__":
    main() 