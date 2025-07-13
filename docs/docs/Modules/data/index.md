# Data

The data module handles external data integration and provides adapters for importing game data from various sources.

## Overview

This module provides data structures and adapters for integrating external game data, particularly from the Irminsul database. It includes JSON deserialization structures and conversion utilities for transforming external data into the library's internal format.

## Submodules

- [Irminsul Adaptor](./irminsul_adaptor.md) - Data adapter for Irminsul database integration

## Key Features

- **External Data Integration**: Seamless integration with external game data sources
- **JSON Deserialization**: Comprehensive JSON structures for all game entities
- **Data Conversion**: Utilities for converting external data to internal formats
- **Type Safety**: Strongly typed data structures with proper error handling
- **Extensible Design**: Easy to extend for new data sources

## Data Sources

The module currently supports:

- **Irminsul Database**: Primary source for character, weapon, and artifact data
- **JSON Files**: Local JSON files for static data
- **Future Sources**: Designed to easily support additional data sources

## Usage Examples

```rust
use aminus::data::*;

// Load character data from Irminsul
let character_data = load_character_data("diluc")?;

// Convert to internal format
let character = character_data.to_character()?;

// Load weapon data
let weapon_data = load_weapon_data("wolfs_gravestone")?;
``` 