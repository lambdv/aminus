# aminus
a genshin impact damage/stat calculation & optimization library.

this is a programmatic implementation of KQMC-like excel spreadsheet calculators. 
- represent character, weapon, artifact stats using stat table (hashmap of stat->float)
- functions module for damage and stat calculation formulas 
- model sequence of character damage instances (rotation) as closures
- roll artifact substats for pool of 5 5-star artifacts, 4-star artifacts or 4 4-star artifacts + 1 5-star artifact using artifact builder
- optimize artifact substat roll distrobution and main stats based on kqmc standards with gradient ascent and greedy algorithms

cargo: https://crates.io/crates/aminus

npm: https://www.npmjs.com/package/aminus 

```rust
    let ayaka = 
        StatFactory::get_character_base_stats("ayaka", 90).unwrap()
        .chain(StatFactory::get_weapon_base_stats("mistsplitter", 90).unwrap())
        .chain(stats! { // snapshot buffs
            Stat::ATKPercent: 0.88, // 4pc no + 4pc tom + ttds 
            Stat::CritRate: 0.55, // 4pc bs + 
            Stat::CryoDMGBonus: 0.73,
            Stat::NormalATKDMGBonus: 0.3,
            Stat::ChargeATKDMGBonus: 0.3,
            Stat::CryoResistanceReduction: 0.4,
        });
    let rotation = rotation! {
        ("n1", Element::Cryo, DamageType::Normal, BaseScaling::ATK, Amplifier::None, 0.84, 3.0, None),
        ("n2", Element::Cryo, DamageType::Normal, BaseScaling::ATK, Amplifier::None, 0.894, 2.0, None),
        ("ca", Element::Cryo, DamageType::Normal, BaseScaling::ATK, Amplifier::None, 3.039, 2.0, None),
        ("skill", Element::Cryo, DamageType::Skill, BaseScaling::ATK, Amplifier::None, 4.07, 2.0, None),
        ("burstcuts", Element::Cryo, DamageType::Burst, BaseScaling::ATK, Amplifier::None, 1.91, 19.0, None),
        ("burstexplosion", Element::Cryo, DamageType::Burst, BaseScaling::ATK, Amplifier::None, 2.86, 1.0, None),
    };
    let energy_recharge_requirements = 1.30;
    let ayaka = optimizers::optimal_kqmc_5_artifacts_stats(&ayaka, &rotation, energy_recharge_requirements);
    let dps = rotation.evaluate(&ayaka)/21.; // 33263.758
```

## Use for Rust
1. add to rust project
   ```bash
   cargo add aminus
   ```
2. use in your code
   ```rust
   use aminus::*;
   ```

## Use for Typescript
1. install from npm
   ```bash
   npm install aminus
   ```
2. use in your code
   ```typescript
   import { StatFactory, StatTable, Rotation, optimizers } from "aminus";
   ```

## Source Installation
 1. **Clone The Repository**
   ```bash
   git clone https://github.com/lambdv/aminus.git 
   ```
 2. **Navagate into the directory**
  ```bash
  cd aminus
  ```
 3. **Install Dependencies**
```
cargo build
```
## Contributing
We are looking for contributers!
If you're interested in cotnrbuting from bug reports to implementing new features or updating docs: just fork the repo to add changes and submitt a pull request.
