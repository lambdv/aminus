use pyo3::prelude::*;

mod py_stat_table;
mod py_artifact;
mod py_formulas;
mod py_utils;

use py_stat_table::*;
use py_artifact::*;
use py_formulas::*;
use py_utils::*;

#[pymodule]
fn aminus_py(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register stat table classes
    m.add_class::<PyStatTable>()?;
    
    // Register artifact classes
    m.add_class::<PyArtifactPiece>()?;
    m.add_class::<PyArtifactBuilder>()?;
    
    // Register formulas class
    m.add_class::<PyFormulas>()?;
    
    // Register utility functions
    m.add_function(wrap_pyfunction!(stat_from_id, m)?)?;
    m.add_function(wrap_pyfunction!(element_from_id, m)?)?;
    m.add_function(wrap_pyfunction!(damage_type_from_id, m)?)?;
    m.add_function(wrap_pyfunction!(scaling_from_id, m)?)?;
    m.add_function(wrap_pyfunction!(amplifier_from_id, m)?)?;
    m.add_function(wrap_pyfunction!(quality_from_id, m)?)?;
    m.add_function(wrap_pyfunction!(stat_from_string, m)?)?;
    m.add_function(wrap_pyfunction!(get_stat_name, m)?)?;
    m.add_function(wrap_pyfunction!(is_elemental_dmg_bonus, m)?)?;
    m.add_function(wrap_pyfunction!(get_roll_quality_multiplier, m)?)?;
    
    Ok(())
}
