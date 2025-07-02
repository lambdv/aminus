# aminus
a genshin impact damage and stat calculation library for rust and typescript.

read docs: https://aminus.irminsul.moe/
npm: https://www.npmjs.com/package/aminus 
cargo: https://crates.io/crates/aminus

```rust
let ayaka = StatFactory::get_character_base_stats("ayaka", 90).unwrap()
    .chain(Box::new(StatFactory::get_weapon_base_stats("mistsplitter", 90).unwrap()))
    .chain(Box::new(StatTable::of(&[ //snapshot buffs
        (Stat::ATKPercent, 0.88),
        (Stat::CritRate, 0.55),
        (Stat::CryoDMGBonus, 0.73),
        (Stat::NormalATKDMGBonus, 0.3),
        (Stat::ChargeATKDMGBonus, 0.3),
        (Stat::CryoResistanceReduction, 0.4),
    ])));
let rotation = Rotation::of(vec![
    default_cryo_na_formula!("n1", &ayaka, 0.84, 3, None),
    default_cryo_na_formula!("n2", &ayaka, 0.894, 2, None),
    default_cryo_na_formula!("ca", &ayaka, 3.039, 2, None),
    default_cryo_e_formula!("skill", &ayaka, 4.07, 2, None),
    default_cryo_q_formula!("burstcuts", &ayaka, 1.91, 19, None),
    default_cryo_q_formula!("burstexplosion", &ayaka, 2.86, 1, None),
]);
let ayaka = optimizers::optimal_kqmc_5_artifacts_stats(&StatTable::unbox(ayaka), &rotation, 1.40);
let dps = rotation.evaluate(&ayaka)/21.;
```
```typescript
let diluc = StatFactory.getCharacterBaseStats("diluc")
let wgs = StatFactory.getWeaponStats("Wolf's Gravestone")
diluc.addTable(wgs)
diluc.addTable(StatTable.of([ //buffs
    [Stat.ATKPercent, 0.2],
    [Stat.ATKPercent, 0.48],
    [Stat.FlatATK, 2000],
    [Stat.PyroDMGBonus, 0.2],
    [Stat.ElementalMastery, 80],
    [Stat.EnergyRecharge, 1.0],
]))

let rotation = new Rotation()
rotation.add("normal", (s) =>
    Formulas.calculateDamage(Element.Pyro, DamageType.Normal, BaseScaling.ATK, Amplifier.None, 1.0, 1.0, s))
rotation.add("skill", (s) =>
    Formulas.calculateDamage(Element.Pyro, DamageType.Skill, BaseScaling.ATK, Amplifier.None, 1.0, 1.0, s))
rotation.add("burst", (s) =>
    Formulas.calculateDamage(Element.Pyro, DamageType.Burst, BaseScaling.ATK, Amplifier.None, 1.0, 1.0, s))

let optimal_main_stats = Optimizers.globalKqmcArtifactMainStatOptimizer(diluc, rotation)
console.log(optimal_main_stats)
let optimal_substats = Optimizers.gradient5StarKqmcArtifactSubstatOptimizer(
    diluc, 
    rotation, 
    new ArtifactPiece(5, 20, Stat.FlatHP), 
    new ArtifactPiece(5, 20, Stat.FlatATK), 
    new ArtifactPiece(5, 20, optimal_main_stats[0]), 
    new ArtifactPiece(5, 20, optimal_main_stats[1]), 
    new ArtifactPiece(5, 20, optimal_main_stats[2]), 
    1.0
)
console.log(optimal_substats)
let builder = new ArtifactBuilder(
    new ArtifactPiece(5, 20, Stat.FlatHP), 
    new ArtifactPiece(5, 20, Stat.FlatATK), 
    new ArtifactPiece(5, 20, optimal_main_stats[0]), 
    new ArtifactPiece(5, 20, optimal_main_stats[1]), 
    new ArtifactPiece(5, 20, optimal_main_stats[2]), 
)
for (let [stat_id, num_rolls] of Object.entries(optimal_substats)) {
    builder.roll(Number(stat_id) , RollQuality.AVG, 5, num_rolls)
}
diluc.addTable(builder.build())

let dps = rotation.evaluate(diluc)
```

## Features
- Stat modeling and building
- Damage Calculation
- Artifact Optimization (based on KQM's Calculation Standard)
- Energy Recharnge Requirements (wip)


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
## Contributing

We welcome contributions from the community! Whether you want to report a bug, suggest a feature, or submit code changes, we'd love to hear from you. Just fork the repository, make your changes, and submit a pull request - no contribution is too small!

