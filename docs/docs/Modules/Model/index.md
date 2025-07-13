# Model

The model module contains struct and enum types to model in-game attributes in code and library specific abstractions to model computation.

## Submodules

- [Stat](./stat.md) - Core statistics and attribute system
- [Statable](./statable.md) - Trait for objects that can have statistics
- [StatTable](./stattable.md) - Table-based statistics management
- [StatTable Builder](./stattable_builder.md) - Builder pattern for creating stat tables
- [Character](./character.md) - Character data structures and properties
- [Weapon](./weapon.md) - Weapon data structures and properties
- [Artifact](./artifact.md) - Artifact data structures and properties
- [Artifact Builder](./artifact_builder.md) - Builder pattern for creating artifacts
- [Rotation](./rotation.md) - Character rotation and ability sequences
- [Operation](./operation.md) - Mathematical operations on statistics

## Architecture

The model module follows a hierarchical structure:

1. **Base Layer**: `Stat` and `Statable` provide the foundation for all statistical data
2. **Container Layer**: `StatTable` manages collections of statistics
3. **Entity Layer**: `Character`, `Weapon`, and `Artifact` represent game objects
4. **Builder Layer**: Various builders provide convenient construction patterns
5. **Behavior Layer**: `Rotation` and `Operation` define how entities behave