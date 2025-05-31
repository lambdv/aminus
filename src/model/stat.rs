/// Stat type enumeration
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, serde::Deserialize)]
pub enum Stat {
    BaseHP, 
    FlatHP, 
    HPPercent,
    BaseATK, 
    FlatATK, 
    ATKPercent,
    BaseDEF, 
    FlatDEF, 
    DEFPercent,
    ElementalMastery, 
    CritRate, 
    CritDMG,
    EnergyRecharge,
    DMGBonus, 
    ElementalDMGBonus,
    PyroDMGBonus, 
    CryoDMGBonus, 
    GeoDMGBonus, 
    DendroDMGBonus, 
    ElectroDMGBonus, 
    HydroDMGBonus, 
    AnemoDMGBonus, 
    PhysicalDMGBonus,
    NormalATKDMGBonus, 
    ChargeATKDMGBonus, 
    PlungeATKDMGBonus,
    SkillDMGBonus, 
    BurstDMGBonus, 
    HealingBonus,
    None,
    //hidden stats
    ReactionBonus,
    DefReduction,
    DefIgnore,
    PyroResistanceReduction,
    HydroResistanceReduction,
    ElectroResistanceReduction,
    CryoResistanceReduction,
    AnemoResistanceReduction,
    GeoResistanceReduction,
    DendroResistanceReduction,
    PhysicalResistanceReduction
}

pub enum Debuffs{
    DefReduction,
    DefIgnore,
    PyroResistanceReduction,
    HydroResistanceReduction,
    ElectroResistanceReduction,
    CryoResistanceReduction,
    AnemoResistanceReduction,
    GeoResistanceReduction,
    DendroResistanceReduction,
    PhysicalResistanceReduction
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum DamageType {
    Normal,
    Charged,
    Plunging,
    Skill,
    Burst,
    None,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum Element {
    Pyro,
    Hydro,
    Electro,
    Anemo,
    Geo,
    Dendro,
    Cryo,
    Physical,
    None,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum BaseScaling {
    ATK,
    DEF,
    HP,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum Amplifier {
    Forward,
    Reverse,
    None,
}

impl Amplifier {
    pub fn multiplier(&self) -> f32 {
        match self {
            Amplifier::Forward => 2.0,
            Amplifier::Reverse => 1.5,
            Amplifier::None => 1.0,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum ReactionType {
    Overloaded,
    Superconduct,
    Electrocharged,
    Swirl,
    Shattered,
    Aggravate,
    Spread,
}

