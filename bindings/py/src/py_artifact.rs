use pyo3::prelude::*;
use aminus::model::artifact_builder::{ArtifactBuilder as RustArtifactBuilder, ArtifactPiece as RustArtifactPiece};
use crate::py_utils::{rust_stat_from_id, rust_quality_from_id};
use crate::py_stat_table::PyStatTable;

// ============================================================================
// PyArtifactPiece
// ============================================================================

#[pyclass]
pub struct PyArtifactPiece {
    pub inner: RustArtifactPiece,
}

#[pymethods]
impl PyArtifactPiece {
    #[new]
    pub fn new(rarity: i8, level: i8, stat_type_id: u32) -> PyResult<Self> {
        let stat_type = rust_stat_from_id(stat_type_id)?;
        
        Ok(PyArtifactPiece {
            inner: RustArtifactPiece { rarity, level, stat_type },
        })
    }

    #[getter]
    pub fn rarity(&self) -> i8 {
        self.inner.rarity
    }

    #[getter]
    pub fn level(&self) -> i8 {
        self.inner.level
    }

    #[getter]
    pub fn stat_type(&self) -> u32 {
        self.inner.stat_type as u32
    }

    pub fn __repr__(&self) -> String {
        format!("PyArtifactPiece(rarity={}, level={}, stat_type={})", 
                self.inner.rarity, self.inner.level, self.inner.stat_type as u32)
    }
}

// ============================================================================
// PyArtifactBuilder
// ============================================================================

#[pyclass]
pub struct PyArtifactBuilder {
    pub inner: RustArtifactBuilder,
}

#[pymethods]
impl PyArtifactBuilder {
    #[new]
    pub fn new(
        flower: Option<&PyArtifactPiece>,
        feather: Option<&PyArtifactPiece>,
        sands: Option<&PyArtifactPiece>,
        goblet: Option<&PyArtifactPiece>,
        circlet: Option<&PyArtifactPiece>,
    ) -> Self {
        PyArtifactBuilder {
            inner: RustArtifactBuilder::new(
                flower.map(|p| p.inner.clone()),
                feather.map(|p| p.inner.clone()),
                sands.map(|p| p.inner.clone()),
                goblet.map(|p| p.inner.clone()),
                circlet.map(|p| p.inner.clone()),
            ),
        }
    }

    #[staticmethod]
    pub fn kqm_all_5_star(sands_main_id: u32, goblet_main_id: u32, circlet_main_id: u32) -> PyResult<Self> {
        let sands_main = rust_stat_from_id(sands_main_id)?;
        let goblet_main = rust_stat_from_id(goblet_main_id)?;
        let circlet_main = rust_stat_from_id(circlet_main_id)?;
        
        Ok(PyArtifactBuilder {
            inner: RustArtifactBuilder::kqm_all_5_star(sands_main, goblet_main, circlet_main),
        })
    }

    #[staticmethod]
    pub fn kqm_all_4_star(sands_main_id: u32, goblet_main_id: u32, circlet_main_id: u32) -> PyResult<Self> {
        let sands_main = rust_stat_from_id(sands_main_id)?;
        let goblet_main = rust_stat_from_id(goblet_main_id)?;
        let circlet_main = rust_stat_from_id(circlet_main_id)?;
        
        Ok(PyArtifactBuilder {
            inner: RustArtifactBuilder::kqm_all_4_star(sands_main, goblet_main, circlet_main),
        })
    }

    pub fn roll(&mut self, substat_value_id: u32, quality_id: u32, rarity: i8, num: i8) -> PyResult<()> {
        let substat_value = rust_stat_from_id(substat_value_id)?;
        let quality = rust_quality_from_id(quality_id)?;
        self.inner.roll(substat_value, quality, rarity, num);
        Ok(())
    }

    pub fn unroll(&mut self, substat_value_id: u32, quality_id: u32, rarity: i8, num: i8) -> PyResult<()> {
        let substat_value = rust_stat_from_id(substat_value_id)?;
        let quality = rust_quality_from_id(quality_id)?;
        self.inner.unroll(substat_value, quality, rarity, num);
        Ok(())
    }

    pub fn build(&self) -> PyStatTable {
        PyStatTable {
            inner: self.inner.build(),
        }
    }

    pub fn main_stats(&self) -> PyStatTable {
        PyStatTable {
            inner: self.inner.main_stats(),
        }
    }

    pub fn sub_stats(&self) -> PyStatTable {
        PyStatTable {
            inner: self.inner.sub_stats(),
        }
    }

    pub fn max_rolls(&self) -> i8 {
        self.inner.max_rolls()
    }

    pub fn current_rolls(&self) -> i8 {
        self.inner.current_rolls()
    }

    pub fn rolls_left(&self) -> i8 {
        self.inner.rolls_left()
    }

    pub fn substat_constraint(&self, stat_type_id: u32, rarity: i8) -> PyResult<i8> {
        let stat_type = rust_stat_from_id(stat_type_id)?;
        Ok(self.inner.substat_constraint(&stat_type, rarity))
    }

    pub fn __repr__(&self) -> String {
        format!("PyArtifactBuilder(max_rolls={}, current_rolls={}, rolls_left={})", 
                self.inner.max_rolls(), self.inner.current_rolls(), self.inner.rolls_left())
    }
} 