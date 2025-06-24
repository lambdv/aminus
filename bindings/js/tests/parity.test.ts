import { describe, test, expect, beforeAll } from '@jest/globals';
import { Stat } from '../src/types';
let aminus: any;

import {
  StatTable,
  StatFactory,
  getCharacterBaseStats,
  getWeaponStats,
  getSubStatValue,
  getMainStatValue,
  fuzzyMatch
} from '../pkg/aminus_js.js';

// StatTable tests (from model/stattable.rs)
describe('StatTable parity', () => {
  test('construct_with_intial_values', () => {
    const s = StatTable.of([
      [Stat.ATKPercent, 1.0],
      [Stat.ATKPercent, 5.0],
    ]);
    expect(s.get(Stat.ATKPercent)).toBe(6.0);
  });

  test('test_adding_and_getting', () => {
    const s = Stat.FlatATK;
    const table = new StatTable();
    expect(table.get(s)).toBe(0.0);
    table.add(s, 10.0);
    expect(table.get(s)).toBe(10.0);
    table.add(s, 10.0);
    expect(table.get(s)).toBe(20.0);
  });

  test('test_adding_stattable', () => {
    const t1 = new StatTable();
    expect(t1.get(Stat.CritDMG)).toBe(0.0);
    t1.add(Stat.FlatATK, 2000.0);
    const t2 = new StatTable();
    t2.add(Stat.CritDMG, 0.5);
    t1.addTable(t2);
    expect(t1.get(Stat.CritDMG)).toBe(0.5);
  });
});

// StatFactory struct tests
describe('StatFactory struct', () => {
  let factory: any;

  beforeAll(() => {
    factory = new StatFactory();
  });

  test('get_character_expected', () => {
    const amber = factory.getCharacterBaseStats('Amber');
    expect(amber.get(Stat.BaseATK)).toBeCloseTo(223.02, 2);
    expect(amber.get(Stat.BaseHP)).toBeCloseTo(9461.18, 2);
    expect(amber.get(Stat.BaseDEF)).toBeCloseTo(600.62, 2);
    expect(amber.get(Stat.ATKPercent)).toBeCloseTo(0.240, 2);
  });

  test('fuzzy_match_test', () => {
    expect(fuzzyMatch('ayaka', 'Kamisato Ayaka')).toBe(true);
  });

  test('get_chara_fuzzy', () => {
    const c1 = factory.getCharacterBaseStats('Kamisato Ayaka');
    const c2 = factory.getCharacterBaseStats('ayaka');
    expect(c1.get(Stat.BaseATK)).toBeCloseTo(c2.get(Stat.BaseATK), 2);
    expect(c1.get(Stat.BaseHP)).toBeCloseTo(c2.get(Stat.BaseHP), 2);
    expect(c1.get(Stat.BaseDEF)).toBeCloseTo(c2.get(Stat.BaseDEF), 2);
  });

  test('get_weapon_stats_works', () => {
    const w = factory.getWeaponStats('A Thousand Blazing Suns');
    expect(w.get(Stat.BaseATK)).toBeCloseTo(741.0, 2);
    expect(w.get(Stat.CritRate)).toBeCloseTo(0.11, 2);
  });

  test('get_main_stat_value', () => {
    expect(factory.getMainStatValue(5, 20, Stat.FlatATK)).toBeCloseTo(311.0, 2);
    expect(factory.getMainStatValue(1, 0, Stat.FlatATK)).toBeCloseTo(8.0, 2);
    expect(factory.getMainStatValue(1, 0, Stat.PyroDMGBonus)).toBeCloseTo(0.031, 3);
    expect(() => factory.getMainStatValue(0, 0, Stat.FlatATK)).toThrow();
    expect(() => factory.getMainStatValue(-1, 0, Stat.FlatATK)).toThrow();
    expect(() => factory.getMainStatValue(6, 0, Stat.FlatATK)).toThrow();
    expect(() => factory.getMainStatValue(1, 5, Stat.FlatATK)).toThrow();
    expect(() => factory.getMainStatValue(5, 21, Stat.FlatATK)).toThrow();
    expect(() => factory.getMainStatValue(4, 17, Stat.FlatATK)).toThrow();
    expect(() => factory.getMainStatValue(5, 20, Stat.BaseATK)).toThrow();
  });

  test('test_get_substat_value', () => {
    expect(factory.getSubStatValue(5, Stat.ATKPercent)).toBeCloseTo(0.0583, 4);
    expect(factory.getSubStatValue(5, Stat.CritRate)).toBeCloseTo(0.0389, 4);
    expect(factory.getSubStatValue(4, Stat.ATKPercent)).toBeCloseTo(0.0466, 4);
    expect(factory.getSubStatValue(1, Stat.ATKPercent)).toBeCloseTo(0.0146, 4);
    expect(() => factory.getSubStatValue(0, Stat.BaseATK)).toThrow();
    expect(() => factory.getSubStatValue(5, Stat.PhysicalDMGBonus)).toThrow();
  });
});

