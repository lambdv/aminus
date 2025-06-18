// python_bindings/src/lib.rs
use pyo3::prelude::*;
use core::add;

#[pyfunction]
fn py_add(a: i32, b: i32) -> i32 {
    add(a, b)
}

#[pymodule]
fn mylib_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_add, m)?)?;
    Ok(())
}
