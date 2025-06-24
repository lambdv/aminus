- rust enums should convert straight to js enums
```js
enum Stat{
    BaseATK,
    CritRate,
    CritDMG,
    ...
}
```

- stat table should be a class that collects stat enums and pairs them with floats or doubles in js
```js
class StatTable{
    stats: {
        stat: Stat,
        value: number
    }[]
    constructor(){
        this.stats = [];
        this.add(Stat.BaseATK, 100);
        this.add(Stat.CritRate, 0.5);
        this.add(Stat.CritDMG, 1.5);
    }
}
```

- stat pair should be an object with a stat and value property. NOT AN ARRAY WITH 2 VALUES
```js
type StatPair = {
    stat: Stat;
    value: number;
}

x: StatPair = {
    stat: Stat.BaseATK,
    value: 100
}

y: StatPair = {
    stat: Stat.CritRate,
    value: 0.5
}

```

# Aminus JS Example Usage

This example demonstrates how to use the aminus JavaScript bindings for Genshin Impact calculations.

## Basic Usage

```javascript
import { StatFactory, StatTable, Stat } from './pkg/aminus_js.js';

// Create a StatFactory instance
const factory = new StatFactory();

// Get character base stats
const dilucStats = factory.getCharacterBaseStats('Diluc');
console.log('Diluc Base ATK:', dilucStats.get(Stat.BaseATK));
console.log('Diluc Base HP:', dilucStats.get(Stat.BaseHP));

// Get weapon stats
const weaponStats = factory.getWeaponStats("Wolf's Gravestone");
console.log('Weapon Base ATK:', weaponStats.get(Stat.BaseATK));

// Get main stat values for artifacts
const hpMainStat = factory.getMainStatValue(5, 20, Stat.FlatHP);
console.log('5-star level 20 HP main stat:', hpMainStat);

// Get sub stat values
const critRateSubStat = factory.getSubStatValue(5, Stat.CritRate);
console.log('5-star Crit Rate sub stat:', critRateSubStat);
```

## Using Legacy Functions (Backward Compatibility)

The legacy standalone functions are still available for backward compatibility:

```javascript
import { 
  getCharacterBaseStats, 
  getWeaponStats, 
  getMainStatValue, 
  getSubStatValue,
  Stat 
} from './pkg/aminus_js.js';

// These work the same way as the StatFactory methods
const dilucStats = getCharacterBaseStats('Diluc');
const weaponStats = getWeaponStats("Wolf's Gravestone");
const hpMainStat = getMainStatValue(5, 20, Stat.FlatHP);
const critRateSubStat = getSubStatValue(5, Stat.CritRate);
```

## StatTable Operations

```javascript
import { StatTable, Stat } from './pkg/aminus_js.js';

// Create an empty StatTable
const stats = new StatTable();

// Add individual stats
stats.add(Stat.ATKPercent, 0.5);
stats.add(Stat.CritRate, 0.2);

// Create StatTable from array
const statsArray = [
  [Stat.BaseATK, 100.0],
  [Stat.ATKPercent, 0.3],
  [Stat.ATKPercent, 0.2], // Will accumulate to 0.5
];
const statsFromArray = StatTable.of(statsArray);

// Combine StatTables
stats.addTable(statsFromArray);

// Get stat values
console.log('ATK%:', stats.get(Stat.ATKPercent)); // Should be 1.0 (0.5 + 0.5)
```

## Artifact Builder

```javascript
import { 
  ArtifactBuilder, 
  ArtifactPiece, 
  Stat, 
  RollQuality 
} from './pkg/aminus_js.js';

// Create artifact pieces
const flower = new ArtifactPiece(5, 20, Stat.FlatHP);
const feather = new ArtifactPiece(5, 20, Stat.FlatATK);
const sands = new ArtifactPiece(5, 20, Stat.ATKPercent);
const goblet = new ArtifactPiece(5, 20, Stat.PyroDMGBonus);
const circlet = new ArtifactPiece(5, 20, Stat.CritRate);

// Create artifact builder
const builder = new ArtifactBuilder(flower, feather, sands, goblet, circlet);

// Roll substats
builder.roll(Stat.CritDMG, RollQuality.AVG, 5, 1);
builder.roll(Stat.ATKPercent, RollQuality.HIGH, 5, 1);

// Get final stats
const finalStats = builder.build();
console.log('Final ATK%:', finalStats.get(Stat.ATKPercent));
console.log('Final Crit DMG:', finalStats.get(Stat.CritDMG));
```

## Damage Calculations

```javascript
import { 
  Formulas, 
  StatTable, 
  Stat, 
  Element, 
  DamageType, 
  BaseScaling, 
  Amplifier 
} from './pkg/aminus_js.js';

// Create character stats
const characterStats = new StatTable();
characterStats.add(Stat.BaseATK, 300.0);
characterStats.add(Stat.ATKPercent, 0.5);
characterStats.add(Stat.FlatATK, 100.0);
characterStats.add(Stat.CritRate, 0.2);
characterStats.add(Stat.CritDMG, 0.5);
characterStats.add(Stat.PyroDMGBonus, 0.3);

// Calculate damage
const damage = Formulas.calculateDamage(
  Element.Pyro,
  DamageType.Skill,
  BaseScaling.ATK,
  Amplifier.None,
  1.0, // instances
  2.0, // motion value
  characterStats,
  null // no buffs
);

console.log('Calculated damage:', damage);
```