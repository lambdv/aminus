use pyo3::prelude::*;
use aminus::functions::formulas::formulas::*;
use aminus::functions::dmg_function::DMGFunction;
use crate::py_stat_table::PyStatTable;
use crate::py_utils::{rust_element_from_id, rust_damage_type_from_id, rust_scaling_from_id, rust_amplifier_from_id};

#[pyclass]
pub struct PyFormulas;

#[pymethods]
impl PyFormulas {
    #[new]
    pub fn new() -> Self {
        PyFormulas
    }

    #[staticmethod]
    pub fn total_atk(stats: &PyStatTable) -> f32 {
        total_atk(&stats.inner)
    }

    #[staticmethod]
    pub fn total_def(stats: &PyStatTable) -> f32 {
        total_def(&stats.inner)
    }

    #[staticmethod]
    pub fn total_hp(stats: &PyStatTable) -> f32 {
        total_hp(&stats.inner)
    }

    #[staticmethod]
    pub fn avg_crit_multiplier(stats: &PyStatTable) -> f32 {
        avg_crit_multiplier(&stats.inner)
    }

    #[staticmethod]
    pub fn def_multiplier(character_level: i8, enemy_level: i8, def_reduction: f32, def_ignore: f32) -> f32 {
        def_multiplier(character_level, enemy_level, def_reduction, def_ignore)
    }

    #[staticmethod]
    pub fn res_multiplier(enemy_base_resistance: f32, resistance_reduction: f32) -> f32 {
        res_multiplier(enemy_base_resistance, resistance_reduction)
    }

    #[staticmethod]
    #[allow(clippy::too_many_arguments)]
    pub fn full_damage_formula(
        instances: f32,
        total_scaling_stat: f32,
        motion_value: f32,
        base_dmg_multiplier: f32,
        additive_base_dmg_bonus: f32,
        avg_crit_multiplier: f32,
        total_dmg_bonus: f32,
        dmg_reduction_target: f32,
        def_multiplier: f32,
        res_multiplier: f32,
        amplifier_multiplier: f32
    ) -> f32 {
        full_damage_formula(
            instances,
            total_scaling_stat,
            motion_value,
            base_dmg_multiplier,
            additive_base_dmg_bonus,
            avg_crit_multiplier,
            total_dmg_bonus,
            dmg_reduction_target,
            def_multiplier,
            res_multiplier,
            amplifier_multiplier
        )
    }

    #[staticmethod]
    pub fn amplifier_multiplier(amplifier: f32, elemental_mastery: f32, reaction_bonus: f32) -> f32 {
        amplifier_multiplier(amplifier, elemental_mastery, reaction_bonus)
    }

    #[staticmethod]
    pub fn calculate_damage(
        element_id: u32,
        damage_type_id: u32,
        scaling_id: u32,
        amplifier_id: u32,
        instances: f32,
        motion_value: f32,
        character: &PyStatTable,
        buffs: Option<&PyStatTable>,
    ) -> PyResult<f32> {
        let element = rust_element_from_id(element_id)?;
        let damage_type = rust_damage_type_from_id(damage_type_id)?;
        let scaling = rust_scaling_from_id(scaling_id)?;
        let amplifier = rust_amplifier_from_id(amplifier_id)?;
        
        let buffs_ref = buffs.map(|b| &b.inner);
        Ok(DMGFunction::calculate_damage(
            element,
            damage_type,
            scaling,
            amplifier,
            instances,
            motion_value,
            Box::new(&character.inner),
            buffs_ref,
        ))
    }

    pub fn __repr__(&self) -> String {
        "PyFormulas()".to_string()
    }
} 