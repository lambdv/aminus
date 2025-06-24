import { describe, test, expect } from '@jest/globals';
import { Stat, Element, DamageType, BaseScaling, Amplifier } from '../src/types';
import { 
  StatTable, 
  statFromString,
  getStatName,
  isElementalDmgBonus,
  Formulas,
} from '../pkg/aminus_js';

describe('Solocalc Tests - Character Damage Calculation', () => {
  test('primitive character damage calculation', () => {
    // Create Diluc base stats
    const dilucStats: [Stat, number][] = [
      [Stat.BaseATK, 334.85],
      [Stat.CritRate, 0.192 + 0.05],
      [Stat.CritDMG, 0.5],
      [Stat.EnergyRecharge, 1.0],
    ];
    const diluc = new StatTable();
    diluc.addTable(StatTable.of(dilucStats));

    // Create weapon stats
    const weaponStats: [Stat, number][] = [
      [Stat.BaseATK, 510.0],
      [Stat.ElementalMastery, 165.0],
    ];
    const weapon = new StatTable();
    weapon.addTable(StatTable.of(weaponStats));

    // Create artifact stats
    const artifactStats: [Stat, number][] = [
      // Main stats
      [Stat.FlatHP, 4780.0],
      [Stat.FlatATK, 311.0],
      [Stat.ATKPercent, 0.466],
      [Stat.PyroDMGBonus, 0.466],
      [Stat.CritRate, 0.311],
      // Sub stats
      [Stat.ATKPercent, 0.0992],
      [Stat.FlatATK, 33.08],
      [Stat.ElementalMastery, 39.6400],
      [Stat.CritRate, 0.0662],
      [Stat.CritDMG, 0.1324],
      [Stat.EnergyRecharge, 0.1102],
    ];
    const artifacts = new StatTable();
    artifacts.addTable(StatTable.of(artifactStats));

    // Combine all stats
    diluc.addTable(weapon);
    diluc.addTable(artifacts);

    // Test individual stat values
    assertApprox(diluc.get(Stat.BaseATK), 844.85, 0.1);
    assertApprox(diluc.get(Stat.ATKPercent), 0.5652, 0.1);
    assertApprox(diluc.get(Stat.FlatATK), 344.08, 0.1);
    assertApprox(diluc.get(Stat.CritRate), 0.6192, 0.01);
    assertApprox(diluc.get(Stat.CritDMG), 0.6324, 0.01);
    assertApprox(diluc.get(Stat.ElementalMastery), 204.64, 1.0);
    assertApprox(diluc.get(Stat.EnergyRecharge), 1.11, 0.1);
    assertApprox(diluc.get(Stat.PyroDMGBonus), 0.466, 0.1);

    // Test total ATK calculation
    const totalAtk = Formulas.totalATK(diluc);
    assertApprox(totalAtk, 1667.0, 10.0);

    // Test damage calculation
    const skillDamage = Formulas.calculateDamage(
      Element.Pyro,
      DamageType.Skill,
      BaseScaling.ATK,
      Amplifier.None,
      1.0, // instances
      1.0, // motion_value
      diluc,
      null // no buffs
    );
    
    assertApprox(skillDamage, 1490.609, 0.1);
  });

  test('stat enum string conversion', () => {
    // Test stat from string conversion
    const atkStat = statFromString('atk');
    expect(atkStat).toBe(Stat.ATKPercent);

    const critRateStat = statFromString('crit rate');
    expect(critRateStat).toBe(Stat.CritRate);

    // Test stat name retrieval
    const atkName = getStatName(Stat.ATKPercent);
    expect(atkName).toBe('ATKPercent');

    // Test elemental DMG bonus check
    const isPyroDmgBonus = isElementalDmgBonus(Stat.PyroDMGBonus);
    expect(isPyroDmgBonus).toBe(true);

    const isAtkBonus = isElementalDmgBonus(Stat.ATKPercent);
    expect(isAtkBonus).toBe(false);
  });

  test('formula calculations', () => {
    // Create a simple stat table for testing formulas
    const stats = new StatTable();
    stats.add(Stat.BaseATK, 100.0);
    stats.add(Stat.ATKPercent, 0.5);
    stats.add(Stat.FlatATK, 50.0);
    stats.add(Stat.BaseDEF, 80.0);
    stats.add(Stat.DEFPercent, 0.3);
    stats.add(Stat.FlatDEF, 20.0);
    stats.add(Stat.BaseHP, 1000.0);
    stats.add(Stat.HPPercent, 0.4);
    stats.add(Stat.FlatHP, 200.0);
    stats.add(Stat.CritRate, 0.5);
    stats.add(Stat.CritDMG, 1.0);

    // Test total ATK: base_atk * (1 + atk_percent) + flat_atk
    // 100 * (1 + 0.5) + 50 = 200
    const totalAtk = Formulas.totalATK(stats);
    assertApprox(totalAtk, 200.0, 0.1);

    // Test total DEF: base_def * (1 + def_percent) + flat_def
    // 80 * (1 + 0.3) + 20 = 124
    const totalDef = Formulas.totalDEF(stats);
    assertApprox(totalDef, 124.0, 0.1);

    // Test total HP: base_hp * (1 + hp_percent) + flat_hp
    // 1000 * (1 + 0.4) + 200 = 1600
    const totalHp = Formulas.totalHP(stats);
    assertApprox(totalHp, 1600.0, 0.1);

    // Test average crit multiplier: 1 + (crit_rate * crit_dmg)
    // 1 + (0.5 * 1.0) = 1.5
    const critMultiplier = Formulas.avgCritMultiplier(stats);
    assertApprox(critMultiplier, 1.5, 0.1);

    // Test def multiplier
    const defMult = Formulas.defMultiplier(90, 100, 0.0, 0.0);
    assertApprox(defMult, 0.487179487179487, 0.0001);
  });
});

