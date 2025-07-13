# Statable

The `Statable` module defines the core trait system for objects that can have statistics and attributes.

## Overview

The `Statable` trait is the foundation for all objects that can hold and manage statistics in the aminus library. It provides a unified interface for accessing and manipulating statistical data.

## Core Types

### StatValue

A tuple type representing a statistic with its value:

```rust
pub type StatValue = (Stat, f32);
```

### StatableIter

A boxed iterator type for iterating over stat-value pairs:

```rust
pub type StatableIter<'a> = Box<dyn Iterator<Item=StatValue>+'a>;
```

## Traits

### Statable

The main trait that defines the interface for objects with statistics:

```rust
pub trait Statable {
    /// Gets all stats from a statable as an iterator with stat-value pair tuples
    fn iter(&self) -> StatableIter;
    
    /// Returns value mapped to stat_type if it's stored, else returns 0
    fn get(&self, stat_type: &Stat) -> f32 {
        self.iter()
            .filter(|x| x.0 == *stat_type)
            .map(|x| x.1)
            .sum()
    }
    
    /// Ternary operator that sums 2 statables together and returns the results
    fn chain(&self, other: Box<dyn Statable>) -> Box<dyn Statable> {
        let mut res = StatTable::new();
        res.add_table(self.iter());
        res.add_table(other.iter());
        Box::new(res)
    }
}
```

### ModifiableStatable

A trait for statable objects that can be directly modified:

```rust
pub trait ModifiableStatable: Statable {
    /// Add amount to stat
    fn add(&mut self, stat_type: &Stat, value: f32) -> f32;
    
    /// Add all stats from other statable to self, returns self for fluent interface
    fn add_table(&mut self, other: StatableIter) -> &mut Self {
        other.for_each(|(k, v)| {
            self.add(&k, v);
        });
        self
    }
}
```

## Key Methods

### Statable Methods

- `iter() -> StatableIter`: Returns an iterator over all stat-value pairs
- `get(stat_type: &Stat) -> f32`: Returns the value for a specific stat, or 0 if not found
- `chain(other: Box<dyn Statable>) -> Box<dyn Statable>`: Combines two statables into a new one

### ModifiableStatable Methods

- `add(stat_type: &Stat, value: f32) -> f32`: Adds a value to a specific stat
- `add_table(other: StatableIter) -> &mut Self`: Adds all stats from another statable

## Usage Examples

```rust
use aminus::model::{Statable, ModifiableStatable, StatTable, Stat};

// Create a statable object
let mut stats = StatTable::new();
stats.add(&Stat::FlatATK, 100.0);
stats.add(&Stat::ATKPercent, 25.0);

// Get a specific stat value
let atk_bonus = stats.get(&Stat::ATKPercent); // 25.0

// Chain two statables together
let other_stats = StatTable::of(&vec![(Stat::FlatATK, 50.0)]);
let combined = stats.chain(Box::new(other_stats));
let total_atk = combined.get(&Stat::FlatATK); // 150.0

// Iterate over all stats
for (stat, value) in stats.iter() {
    println!("{}: {}", stat, value);
}
```

## Design Philosophy

The `Statable` trait provides a unified interface that allows different types of objects (characters, artifacts, weapons, etc.) to be treated uniformly when dealing with statistics. This enables:

- **Composition**: Multiple statables can be combined using the `chain` method
- **Polymorphism**: Any object implementing `Statable` can be used in generic contexts
- **Flexibility**: The trait allows for both immutable and mutable access patterns
- **Extensibility**: New statable types can easily implement the trait interface 