// Legacy StatFactory tests (for backward compatibility)
describe('Legacy StatFactory functions', () => {
  test('get_character_expected', () => {
    const amber = getCharacterBaseStats('Amber');
    expect(amber.get(Stat.BaseATK)).toBeCloseTo(223.02, 2);
    expect(amber.get(Stat.BaseHP)).toBeCloseTo(9461.18, 2);
    expect(amber.get(Stat.BaseDEF)).toBeCloseTo(600.62, 2);
    expect(amber.get(Stat.ATKPercent)).toBeCloseTo(0.240, 2);
  });

  test('fuzzy_match_test', () => {
    expect(fuzzyMatch('ayaka', 'Kamisato Ayaka')).toBe(true);
  });

  test('get_chara_fuzzy', () => {
    const c1 = getCharacterBaseStats('Kamisato Ayaka');
    const c2 = getCharacterBaseStats('ayaka');
    expect(c1.get(Stat.BaseATK)).toBeCloseTo(c2.get(Stat.BaseATK), 2);
    expect(c1.get(Stat.BaseHP)).toBeCloseTo(c2.get(Stat.BaseHP), 2);
    expect(c1.get(Stat.BaseDEF)).toBeCloseTo(c2.get(Stat.BaseDEF), 2);
  });

  test('get_weapon_stats_works', () => {
    const w = getWeaponStats('A Thousand Blazing Suns');
    expect(w.get(Stat.BaseATK)).toBeCloseTo(741.0, 2);
    expect(w.get(Stat.CritRate)).toBeCloseTo(0.11, 2);
  });

  test('get_main_stat_value', () => {
    expect(getMainStatValue(5, 20, Stat.FlatATK)).toBeCloseTo(311.0, 2);
    expect(getMainStatValue(1, 0, Stat.FlatATK)).toBeCloseTo(8.0, 2);
    expect(getMainStatValue(1, 0, Stat.PyroDMGBonus)).toBeCloseTo(0.031, 3);
    expect(() => getMainStatValue(0, 0, Stat.FlatATK)).toThrow();
    expect(() => getMainStatValue(-1, 0, Stat.FlatATK)).toThrow();
    expect(() => getMainStatValue(6, 0, Stat.FlatATK)).toThrow();
    expect(() => getMainStatValue(1, 5, Stat.FlatATK)).toThrow();
    expect(() => getMainStatValue(5, 21, Stat.FlatATK)).toThrow();
    expect(() => getMainStatValue(4, 17, Stat.FlatATK)).toThrow();
    expect(() => getMainStatValue(5, 20, Stat.BaseATK)).toThrow();
  });

  test('test_get_substat_value', () => {
    expect(getSubStatValue(5, Stat.ATKPercent)).toBeCloseTo(0.0583, 4);
    expect(getSubStatValue(5, Stat.CritRate)).toBeCloseTo(0.0389, 4);
    expect(getSubStatValue(4, Stat.ATKPercent)).toBeCloseTo(0.0466, 4);
    expect(getSubStatValue(1, Stat.ATKPercent)).toBeCloseTo(0.0146, 4);
    expect(() => getSubStatValue(0, Stat.BaseATK)).toThrow();
    expect(() => getSubStatValue(5, Stat.PhysicalDMGBonus)).toThrow();
  });
});

describe('Parity Tests', () => {
  beforeAll(async () => {
    aminus = await import('../pkg/aminus_js.js');
  });

  test('fetchCharacterBaseStats matches getCharacterBaseStats', async () => {
    if (!aminus.fetchCharacterBaseStats) return;
    const local = aminus.getCharacterBaseStats('Amber');
    const fetched = await aminus.fetchCharacterBaseStats('Amber');
    expect(fetched.get(Stat.BaseATK)).toBeCloseTo(local.get(Stat.BaseATK), 2);
    expect(fetched.get(Stat.BaseHP)).toBeCloseTo(local.get(Stat.BaseHP), 2);
    expect(fetched.get(Stat.BaseDEF)).toBeCloseTo(local.get(Stat.BaseDEF), 2);
  });

  test('fetchWeaponStats matches getWeaponStats', async () => {
    if (!aminus.fetchWeaponStats) return;
    const local = aminus.getWeaponStats("Wolf's Gravestone");
    const fetched = await aminus.fetchWeaponStats("Wolf's Gravestone");
    expect(fetched.get(Stat.BaseATK)).toBeCloseTo(local.get(Stat.BaseATK), 2);
  });
}); 