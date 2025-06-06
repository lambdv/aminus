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

impl Stat{
    pub fn is_elemental_dmg_bonus(&self) -> bool{
        match &self {
            Stat::PyroDMGBonus=>true, 
            Stat::CryoDMGBonus =>true,
            Stat::GeoDMGBonus =>true,
            Stat::DendroDMGBonus =>true,
            Stat::ElectroDMGBonus =>true,
            Stat::HydroDMGBonus=>true,
            Stat::AnemoDMGBonus =>true,
            _ => false
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Stat::BaseHP => "BaseHP",
            Stat::FlatHP => "FlatHP",
            Stat::HPPercent => "HPPercent",
            Stat::BaseATK => "BaseATK",
            Stat::FlatATK => "FlatATK",
            Stat::ATKPercent => "ATKPercent",
            Stat::BaseDEF => "BaseDEF",
            Stat::FlatDEF => "FlatDEF",
            Stat::DEFPercent => "DEFPercent",
            Stat::ElementalMastery => "ElementalMastery",
            Stat::CritRate => "CritRate",
            Stat::CritDMG => "CritDMG",
            Stat::EnergyRecharge => "EnergyRecharge",
            Stat::DMGBonus => "DMGBonus",
            Stat::ElementalDMGBonus => "ElementalDMGBonus",
            Stat::PyroDMGBonus => "PyroDMGBonus",
            Stat::CryoDMGBonus => "CryoDMGBonus",
            Stat::GeoDMGBonus => "GeoDMGBonus",
            Stat::DendroDMGBonus => "DendroDMGBonus",
            Stat::ElectroDMGBonus => "ElectroDMGBonus",
            Stat::HydroDMGBonus => "HydroDMGBonus",
            Stat::AnemoDMGBonus => "AnemoDMGBonus",
            Stat::PhysicalDMGBonus => "PhysicalDMGBonus",
            Stat::NormalATKDMGBonus => "NormalATKDMGBonus",
            Stat::ChargeATKDMGBonus => "ChargeATKDMGBonus",
            Stat::PlungeATKDMGBonus => "PlungeATKDMGBonus",
            Stat::SkillDMGBonus => "SkillDMGBonus",
            Stat::BurstDMGBonus => "BurstDMGBonus",
            Stat::HealingBonus => "HealingBonus",
            Stat::None => "None",
            Stat::ReactionBonus => "ReactionBonus",
            Stat::DefReduction => "DefReduction",
            Stat::DefIgnore => "DefIgnore",
            Stat::PyroResistanceReduction => "PyroResistanceReduction",
            Stat::HydroResistanceReduction => "HydroResistanceReduction",
            Stat::ElectroResistanceReduction => "ElectroResistanceReduction",
            Stat::CryoResistanceReduction => "CryoResistanceReduction",
            Stat::AnemoResistanceReduction => "AnemoResistanceReduction",
            Stat::GeoResistanceReduction => "GeoResistanceReduction",
            Stat::DendroResistanceReduction => "DendroResistanceReduction",
            Stat::PhysicalResistanceReduction => "PhysicalResistanceReduction",
        }
    }
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

impl std::fmt::Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.as_str();
        write!(f, "{s}")
    }
}