import { Stat, Element, DamageType, BaseScaling, Amplifier } from '../src/types';
import { describe, test, expect, beforeAll, beforeEach } from '@jest/globals';

let wasm: any;

beforeAll(async () => {
  wasm = await import('../pkg/aminus_js.js');
});

describe('Optimizer Tests', () => {
  let characterStats: any;
  let rotation: any;

  beforeEach(() => {
    characterStats = new wasm.StatTable();
    characterStats.add(Stat.BaseATK, 100.0);
    characterStats.add(Stat.ATKPercent, 0.5);
    characterStats.add(Stat.FlatATK, 100.0);
    characterStats.add(Stat.CritRate, 0.05);
    characterStats.add(Stat.CritDMG, 0.5);
    characterStats.add(Stat.EnergyRecharge, 1.0);

    rotation = new wasm.Rotation();
    rotation.add('test_attack', (s: any) => s.get(Stat.FlatATK));
  });

  test('should create rotation and add damage operation', () => {
    expect(rotation).toBeDefined();
    expect(typeof rotation.evaluate).toBe('function');
  });

  test('should optimize artifact main stats', () => {
    const result = wasm.Optimizers.globalKqmcArtifactMainStatOptimizer(characterStats, rotation);
    expect(Array.isArray(result)).toBe(true);
    expect(result.length).toBe(3); // [sands, goblet, circlet]
    expect(result.every((id: number) => typeof id === 'number')).toBe(true);
    expect(result.every((id: number) => id >= 0)).toBe(true);
  });

  test('should optimize artifact substats', () => {
    const flower = new wasm.ArtifactPiece(5, 20, Stat.FlatHP);
    const feather = new wasm.ArtifactPiece(5, 20, Stat.FlatATK);
    const sands = new wasm.ArtifactPiece(5, 20, Stat.ATKPercent);
    const goblet = new wasm.ArtifactPiece(5, 20, Stat.PyroDMGBonus);
    const circlet = new wasm.ArtifactPiece(5, 20, Stat.CritRate);

    const result = wasm.Optimizers.gradient5StarKqmcArtifactSubstatOptimizer(
      characterStats,
      rotation,
      flower,
      feather,
      sands,
      goblet,
      circlet,
      1.0
    );

    expect(typeof result).toBe('object');
    expect(Object.keys(result).length).toBeGreaterThan(0);
    for (const [_, count] of Object.entries(result)) {
      expect(typeof count).toBe('number');
      expect(count).toBeGreaterThanOrEqual(0);
    }
  });

  test('should calculate stat gradients', () => {
    const slopes: Record<number, number> = {
      [Stat.ATKPercent]: 1.0,
      [Stat.CritRate]: 1.0,
      [Stat.CritDMG]: 1.0,
      [Stat.ElementalMastery]: 1.0
    };
    const result = wasm.Optimizers.statGradients(characterStats, rotation, slopes);
    expect(typeof result).toBe('object');
    expect(Object.keys(result).length).toBeGreaterThan(0);
    for (const [_, gradient] of Object.entries(result)) {
      expect(typeof gradient).toBe('number');
    }
  });

  test('should calculate ReLU heuristic', () => {
    const slopes: Record<number, number> = {
      [Stat.ATKPercent]: 1.0,
      [Stat.CritRate]: 1.0,
      [Stat.CritDMG]: 1.0,
      [Stat.ElementalMastery]: 1.0,
      [Stat.EnergyRecharge]: 1.0
    };
    const result = wasm.Optimizers.reluHeuristic(characterStats, rotation, slopes);
    expect(Array.isArray(result)).toBe(true);
    expect(result.every((id: number) => typeof id === 'number')).toBe(true);
    expect(result.every((id: number) => id >= 0)).toBe(true);
  });

  test('should handle different damage types', () => {
    const skillRotation = new wasm.Rotation();
    skillRotation.add('skill_attack', (s: any) => s.get(Stat.BaseATK) * 2);
    const result = wasm.Optimizers.globalKqmcArtifactMainStatOptimizer(characterStats, skillRotation);
    expect(Array.isArray(result)).toBe(true);
    expect(result.length).toBe(3);
  });

  test('should handle energy recharge requirements', () => {
    const flower = new wasm.ArtifactPiece(5, 20, Stat.FlatHP);
    const feather = new wasm.ArtifactPiece(5, 20, Stat.FlatATK);
    const sands = new wasm.ArtifactPiece(5, 20, Stat.EnergyRecharge);
    const goblet = new wasm.ArtifactPiece(5, 20, Stat.ElectroDMGBonus);
    const circlet = new wasm.ArtifactPiece(5, 20, Stat.CritRate);
    const result = wasm.Optimizers.gradient5StarKqmcArtifactSubstatOptimizer(
      characterStats,
      rotation,
      flower,
      feather,
      sands,
      goblet,
      circlet,
      1.5
    );
    expect(typeof result).toBe('object');
    expect(Object.keys(result).length).toBeGreaterThan(0);
  });

  test('should evaluate rotation damage', () => {
    const damage = rotation.evaluate(characterStats);
    expect(typeof damage).toBe('number');
    expect(damage).toBeGreaterThan(0);
  });

  test('should perform complete optimization workflow (example functionality)', () => {
    const characterStats = new wasm.StatTable();
    characterStats.add(Stat.BaseATK, 106.0);
    characterStats.add(Stat.BaseHP, 15552.0);
    characterStats.add(Stat.BaseDEF, 876.0);
    characterStats.add(Stat.CritRate, 0.05);
    characterStats.add(Stat.CritDMG, 0.5);
    characterStats.add(Stat.EnergyRecharge, 1.0);
    characterStats.add(Stat.BaseATK, 454.0);
    characterStats.add(Stat.ElementalMastery, 221.0);
    const rotation = new wasm.Rotation();
    rotation.add('pyro_attack', (s: any) => s.get(Stat.BaseATK) + s.get(Stat.FlatATK));
    const mainStatsResult = wasm.Optimizers.globalKqmcArtifactMainStatOptimizer(characterStats, rotation);
    expect(Array.isArray(mainStatsResult)).toBe(true);
    expect(mainStatsResult.length).toBe(3);
    expect(mainStatsResult.every((id: number) => typeof id === 'number')).toBe(true);
    expect(mainStatsResult.every((id: number) => id >= 0)).toBe(true);
    const statNames = Object.keys(Stat).filter(key => isNaN(Stat[key as keyof typeof Stat]));
    const mainStatNames = mainStatsResult.map((id: number) => statNames[id] || `Unknown(${id})`);
    expect(mainStatNames.length).toBe(3);
    expect(mainStatNames.every((name: string) => name !== 'Unknown')).toBe(true);
    const flower = new wasm.ArtifactPiece(5, 20, Stat.FlatHP);
    const feather = new wasm.ArtifactPiece(5, 20, Stat.FlatATK);
    const sands = new wasm.ArtifactPiece(5, 20, Stat.ATKPercent);
    const goblet = new wasm.ArtifactPiece(5, 20, Stat.PyroDMGBonus);
    const circlet = new wasm.ArtifactPiece(5, 20, Stat.CritRate);
    const substatsResult = wasm.Optimizers.gradient5StarKqmcArtifactSubstatOptimizer(
      characterStats,
      rotation,
      flower,
      feather,
      sands,
      goblet,
      circlet,
      1.2
    );
    expect(typeof substatsResult).toBe('object');
    expect(Object.keys(substatsResult).length).toBeGreaterThan(0);
    const substatDistribution: Record<string, number> = {};
    for (const [statId, count] of Object.entries(substatsResult)) {
      const statName = statNames[parseInt(statId)] || `Unknown(${statId})`;
      substatDistribution[statName] = count as number;
      expect(typeof count).toBe('number');
      expect(count).toBeGreaterThanOrEqual(0);
    }
    const slopes: Record<number, number> = {};
    const possibleSubStats = [
      Stat.HPPercent, Stat.FlatHP, Stat.ATKPercent, Stat.FlatATK,
      Stat.DEFPercent, Stat.FlatDEF, Stat.ElementalMastery,
      Stat.CritRate, Stat.CritDMG, Stat.EnergyRecharge
    ];
    possibleSubStats.forEach(stat => {
      slopes[stat] = 1.0;
    });
    const gradients = wasm.Optimizers.statGradients(characterStats, rotation, slopes);
    expect(typeof gradients).toBe('object');
    expect(Object.keys(gradients).length).toBeGreaterThan(0);
    const gradientDistribution: Record<string, number> = {};
    for (const [statId, gradient] of Object.entries(gradients)) {
      const statName = statNames[parseInt(statId)] || `Unknown(${statId})`;
      gradientDistribution[statName] = gradient as number;
      expect(typeof gradient).toBe('number');
    }
    const effectiveStats = wasm.Optimizers.reluHeuristic(characterStats, rotation, slopes);
    expect(Array.isArray(effectiveStats)).toBe(true);
    expect(effectiveStats.every((id: number) => typeof id === 'number')).toBe(true);
    expect(effectiveStats.every((id: number) => id >= 0)).toBe(true);
    const effectiveStatNames = effectiveStats.map((id: number) => statNames[id] || `Unknown(${id})`);
    expect(effectiveStatNames.length).toBeGreaterThan(0);
    expect(effectiveStatNames.every((name: string) => name !== 'Unknown')).toBe(true);
    const beforeDamage = rotation.evaluate(characterStats);
    expect(typeof beforeDamage).toBe('number');
    expect(beforeDamage).toBeGreaterThan(0);
    const optimizedStats = new wasm.StatTable();
    optimizedStats.add(Stat.BaseATK, characterStats.get(Stat.BaseATK));
    optimizedStats.add(Stat.BaseHP, characterStats.get(Stat.BaseHP));
    optimizedStats.add(Stat.BaseDEF, characterStats.get(Stat.BaseDEF));
    optimizedStats.add(Stat.CritRate, characterStats.get(Stat.CritRate));
    optimizedStats.add(Stat.CritDMG, characterStats.get(Stat.CritDMG));
    optimizedStats.add(Stat.EnergyRecharge, characterStats.get(Stat.EnergyRecharge));
    optimizedStats.add(Stat.ElementalMastery, characterStats.get(Stat.ElementalMastery));
    optimizedStats.add(mainStatsResult[0], 0.466);
    optimizedStats.add(mainStatsResult[1], 0.466);
    optimizedStats.add(mainStatsResult[2], 0.311);
    const afterDamage = rotation.evaluate(optimizedStats);
    expect(typeof afterDamage).toBe('number');
    expect(afterDamage).toBeGreaterThan(0);
    expect(afterDamage).toBeGreaterThanOrEqual(beforeDamage);
  });
}); 