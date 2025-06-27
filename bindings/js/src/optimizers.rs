use wasm_bindgen::prelude::*;
use aminus::functions::optimizers::optimizers::*;
use aminus::model::rotation::Rotation as RustRotation;
use aminus::model::operation::Operation;
use aminus::functions::dmg_function::DMGFunction;
use crate::stat_table::StatTable;
use crate::utils::{element_from_id, damage_type_from_id, scaling_from_id, amplifier_from_id, stat_from_id};

// ============================================================================
// Rotation WASM wrapper
// ============================================================================

#[wasm_bindgen]
pub struct Rotation {
    pub(crate) inner: RustRotation,
}

#[wasm_bindgen]
impl Rotation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Rotation {
        Rotation {
            inner: RustRotation::new(),
        }
    }

    #[wasm_bindgen(js_name = "addDamageOperation")]
    pub fn add_damage_operation(
        &mut self,
        name: String,
        element_id: u32,
        damage_type_id: u32,
        scaling_id: u32,
        amplifier_id: u32,
        instances: f32,
        motion_value: f32,
    ) -> Result<(), JsValue> {
        let element = element_from_id(element_id)?;
        let damage_type = damage_type_from_id(damage_type_id)?;
        let scaling = scaling_from_id(scaling_id)?;
        let amplifier = amplifier_from_id(amplifier_id)?;

        let operation: Operation = Box::new(move |stats| {
            DMGFunction::calculate_damage(
                element,
                damage_type,
                scaling,
                amplifier,
                instances,
                motion_value,
                Box::new(stats),
                None,
            )
        });

        self.inner.add(name, operation);
        Ok(())
    }

    #[wasm_bindgen(js_name = "evaluate")]
    pub fn evaluate(&self, stats: &StatTable) -> f32 {
        self.inner.evaluate(&stats.inner)
    }
}

// ============================================================================
// Optimizer functions
// ============================================================================

#[wasm_bindgen]
pub struct Optimizers;

#[wasm_bindgen]
impl Optimizers {
    #[wasm_bindgen(js_name = "globalKqmcArtifactMainStatOptimizer")]
    pub fn global_kqmc_artifact_main_stat_optimizer(
        stats: &StatTable,
        target: &Rotation,
    ) -> Result<js_sys::Array, JsValue> {
        let result = global_kqmc_artifact_main_stat_optimizer(&stats.inner, &target.inner);
        
        let array = js_sys::Array::new();
        array.push(&JsValue::from(result.0 as u32));
        array.push(&JsValue::from(result.1 as u32));
        array.push(&JsValue::from(result.2 as u32));
        
        Ok(array)
    }

    #[wasm_bindgen(js_name = "gradient5StarKqmcArtifactSubstatOptimizer")]
    pub fn gradient_5_star_kqmc_artifact_substat_optimizer(
        stats: &StatTable,
        target: &Rotation,
        flower: Option<crate::artifact::ArtifactPiece>,
        feather: Option<crate::artifact::ArtifactPiece>,
        sands: Option<crate::artifact::ArtifactPiece>,
        goblet: Option<crate::artifact::ArtifactPiece>,
        circlet: Option<crate::artifact::ArtifactPiece>,
        energy_recharge_requirements: f32,
    ) -> Result<js_sys::Object, JsValue> {
        let flower_inner = flower.map(|f| f.inner);
        let feather_inner = feather.map(|f| f.inner);
        let sands_inner = sands.map(|f| f.inner);
        let goblet_inner = goblet.map(|f| f.inner);
        let circlet_inner = circlet.map(|f| f.inner);

        let result = gradient_5_star_kqmc_artifact_substat_optimizer(
            &stats.inner,
            &target.inner,
            flower_inner,
            feather_inner,
            sands_inner,
            goblet_inner,
            circlet_inner,
            energy_recharge_requirements,
        );

        // Convert HashMap to JavaScript object
        let obj = js_sys::Object::new();
        for (stat, count) in result {
            let key = JsValue::from(stat as u32);
            let value = JsValue::from(count);
            js_sys::Reflect::set(&obj, &key, &value)?;
        }

        Ok(obj)
    }

    #[wasm_bindgen(js_name = "statGradients")]
    pub fn stat_gradients(
        base: &StatTable,
        target: &Rotation,
        slopes: &js_sys::Object,
    ) -> Result<js_sys::Object, JsValue> {
        // Convert JavaScript object to HashMap
        let mut slopes_map = std::collections::HashMap::new();
        let keys = js_sys::Object::keys(slopes);
        
        for i in 0..keys.length() {
            let key = keys.get(i);
            let value = js_sys::Reflect::get(slopes, &key)?;
            
            let stat_id = key.as_f64().ok_or("Invalid stat ID")? as u32;
            let slope_value = value.as_f64().ok_or("Invalid slope value")? as f32;
            
            let stat = stat_from_id(stat_id)?;
            slopes_map.insert(stat, slope_value);
        }

        let gradients = stat_gradients(&base.inner, &target.inner, &slopes_map);

        // Convert HashMap back to JavaScript object
        let obj = js_sys::Object::new();
        for (stat, gradient) in gradients {
            let key = JsValue::from(stat as u32);
            let value = JsValue::from(gradient);
            js_sys::Reflect::set(&obj, &key, &value)?;
        }

        Ok(obj)
    }

    #[wasm_bindgen(js_name = "reluHeuristic")]
    pub fn relu_heuristic(
        base: &StatTable,
        target: &Rotation,
        slopes: &js_sys::Object,
    ) -> Result<js_sys::Array, JsValue> {
        // Convert JavaScript object to HashMap
        let mut slopes_map = std::collections::HashMap::new();
        let keys = js_sys::Object::keys(slopes);
        
        for i in 0..keys.length() {
            let key = keys.get(i);
            let value = js_sys::Reflect::get(slopes, &key)?;
            
            let stat_id = key.as_f64().ok_or("Invalid stat ID")? as u32;
            let slope_value = value.as_f64().ok_or("Invalid slope value")? as f32;
            
            let stat = stat_from_id(stat_id)?;
            slopes_map.insert(stat, slope_value);
        }

        let effective_stats = relu_heuristic(&base.inner, &target.inner, &slopes_map);

        // Convert HashSet to JavaScript array
        let array = js_sys::Array::new();
        for stat in effective_stats {
            array.push(&JsValue::from(stat as u32));
        }

        Ok(array)
    }
} 