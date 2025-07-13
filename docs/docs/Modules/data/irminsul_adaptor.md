# Irminsul Adaptor

The `Irminsul Adaptor` module provides data structures and utilities for integrating with the Irminsul database.

## Overview

This module contains JSON deserialization structures and conversion utilities for transforming external game data from the Irminsul database into the library's internal format. It handles character, weapon, and artifact data.

## Core Types

### Character Data

```rust
pub struct CharacterJSON {
    pub name: String,
    pub rarity: u8,
    pub element: String,
    pub weapon: String,
    pub ascension_stat: String,
    pub base_stats: Vec<CharacterBaseStatJSON>,
}

pub struct CharacterBaseStatJSON {
    pub lvl: String,
    pub base_hp: String,
    pub base_atk: String,
    pub base_def: String,
    pub stat_type: String,
    pub stat_value: String,
    pub phase: u8,
}
```

### Weapon Data

```rust
pub struct WeaponJSON {
    pub name: String,
    pub rarity: u8,
    pub category: String,
    pub base_stats: Vec<WeaponBaseStatJSON>,
}

pub struct WeaponBaseStatJSON {
    pub level: String,
    pub base_atk: String,
    pub sub_stat_type: Option<String>,
    pub sub_stat_value: Option<String>,
    pub ascension_phase: Option<u8>,
}
```

### Artifact Data

```rust
pub struct AllArtifactMainStatJson {
    pub five_star: ArtifactMainStatJson,
    pub four_star: ArtifactMainStatJson,
    pub three_star: ArtifactMainStatJson,
    pub two_star: ArtifactMainStatJson,
    pub one_star: ArtifactMainStatJson,
}

pub struct ArtifactMainStatJson {
    pub flat_hp: Vec<f32>,
    pub flat_atk: Vec<f32>,
    pub hp_percent: Vec<f32>,
    pub atk_percent: Vec<f32>,
    pub def_percent: Vec<f32>,
    pub physical_dmg_bonus: Vec<f32>,
    pub elemental_dmg_percent: Vec<f32>,
    pub elemental_mastery: Vec<f32>,
    pub energy_recharge: Vec<f32>,
    pub crit_rate: Vec<f32>,
    pub crit_dmg: Vec<f32>,
    pub healing_bonus: Vec<f32>,
}
```

## Key Methods

### Conversion Methods

- `to_stattable() -> Result<StatTable>`: Converts JSON data to StatTable format
- `name() -> &str`: Returns the name of the data object (implements NamedJSON trait)

## Usage Examples

```rust
use aminus::data::irminsul_adaptor::*;

// Load character data from JSON
let character_json: CharacterJSON = serde_json::from_str(json_data)?;

// Convert to internal format
let base_stats = character_json.base_stats[0].to_stattable()?;

// Access character properties
let character_name = character_json.name();
let character_element = &character_json.element;
```

## Data Sources

The adaptor is designed to work with:

- **Irminsul Database**: Primary source for game data
- **JSON Files**: Local JSON files containing game data
- **API Responses**: HTTP responses from data APIs

## Error Handling

The module uses `anyhow::Result` for comprehensive error handling:

- **Parsing Errors**: Invalid JSON or malformed data
- **Conversion Errors**: Failed stat conversions or invalid values
- **Missing Data**: Required fields that are not present

## Integration

This module serves as the bridge between external data sources and the library's internal data structures, ensuring:

- **Type Safety**: Strongly typed data structures
- **Data Validation**: Proper validation of incoming data
- **Format Conversion**: Seamless conversion between formats
- **Error Recovery**: Graceful handling of data inconsistencies 