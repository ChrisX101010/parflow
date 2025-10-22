use std::os::raw::c_int;

/// Expose a simple C ABI function that runs the parallel example and returns the sum.
#[no_mangle]
pub extern "C" fn run_orchestrator_par() -> c_int {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let vec = rt.block_on(parflow_core::run_example_par());
    vec.into_iter().sum::<i32>() as c_int
}

/// Expose sequential version.
#[no_mangle]
pub extern "C" fn run_orchestrator_seq() -> c_int {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let vec = rt.block_on(parflow_core::run_example_seq());
    vec.into_iter().sum::<i32>() as c_int
}
