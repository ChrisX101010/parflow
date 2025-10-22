use wasm_bindgen::prelude::*;

/// run_js_par returns a JS Promise that resolves to the sum of results.
#[wasm_bindgen]
pub async fn run_js_par() -> JsValue {
    let v = parflow_core::run_example_par().await;
    let sum: i32 = v.into_iter().sum();
    JsValue::from_f64(sum as f64)
}

/// run_js_seq returns a JS Promise that resolves to the sum of results.
#[wasm_bindgen]
pub async fn run_js_seq() -> JsValue {
    let v = parflow_core::run_example_seq().await;
    let sum: i32 = v.into_iter().sum();
    JsValue::from_f64(sum as f64)
}
