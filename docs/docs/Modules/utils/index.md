# Utils

The utils module contains utility functions and helper modules that support the other modules in the aminus library.

## Overview

This module provides common utility functions that are used across the library, including string processing, percentage parsing, and testing utilities. These utilities help maintain consistency and reduce code duplication throughout the codebase.

## Submodules

- [Percentage](./percentage.md) - Percentage parsing and conversion utilities
- [Standardize](./standardize.md) - String standardization and normalization
- [Testing](./testing.md) - Testing utilities and macros

## Key Features

- **String Processing**: Utilities for string normalization and comparison
- **Percentage Handling**: Functions for parsing and converting percentage values
- **Testing Support**: Macros and utilities for testing floating-point comparisons
- **Cross-Module Support**: Common utilities used throughout the library

## Usage Examples

```rust
use aminus::utils::*;

// Parse percentage strings
let value = parse_percentage("25.5%")?; // 0.255

// Standardize strings for comparison
let normalized = flatten_str("ATK_Percent"); // "atkpercent"

// Test with approximate equality
assert_aprx!(0.1 + 0.2, 0.3, 0.0001);
```

## Design Philosophy

The utils module follows these principles:

- **Single Responsibility**: Each utility function has a clear, focused purpose
- **Reusability**: Functions are designed to be used across multiple modules
- **Error Handling**: Proper error handling with Result types where appropriate
- **Performance**: Efficient implementations for commonly used operations 