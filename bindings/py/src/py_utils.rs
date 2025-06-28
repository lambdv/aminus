use pyo3::prelude::*;
use std::str::FromStr;
use aminus::model::stat::{Stat, Element, DamageType, BaseScaling, Amplifier};
use aminus::model::artifact_builder::RollQuality;

// ============================================================================
// Helper functions to convert IDs to enums
// ============================================================================

pub fn rust_stat_from_id(id: u32) -> PyResult<Stat> {
    match id {
        0 => Ok(Stat::BaseHP),
        1 => Ok(Stat::FlatHP),
        2 => Ok(Stat::HPPercent),
        3 => Ok(Stat::BaseATK),
        4 => Ok(Stat::FlatATK),
        5 => Ok(Stat::ATKPercent),
        6 => Ok(Stat::BaseDEF),
        7 => Ok(Stat::FlatDEF),
        8 => Ok(Stat::DEFPercent),
        9 => Ok(Stat::ElementalMastery),
        10 => Ok(Stat::CritRate),
        11 => Ok(Stat::CritDMG),
        12 => Ok(Stat::EnergyRecharge),
        13 => Ok(Stat::DMGBonus),
        14 => Ok(Stat::ElementalDMGBonus),
        15 => Ok(Stat::PyroDMGBonus),
        16 => Ok(Stat::CryoDMGBonus),
        17 => Ok(Stat::GeoDMGBonus),
        18 => Ok(Stat::DendroDMGBonus),
        19 => Ok(Stat::ElectroDMGBonus),
        20 => Ok(Stat::HydroDMGBonus),
        21 => Ok(Stat::AnemoDMGBonus),
        22 => Ok(Stat::PhysicalDMGBonus),
        23 => Ok(Stat::NormalATKDMGBonus),
        24 => Ok(Stat::ChargeATKDMGBonus),
        25 => Ok(Stat::PlungeATKDMGBonus),
        26 => Ok(Stat::SkillDMGBonus),
        27 => Ok(Stat::BurstDMGBonus),
        28 => Ok(Stat::HealingBonus),
        29 => Ok(Stat::None),
        30 => Ok(Stat::ReactionBonus),
        31 => Ok(Stat::DefReduction),
        32 => Ok(Stat::DefIgnore),
        33 => Ok(Stat::PyroResistanceReduction),
        34 => Ok(Stat::HydroResistanceReduction),
        35 => Ok(Stat::ElectroResistanceReduction),
        36 => Ok(Stat::CryoResistanceReduction),
        37 => Ok(Stat::AnemoResistanceReduction),
        38 => Ok(Stat::GeoResistanceReduction),
        39 => Ok(Stat::DendroResistanceReduction),
        40 => Ok(Stat::PhysicalResistanceReduction),
        _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid stat ID")),
    }
}

pub fn rust_element_from_id(id: u32) -> PyResult<Element> {
    match id {
        0 => Ok(Element::Pyro),
        1 => Ok(Element::Hydro),
        2 => Ok(Element::Electro),
        3 => Ok(Element::Cryo),
        4 => Ok(Element::Anemo),
        5 => Ok(Element::Geo),
        6 => Ok(Element::Dendro),
        7 => Ok(Element::Physical),
        _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid element ID")),
    }
}

pub fn rust_damage_type_from_id(id: u32) -> PyResult<DamageType> {
    match id {
        0 => Ok(DamageType::Normal),
        1 => Ok(DamageType::Charged),
        2 => Ok(DamageType::Plunging),
        3 => Ok(DamageType::Skill),
        4 => Ok(DamageType::Burst),
        _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid damage type ID")),
    }
}

pub fn rust_scaling_from_id(id: u32) -> PyResult<BaseScaling> {
    match id {
        0 => Ok(BaseScaling::ATK),
        1 => Ok(BaseScaling::DEF),
        2 => Ok(BaseScaling::HP),
        _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid scaling ID")),
    }
}

pub fn rust_amplifier_from_id(id: u32) -> PyResult<Amplifier> {
    match id {
        0 => Ok(Amplifier::None),
        1 => Ok(Amplifier::Forward),
        2 => Ok(Amplifier::Reverse),
        _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid amplifier ID")),
    }
}

pub fn rust_quality_from_id(id: u32) -> PyResult<RollQuality> {
    match id {
        0 => Ok(RollQuality::LOW),
        1 => Ok(RollQuality::MID),
        2 => Ok(RollQuality::HIGH),
        3 => Ok(RollQuality::MAX),
        4 => Ok(RollQuality::AVG),
        _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid quality ID")),
    }
}

// ============================================================================
// Python exposed utility functions
// ============================================================================

#[pyfunction]
pub fn stat_from_id(id: u32) -> PyResult<u32> {
    rust_stat_from_id(id)?; // Validate the ID
    Ok(id)
}

#[pyfunction]
pub fn element_from_id(id: u32) -> PyResult<u32> {
    rust_element_from_id(id)?; // Validate the ID
    Ok(id)
}

#[pyfunction]
pub fn damage_type_from_id(id: u32) -> PyResult<u32> {
    rust_damage_type_from_id(id)?; // Validate the ID
    Ok(id)
}

#[pyfunction]
pub fn scaling_from_id(id: u32) -> PyResult<u32> {
    rust_scaling_from_id(id)?; // Validate the ID
    Ok(id)
}

#[pyfunction]
pub fn amplifier_from_id(id: u32) -> PyResult<u32> {
    rust_amplifier_from_id(id)?; // Validate the ID
    Ok(id)
}

#[pyfunction]
pub fn quality_from_id(id: u32) -> PyResult<u32> {
    rust_quality_from_id(id)?; // Validate the ID
    Ok(id)
}

#[pyfunction]
pub fn stat_from_string(name: &str) -> Option<u32> {
    Stat::from_str(name).ok().map(|s| s as u32)
}

#[pyfunction]
pub fn get_stat_name(stat_id: u32) -> Option<String> {
    rust_stat_from_id(stat_id).ok().map(|s| s.as_str().to_string())
}

#[pyfunction]
pub fn is_elemental_dmg_bonus(stat_id: u32) -> bool {
    rust_stat_from_id(stat_id).map(|s| s.is_elemental_dmg_bonus()).unwrap_or(false)
}

#[pyfunction]
pub fn get_roll_quality_multiplier(quality_id: u32) -> PyResult<f32> {
    let quality = rust_quality_from_id(quality_id)?;
    Ok(quality.multiplier())
} 