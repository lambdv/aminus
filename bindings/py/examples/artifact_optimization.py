#!/usr/bin/env python3
"""
Artifact optimization example for Aminus Python bindings
Shows how to compare different artifact builds
"""

import aminus_py as aminus

def create_base_character():
    """Create a base character with weapon stats"""
    stats = aminus.StatTable()
    
    # Hu Tao base stats (example)
    stats.add(aminus.Stats.BASE_HP, 15552)
    stats.add(aminus.Stats.BASE_ATK, 106)
    stats.add(aminus.Stats.BASE_DEF, 876)
    
    # Staff of Homa weapon
    stats.add(aminus.Stats.FLAT_ATK, 608)
    stats.add(aminus.Stats.CRIT_DMG, 0.662)
    
    return stats

def build_crit_rate_circlet():
    """Build with Crit Rate circlet"""
    builder = aminus.ArtifactBuilder.kqm_all_5_star(
        aminus.Stats.HP_PERCENT,      # Sands: HP%
        aminus.Stats.PYRO_DMG_BONUS,  # Goblet: Pyro DMG
        aminus.Stats.CRIT_RATE        # Circlet: Crit Rate
    )
    
    # Simulate good substat rolls
    builder.roll(aminus.Stats.CRIT_DMG, aminus.RollQuality.HIGH, 5, 8)
    builder.roll(aminus.Stats.CRIT_RATE, aminus.RollQuality.MID, 5, 3)
    builder.roll(aminus.Stats.HP_PERCENT, aminus.RollQuality.AVG, 5, 6)
    builder.roll(aminus.Stats.FLAT_HP, aminus.RollQuality.MID, 5, 1)
    
    return builder

def build_crit_dmg_circlet():
    """Build with Crit DMG circlet"""
    builder = aminus.ArtifactBuilder.kqm_all_5_star(
        aminus.Stats.HP_PERCENT,      # Sands: HP%
        aminus.Stats.PYRO_DMG_BONUS,  # Goblet: Pyro DMG
        aminus.Stats.CRIT_DMG         # Circlet: Crit DMG
    )
    
    # Simulate good substat rolls
    builder.roll(aminus.Stats.CRIT_RATE, aminus.RollQuality.HIGH, 5, 8)
    builder.roll(aminus.Stats.CRIT_DMG, aminus.RollQuality.MID, 5, 3)
    builder.roll(aminus.Stats.HP_PERCENT, aminus.RollQuality.AVG, 5, 6)
    builder.roll(aminus.Stats.FLAT_HP, aminus.RollQuality.MID, 5, 1)
    
    return builder

def calculate_dps(character_stats):
    """Calculate approximate DPS for comparison"""
    # Hu Tao E skill enhances normal attacks
    charged_attack_damage = aminus.Formulas.calculate_damage(
        element_id=aminus.Elements.PYRO,
        damage_type_id=aminus.DamageTypes.CHARGED,
        scaling_id=aminus.BaseScaling.ATK,
        amplifier_id=aminus.Amplifiers.FORWARD,  # Vaporize
        instances=1.0,
        motion_value=2.426,  # Hu Tao CA scaling
        character=character_stats,
        buffs=None
    )
    
    # Assume 1 CA per second for DPS calculation
    return charged_attack_damage

def main():
    print("=== Artifact Optimization Example ===\n")
    
    # Create base character
    base_stats = create_base_character()
    
    # Build 1: Crit Rate circlet
    print("Build 1: Crit Rate Circlet")
    cr_builder = build_crit_rate_circlet()
    cr_artifacts = cr_builder.build()
    
    cr_total = aminus.StatTable()
    cr_total.add_table(base_stats)
    cr_total.add_table(cr_artifacts)
    
    cr_atk = aminus.Formulas.total_atk(cr_total)
    cr_hp = aminus.Formulas.total_hp(cr_total)
    cr_crit_mult = aminus.Formulas.avg_crit_multiplier(cr_total)
    cr_dps = calculate_dps(cr_total)
    
    print(f"  Total ATK: {cr_atk:.0f}")
    print(f"  Total HP: {cr_hp:.0f}")
    print(f"  Crit Rate: {cr_total.get(aminus.Stats.CRIT_RATE):.1%}")
    print(f"  Crit DMG: {cr_total.get(aminus.Stats.CRIT_DMG):.1%}")
    print(f"  Avg Crit Mult: {cr_crit_mult:.2f}")
    print(f"  CA Damage: {cr_dps:.0f}")
    
    # Build 2: Crit DMG circlet
    print("\nBuild 2: Crit DMG Circlet")
    cd_builder = build_crit_dmg_circlet()
    cd_artifacts = cd_builder.build()
    
    cd_total = aminus.StatTable()
    cd_total.add_table(base_stats)
    cd_total.add_table(cd_artifacts)
    
    cd_atk = aminus.Formulas.total_atk(cd_total)
    cd_hp = aminus.Formulas.total_hp(cd_total)
    cd_crit_mult = aminus.Formulas.avg_crit_multiplier(cd_total)
    cd_dps = calculate_dps(cd_total)
    
    print(f"  Total ATK: {cd_atk:.0f}")  
    print(f"  Total HP: {cd_hp:.0f}")
    print(f"  Crit Rate: {cd_total.get(aminus.Stats.CRIT_RATE):.1%}")
    print(f"  Crit DMG: {cd_total.get(aminus.Stats.CRIT_DMG):.1%}")
    print(f"  Avg Crit Mult: {cd_crit_mult:.2f}")
    print(f"  CA Damage: {cd_dps:.0f}")
    
    # Comparison
    print("\n=== Comparison ===")
    print(f"Crit Rate build damage: {cr_dps:.0f}")
    print(f"Crit DMG build damage: {cd_dps:.0f}")
    
    if cr_dps > cd_dps:
        improvement = ((cr_dps - cd_dps) / cd_dps) * 100
        print(f"Crit Rate build is {improvement:.1f}% better")
    else:
        improvement = ((cd_dps - cr_dps) / cr_dps) * 100
        print(f"Crit DMG build is {improvement:.1f}% better")
    
    # Show artifact roll efficiency
    print(f"\nArtifact Efficiency:")
    print(f"Crit Rate build: {cr_builder.current_rolls()}/{cr_builder.max_rolls()} rolls used")
    print(f"Crit DMG build: {cd_builder.current_rolls()}/{cd_builder.max_rolls()} rolls used")

if __name__ == "__main__":
    main() 