# Aminus Python Bindings

Python bindings for the Aminus Genshin Impact damage calculation library written in Rust.

## Features

- **Fast**: Near-native Rust performance for damage calculations
- **Accurate**: Faithful implementation of Genshin Impact's damage formulas  
- **Complete**: Supports artifacts, stats, formulas, and optimization
- **Easy to use**: Clean Python API with helpful constants

## Installation

### Requirements

- Python 3.8+
- Rust (if building from source)

### Build from Source

1. Install maturin:
```bash
pip install maturin
```

2. Build and install the package:
```bash
cd bindings/py
maturin develop
```

## Quick Start

```python
import aminus_py

# Create aliases for cleaner API
StatTable = aminus_py.PyStatTable
ArtifactBuilder = aminus_py.PyArtifactBuilder
Formulas = aminus_py.PyFormulas

# Create character stats
stats = StatTable()
stats.add(3, 800)    # BASE_ATK
stats.add(5, 0.466)  # ATK_PERCENT
stats.add(10, 0.6)   # CRIT_RATE
stats.add(11, 1.2)   # CRIT_DMG

# Calculate total attack
total_atk = Formulas.total_atk(stats)
print(f"Total ATK: {total_atk}")

# Create and build artifacts
builder = ArtifactBuilder.kqm_all_5_star(5, 15, 10)  # ATK%, Pyro DMG, Crit Rate
builder.roll(11, 2, 5, 6)  # Add 6 high crit dmg rolls
artifact_stats = builder.build()

# Calculate damage
damage = Formulas.calculate_damage(
    0, 3, 0, 1,      # Pyro, Skill, ATK scaling, Vaporize
    1.0, 2.0,        # 1 instance, 200% scaling
    stats, None      # Character stats, no buffs
)
print(f"Skill damage: {damage}")
```

## API Overview

### Core Classes

- **`PyStatTable`** - Container for character/weapon/artifact stats
- **`PyArtifactBuilder`** - Builder for realistic artifact stat simulation
- **`PyFormulas`** - Static methods for damage calculations and formulas

### Stat IDs (Constants)

```python
# Core stats
BASE_ATK = 3
FLAT_ATK = 4
ATK_PERCENT = 5
CRIT_RATE = 10
CRIT_DMG = 11

# Elemental damage bonuses
PYRO_DMG_BONUS = 15
HYDRO_DMG_BONUS = 20
ELECTRO_DMG_BONUS = 19
# ... etc
```

### Element/Damage Type IDs

```python
# Elements
PYRO = 0
HYDRO = 1
ELECTRO = 2
# ... etc

# Damage types
NORMAL = 0
CHARGED = 1
SKILL = 3
BURST = 4

# Amplifiers
NONE = 0
FORWARD = 1  # Vaporize/Melt forward reaction
REVERSE = 2  # Vaporize/Melt reverse reaction
```

## Examples

### Character Build Comparison

```python
import aminus_py

StatTable = aminus_py.PyStatTable
ArtifactBuilder = aminus_py.PyArtifactBuilder
Formulas = aminus_py.PyFormulas

# Create base character
character = StatTable()
character.add(3, 311)   # Character base ATK
character.add(4, 674)   # Weapon ATK
character.add(11, 0.662) # Weapon crit dmg

# Build 1: Crit Rate circlet
build1 = ArtifactBuilder.kqm_all_5_star(5, 15, 10)  # ATK%, Pyro, Crit Rate
build1.roll(11, 2, 5, 8)  # High crit dmg rolls

# Build 2: Crit DMG circlet  
build2 = ArtifactBuilder.kqm_all_5_star(5, 15, 11)  # ATK%, Pyro, Crit DMG
build2.roll(10, 2, 5, 8)  # High crit rate rolls

# Compare damage
char1 = StatTable()
char1.add_table(character)
char1.add_table(build1.build())

char2 = StatTable()
char2.add_table(character)
char2.add_table(build2.build())

damage1 = Formulas.calculate_damage(0, 3, 0, 1, 1.0, 2.0, char1, None)
damage2 = Formulas.calculate_damage(0, 3, 0, 1, 1.0, 2.0, char2, None)

print(f"Crit Rate build: {damage1:.0f}")
print(f"Crit DMG build: {damage2:.0f}")
```

### Utility Functions

```python
# Get readable stat names
print(aminus_py.get_stat_name(10))  # "CritRate"

# Check if stat is elemental damage
print(aminus_py.is_elemental_dmg_bonus(15))  # True (Pyro DMG)

# Get artifact roll multipliers
print(aminus_py.get_roll_quality_multiplier(2))  # 0.9 (HIGH quality)
```

## Performance

The bindings provide excellent performance for:
- Real-time damage calculations
- Artifact optimization algorithms
- Bulk character comparisons

Typical performance: **>100,000 damage calculations per second** on modern hardware.

## Documentation

- See `USAGE.md` for detailed API documentation
- Check `examples/` directory for comprehensive examples
- `test_bindings.py` demonstrates all core functionality

## Contributing

1. Follow the existing code style
2. Add tests for new functionality
3. Update documentation as needed

## License

MIT License - see main project LICENSE file for details. 