# Aminus Python Bindings Usage Guide

This guide demonstrates how to use the Aminus Python bindings for Genshin Impact damage calculations.

## Installation

1. Install maturin (build tool for Python-Rust projects):
```bash
pip install maturin
```

2. Build and install the bindings:
```bash
cd bindings/py
maturin develop
```

## Basic Usage

### Import the Module

```python
import aminus_py

# Create convenient aliases
StatTable = aminus_py.PyStatTable
ArtifactBuilder = aminus_py.PyArtifactBuilder  
Formulas = aminus_py.PyFormulas
```

### Stat Constants

For easier usage, define stat constants:

```python
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
    # ... more stats
```

## Core Components

### 1. StatTable

The `StatTable` class manages character, weapon, and artifact stats.

```python
# Create empty stat table
stats = StatTable()

# Add stats
stats.add(Stats.BASE_ATK, 800)
stats.add(Stats.ATK_PERCENT, 0.466)
stats.add(Stats.CRIT_RATE, 0.6)

# Get stat values
atk_percent = stats.get(Stats.ATK_PERCENT)
print(f"ATK%: {atk_percent:.1%}")

# Create from list of tuples
stats2 = StatTable.of([
    (Stats.BASE_ATK, 800),
    (Stats.FLAT_ATK, 200),
    (Stats.CRIT_RATE, 0.5)
])

# Combine stat tables
stats.add_table(stats2)
```

### 2. ArtifactBuilder

Build artifact stats with realistic substat rolls.

```python
# Create KQM standard 5-star artifacts
builder = ArtifactBuilder.kqm_all_5_star(
    Stats.ATK_PERCENT,     # Sands main stat
    Stats.PYRO_DMG_BONUS,  # Goblet main stat
    Stats.CRIT_RATE        # Circlet main stat
)

# Add substat rolls
class RollQuality:
    LOW = 0
    MID = 1
    HIGH = 2
    MAX = 3
    AVG = 4

builder.roll(Stats.CRIT_DMG, RollQuality.HIGH, 5, 6)  # 6 high crit dmg rolls
builder.roll(Stats.CRIT_RATE, RollQuality.MID, 5, 4)  # 4 mid crit rate rolls

# Get final artifact stats
artifact_stats = builder.build()
print(f"Rolls used: {builder.current_rolls()}/{builder.max_rolls()}")
```

### 3. Formulas

Calculate damage and derived stats.

```python
# Calculate total stats
total_atk = Formulas.total_atk(character_stats)
total_hp = Formulas.total_hp(character_stats)
avg_crit_mult = Formulas.avg_crit_multiplier(character_stats)

# Calculate damage
class Elements:
    PYRO = 0
    HYDRO = 1
    # ... more elements

class DamageTypes:
    NORMAL = 0
    CHARGED = 1
    SKILL = 3
    BURST = 4

class Amplifiers:
    NONE = 0
    FORWARD = 1  # Vaporize/Melt (forward)
    REVERSE = 2  # Vaporize/Melt (reverse)

damage = Formulas.calculate_damage(
    element_id=Elements.PYRO,
    damage_type_id=DamageTypes.SKILL,
    scaling_id=0,  # ATK scaling
    amplifier_id=Amplifiers.FORWARD,  # Vaporize
    instances=1.0,
    motion_value=2.0,  # 200% ATK scaling
    character=character_stats,
    buffs=None
)
```

## Complete Example

```python
import aminus_py

# Aliases and constants
StatTable = aminus_py.PyStatTable
ArtifactBuilder = aminus_py.PyArtifactBuilder
Formulas = aminus_py.PyFormulas

class Stats:
    BASE_ATK = 3
    FLAT_ATK = 4
    ATK_PERCENT = 5
    CRIT_RATE = 10
    CRIT_DMG = 11
    PYRO_DMG_BONUS = 15

class RollQuality:
    HIGH = 2
    AVG = 4

def calculate_character_damage():
    # 1. Create character base stats
    character = StatTable()
    character.add(Stats.BASE_ATK, 311)  # Character base
    character.add(Stats.FLAT_ATK, 674)  # Weapon ATK
    character.add(Stats.CRIT_DMG, 0.662)  # Weapon substat
    
    # 2. Create artifacts
    artifacts = ArtifactBuilder.kqm_all_5_star(
        Stats.ATK_PERCENT,
        Stats.PYRO_DMG_BONUS, 
        Stats.CRIT_RATE
    )
    artifacts.roll(Stats.CRIT_DMG, RollQuality.HIGH, 5, 8)
    artifacts.roll(Stats.ATK_PERCENT, RollQuality.AVG, 5, 6)
    
    # 3. Combine stats
    character.add_table(artifacts.build())
    
    # 4. Calculate damage
    total_atk = Formulas.total_atk(character)
    damage = Formulas.calculate_damage(
        0, 3, 0, 1,  # Pyro, Skill, ATK, Vaporize
        1.0, 2.0,    # 1 instance, 200% scaling
        character, None
    )
    
    print(f"Total ATK: {total_atk:.0f}")
    print(f"Skill damage: {damage:.0f}")

calculate_character_damage()
```

## Utility Functions

```python
# Get stat names
stat_name = aminus_py.get_stat_name(Stats.CRIT_RATE)
print(f"Stat name: {stat_name}")  # "CritRate"

# Check if stat is elemental damage bonus
is_elemental = aminus_py.is_elemental_dmg_bonus(Stats.PYRO_DMG_BONUS)
print(f"Is elemental: {is_elemental}")  # True

# Get roll quality multipliers
high_mult = aminus_py.get_roll_quality_multiplier(RollQuality.HIGH)
print(f"High roll multiplier: {high_mult}")  # 0.9
```

## Performance

The Rust backend provides excellent performance for:
- Complex damage calculations with multiple buffs
- Artifact optimization across many combinations
- Real-time damage updates in interactive applications

For bulk calculations, the Python bindings maintain near-native Rust performance while providing Python's ease of use.

## Examples

See the `examples/` directory for more comprehensive usage examples:
- `test_bindings.py` - Basic functionality test
- `improved_usage.py` - Complete example with constants
- `artifact_optimization.py` - Comparing different builds 