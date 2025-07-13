# Functions

The functions module provides mathematical functions, formulas, optimization algorithms, and factory patterns for creating game objects.

## Overview

This module contains the computational logic for damage calculations, optimization algorithms, and various mathematical formulas used in the game mechanics. It also provides factory patterns for creating complex game objects.

## Submodules

- [Formulas](./formulas.md) - Mathematical formulas and calculations
- [Damage Function](./dmg_function.md) - Damage calculation functions and logic
- [Optimizers](./optimizers.md) - Optimization algorithms for artifact and stat optimization
- [Factories](./factories.md) - Factory patterns for creating game objects
- [Stat Factory](./stat_factory.md) - Specialized factory for creating statistics

## Architecture

The functions module is organized around several key concepts:

1. **Mathematical Foundation**: `Formulas` provides the basic mathematical operations and formulas
2. **Damage Calculation**: `Damage Function` implements the core damage calculation logic
3. **Optimization**: `Optimizers` provides algorithms for finding optimal configurations
4. **Object Creation**: `Factories` and `Stat Factory` provide patterns for creating complex objects

## Key Features

- **Damage Calculations**: Comprehensive damage calculation system supporting all damage types and elements
- **Optimization Algorithms**: Advanced optimization for artifact sets and stat distributions
- **Factory Patterns**: Clean object creation with builder patterns
- **Mathematical Formulas**: Game-accurate formulas for all calculations
- **Extensible Design**: Modular architecture allowing easy extension and modification

## Usage Examples

```rust
use aminus::functions::*;

// Calculate damage
let damage = calculate_damage(&character, &target, &skill);

// Optimize artifacts
let optimal_artifacts = optimize_artifacts(&character, &constraints);

// Create stats using factory
let stats = StatFactory::create_attack_stats(100.0, 25.0);
``` 