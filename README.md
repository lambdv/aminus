# aminus
a genshin impact damage and stat calculation library.

```rust
let mut diluc = StatTable::of(&[
    (Stat::BaseATK, 334.85),
    (Stat::CritRate, 0.192 + 0.05),
    (Stat::CritDMG, 0.5),
    (Stat::EnergyRecharge, 1.0),
]);
let weapon = StatTable::of(&[
    (Stat::BaseATK, 510.0),
    (Stat::ElementalMastery, 165.0),
]);
let artifacts = StatTable::of(&[
    //mainstats
    (Stat::FlatHP, 4780.0),
    (Stat::FlatATK, 311.0),
    (Stat::ATKPercent, 0.466),
    (Stat::PyroDMGBonus, 0.466),
    (Stat::CritRate, 0.311),
    //substats
    (Stat::ATKPercent, 0.0992),
    (Stat::FlatATK, 33.08),
    (Stat::ElementalMastery, 39.6400),
    (Stat::CritRate, 0.0662),
    (Stat::CritDMG, 0.1324),
    (Stat::EnergyRecharge, 0.1102),
]);
diluc.add_table(&weapon);
diluc.add_table(&artifacts);
let r = Rotation::of(vec![
    (String::from("skill vape"), Box::new(|s: &dyn Statable| calculate_damage(
        Element::Pyro, DamageType::Skill, BaseScaling::ATK, Amplifier::None, 1.0, 1.0, s, None  
    ))),
]);
let dps = r.execute(&diluc)/20.0;
```

## Installation
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


