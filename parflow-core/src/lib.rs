//! Core asynchronous utilities for ParFlow
//! Provides cross-platform async primitives for both native and WASM targets

// For non-WASM platforms
#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;

#[cfg(not(target_arch = "wasm32"))]
/// Run example parallel computation
/// 
/// Demonstrates parallel task execution with async/await on native platforms
/// 
/// # Returns
/// 
/// Vector of computed results from parallel tasks
pub async fn run_example_par() -> Vec<i32> {
    use tokio::time::sleep;

    let result1 = tokio::spawn(async {
        sleep(Duration::from_millis(100)).await;
        1
    });

    let result2 = tokio::spawn(async {
        sleep(Duration::from_millis(50)).await;
        2
    });

    vec![result1.await.unwrap(), result2.await.unwrap()]
}

#[cfg(not(target_arch = "wasm32"))]
/// Run example sequential computation
/// 
/// Demonstrates sequential task execution with async/await on native platforms
/// 
/// # Returns
/// 
/// Vector of computed results from sequential tasks
pub async fn run_example_seq() -> Vec<i32> {
    use tokio::time::sleep;

    let result1 = async {
        sleep(Duration::from_millis(100)).await;
        1
    }
    .await;

    let result2 = async {
        sleep(Duration::from_millis(50)).await;
        2
    }
    .await;

    vec![result1, result2]
}

// For WASM platforms - simplified version
#[cfg(target_arch = "wasm32")]
/// Run example parallel computation (WASM)
/// 
/// Simplified parallel execution for WebAssembly targets
/// 
/// # Returns
/// 
/// Vector of computed results
pub async fn run_example_par() -> Vec<i32> {
    // For WASM, we'll use a simple delay simulation
    vec![1, 2]
}

#[cfg(target_arch = "wasm32")]
/// Run example sequential computation (WASM)
/// 
/// Simplified sequential execution for WebAssembly targets
/// 
/// # Returns
/// 
/// Vector of computed results
pub async fn run_example_seq() -> Vec<i32> {
    vec![1, 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_seq() {
        let res = run_example_seq().await;
        assert_eq!(res, vec![1, 2]);
    }

    #[tokio::test]
    async fn test_par() {
        let res = run_example_par().await;
        assert_eq!(res, vec![1, 2]);
    }
}
