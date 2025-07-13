# StatTable

The `StatTable` module provides a table-based implementation for managing collections of statistics.

## Overview

`StatTable` is a concrete implementation of the `Statable` trait that stores statistics in a hash map structure. It provides efficient lookup and modification of statistical values.

## Key Features

- **Hash Map Storage**: Uses HashMap for efficient stat lookup
- **Statable Implementation**: Full implementation of the Statable trait
- **ModifiableStatable Implementation**: Supports direct modification of stats
- **Builder Pattern**: Includes builder methods for convenient construction
- **Memory Efficient**: Only stores non-zero statistics

## Core Methods

### Construction

- `new() -> StatTable`: Creates an empty stat table
- `of(stats: &[StatValue]) -> StatTable`: Creates a stat table from an array of stat values

### Statable Methods

- `iter() -> StatableIter`: Returns an iterator over all stat-value pairs
- `get(stat_type: &Stat) -> f32`: Returns the value for a specific stat

### ModifiableStatable Methods

- `add(stat_type: &Stat, value: f32) -> f32`: Adds a value to a specific stat
- `add_table(other: StatableIter) -> &mut Self`: Adds all stats from another statable

## Usage Examples

```rust
use aminus::model::{StatTable, Stat, Statable, ModifiableStatable};

// Create an empty stat table
let mut stats = StatTable::new();

// Add some stats
stats.add(&Stat::FlatATK, 100.0);
stats.add(&Stat::ATKPercent, 25.0);

// Create from array
let stats = StatTable::of(&[
    (Stat::FlatATK, 100.0),
    (Stat::ATKPercent, 25.0),
    (Stat::CritRate, 5.0),
]);

// Get a specific stat
let atk_bonus = stats.get(&Stat::ATKPercent); // 25.0

// Iterate over all stats
for (stat, value) in stats.iter() {
    println!("{}: {}", stat, value);
}
```

## Performance Characteristics

- **Lookup**: O(1) average case for stat retrieval
- **Insertion**: O(1) average case for adding stats
- **Memory**: Only stores non-zero statistics, memory efficient
- **Iteration**: O(n) where n is the number of non-zero stats 
