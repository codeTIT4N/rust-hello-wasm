use wasm_bindgen::prelude::*; //setup wasm-bindgen

// procedural macro to get our function setup
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
