use pyo3::prelude::*;
use aminus::stats::Stats;

#[pyclass]
pub enum PyStats {
    Stats(Stats),
}

#[pymodule]
fn aminus_stats(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyStats>()?;
    Ok(())
}

#[pymethods]
impl PyStats {
    #[new]
    fn new(stats: Stats) -> Self {
        PyStats::Stats(stats)
    }

    fn get_stats(&self) -> Stats {
        match self {
            PyStats::Stats(stats) => stats.clone(),
        }
    }
}