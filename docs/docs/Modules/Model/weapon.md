# Weapon

The `Weapon` module defines data structures for representing game weapons and their properties.

## Overview

This module provides the `Weapon` struct and related types that represent in-game weapons, including their base statistics, refinement level, and weapon-specific data.

## Core Types

### Weapon Struct

```rust
pub struct Weapon {
    pub name: String,
    pub level: u8,
    pub refinement: u8,
    pub base_stats: StatTable,
}
```

## Key Properties

- **name**: Weapon's name identifier
- **level**: Weapon level (1-90)
- **refinement**: Refinement level (1-5)
- **base_stats**: Base weapon statistics

## Methods

### Construction

- `new(name: String, level: u8, refinement: u8) -> Weapon`: Creates a new weapon
- `with_stats(stats: StatTable) -> Weapon`: Sets the weapon's base statistics

### Statable Implementation

The `Weapon` struct implements the `Statable` trait, providing:

- `iter() -> StatableIter`: Returns all weapon statistics
- `get(stat_type: &Stat) -> f32`: Returns the value for a specific stat

## Usage Examples

```rust
use aminus::model::{Weapon, Stat, Statable};

// Create a weapon
let weapon = Weapon::new("Wolf's Gravestone".to_string(), 90, 1);

// Get weapon attack
let weapon_atk = weapon.get(&Stat::BaseATK);

// Get all weapon stats
for (stat, value) in weapon.iter() {
    println!("{}: {}", stat, value);
}
```

## Weapon Statistics

Weapons typically provide:

- **Base ATK**: Primary attack value
- **Sub-stat**: Secondary statistic (ATK%, Crit Rate, Crit DMG, etc.)
- **Passive Effects**: Special abilities (handled separately from base stats) 