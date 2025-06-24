// Enums matching the Rust enums
export enum Stat {
  BaseHP = 0,
  FlatHP = 1,
  HPPercent = 2,
  BaseATK = 3,
  FlatATK = 4,
  ATKPercent = 5,
  BaseDEF = 6,
  FlatDEF = 7,
  DEFPercent = 8,
  ElementalMastery = 9,
  CritRate = 10,
  CritDMG = 11,
  EnergyRecharge = 12,
  DMGBonus = 13,
  ElementalDMGBonus = 14,
  PyroDMGBonus = 15,
  CryoDMGBonus = 16,
  GeoDMGBonus = 17,
  DendroDMGBonus = 18,
  ElectroDMGBonus = 19,
  HydroDMGBonus = 20,
  AnemoDMGBonus = 21,
  PhysicalDMGBonus = 22,
  NormalATKDMGBonus = 23,
  ChargeATKDMGBonus = 24,
  PlungeATKDMGBonus = 25,
  SkillDMGBonus = 26,
  BurstDMGBonus = 27,
  HealingBonus = 28,
  None = 29,
  ReactionBonus = 30,
  DefReduction = 31,
  DefIgnore = 32,
  PyroResistanceReduction = 33,
  HydroResistanceReduction = 34,
  ElectroResistanceReduction = 35,
  CryoResistanceReduction = 36,
  AnemoResistanceReduction = 37,
  GeoResistanceReduction = 38,
  DendroResistanceReduction = 39,
  PhysicalResistanceReduction = 40,
}

export enum Element {
  Pyro = 0,
  Hydro = 1,
  Electro = 2,
  Cryo = 3,
  Anemo = 4,
  Geo = 5,
  Dendro = 6,
  Physical = 7,
}

export enum DamageType {
  Normal = 0,
  Charged = 1,
  Plunging = 2,
  Skill = 3,
  Burst = 4,
}

export enum BaseScaling {
  ATK = 0,
  DEF = 1,
  HP = 2,
}

export enum Amplifier {
  None = 0,
  Forward = 1,
  Reverse = 2,
}

export enum RollQuality {
  LOW = 0,
  MID = 1,
  HIGH = 2,
  MAX = 3,
  AVG = 4,
}

// Type for stat arrays
export type StatEntry = [Stat, number];
export type StatArray = StatEntry[]; 