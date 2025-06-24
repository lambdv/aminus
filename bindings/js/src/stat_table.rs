use wasm_bindgen::prelude::*;
use js_sys;
use aminus::model::stattable::StatTable as RustStatTable;
use aminus::model::statable::{Statable, ModifiableStatable};
use crate::utils::stat_from_id;

// ============================================================================
// StatTable WASM wrapper
// ============================================================================

#[wasm_bindgen]
pub struct StatTable {
    pub(crate) inner: RustStatTable,
}

#[wasm_bindgen]
impl StatTable {
    #[wasm_bindgen(constructor)]
    pub fn new() -> StatTable {
        StatTable {
            inner: RustStatTable::new(),
        }
    }

    #[wasm_bindgen(js_name = "of")]
    pub fn of(stats: &js_sys::Array) -> Result<StatTable, JsValue> {
        let mut table = RustStatTable::new();
        for i in 0..stats.length() {
            let item = stats.get(i);
            let array = js_sys::Array::from(&item);
            if array.length() != 2 {
                return Err(JsValue::from_str("Each stat entry must be [stat, value]"));
            }
            
            let stat_val = array.get(0).as_f64().ok_or("Invalid stat enum value")? as u32;
            let value = array.get(1).as_f64().ok_or("Invalid stat value")? as f32;
            
            let stat = stat_from_id(stat_val)?;
            table.add(&stat, value);
        }
        Ok(StatTable { inner: table })
    }

    #[wasm_bindgen]
    pub fn get(&self, stat_id: u32) -> f32 {
        if let Ok(stat) = stat_from_id(stat_id) {
            self.inner.get(&stat)
        } else {
            0.0
        }
    }

    #[wasm_bindgen]
    pub fn add(&mut self, stat_id: u32, value: f32) -> f32 {
        if let Ok(stat) = stat_from_id(stat_id) {
            self.inner.add(&stat, value)
        } else {
            0.0
        }
    }

    #[wasm_bindgen(js_name = "addTable")]
    pub fn add_table(&mut self, other: &StatTable) {
        self.inner.add_table(other.inner.iter());
    }

    #[wasm_bindgen(js_name = "toArray")]
    pub fn to_array(&self) -> js_sys::Array {
        let array = js_sys::Array::new();
        for (stat, value) in self.inner.iter() {
            let entry = js_sys::Array::new();
            entry.push(&JsValue::from(stat as u32));
            entry.push(&JsValue::from(value));
            array.push(&entry);
        }
        array
    }
} 