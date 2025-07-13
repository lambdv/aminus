# Stat

The `Stat` module defines the core statistics and attribute system used throughout the aminus library.

## Overview

This module contains enums and types that represent all possible statistics and attributes in the game, including basic stats, damage bonuses, elemental bonuses, and various modifiers.

## Core Types

### Stat Enum

The main `Stat` enum represents all possible statistics in the game:

```rust
pub enum Stat {
    // Base Stats
    BaseHP, FlatHP, HPPercent,
    BaseATK, FlatATK, ATKPercent,
    BaseDEF, FlatDEF, DEFPercent,
    
    // Combat Stats
    ElementalMastery, CritRate, CritDMG, EnergyRecharge,
    
    // Damage Bonuses
    DMGBonus, ElementalDMGBonus,
    PyroDMGBonus, CryoDMGBonus, GeoDMGBonus, DendroDMGBonus,
    ElectroDMGBonus, HydroDMGBonus, AnemoDMGBonus, PhysicalDMGBonus,
    
    // Attack Type Bonuses
    NormalATKDMGBonus, ChargeATKDMGBonus, PlungeATKDMGBonus,
    SkillDMGBonus, BurstDMGBonus,
    
    // Utility
    HealingBonus, None,
    
    // Hidden Stats
    ReactionBonus, DefReduction, DefIgnore,
    PyroResistanceReduction, HydroResistanceReduction,
    ElectroResistanceReduction, CryoResistanceReduction,
    AnemoResistanceReduction, GeoResistanceReduction,
    DendroResistanceReduction, PhysicalResistanceReduction
}
```

### StatValue Type

A tuple type representing a statistic with its value:

```rust
pub type StatValue = (Stat, f32);
```

## Supporting Enums

### DamageType

Represents different types of damage:

```rust
pub enum DamageType {
    Normal, Charged, Plunging, Skill, Burst, None
}
```

### Element

Represents the seven elements plus physical damage:

```rust
pub enum Element {
    Pyro, Hydro, Electro, Anemo, Geo, Dendro, Cryo, Physical, None
}
```

### BaseScaling

Defines what base stat a skill scales from:

```rust
pub enum BaseScaling {
    ATK, DEF, HP
}
```

### Amplifier

Represents elemental reaction amplifiers:

```rust
pub enum Amplifier {
    Forward,  // 2.0x multiplier
    Reverse,  // 1.5x multiplier
    None      // 1.0x multiplier
}
```

### ReactionType

Defines different elemental reaction types:

```rust
pub enum ReactionType {
    Overloaded, Superconduct, Electrocharged, Swirl,
    Shattered, Aggravate, Spread
}
```

## Key Methods

### Stat Methods

- `with_value(value: f32) -> StatValue`: Creates a StatValue tuple
- `is_elemental_dmg_bonus() -> bool`: Checks if the stat is an elemental damage bonus
- `as_str() -> &'static str`: Returns the string representation

### Amplifier Methods

- `multiplier() -> f32`: Returns the multiplier value for the amplifier

## Usage Examples

```rust
use aminus::model::stat::*;

// Create a stat with a value
let atk_bonus = Stat::ATKPercent.with_value(25.0);

// Check if a stat is elemental damage bonus
let is_elemental = Stat::PyroDMGBonus.is_elemental_dmg_bonus(); // true

// Get amplifier multiplier
let multiplier = Amplifier::Forward.multiplier(); // 2.0
```

## String Conversion

The `Stat` enum implements `FromStr` and `Display` traits, allowing conversion to and from strings:

```rust
let stat: Stat = "atkpercent".parse().unwrap();
let stat_string = stat.to_string(); // "ATKPercent"
``` 