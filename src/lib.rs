#![allow(unused_imports, unused_variables)]
#[macro_use]
pub mod utils;
pub mod model;
pub use model::*;
pub mod functions;
pub use functions::*;
pub use utils::*;
pub mod data;
pub use data::*;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Python bindings - only compiled when python-bindings feature is enabled
#[cfg(feature = "python-bindings")]
mod python_bindings {
    use pyo3::prelude::*;
    use crate::add;

    #[pyfunction]
    fn py_add(a: i32, b: i32) -> i32 {
        add(a, b)
    }

    #[pymodule]
    fn aminus_py(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(py_add, m)?)?;
        Ok(())
    }
}

// JavaScript/WASM bindings - only compiled when js-bindings feature is enabled
#[cfg(feature = "js-bindings")]
mod js_bindings {
    use wasm_bindgen::prelude::*;
    use crate::add;

    #[wasm_bindgen]
    pub fn js_add(a: i32, b: i32) -> i32 {
        add(a, b)
    }
}