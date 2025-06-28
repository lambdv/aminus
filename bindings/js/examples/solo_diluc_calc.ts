import { describe, test, expect } from '@jest/globals';
import { Stat, Element, DamageType, BaseScaling, Amplifier, RollQuality } from '../src/types';
import { StatTable, statFromString, getStatName, isElementalDmgBonus, Formulas, StatFactory, Rotation, Optimizers, ArtifactPiece, ArtifactBuilder} from '../pkg/aminus_js';

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
console.log(dps)



