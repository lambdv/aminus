use wasm_bindgen::prelude::*;
use aminus::functions::formulas::formulas::*;
use aminus::functions::dmg_function::DMGFunction;
use crate::stat_table::StatTable;
use crate::utils::{element_from_id, damage_type_from_id, scaling_from_id, amplifier_from_id};

// ============================================================================
// Formula functions that work with StatTable wrappers
// ============================================================================

#[wasm_bindgen]
pub struct Formulas;

#[wasm_bindgen]
impl Formulas {
    #[wasm_bindgen(js_name = "totalATK")]
    pub fn calculate_total_atk_from_table(stats: &StatTable) -> f32 {
        total_atk(&stats.inner)
    }

    #[wasm_bindgen(js_name = "totalDEF")]
    pub fn calculate_total_def_from_table(stats: &StatTable) -> f32 {
        total_def(&stats.inner)
    }

    #[wasm_bindgen(js_name = "totalHP")]
    pub fn calculate_total_hp_from_table(stats: &StatTable) -> f32 {
        total_hp(&stats.inner)
    }

    #[wasm_bindgen(js_name = "avgCritMultiplier")]
    pub fn calculate_avg_crit_multiplier_from_table(stats: &StatTable) -> f32 {
        avg_crit_multiplier(&stats.inner)
    }

    #[wasm_bindgen(js_name = "defMultiplier")]
    pub fn calculate_def_multiplier(character_level: i8, enemy_level: i8, def_reduction: f32, def_ignore: f32) -> f32 {
        def_multiplier(character_level, enemy_level, def_reduction, def_ignore)
    }

    #[wasm_bindgen(js_name = "resMultiplier")]
    pub fn calculate_res_multiplier(enemy_base_resistance: f32, resistance_reduction: f32) -> f32 {
        res_multiplier(enemy_base_resistance, resistance_reduction)
    }

    #[wasm_bindgen(js_name = "fullDamageFormula")]
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

    #[wasm_bindgen(js_name = "amplifierMultiplier")]
    pub fn amplifier_multiplier(amplifier: f32, elemental_mastery: f32, reaction_bonus: f32) -> f32 {
        amplifier_multiplier(amplifier, elemental_mastery, reaction_bonus)
    }

    #[wasm_bindgen(js_name = "calculateDamage")]
    pub fn calculate_damage(
        element_id: u32,
        damage_type_id: u32,
        scaling_id: u32,
        amplifier_id: u32,
        instances: f32,
        motion_value: f32,
        character: &StatTable,
        buffs: Option<StatTable>,
    ) -> Result<f32, JsValue> {
        let element = element_from_id(element_id)?;
        let damage_type = damage_type_from_id(damage_type_id)?;
        let scaling = scaling_from_id(scaling_id)?;
        let amplifier = amplifier_from_id(amplifier_id)?;
        
        let buffs_ref = buffs.as_ref().map(|b| &b.inner);
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
}