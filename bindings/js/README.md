# Aminus JavaScript/TypeScript Bindings

JavaScript and TypeScript bindings for the Aminus Genshin Impact calculation library.

## Features

- **Full TypeScript Support**: Complete type definitions for all functions and classes
- **ESM Module Support**: Uses modern ES modules
- **WebAssembly**: High-performance calculations compiled from Rust
- **Cross-Platform**: Works in Node.js, browsers, and bundlers
- **Zero Dependencies**: No runtime dependencies besides the WASM module
- **Artifact Optimization**: Advanced algorithms for finding optimal artifact combinations

## Installation

```bash
npm install aminus-js
```

## Quick Start

### TypeScript

```typescript
import { loadWasm, Stat, Element, DamageType, BaseScaling, Amplifier } from 'aminus-js';

async function calculateDamage() {
  // Load the WASM module
  const aminus = await loadWasm();
  
  // Create a character stat table
  const character = aminus.StatTableWrapper.fromArray([
    [Stat.BaseATK, 334.85],
    [Stat.ATKPercent, 0.5],
    [Stat.CritRate, 0.6],
    [Stat.CritDMG, 1.2],
    [Stat.PyroDMGBonus, 0.466],
  ]);
  
  // Calculate damage
  const damage = aminus.calculateDamage(
    Element.Pyro,
    DamageType.Skill,
    BaseScaling.ATK,
    Amplifier.None,
    1.0, // instances
    2.5, // motion value
    character
  );
  
  console.log(`Skill damage: ${damage}`);
}

calculateDamage();
```

### JavaScript (ESM)

```javascript
import { loadWasm, Stat, Element, DamageType, BaseScaling, Amplifier } from 'aminus-js';

const aminus = await loadWasm();

// Create artifact builder
const builder = aminus.ArtifactBuilderWrapper.kqmAll5Star(
  Stat.ATKPercent,     // sands
  Stat.PyroDMGBonus,   // goblet
  Stat.CritRate        // circlet
);

// Add substat rolls
builder.roll(Stat.CritDMG, 4, 5, 6); // RollQuality.AVG, rarity 5, 6 rolls

// Get final stats
const artifacts = builder.build();
console.log(`Total Crit DMG: ${artifacts.get(Stat.CritDMG)}`);
```

### Artifact Optimization

```typescript
import { 
  createStatTable, 
  createRotation, 
  optimizeArtifactMainStats,
  optimizeArtifactSubstats,
  Stat, Element, DamageType, BaseScaling, Amplifier 
} from 'aminus-js';

async function optimizeCharacter() {
  // Create character stats
  const characterStats = createStatTable();
  characterStats.set(Stat.BaseATK, 106.0);
  characterStats.set(Stat.CritRate, 0.05);
  characterStats.set(Stat.CritDMG, 0.5);

  // Create rotation (damage sequence)
  const rotation = createRotation();
  rotationadd(
    'pyro_attack',
    Element.Pyro,
    DamageType.Normal,
    BaseScaling.ATK,
    Amplifier.None,
    1.0, // instances
    1.0  // motion value
  );

  // Optimize artifact main stats
  const bestMainStats = await optimizeArtifactMainStats(characterStats, rotation);
  console.log('Best main stats:', bestMainStats); // [sands, goblet, circlet]

  // Optimize artifact substats
  const substatDistribution = await optimizeArtifactSubstats(
    characterStats,
    rotation,
    1.2, // energy recharge requirement
    flower, feather, sands, goblet, circlet
  );
  console.log('Optimal substat distribution:', substatDistribution);
}
```

