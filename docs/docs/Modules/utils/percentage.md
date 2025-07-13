# Percentage

The `Percentage` module provides utilities for parsing and converting percentage values.

## Overview

This module contains functions for handling percentage strings and converting them to decimal values. It's commonly used when parsing external data that contains percentage representations.

## Functions

### parse_percentage

```rust
pub fn parse_percentage(input: &str) -> Result<f32, std::num::ParseFloatError>
```

Parses a string representing a percentage value and converts it to a decimal f32.

**Parameters:**
- `input`: String containing a percentage value (e.g., "25.5%" or "0.255")

**Returns:**
- `Ok(f32)`: Decimal representation of the percentage
- `Err(ParseFloatError)`: If parsing fails

**Examples:**
- `"25.5%"` → `0.255`
- `"100%"` → `1.0`
- `"0.5"` → `0.5` (no % symbol)
- `"0.05"` → `0.05`

## Usage Examples

```rust
use aminus::utils::percentage::parse_percentage;

// Parse percentage strings
let value1 = parse_percentage("25.5%")?; // 0.255
let value2 = parse_percentage("100%")?;  // 1.0
let value3 = parse_percentage("0.5")?;   // 0.5

// Handle errors
match parse_percentage("invalid") {
    Ok(value) => println!("Parsed: {}", value),
    Err(e) => println!("Parse error: {}", e),
}
```

## Error Handling

The function returns a `Result` type to handle parsing errors gracefully:

- **Valid inputs**: Strings containing numbers with optional % suffix
- **Invalid inputs**: Non-numeric strings or malformed percentages
- **Edge cases**: Empty strings, whitespace-only strings

## Common Use Cases

- **Parsing external data**: Converting percentage strings from JSON or CSV files
- **User input**: Handling percentage input from user interfaces
- **Data validation**: Ensuring percentage values are in the correct format 