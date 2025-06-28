#!/usr/bin/env python3
"""
Basic usage example for Aminus Python bindings
"""

import aminus_py as aminus

def main():
    print("=== Aminus Python Bindings Example ===\n")
    
    # 1. Create a stat table for a character
    print("1. Creating character stats...")
    character_stats = aminus.StatTable()
    
    # Add base stats (example values for a typical DPS character)
    character_stats.add(aminus.Stats.BASE_ATK, 311)    # Character base ATK
    character_stats.add(aminus.Stats.BASE_HP, 12981)   # Character base HP
    character_stats.add(aminus.Stats.BASE_DEF, 751)    # Character base DEF
    
    print(f"Character base stats: {character_stats}")
    
    # 2. Create weapon stats
    print("\n2. Adding weapon stats...")
    character_stats.add(aminus.Stats.FLAT_ATK, 674)   # 5* weapon ATK
    character_stats.add(aminus.Stats.CRIT_DMG, 0.662) # Crit DMG substat
    
    # 3. Create artifacts using the builder
    print("\n3. Building artifacts...")
    artifact_builder = aminus.ArtifactBuilder.kqm_all_5_star(
        aminus.Stats.ATK_PERCENT,     # Sands: ATK%
        aminus.Stats.PYRO_DMG_BONUS,  # Goblet: Pyro DMG
        aminus.Stats.CRIT_RATE        # Circlet: Crit Rate
    )
    
    # Add some realistic substat rolls
    artifact_builder.roll(aminus.Stats.CRIT_DMG, aminus.RollQuality.HIGH, 5, 6)
    artifact_builder.roll(aminus.Stats.CRIT_RATE, aminus.RollQuality.MID, 5, 4)
    artifact_builder.roll(aminus.Stats.ATK_PERCENT, aminus.RollQuality.AVG, 5, 8)
    artifact_builder.roll(aminus.Stats.FLAT_ATK, aminus.RollQuality.LOW, 5, 2)
    
    artifact_stats = artifact_builder.build()
    print(f"Artifact rolls: {artifact_builder.current_rolls()}/{artifact_builder.max_rolls()}")
    print(f"Artifact stats: {artifact_stats}")
    
    # 4. Combine all stats
    print("\n4. Calculating final stats...")
    character_stats.add_table(artifact_stats)
    
    # 5. Calculate derived stats
    print("\n5. Derived calculations...")
    total_atk = aminus.Formulas.total_atk(character_stats)
    total_hp = aminus.Formulas.total_hp(character_stats)
    total_def = aminus.Formulas.total_def(character_stats)
    crit_multiplier = aminus.Formulas.avg_crit_multiplier(character_stats)
    
    print(f"Total ATK: {total_atk:.0f}")
    print(f"Total HP: {total_hp:.0f}")
    print(f"Total DEF: {total_def:.0f}")
    print(f"Average Crit Multiplier: {crit_multiplier:.2f}")
    
    # 6. Calculate damage
    print("\n6. Damage calculation example...")
    damage = aminus.Formulas.calculate_damage(
        element_id=aminus.Elements.PYRO,
        damage_type_id=aminus.DamageTypes.SKILL,
        scaling_id=aminus.BaseScaling.ATK,
        amplifier_id=aminus.Amplifiers.FORWARD,  # Vaporize
        instances=1.0,
        motion_value=2.0,  # 200% ATK scaling
        character=character_stats,
        buffs=None
    )
    print(f"Skill damage (with Vaporize): {damage:.0f}")
    
    # 7. Utility functions
    print("\n7. Utility functions...")
    print(f"Stat name for ID 5: {aminus.get_stat_name(5)}")
    print(f"Is Pyro DMG an elemental bonus? {aminus.is_elemental_dmg_bonus(aminus.Stats.PYRO_DMG_BONUS)}")
    print(f"High roll quality multiplier: {aminus.get_roll_quality_multiplier(aminus.RollQuality.HIGH)}")

if __name__ == "__main__":
    main() 