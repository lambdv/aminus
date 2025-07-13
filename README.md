# aminus
a genshin impact damage and stat calculation library for rust and typescript.

npm: https://www.npmjs.com/package/aminus 

cargo: https://crates.io/crates/aminus

```rust
let ayaka = StatFactory::get_character_base_stats("ayaka").unwrap()
    .chain(Box::new(StatFactory::get_weapon_base_stats("mistsplitter").unwrap()))
    .chain(Box::new(StatTable::of(&[ //snapshot buffs
        (Stat::ATKPercent, 0.88),
        (Stat::CritRate, 0.55),
        (Stat::CryoDMGBonus, 0.73),
        (Stat::NormalATKDMGBonus, 0.3),
        (Stat::ChargeATKDMGBonus, 0.3),
        (Stat::CryoResistanceReduction, 0.4),
    ])));
let rotation = Rotation::of(vec![
    default_cryo_na_formula("n1", &ayaka, 0.84, 3, None),
    default_cryo_na_formula("n2", &ayaka, 0.894, 2, None),
    default_cryo_na_formula("ca", &ayaka, 3.039, 2, None),
    default_cryo_e_formula("skill", &ayaka, 4.07, 2, None),
    default_cryo_q_formula("burstcuts", &ayaka, 1.91, 19, None),
    default_cryo_q_formula("burstexplosion", &ayaka, 2.86, 1, None),
]);
let ayaka = optimizers::optimal_kqmc_5_artifacts_stats(&StatTable::unbox(ayaka), &rotation, 1.30);
let dps = rotation.evaluate(&ayaka)/21.;
```

## Features
- Stat Modeling
- Damage Calculation
- Artifact Mainstat & Substat Optimization (based on KQM Calculation Standard)
- Energy Recharnge Requirements Calculation (wip)

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