#![allow(unused_imports, unused_variables)]
pub mod model;
pub use model::*;
pub mod functions;
pub use functions::*;
pub mod utils;
pub use utils::*;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}