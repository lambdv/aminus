use wasm_bindgen::prelude::*;
use aminus::model::artifact_builder::{ArtifactBuilder as RustArtifactBuilder, ArtifactPiece as RustArtifactPiece};
use crate::utils::{stat_from_id, quality_from_id};
use crate::stat_table::StatTable;

// ============================================================================
// ArtifactPiece WASM wrapper
// ============================================================================

#[wasm_bindgen]
pub struct ArtifactPiece {
    pub(crate) inner: RustArtifactPiece,
}

#[wasm_bindgen]
impl ArtifactPiece {
    #[wasm_bindgen(constructor)]
    pub fn new(rarity: i8, level: i8, stat_type_id: u32) -> Result<ArtifactPiece, JsValue> {
        let stat_type = stat_from_id(stat_type_id)?;
        
        Ok(ArtifactPiece {
            inner: RustArtifactPiece { rarity, level, stat_type },
        })
    }

    #[wasm_bindgen(getter)]
    pub fn rarity(&self) -> i8 {
        self.inner.rarity
    }

    #[wasm_bindgen(getter)]
    pub fn level(&self) -> i8 {
        self.inner.level
    }

    #[wasm_bindgen(getter, js_name = "statType")]
    pub fn stat_type(&self) -> u32 {
        self.inner.stat_type as u32
    }
}

// ============================================================================
// ArtifactBuilder WASM wrapper
// ============================================================================

#[wasm_bindgen]
pub struct ArtifactBuilder {
    pub(crate) inner: RustArtifactBuilder,
}

#[wasm_bindgen]
impl ArtifactBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(
        flower: Option<ArtifactPiece>,
        feather: Option<ArtifactPiece>,
        sands: Option<ArtifactPiece>,
        goblet: Option<ArtifactPiece>,
        circlet: Option<ArtifactPiece>,
    ) -> ArtifactBuilder {
        ArtifactBuilder {
            inner: RustArtifactBuilder::new(
                flower.map(|w| w.inner),
                feather.map(|w| w.inner),
                sands.map(|w| w.inner),
                goblet.map(|w| w.inner),
                circlet.map(|w| w.inner),
            ),
        }
    }

    #[wasm_bindgen(js_name = "kqmAll5Star")]
    pub fn kqm_all_5_star(sands_main_id: u32, goblet_main_id: u32, circlet_main_id: u32) -> Result<ArtifactBuilder, JsValue> {
        let sands_main = stat_from_id(sands_main_id)?;
        let goblet_main = stat_from_id(goblet_main_id)?;
        let circlet_main = stat_from_id(circlet_main_id)?;
        
        Ok(ArtifactBuilder {
            inner: RustArtifactBuilder::kqm_all_5_star(sands_main, goblet_main, circlet_main),
        })
    }

    #[wasm_bindgen(js_name = "kqmAll4Star")]
    pub fn kqm_all_4_star(sands_main_id: u32, goblet_main_id: u32, circlet_main_id: u32) -> Result<ArtifactBuilder, JsValue> {
        let sands_main = stat_from_id(sands_main_id)?;
        let goblet_main = stat_from_id(goblet_main_id)?;
        let circlet_main = stat_from_id(circlet_main_id)?;
        
        Ok(ArtifactBuilder {
            inner: RustArtifactBuilder::kqm_all_4_star(sands_main, goblet_main, circlet_main),
        })
    }

    #[wasm_bindgen]
    pub fn roll(&mut self, substat_value_id: u32, quality_id: u32, rarity: i8, num: i8) -> Result<(), JsValue> {
        let substat_value = stat_from_id(substat_value_id)?;
        let quality = quality_from_id(quality_id)?;
        self.inner.roll(substat_value, quality, rarity, num);
        Ok(())
    }

    #[wasm_bindgen]
    pub fn unroll(&mut self, substat_value_id: u32, quality_id: u32, rarity: i8, num: i8) -> Result<(), JsValue> {
        let substat_value = stat_from_id(substat_value_id)?;
        let quality = quality_from_id(quality_id)?;
        self.inner.unroll(substat_value, quality, rarity, num);
        Ok(())
    }

    #[wasm_bindgen]
    pub fn build(&self) -> StatTable {
        StatTable {
            inner: self.inner.build(),
        }
    }

    #[wasm_bindgen(js_name = "mainStats")]
    pub fn main_stats(&self) -> StatTable {
        StatTable {
            inner: self.inner.main_stats(),
        }
    }

    #[wasm_bindgen(js_name = "subStats")]
    pub fn sub_stats(&self) -> StatTable {
        StatTable {
            inner: self.inner.sub_stats(),
        }
    }

    #[wasm_bindgen(js_name = "maxRolls")]
    pub fn max_rolls(&self) -> i8 {
        self.inner.max_rolls()
    }

    #[wasm_bindgen(js_name = "currentRolls")]
    pub fn current_rolls(&self) -> i8 {
        self.inner.current_rolls()
    }

    #[wasm_bindgen(js_name = "rollsLeft")]
    pub fn rolls_left(&self) -> i8 {
        self.inner.rolls_left()
    }

    #[wasm_bindgen(js_name = "substatConstraint")]
    pub fn substat_constraint(&self, stat_type_id: u32, rarity: i8) -> Result<i8, JsValue> {
        let stat_type = stat_from_id(stat_type_id)?;
        Ok(self.inner.substat_constraint(&stat_type, rarity))
    }
} 