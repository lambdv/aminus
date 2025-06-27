import { 
  createStatTable, 
  createRotation, 
  createArtifactPiece,
  optimizeArtifactMainStats,
  optimizeArtifactSubstats,
  calculateStatGradients,
  calculateReluHeuristic,
  Stat,
  Element,
  DamageType,
  BaseScaling,
  Amplifier
} from '../src/index.js';

describe('Optimizer Tests', () => {
  let characterStats: any;
  let rotation: any;

  beforeEach(async () => {
    // Create base character stats for testing
    characterStats = await createStatTable();
    characterStats.set(Stat.BaseATK, 100.0);
    characterStats.set(Stat.ATKPercent, 0.5);
    characterStats.set(Stat.FlatATK, 100.0);
    characterStats.set(Stat.CritRate, 0.05);
    characterStats.set(Stat.CritDMG, 0.5);
    characterStats.set(Stat.EnergyRecharge, 1.0);

    // Create a simple rotation for testing
    rotation = await createRotation();
    rotation.addDamageOperation(
      'test_attack',
      Element.Pyro,
      DamageType.Normal,
      BaseScaling.ATK,
      Amplifier.None,
      1.0,
      1.0
    );
  });

  test('should create rotation and add damage operation', () => {
    expect(rotation).toBeDefined();
    expect(typeof rotation.evaluate).toBe('function');
  });

  test('should optimize artifact main stats', async () => {
    const result = await optimizeArtifactMainStats(characterStats, rotation);
    
    expect(Array.isArray(result)).toBe(true);
    expect(result.length).toBe(3); // [sands, goblet, circlet]
    expect(result.every((id: number) => typeof id === 'number')).toBe(true);
    expect(result.every((id: number) => id >= 0)).toBe(true);
  });

  test('should optimize artifact substats', async () => {
    // Create artifact pieces for testing
    const flower = await createArtifactPiece(5, 20, Stat.FlatHP);
    const feather = await createArtifactPiece(5, 20, Stat.FlatATK);
    const sands = await createArtifactPiece(5, 20, Stat.ATKPercent);
    const goblet = await createArtifactPiece(5, 20, Stat.PyroDMGBonus);
    const circlet = await createArtifactPiece(5, 20, Stat.CritRate);

    const result = await optimizeArtifactSubstats(
      characterStats,
      rotation,
      1.0, // energy recharge requirement
      flower,
      feather,
      sands,
      goblet,
      circlet
    );

    expect(typeof result).toBe('object');
    expect(Object.keys(result).length).toBeGreaterThan(0);
    
    // Check that all values are numbers
    for (const [_, count] of Object.entries(result)) {
      expect(typeof count).toBe('number');
      expect(count).toBeGreaterThanOrEqual(0);
    }
  });

  test('should calculate stat gradients', async () => {
    // Create slopes object
    const slopes: Record<number, number> = {
      [Stat.ATKPercent]: 1.0,
      [Stat.CritRate]: 1.0,
      [Stat.CritDMG]: 1.0,
      [Stat.ElementalMastery]: 1.0
    };

    const result = await calculateStatGradients(characterStats, rotation, slopes);

    expect(typeof result).toBe('object');
    expect(Object.keys(result).length).toBeGreaterThan(0);
    
    // Check that all values are numbers
    for (const [_, gradient] of Object.entries(result)) {
      expect(typeof gradient).toBe('number');
    }
  });

  test('should calculate ReLU heuristic', async () => {
    // Create slopes object
    const slopes: Record<number, number> = {
      [Stat.ATKPercent]: 1.0,
      [Stat.CritRate]: 1.0,
      [Stat.CritDMG]: 1.0,
      [Stat.ElementalMastery]: 1.0,
      [Stat.EnergyRecharge]: 1.0
    };

    const result = await calculateReluHeuristic(characterStats, rotation, slopes);

    expect(Array.isArray(result)).toBe(true);
    expect(result.every((id: number) => typeof id === 'number')).toBe(true);
    expect(result.every((id: number) => id >= 0)).toBe(true);
  });

  test('should handle different damage types', async () => {
    // Test with different damage types
    const skillRotation = await createRotation();
    skillRotation.addDamageOperation(
      'skill_attack',
      Element.Electro,
      DamageType.Skill,
      BaseScaling.ATK,
      Amplifier.None,
      1.0,
      2.0
    );

    const result = await optimizeArtifactMainStats(characterStats, skillRotation);
    expect(Array.isArray(result)).toBe(true);
    expect(result.length).toBe(3);
  });

  test('should handle energy recharge requirements', async () => {
    const flower = await createArtifactPiece(5, 20, Stat.FlatHP);
    const feather = await createArtifactPiece(5, 20, Stat.FlatATK);
    const sands = await createArtifactPiece(5, 20, Stat.EnergyRecharge);
    const goblet = await createArtifactPiece(5, 20, Stat.ElectroDMGBonus);
    const circlet = await createArtifactPiece(5, 20, Stat.CritRate);

    // Test with higher energy recharge requirement
    const result = await optimizeArtifactSubstats(
      characterStats,
      rotation,
      1.5, // higher energy recharge requirement
      flower,
      feather,
      sands,
      goblet,
      circlet
    );

    expect(typeof result).toBe('object');
    expect(Object.keys(result).length).toBeGreaterThan(0);
  });

  test('should evaluate rotation damage', () => {
    const damage = rotation.evaluate(characterStats);
    expect(typeof damage).toBe('number');
    expect(damage).toBeGreaterThan(0);
  });

  // Comprehensive test that includes the example functionality
  test('should perform complete optimization workflow (example functionality)', async () => {
    // Create base character stats (similar to Hu Tao)
    const characterStats = await createStatTable();
    characterStats.set(Stat.BaseATK, 106.0);
    characterStats.set(Stat.BaseHP, 15552.0);
    characterStats.set(Stat.BaseDEF, 876.0);
    characterStats.set(Stat.CritRate, 0.05);
    characterStats.set(Stat.CritDMG, 0.5);
    characterStats.set(Stat.EnergyRecharge, 1.0);

    // Add weapon stats (Dragon's Bane equivalent)
    characterStats.set(Stat.BaseATK, 454.0);
    characterStats.set(Stat.ElementalMastery, 221.0);

    // Create a rotation with a Pyro damage operation
    const rotation = await createRotation();
    rotation.addDamageOperation(
      'pyro_attack',
      Element.Pyro,
      DamageType.Normal,
      BaseScaling.ATK,
      Amplifier.None,
      1.0, // instances
      1.0  // motion value
    );

    // Test 1: Optimize artifact main stats
    const mainStatsResult = await optimizeArtifactMainStats(characterStats, rotation);
    expect(Array.isArray(mainStatsResult)).toBe(true);
    expect(mainStatsResult.length).toBe(3); // [sands, goblet, circlet]
    expect(mainStatsResult.every((id: number) => typeof id === 'number')).toBe(true);
    expect(mainStatsResult.every((id: number) => id >= 0)).toBe(true);

    // Convert stat IDs back to readable names for verification
    const statNames = Object.keys(Stat).filter(key => isNaN(Stat[key as keyof typeof Stat]));
    const mainStatNames = mainStatsResult.map((id: number) => statNames[id] || `Unknown(${id})`);
    expect(mainStatNames.length).toBe(3);
    expect(mainStatNames.every((name: string) => name !== 'Unknown')).toBe(true);

    // Test 2: Optimize artifact substats
    const flower = await createArtifactPiece(5, 20, Stat.FlatHP);
    const feather = await createArtifactPiece(5, 20, Stat.FlatATK);
    const sands = await createArtifactPiece(5, 20, Stat.ATKPercent);
    const goblet = await createArtifactPiece(5, 20, Stat.PyroDMGBonus);
    const circlet = await createArtifactPiece(5, 20, Stat.CritRate);

    const substatsResult = await optimizeArtifactSubstats(
      characterStats,
      rotation,
      1.2, // energy recharge requirement
      flower,
      feather,
      sands,
      goblet,
      circlet
    );

    expect(typeof substatsResult).toBe('object');
    expect(Object.keys(substatsResult).length).toBeGreaterThan(0);
    
    // Convert substat distribution to readable format
    const substatDistribution: Record<string, number> = {};
    for (const [statId, count] of Object.entries(substatsResult)) {
      const statName = statNames[parseInt(statId)] || `Unknown(${statId})`;
      substatDistribution[statName] = count as number;
      expect(typeof count).toBe('number');
      expect(count).toBeGreaterThanOrEqual(0);
    }

    // Test 3: Calculate stat gradients
    const slopes: Record<number, number> = {};
    const possibleSubStats = [
      Stat.HPPercent, Stat.FlatHP, Stat.ATKPercent, Stat.FlatATK,
      Stat.DEFPercent, Stat.FlatDEF, Stat.ElementalMastery,
      Stat.CritRate, Stat.CritDMG, Stat.EnergyRecharge
    ];
    
    possibleSubStats.forEach(stat => {
      slopes[stat] = 1.0; // Default slope value
    });

    const gradients = await calculateStatGradients(characterStats, rotation, slopes);
    expect(typeof gradients).toBe('object');
    expect(Object.keys(gradients).length).toBeGreaterThan(0);
    
    // Convert gradients to readable format
    const gradientDistribution: Record<string, number> = {};
    for (const [statId, gradient] of Object.entries(gradients)) {
      const statName = statNames[parseInt(statId)] || `Unknown(${statId})`;
      gradientDistribution[statName] = gradient as number;
      expect(typeof gradient).toBe('number');
    }

    // Test 4: Calculate ReLU heuristic
    const effectiveStats = await calculateReluHeuristic(characterStats, rotation, slopes);
    expect(Array.isArray(effectiveStats)).toBe(true);
    expect(effectiveStats.every((id: number) => typeof id === 'number')).toBe(true);
    expect(effectiveStats.every((id: number) => id >= 0)).toBe(true);
    
    // Convert to readable stat names
    const effectiveStatNames = effectiveStats.map((id: number) => statNames[id] || `Unknown(${id})`);
    expect(effectiveStatNames.length).toBeGreaterThan(0);
    expect(effectiveStatNames.every((name: string) => name !== 'Unknown')).toBe(true);

    // Test 5: Verify that optimization improves damage
    const beforeDamage = rotation.evaluate(characterStats);
    expect(typeof beforeDamage).toBe('number');
    expect(beforeDamage).toBeGreaterThan(0);

    // Create optimized character with artifacts (using the best main stats)
    const optimizedStats = await createStatTable();
    // Copy base stats
    for (const [stat, value] of Object.entries(characterStats)) {
      optimizedStats.set(parseInt(stat), value);
    }
    
    // Add main stat values (simplified - in reality you'd get these from the optimizer)
    optimizedStats.set(mainStatsResult[0], 0.466); // Sands main stat value
    optimizedStats.set(mainStatsResult[1], 0.466); // Goblet main stat value  
    optimizedStats.set(mainStatsResult[2], 0.311); // Circlet main stat value

    const afterDamage = rotation.evaluate(optimizedStats);
    expect(typeof afterDamage).toBe('number');
    expect(afterDamage).toBeGreaterThan(0);
    
    // The optimized damage should be greater than or equal to the base damage
    expect(afterDamage).toBeGreaterThanOrEqual(beforeDamage);
  });
}); 