describe('StatTable Tests', () => {
  test('StatTable creation and manipulation', () => {
    // Test empty StatTable
    const table = new StatTable();
    expect(table.get(Stat.ATKPercent)).toBe(0.0);

    // Test adding stats
    table.add(Stat.ATKPercent, 0.5);
    expect(table.get(Stat.ATKPercent)).toBe(0.5);

    // Test accumulation
    table.add(Stat.ATKPercent, 0.3);
    assertApprox(table.get(Stat.ATKPercent), 0.8, 0.001);

    // Test of creation
    const stats: [Stat, number][] = [
      [Stat.BaseATK, 100.0],
      [Stat.ATKPercent, 0.5],
      [Stat.ATKPercent, 0.2], // Should accumulate
    ];
    const table2 = StatTable.of(stats);
    expect(table2.get(Stat.BaseATK)).toBe(100.0);
    assertApprox(table2.get(Stat.ATKPercent), 0.7, 0.001);

    // Test toArray conversion
    const array = table2.toArray();
    expect(array.length).toBeGreaterThan(0);
    
    // Find the ATKPercent entry
    let foundAtkPercent = false;
    for (let i = 0; i < array.length; i++) {
      const [stat, value] = array[i];
      if (stat === Stat.ATKPercent) {
        assertApprox(value, 0.7, 0.001);
        foundAtkPercent = true;
        break;
      }
    }
    expect(foundAtkPercent).toBe(true);

    // Test addTable
    const table3 = new StatTable();
    table3.add(Stat.CritRate, 0.3);
    table2.addTable(table3);
    assertApprox(table2.get(Stat.CritRate), 0.3, 0.001);
  });
}); 


// Helper function to approximate equality (equivalent to assert_aprx! macro)
function assertApprox(actual: number, expected: number, epsilon: number = 0.01): void {
  const diff = Math.abs(actual - expected);
  if (diff > epsilon) {
    throw new Error(`assertion failed: (${actual} ≈ ${expected}) (epsilon: ${epsilon}, diff: ${diff})`);
  }
}