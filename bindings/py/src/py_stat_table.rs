use pyo3::prelude::*;
use aminus::model::stattable::StatTable as RustStatTable;
use aminus::model::statable::{Statable, ModifiableStatable};
use crate::py_utils::rust_stat_from_id;

#[pyclass]
pub struct PyStatTable {
    pub inner: RustStatTable,
}

#[pymethods]
impl PyStatTable {
    #[new]
    pub fn new() -> Self {
        PyStatTable {
            inner: RustStatTable::new(),
        }
    }

    #[staticmethod]
    pub fn of(stats: Vec<(u32, f32)>) -> PyResult<PyStatTable> {
        let mut table = RustStatTable::new();
        for (stat_id, value) in stats {
            let stat = rust_stat_from_id(stat_id)?;
            table.add(&stat, value);
        }
        Ok(PyStatTable { inner: table })
    }

    pub fn get(&self, stat_id: u32) -> PyResult<f32> {
        let stat = rust_stat_from_id(stat_id)?;
        Ok(self.inner.get(&stat))
    }

    pub fn add(&mut self, stat_id: u32, value: f32) -> PyResult<f32> {
        let stat = rust_stat_from_id(stat_id)?;
        Ok(self.inner.add(&stat, value))
    }

    pub fn add_table(&mut self, other: &PyStatTable) {
        self.inner.add_table(other.inner.iter());
    }

    pub fn to_list(&self) -> Vec<(u32, f32)> {
        self.inner.iter()
            .map(|(stat, value)| (stat as u32, value))
            .collect()
    }

    pub fn __repr__(&self) -> String {
        let stats: Vec<String> = self.inner.iter()
            .map(|(stat, value)| format!("({}, {:.2})", stat as u32, value))
            .collect();
        format!("PyStatTable([{}])", stats.join(", "))
    }

    pub fn __len__(&self) -> usize {
        self.inner.iter().count()
    }
} 