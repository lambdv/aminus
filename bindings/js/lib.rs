// js_bindings/src/lib.rs
use wasm_bindgen::prelude::*;
use core::add;

#[wasm_bindgen]
pub fn js_add(a: i32, b: i32) -> i32 {
    add(a, b)
}
