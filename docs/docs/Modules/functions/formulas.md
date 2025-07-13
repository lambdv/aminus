# Formulas

The `Formulas` module provides mathematical formulas and calculations used throughout the aminus library.

## Overview

This module contains the core mathematical formulas for damage calculations, stat computations, and various game mechanics. These formulas are based on the actual game mechanics and provide accurate calculations.

## Key Functions

### Damage Calculation

- `calculate_damage_multiplier(base_scaling: BaseScaling, talent_multiplier: f32, character_stats: &dyn Statable) -> f32`: Calculates damage multiplier based on base scaling and talent multiplier
- `calculate_critical_damage(crit_rate: f32, crit_dmg: f32, base_damage: f32) -> f32`: Calculates expected critical damage
- `calculate_elemental_reaction_damage(base_damage: f32, elemental_mastery: f32, reaction_type: ReactionType) -> f32`: Calculates elemental reaction damage

### Stat Calculations

- `calculate_total_atk(base_atk: f32, flat_atk: f32, atk_percent: f32) -> f32`: Calculates total attack value
- `calculate_total_hp(base_hp: f32, flat_hp: f32, hp_percent: f32) -> f32`: Calculates total HP value
- `calculate_total_def(base_def: f32, flat_def: f32, def_percent: f32) -> f32`: Calculates total DEF value

### Utility Calculations

- `calculate_damage_reduction(defense: f32, level: u8) -> f32`: Calculates damage reduction from defense
- `calculate_resistance_multiplier(resistance: f32) -> f32`: Calculates resistance multiplier
- `calculate_amplifier_multiplier(amplifier: Amplifier) -> f32`: Calculates elemental reaction amplifier

## Usage Examples

```rust
use aminus::functions::formulas::*;
use aminus::model::{Stat, Statable, BaseScaling};

// Calculate damage multiplier
let multiplier = calculate_damage_multiplier(
    BaseScaling::ATK,
    150.0,  // 150% talent multiplier
    &character_stats
);

// Calculate critical damage
let crit_damage = calculate_critical_damage(0.6, 1.5, 1000.0);

// Calculate elemental reaction damage
let reaction_damage = calculate_elemental_reaction_damage(
    1000.0,
    200.0,  // Elemental Mastery
    ReactionType::Vaporize
);
```

## Formula Accuracy

All formulas in this module are based on:

- **Official Game Data**: Formulas derived from official game mechanics
- **Community Testing**: Validated against community testing and data mining
- **Mathematical Precision**: High-precision floating-point calculations
- **Edge Case Handling**: Proper handling of edge cases and boundary conditions 