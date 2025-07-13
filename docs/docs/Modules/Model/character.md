# Character

The `Character` module defines data structures for representing game characters and their properties.

## Overview

This module provides the `Character` struct and related types that represent in-game characters, including their base statistics, level information, and character-specific data.

## Core Types

### Character Struct

```rust
pub struct Character {
    pub name: String,
    pub level: u8,
    pub ascension: u8,
    pub constellation: u8,
    pub base_stats: StatTable,
    pub weapon: Option<Weapon>,
    pub artifacts: Vec<Artifact>,
}
```

## Key Properties

- **name**: Character's name identifier
- **level**: Character level (1-90)
- **ascension**: Ascension phase (0-6)
- **constellation**: Constellation level (0-6)
- **base_stats**: Base character statistics
- **weapon**: Equipped weapon (optional)
- **artifacts**: List of equipped artifacts

## Methods

### Construction

- `new(name: String, level: u8, ascension: u8) -> Character`: Creates a new character
- `with_weapon(weapon: Weapon) -> Character`: Sets the character's weapon
- `with_artifacts(artifacts: Vec<Artifact>) -> Character`: Sets the character's artifacts

### Statable Implementation

The `Character` struct implements the `Statable` trait, providing:

- `iter() -> StatableIter`: Returns all stats from character, weapon, and artifacts
- `get(stat_type: &Stat) -> f32`: Returns the total value for a specific stat

## Usage Examples

```rust
use aminus::model::{Character, Stat, Statable};

// Create a character
let mut character = Character::new("Diluc".to_string(), 90, 6);

// Add a weapon
character = character.with_weapon(weapon);

// Add artifacts
character = character.with_artifacts(artifacts);

// Get total attack
let total_atk = character.get(&Stat::FlatATK);

// Get all stats
for (stat, value) in character.iter() {
    println!("{}: {}", stat, value);
}
```

## Stat Calculation

The character's total stats are calculated by combining:

1. **Base Character Stats**: Level and ascension-based statistics
2. **Weapon Stats**: Statistics from the equipped weapon
3. **Artifact Stats**: Statistics from all equipped artifacts

The `Statable` implementation automatically sums all these sources when retrieving statistics. 