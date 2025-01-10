use wasm_bindgen::prelude::*;

/// Add two numbers
#[wasm_bindgen]
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

/// Subtract two numbers
#[wasm_bindgen]
pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

/// Multiply two numbers
#[wasm_bindgen]
pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// Divide two numbers
#[wasm_bindgen]
pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero!".to_string())
    } else {
        Ok(a / b)
    }
}
