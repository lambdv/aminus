use std::str::FromStr;
use crate::utils::standardize::*;

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

impl FromStr for Stat {
    type Err = ();
    fn from_str(name: &str) -> Result<Self, Self::Err> {
        match flatten_str(name).as_str() {
            "basehp" => Ok(Stat::BaseHP),
            "flathp" => Ok(Stat::FlatHP),
            "hppercent" => Ok(Stat::HPPercent),
            "hp" => Ok(Stat::HPPercent),
            "baseatk" => Ok(Stat::BaseATK),
            "flatatk" => Ok(Stat::FlatATK),
            "atkpercent" => Ok(Stat::ATKPercent),
            "atk" => Ok(Stat::ATKPercent),
            "basedef" => Ok(Stat::BaseDEF),
            "flatdef" => Ok(Stat::FlatDEF),
            "defpercent" => Ok(Stat::DEFPercent),
            "def" => Ok(Stat::DEFPercent),
            "elementalmastery" | "em" => Ok(Stat::ElementalMastery),
            "critrate" | "cr" => Ok(Stat::CritRate),
            "critdmg" | "cd" => Ok(Stat::CritDMG),
            "energyrecharge" | "er" => Ok(Stat::EnergyRecharge),
            "dmgbonus" => Ok(Stat::DMGBonus),
            "elementaldmgbonus" | "elementaldmg" => Ok(Stat::ElementalDMGBonus),
            "pyrodmgbonus" => Ok(Stat::PyroDMGBonus),
            "cryodmgbonus" => Ok(Stat::CryoDMGBonus),
            "geodmgbonus" => Ok(Stat::GeoDMGBonus),
            "dendrodmgbonus" => Ok(Stat::DendroDMGBonus),
            "electrodmgbonus" => Ok(Stat::ElectroDMGBonus),
            "hydrodmgbonus" => Ok(Stat::HydroDMGBonus),
            "anemodmgbonus" => Ok(Stat::AnemoDMGBonus),
            "physicaldmgbonus" | "physicaldmg" => Ok(Stat::PhysicalDMGBonus),
            "normalatkdmgbonus" => Ok(Stat::NormalATKDMGBonus),
            "chargeatkdmgbonus" => Ok(Stat::ChargeATKDMGBonus),
            "plungeatkdmgbonus" => Ok(Stat::PlungeATKDMGBonus),
            "skilldmgbonus" => Ok(Stat::SkillDMGBonus),
            "burstdmgbonus" => Ok(Stat::BurstDMGBonus),
            "healingbonus" | "hb" => Ok(Stat::HealingBonus),
            "none" | "n" => Ok(Stat::None),
            "reactionbonus" => Ok(Stat::ReactionBonus),
            "defreduction" => Ok(Stat::DefReduction),
            "defignore" => Ok(Stat::DefIgnore),
            "pyroresistancereduction" => Ok(Stat::PyroResistanceReduction),
            "hydroresistancereduction" => Ok(Stat::HydroResistanceReduction),
            "electroresistancereduction" => Ok(Stat::ElectroResistanceReduction),
            "cryoresistancereduction" => Ok(Stat::CryoResistanceReduction),
            "anemoresistancereduction" => Ok(Stat::AnemoResistanceReduction),
            "georesistancereduction" => Ok(Stat::GeoResistanceReduction),
            "dendroresistancereduction" => Ok(Stat::DendroResistanceReduction),
            "physicalresistancereduction" => Ok(Stat::PhysicalResistanceReduction),
            "physicaldmgpercent" | "physical dmg%" => Ok(Stat::PhysicalDMGBonus),
            _ => Err(())
        }
    }
}

