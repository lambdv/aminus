import { describe, test, expect } from '@jest/globals';
import { Stat, RollQuality } from '../src/types';
import {
  ArtifactBuilder,
  ArtifactPiece,
  StatFactory,
  getRollQualityMultiplier,
  getSubStatValue,
  getMainStatValue,
  getCharacterBaseStats,
  getWeaponStats
} from '../pkg/aminus_js.js';

describe('Artifact Builder Tests', () => {
  test('default artifact builder', () => {
    // Create artifact pieces
    const flower = new ArtifactPiece(5, 20, Stat.FlatHP);
    const feather = new ArtifactPiece(5, 20, Stat.FlatATK);
    const sands = new ArtifactPiece(5, 20, Stat.EnergyRecharge);
    const goblet = new ArtifactPiece(5, 20, Stat.ATKPercent);
    const circlet = new ArtifactPiece(5, 20, Stat.ATKPercent);

    // Create artifact builder
    const builder = new ArtifactBuilder(flower, feather, sands, goblet, circlet);

    // Test substat constraints
    // Builder with no artifacts with main stat of type x has constraint of 30
    expect(builder.substatConstraint(Stat.FlatDEF, 5)).toBe(30);
    expect(builder.substatConstraint(Stat.CritDMG, 5)).toBe(30);
    expect(builder.substatConstraint(Stat.CritRate, 5)).toBe(30);
    
    // Builder with 1 artifact with main stat of type x has constraint of 24
    expect(builder.substatConstraint(Stat.EnergyRecharge, 5)).toBe(24);
    expect(builder.substatConstraint(Stat.FlatATK, 5)).toBe(24);
    
    // Builder with 2 artifacts with main stat of type x has constraint of 18
    expect(builder.substatConstraint(Stat.ATKPercent, 5)).toBe(18);

    // Test main stats (data is baked into binary at compile time)
    const mainStats = builder.mainStats();
    assertApprox(mainStats.get(Stat.FlatHP), 4780.0, 0.1);
    assertApprox(mainStats.get(Stat.FlatATK), 311.0, 0.1);
    assertApprox(mainStats.get(Stat.EnergyRecharge), 0.518, 0.001);
    assertApprox(mainStats.get(Stat.ATKPercent), 0.932, 0.001); // 0.466 * 2

    // Test initial sub stats (should be empty)
    const subStats = builder.subStats();
    expect(subStats.get(Stat.CritRate)).toBe(0.0);

    // Test rolling
    builder.roll(Stat.CritRate, RollQuality.AVG, 5, 1);
    const avgMultiplier = getRollQualityMultiplier(RollQuality.AVG);
    const expectedCritRate = getSubStatValue(5, Stat.CritRate) * avgMultiplier;
    const actualCritRate = builder.subStats().get(Stat.CritRate);
    assertApprox(actualCritRate, expectedCritRate, 0.001);

    // Test rolling again (should accumulate)
    builder.roll(Stat.CritRate, RollQuality.MAX, 5, 1);
    const maxMultiplier = getRollQualityMultiplier(RollQuality.MAX);
    const additionalCritRate = getSubStatValue(5, Stat.CritRate) * maxMultiplier;
    const totalExpectedCritRate = expectedCritRate + additionalCritRate;  
    const totalActualCritRate = builder.subStats().get(Stat.CritRate);
    assertApprox(totalActualCritRate, totalExpectedCritRate, 0.001);

    // Test build (combines main and sub stats) - data is baked into binary
    const finalStats = builder.build();
    assertApprox(finalStats.get(Stat.FlatHP), 4780.0, 0.1);
    assertApprox(finalStats.get(Stat.FlatATK), 311.0, 0.1);
    assertApprox(finalStats.get(Stat.EnergyRecharge), 0.518, 0.001);
    assertApprox(finalStats.get(Stat.ATKPercent), 0.932, 0.001);
    assertApprox(finalStats.get(Stat.CritRate), totalExpectedCritRate, 0.001);
  });

  test('KQM artifact builder (5-star)', () => {
    const builder = ArtifactBuilder.kqmAll5Star(
      Stat.EnergyRecharge,  // sands
      Stat.ATKPercent,      // goblet
      Stat.ATKPercent       // circlet
    );

    // Test substat constraints for KQM builder (should be different from default)
    expect(builder.substatConstraint(Stat.FlatDEF, 5)).toBe(10+2);
    expect(builder.substatConstraint(Stat.CritDMG, 5)).toBe(10+2);
    expect(builder.substatConstraint(Stat.CritRate, 5)).toBe(10+2);
    
    // Builder with 1 artifact with main stat of type x has constraint reduced by 2
    expect(builder.substatConstraint(Stat.EnergyRecharge, 5)).toBe(8+2);
    
    // Builder with 2 artifacts with main stat of type x has constraint reduced by 4
    expect(builder.substatConstraint(Stat.ATKPercent, 5)).toBe(6+2);

    // Test that it has some rolls already (KQM pre-allocates 2 rolls per substat)
    expect(builder.currentRolls()).toBeGreaterThan(0);
    expect(builder.rollsLeft()).toBeGreaterThanOrEqual(0);
  });

  test('KQM artifact builder (4-star)', () => {
    // Note: KQM builders use factory functions for rolling substats
    const builder = ArtifactBuilder.kqmAll4Star(
      Stat.EnergyRecharge,
      Stat.ATKPercent,
      Stat.CritRate
    );

    // 4-star artifacts should have different max rolls
    const maxRolls = builder.maxRolls();
    expect(maxRolls).toBeLessThan(45); // 4-star artifacts have fewer max rolls than 5-star

    // Test that substats are properly set up
    const subStats = builder.subStats();
    expect(subStats.get(Stat.ATKPercent)).toBeGreaterThan(0);
    expect(subStats.get(Stat.CritRate)).toBeGreaterThan(0);
  });

  test('artifact piece properties', () => {
    const piece = new ArtifactPiece(5, 20, Stat.FlatHP);
    
    expect(piece.rarity).toBe(5);
    expect(piece.level).toBe(20);
    expect(piece.statType).toBe(Stat.FlatHP);
  });

  test('roll quality multipliers', () => {
    // Test that different roll qualities produce different values
    const builder = ArtifactBuilder.kqmAll5Star(
      Stat.ATKPercent,
      Stat.PyroDMGBonus,
      Stat.CritRate
    );

    // Clear existing rolls and test different qualities
    builder.unroll(Stat.CritDMG, RollQuality.AVG, 5, 10); // Clear any existing

    // Roll with different qualities
    builder.roll(Stat.CritDMG, RollQuality.LOW, 5, 1);
    const lowValue = builder.subStats().get(Stat.CritDMG);

    builder.roll(Stat.CritDMG, RollQuality.HIGH, 5, 1);
    const combinedValue = builder.subStats().get(Stat.CritDMG);

    // High quality should add more than low quality
    const highContribution = combinedValue - lowValue;
    expect(combinedValue).toBeGreaterThan(lowValue);
    expect(highContribution).toBeGreaterThan(0);
  });

  test('rolling constraints and limits', () => {
    const builder = new ArtifactBuilder(
      new ArtifactPiece(5, 20, Stat.FlatHP),
      new ArtifactPiece(5, 20, Stat.FlatATK),
      new ArtifactPiece(5, 20, Stat.EnergyRecharge),
      new ArtifactPiece(5, 20, Stat.PyroDMGBonus),
      new ArtifactPiece(5, 20, Stat.CritRate)
    );

    const initialRolls = builder.currentRolls();
    expect(initialRolls).toBe(0);

    // Test rolling
    builder.roll(Stat.FlatATK, RollQuality.AVG, 5, 1);
    expect(builder.currentRolls()).toBe(1);

    // Test rolling the same stat again
    builder.roll(Stat.FlatATK, RollQuality.AVG, 5, 1);
    expect(builder.currentRolls()).toBe(2);

    // Test rolling different quality
    builder.roll(Stat.FlatATK, RollQuality.HIGH, 5, 1);
    expect(builder.currentRolls()).toBe(3);

    // Test unrolling
    builder.unroll(Stat.FlatATK, RollQuality.AVG, 5, 1);
    expect(builder.currentRolls()).toBe(2);
  });

  test('factory functions using StatFactory struct', () => {
    const factory = new StatFactory();
    
    // Test main stat values
    const hp5Star20 = factory.getMainStatValue(5, 20, Stat.FlatHP);
    assertApprox(hp5Star20, 4780.0, 0.1);

    const atk5Star20 = factory.getMainStatValue(5, 20, Stat.FlatATK);
    assertApprox(atk5Star20, 311.0, 0.1);

    // Test sub stat values
    const critRate5Star = factory.getSubStatValue(5, Stat.CritRate);
    expect(critRate5Star).toBeGreaterThan(0);

    const atkPercent5Star = factory.getSubStatValue(5, Stat.ATKPercent);
    expect(atkPercent5Star).toBeGreaterThan(0);

    // Test character stats
    const dilucStats = factory.getCharacterBaseStats("Diluc");
    expect(dilucStats.get(Stat.BaseHP)).toBeGreaterThan(0);
    expect(dilucStats.get(Stat.BaseATK)).toBeGreaterThan(0);
    expect(dilucStats.get(Stat.BaseDEF)).toBeGreaterThan(0);

    // Test weapon stats
    const weaponStats = factory.getWeaponStats("Wolf's Gravestone");
    expect(weaponStats.get(Stat.BaseATK)).toBeGreaterThan(0);
  });

  test('legacy factory functions (backward compatibility)', () => {
    // Note: These tests now work because JSON data is baked into the binary at compile time
    
    // Test main stat values
    const hp5Star20 = getMainStatValue(5, 20, Stat.FlatHP);
    assertApprox(hp5Star20, 4780.0, 0.1);

    const atk5Star20 = getMainStatValue(5, 20, Stat.FlatATK);
    assertApprox(atk5Star20, 311.0, 0.1);

    // Test sub stat values
    const critRate5Star = getSubStatValue(5, Stat.CritRate);
    expect(critRate5Star).toBeGreaterThan(0);

    const atkPercent5Star = getSubStatValue(5, Stat.ATKPercent);
    expect(atkPercent5Star).toBeGreaterThan(0);

    // Test character stats
    const dilucStats = getCharacterBaseStats("Diluc");
    expect(dilucStats.get(Stat.BaseHP)).toBeGreaterThan(0);
    expect(dilucStats.get(Stat.BaseATK)).toBeGreaterThan(0);
    expect(dilucStats.get(Stat.BaseDEF)).toBeGreaterThan(0);

    // Test weapon stats
    const weaponStats = getWeaponStats("Wolf's Gravestone");
    expect(weaponStats.get(Stat.BaseATK)).toBeGreaterThan(0);
  });
});

function assertApprox(actual: number, expected: number, epsilon: number = 0.01): void {
  const diff = Math.abs(actual - expected);
  if (diff > epsilon) {
    throw new Error(`assertion failed: (${actual} ≈ ${expected}) (epsilon: ${epsilon}, diff: ${diff})`);
  }
}