import { Stat, Element, DamageType, BaseScaling, Amplifier } from './types';

// Rotation class for defining damage operations
export declare class Rotation {
  constructor();
  
  /**
   * Add a damage operation to the rotation
   * @param name - Name of the operation
   * @param op - A function that takes a StatTable and returns a number
   */
  add(
    name: string,
    op: (s: StatTable) => number
  ): void;
  
  /**
   * Evaluate the rotation against given stats
   * @param stats - StatTable containing character stats
   * @returns Total damage value
   */
  evaluate(stats: any): number;
}

// Optimizers class containing optimization algorithms
export declare class Optimizers {
  constructor();
  
  /**
   * Find the best artifact main stat combination for a character
   * @param stats - Character base stats
   * @param target - Rotation to optimize for
   * @returns Array of [sands, goblet, circlet] stat IDs
   */
  globalKqmcArtifactMainStatOptimizer(stats: any, target: Rotation): number[];
  
  /**
   * Optimize artifact substats using gradient descent
   * @param stats - Character base stats
   * @param target - Rotation to optimize for
   * @param flower - Flower artifact piece (optional)
   * @param feather - Feather artifact piece (optional)
   * @param sands - Sands artifact piece (optional)
   * @param goblet - Goblet artifact piece (optional)
   * @param circlet - Circlet artifact piece (optional)
   * @param energyRechargeRequirements - Required energy recharge value
   * @returns Object mapping stat IDs to roll counts
   */
  gradient5StarKqmcArtifactSubstatOptimizer(
    stats: any,
    target: Rotation,
    flower?: any,
    feather?: any,
    sands?: any,
    goblet?: any,
    circlet?: any,
    energyRechargeRequirements: number
  ): Record<number, number>;
  
  /**
   * Calculate gradients for each stat based on their impact on damage
   * @param base - Base stats
   * @param target - Rotation to evaluate
   * @param slopes - Object mapping stat IDs to slope values
   * @returns Object mapping stat IDs to gradient values
   */
  statGradients(base: any, target: Rotation, slopes: Record<number, number>): Record<number, number>;
  
  /**
   * Find stats that actually increase damage (ReLU heuristic)
   * @param base - Base stats
   * @param target - Rotation to evaluate
   * @param slopes - Object mapping stat IDs to slope values
   * @returns Array of effective stat IDs
   */
  reluHeuristic(base: any, target: Rotation, slopes: Record<number, number>): number[];
}

// Convenience function types
export declare function createRotation(): Promise<Rotation>;
export declare function createOptimizers(): Promise<Optimizers>;

export declare function optimizeArtifactMainStats(stats: any, target: Rotation): Promise<number[]>;
export declare function optimizeArtifactSubstats(
  stats: any,
  target: Rotation,
  energyRechargeRequirements?: number,
  flower?: any,
  feather?: any,
  sands?: any,
  goblet?: any,
  circlet?: any
): Promise<Record<number, number>>;

export declare function calculateStatGradients(
  base: any, 
  target: Rotation, 
  slopes: Record<number, number>
): Promise<Record<number, number>>;

export declare function calculateReluHeuristic(
  base: any, 
  target: Rotation, 
  slopes: Record<number, number>
): Promise<number[]>; 