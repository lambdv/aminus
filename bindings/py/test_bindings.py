#!/usr/bin/env python3
"""
Simple test script for Aminus Python bindings
"""

import aminus_py

def test_stat_table():
    """Test StatTable functionality"""
    print("=== Testing StatTable ===")
    
    # Create a stat table
    stats = aminus_py.PyStatTable()
    
    # Add some stats
    stats.add(5, 0.466)  # ATK_PERCENT = 5
    stats.add(10, 0.6)   # CRIT_RATE = 10  
    stats.add(11, 1.2)   # CRIT_DMG = 11
    
    print(f"ATK%: {stats.get(5):.1%}")
    print(f"Crit Rate: {stats.get(10):.1%}")
    print(f"Crit DMG: {stats.get(11):.1%}")
    print(f"StatTable: {stats}")
    
    # Test static creation
    stats2 = aminus_py.PyStatTable.of([(3, 800), (4, 200)])  # BASE_ATK, FLAT_ATK
    print(f"Stats2: {stats2}")

def test_formulas():
    """Test damage formulas"""
    print("\n=== Testing Formulas ===")
    
    # Create character stats
    character = aminus_py.PyStatTable()
    character.add(3, 800)    # BASE_ATK
    character.add(4, 200)    # FLAT_ATK
    character.add(5, 0.466)  # ATK_PERCENT
    character.add(10, 0.6)   # CRIT_RATE
    character.add(11, 1.2)   # CRIT_DMG
    
    # Test formula functions
    total_atk = aminus_py.PyFormulas.total_atk(character)
    avg_crit = aminus_py.PyFormulas.avg_crit_multiplier(character)
    
    print(f"Total ATK: {total_atk:.0f}")
    print(f"Average Crit Multiplier: {avg_crit:.2f}")
    
    # Test damage calculation
    damage = aminus_py.PyFormulas.calculate_damage(
        0,    # Element: Pyro
        3,    # Damage Type: Skill
        0,    # Scaling: ATK
        1,    # Amplifier: Forward (Vaporize)
        1.0,  # Instances
        2.0,  # Motion Value (200% ATK)
        character,
        None  # No buffs
    )
    print(f"Skill damage: {damage:.0f}")

def test_artifact_builder():
    """Test artifact builder"""
    print("\n=== Testing Artifact Builder ===")
    
    # Create KQM preset
    builder = aminus_py.PyArtifactBuilder.kqm_all_5_star(5, 15, 10)  # ATK%, Pyro DMG, Crit Rate
    print(f"Initial builder: {builder}")
    
    # Add some rolls
    builder.roll(11, 2, 5, 4)  # CRIT_DMG, HIGH quality, 5-star, 4 rolls
    builder.roll(10, 1, 5, 2)  # CRIT_RATE, MID quality, 5-star, 2 rolls
    
    artifact_stats = builder.build()
    print(f"Artifact stats: {artifact_stats}")
    print(f"Rolls used: {builder.current_rolls()}/{builder.max_rolls()}")

def test_utilities():
    """Test utility functions"""
    print("\n=== Testing Utilities ===")
    
    # Test stat name lookup
    print(f"Stat 5 name: {aminus_py.get_stat_name(5)}")
    print(f"Stat 15 name: {aminus_py.get_stat_name(15)}")
    
    # Test elemental bonus check
    print(f"Is stat 15 elemental DMG bonus? {aminus_py.is_elemental_dmg_bonus(15)}")
    print(f"Is stat 5 elemental DMG bonus? {aminus_py.is_elemental_dmg_bonus(5)}")
    
    # Test roll quality
    print(f"High roll quality multiplier: {aminus_py.get_roll_quality_multiplier(2)}")

def main():
    print("=== Aminus Python Bindings Test ===")
    
    try:
        test_stat_table()
        test_formulas()
        test_artifact_builder()
        test_utilities()
        print("\n✅ All tests passed!")
    except Exception as e:
        print(f"\n❌ Test failed: {e}")
        import traceback
        traceback.print_exc()

if __name__ == "__main__":
    main() 