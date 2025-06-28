use wasm_bindgen::prelude::*;
use aminus::functions::stat_factory::StatFactory as RustStatFactory;
use crate::stat_table::StatTable;
use crate::utils::stat_from_id;

// ============================================================================
// StatFactory struct wrapper
// ============================================================================

#[wasm_bindgen]
pub struct StatFactory;

#[wasm_bindgen]
impl StatFactory {
    #[wasm_bindgen(js_name = "getCharacterBaseStats")]
    pub fn get_character_base_stats(name: &str) -> Result<StatTable, JsValue> {
        match RustStatFactory::get_character_base_stats(name) {
            Ok(stats) => Ok(StatTable { inner: stats }),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }

    #[wasm_bindgen(js_name = "getWeaponStats")]
    pub fn get_weapon_stats(name: &str) -> Result<StatTable, JsValue> {
        match RustStatFactory::get_weapon_stats(name) {
            Ok(stats) => Ok(StatTable { inner: stats }),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }

    #[wasm_bindgen(js_name = "getMainStatValue")]
    pub fn get_main_stat_value(rarity: i8, level: i8, stat_type_id: u32) -> Result<f32, JsValue> {
        let stat_type = stat_from_id(stat_type_id)?;
        match RustStatFactory::get_main_stat_value(rarity, level, &stat_type) {
            Ok(value) => Ok(value),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }

    #[wasm_bindgen(js_name = "getSubStatValue")]
    pub fn get_sub_stat_value(rarity: i8, stat_type_id: u32) -> Result<f32, JsValue> {
        let stat_type = stat_from_id(stat_type_id)?;
        match RustStatFactory::get_sub_stat_value(rarity, stat_type) {
            Ok(value) => Ok(value),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[wasm_bindgen(js_name = "fetchCharacterBaseStats")]
    pub async fn fetch_character_base_stats(name: &str) -> Result<StatTable, JsValue> {
        match RustStatFactory::fetch_character_base_stats(name).await {
            Ok(stats) => Ok(StatTable { inner: stats }),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[wasm_bindgen(js_name = "fetchWeaponStats")]
    pub async fn fetch_weapon_stats(name: &str) -> Result<StatTable, JsValue> {
        match RustStatFactory::fetch_weapon_stats(name).await {
            Ok(stats) => Ok(StatTable { inner: stats }),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }
}

// ============================================================================
// Legacy standalone functions (for backward compatibility)
// ============================================================================

#[wasm_bindgen(js_name = "getCharacterBaseStats")]
pub fn get_character_base_stats(name: &str) -> Result<StatTable, JsValue> {
    StatFactory::get_character_base_stats(name)
}

#[wasm_bindgen(js_name = "getWeaponStats")]
pub fn get_weapon_stats(name: &str) -> Result<StatTable, JsValue> {
    StatFactory::get_weapon_stats(name)
}

#[wasm_bindgen(js_name = "getMainStatValue")]
pub fn get_main_stat_value(rarity: i8, level: i8, stat_type_id: u32) -> Result<f32, JsValue> {
    StatFactory::get_main_stat_value(rarity, level, stat_type_id)
}

#[wasm_bindgen(js_name = "getSubStatValue")]
pub fn get_sub_stat_value(rarity: i8, stat_type_id: u32) -> Result<f32, JsValue> {
    StatFactory::get_sub_stat_value(rarity, stat_type_id)
}

#[cfg(not(target_arch = "wasm32"))]
#[wasm_bindgen(js_name = "fetchCharacterBaseStats")]
pub async fn fetch_character_base_stats(name: &str) -> Result<StatTable, JsValue> {
    StatFactory::fetch_character_base_stats(name).await
}

#[cfg(not(target_arch = "wasm32"))]
#[wasm_bindgen(js_name = "fetchWeaponStats")]
pub async fn fetch_weapon_stats(name: &str) -> Result<StatTable, JsValue> {
    StatFactory::fetch_weapon_stats(name).await
}