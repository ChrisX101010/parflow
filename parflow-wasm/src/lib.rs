//! WebAssembly bindings for ParFlow
//! 
//! Provides WASM-compatible interfaces for cross-language orchestration
//! and performance optimization between Rust and JavaScript.

use wasm_bindgen::prelude::*;

/// Run parallel computation from JavaScript
/// 
/// This function demonstrates cross-language parallel execution
/// by running computations that can be called from JavaScript.
/// 
/// # Returns
/// 
/// A JavaScript Promise that resolves to the sum of parallel computation results
#[wasm_bindgen]
pub async fn run_js_par() -> JsValue {
    let v = parflow_core::run_example_par().await;
    let sum: i32 = v.into_iter().sum();
    JsValue::from_f64(sum as f64)
}

/// Run sequential computation from JavaScript
/// 
/// This function demonstrates cross-language sequential execution
/// for comparison with parallel performance.
/// 
/// # Returns
/// 
/// A JavaScript Promise that resolves to the sum of sequential computation results
#[wasm_bindgen]
pub async fn run_js_seq() -> JsValue {
    let v = parflow_core::run_example_seq().await;
    let sum: i32 = v.into_iter().sum();
    JsValue::from_f64(sum as f64)
}