## Building

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Node.js](https://nodejs.org/) (16+)

### Build Commands

```bash
# Install dependencies
npm install

# Build for bundlers (default)
npm run build

# Build for Node.js
npm run build:nodejs

# Build for web browsers
npm run build:web

# Run tests
npm test

# Development mode (build + test)
npm run dev
```

## API Reference

### Types

#### Enums

- `Stat`: Character and artifact statistics
- `Element`: Elemental types (Pyro, Hydro, etc.)
- `DamageType`: Attack types (Normal, Charged, Skill, Burst, etc.)
- `BaseScaling`: Damage scaling base (ATK, DEF, HP)
- `Amplifier`: Reaction amplifiers (None, Forward, Reverse)
- `RollQuality`: Artifact substat roll quality

#### Core Classes

##### StatTableWrapper

Manages character statistics.

```typescript
const stats = new StatTableWrapper();
stats.add(Stat.ATKPercent, 0.5);
const atkPercent = stats.get(Stat.ATKPercent);

// Create from array
const stats2 = StatTableWrapper.fromArray([
  [Stat.BaseATK, 100],
  [Stat.ATKPercent, 0.5]
]);
```

##### ArtifactBuilderWrapper

Builds artifact stat combinations.

```typescript
// Manual construction
const builder = new ArtifactBuilderWrapper(flower, feather, sands, goblet, circlet);

// KQM presets
const builder = ArtifactBuilderWrapper.kqmAll5Star(
  Stat.ATKPercent,     // sands main stat
  Stat.PyroDMGBonus,   // goblet main stat
  Stat.CritRate        // circlet main stat
);

// Roll substats
builder.roll(Stat.CritDMG, RollQuality.AVG, 5, 4);
const finalStats = builder.build();
```

##### Rotation

Defines a sequence of damage operations for optimization.

```typescript
const rotation = createRotation();

// Add damage operations
rotationadd(
  'normal_attack',
  Element.Pyro,
  DamageType.Normal,
  BaseScaling.ATK,
  Amplifier.None,
  1.0, // instances
  1.0  // motion value
);

rotationadd(
  'skill_attack',
  Element.Pyro,
  DamageType.Skill,
  BaseScaling.ATK,
  Amplifier.None,
  1.0,
  2.5
);

// Evaluate total damage
const totalDamage = rotation.evaluate(characterStats);
```

### Functions

#### Damage Calculation

```typescript
const damage = calculateDamage(
  element: Element,
  damageType: DamageType,
  scaling: BaseScaling,
  amplifier: Amplifier,
  instances: number,
  motionValue: number,
  character: StatTableWrapper,
  buffs?: StatTableWrapper
): number;
```

#### Artifact Optimization

```typescript
// Optimize artifact main stats
const bestMainStats = await optimizeArtifactMainStats(
  characterStats: StatTableWrapper,
  target: Rotation
): Promise<number[]>; // [sands, goblet, circlet] stat IDs

// Optimize artifact substats
const substatDistribution = await optimizeArtifactSubstats(
  characterStats: StatTableWrapper,
  target: Rotation,
  energyRechargeRequirements?: number,
  flower?: ArtifactPiece,
  feather?: ArtifactPiece,
  sands?: ArtifactPiece,
  goblet?: ArtifactPiece,
  circlet?: ArtifactPiece
): Promise<Record<number, number>>; // stat ID -> roll count

// Calculate stat gradients
const gradients = await calculateStatGradients(
  base: StatTableWrapper,
  target: Rotation,
  slopes: Record<number, number>
): Promise<Record<number, number>>;

// Find effective stats (ReLU heuristic)
const effectiveStats = await calculateReluHeuristic(
  base: StatTableWrapper,
  target: Rotation,
  slopes: Record<number, number>
): Promise<number[]>;
```

#### Formula Functions

```typescript
// Calculate total stats
const totalATK = calculateTotalAtkFromTable(stats);
const totalDEF = calculateTotalDefFromTable(stats);
const totalHP = calculateTotalHpFromTable(stats);

// Calculate multipliers
const critMultiplier = calculateAvgCritMultiplierFromTable(stats);
const defMultiplier = calculateDefMultiplier(charLevel, enemyLevel, defReduction, defIgnore);
```

#### Utility Functions

```typescript
// Convert stat names
const stat = statFromString('atk'); // Returns Stat.ATKPercent
const name = getStatName(Stat.ATKPercent); // Returns 'ATKPercent'

// Check stat properties
const isElemental = isElementalDmgBonus(Stat.PyroDMGBonus); // true
```

## Examples

### Character Damage Calculation

```typescript
import { loadWasm, Stat, Element, DamageType, BaseScaling, Amplifier } from 'aminus-js';

async function calculateDilucDamage() {
  const aminus = await loadWasm();
  
  // Diluc base stats
  const character = aminus.StatTableWrapper.fromArray([
    [Stat.BaseATK, 334.85],
    [Stat.CritRate, 0.242],
    [Stat.CritDMG, 0.5],
  ]);
  
  // Wolf's Gravestone weapon
  const weapon = aminus.StatTableWrapper.fromArray([
    [Stat.BaseATK, 608],
    [Stat.ATKPercent, 0.496],
  ]);
  
  // Artifact stats
  const artifacts = aminus.StatTableWrapper.fromArray([
    [Stat.FlatATK, 311],
    [Stat.ATKPercent, 0.466],
    [Stat.PyroDMGBonus, 0.466],
    [Stat.CritRate, 0.311],
    [Stat.CritDMG, 0.622],
  ]);
  
  // Combine all stats
  character.addTable(weapon);
  character.addTable(artifacts);
  
  // Calculate E skill damage
  const skillDamage = aminus.calculateDamage(
    Element.Pyro,
    DamageType.Skill,
    BaseScaling.ATK,
    Amplifier.None,
    1.0,
    2.016, // Diluc E motion value
    character
  );
  
  console.log(`Diluc E damage: ${skillDamage.toFixed(0)}`);
}
```

### Artifact Optimization

```typescript
import { loadWasm, Stat, RollQuality } from 'aminus-js';

async function optimizeArtifacts() {
  const aminus = await loadWasm();
  
  // Create KQM 5-star artifact set
  const builder = aminus.ArtifactBuilderWrapper.kqmAll5Star(
    Stat.ATKPercent,     // sands
    Stat.PyroDMGBonus,   // goblet
    Stat.CritRate        // circlet
  );
  
  // Simulate rolling substats
  builder.roll(Stat.CritDMG, RollQuality.HIGH, 5, 6);
  builder.roll(Stat.ATKPercent, RollQuality.AVG, 5, 4);
  builder.roll(Stat.FlatATK, RollQuality.LOW, 5, 2);
  
  const artifacts = builder.build();
  
  console.log('Final artifact stats:');
  console.log(`ATK%: ${(artifacts.get(Stat.ATKPercent) * 100).toFixed(1)}%`);
  console.log(`Crit Rate: ${(artifacts.get(Stat.CritRate) * 100).toFixed(1)}%`);
  console.log(`Crit DMG: ${(artifacts.get(Stat.CritDMG) * 100).toFixed(1)}%`);
  console.log(`Pyro DMG: ${(artifacts.get(Stat.PyroDMGBonus) * 100).toFixed(1)}%`);
}
```

## Browser Usage

For browser environments, you may need to handle WASM loading differently:

```html
<!DOCTYPE html>
<html>
<head>
  <script type="module">
    import { loadWasm, Stat } from './node_modules/aminus-js/pkg/aminus_js.js';
    
    async function main() {
      const aminus = await loadWasm();
      
      const stats = new aminus.StatTableWrapper();
      stats.add(Stat.ATKPercent, 0.5);
      
      console.log('ATK%:', stats.get(Stat.ATKPercent));
    }
    
    main().catch(console.error);
  </script>
</head>
<body>
  <h1>Aminus Web Example</h1>
</body>
</html>
```

## Running Tests

Run all tests:
```bash
npm test
```

Run tests in watch mode:
```bash
npm run test:watch
```

Test files are located in the `tests/` directory.

## Contributing

1. Make changes to the Rust source code in `src/`
2. Run `npm run build` to compile WASM
3. Add or update tests in `