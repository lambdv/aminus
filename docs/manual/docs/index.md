---
sidebar_position: 1
---

# Introduction

Aminus is a gesnhin impact calculation library. 

```rust
import com.github.lambdv.ParametricTransformer.core.*;
var ayaka = Characters.of("ayaka")
.equip(Weapons.of("mistsplitter"))
.add(StatTable.of(
    Stat.ATKPercent, 0.20 + 0.20 + 0.48,
    Stat.CritRate, 0.4 + 0.15,
    Stat.ElementalDMGBonus, 0.15 + 0.12 + 0.28 + 0.18 + (0.0004*800),
    Stat.NormalATKDMGBonus, 0.3,
    Stat.ChargeATKDMGBonus, 0.3,
    Stat.CryoResistanceReduction, 0.4));
var ayakaRotation = new Rotation()
    .add("n1", DamageFormulas.defaultCryoNormalATK(3.0, 0.84))
    .add("n2", DamageFormulas.defaultCryoNormalATK(2.0, 0.894))
    .add("ca", DamageFormulas.defaultCryoChargedATK(2.0, 3.039))
    .add("skill", DamageFormulas.defaultCryoSkillATK(2.0, 4.07))
    .add("burstcutts", DamageFormulas.defaultCryoBurstATK(19.0, 1.91))
    .add("burstexplosion", DamageFormulas.defaultCryoBurstATK(1.0, 2.86));
ayaka.optimize(Optimizers.KQMSArtifactOptimizer(ayakaRotation, 1.30));
var dps = ayakaRotation.compute(ayaka)/21;
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

