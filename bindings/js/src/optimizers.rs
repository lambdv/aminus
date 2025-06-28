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

    /// Add a JS function as an operation to the rotation
    #[wasm_bindgen(js_name = "add")]
    pub fn add_js_operation(&mut self, name: String, op: &js_sys::Function) -> Result<(), JsValue> {
        // Clone the function for the closure
        let op = op.clone();
        // The closure must take a StatTable and return a f32
        let operation: Operation = Box::new(move |stats| {
            // Create a new StatTable from the Statable by iterating through its stats
            let mut js_stats = crate::stat_table::StatTable::new();
            
            // Copy all stats from the Statable to the JS StatTable
            for (stat, value) in stats.iter() {
                js_stats.add(stat as u32, value);
            }
            
            // Call the JS function
            let this = JsValue::NULL;
            let arg = wasm_bindgen::JsValue::from(js_stats);
            let result = op.call1(&this, &arg);
            match result {
                Ok(val) => val.as_f64().unwrap_or(0.0) as f32,
                Err(_) => 0.0,
            }
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
            
            // Convert string key to number
            let stat_id = if key.is_string() {
                key.as_string().unwrap().parse::<u32>().map_err(|_| JsValue::from_str("Invalid stat ID"))?
            } else {
                key.as_f64().ok_or("Invalid stat ID")? as u32
            };
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
            
            // Convert string key to number
            let stat_id = if key.is_string() {
                key.as_string().unwrap().parse::<u32>().map_err(|_| JsValue::from_str("Invalid stat ID"))?
            } else {
                key.as_f64().ok_or("Invalid stat ID")? as u32
            };
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