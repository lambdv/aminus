import { Stat, Element, DamageType, BaseScaling, Amplifier } from '../src/types';
import { StatTable, StatFactory, Rotation, Formulas } from '../pkg/aminus_js';

const s = new StatTable();
s.add(Stat.FlatATK, 100);

const r = new Rotation();
r.add("test", (s) =>
    Formulas.calculateDamage(Element.Pyro, DamageType.Normal, BaseScaling.ATK, Amplifier.None, 1.0, 1.0, s)
)

const result = r.evaluate(s);
console.log(`Evaluation result: ${result}`);
