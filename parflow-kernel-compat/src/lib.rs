//! Kernel-compatible error handling and system interfaces for ParFlow
//! Inspired by Linux kernel Rust integration patterns

use thiserror::Error;
use std::time::Instant;

/// Kernel-style error types for system-level operations
#[derive(Error, Debug, Clone)]
pub enum KernelError {
    /// Memory allocation failure
    #[error("memory allocation failed: {context}")]
    AllocationError { context: String },
    
    /// System call or OS interaction failure
    #[error("system call failed: {context}")]
    SyscallError { context: String },
    
    /// Hardware feature not available
    #[error("hardware feature not supported: {feature}")]
    HardwareUnsupported { feature: String },
    
    /// Performance optimization not applicable
    #[error("performance optimization not applicable: {reason}")]
    OptimizationError { reason: String },
    
    /// Cross-language interoperability error
    #[error("cross-language call failed: {details}")]
    InteropError { details: String },
}

/// Kernel-style result type
pub type KResult<T> = Result<T, KernelError>;

/// System information structure (kernel-inspired)
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub architecture: String,
    pub kernel_version: String,
    pub memory_pages: usize,
    pub cpu_cores: usize,
    pub cache_line_size: usize,
}

impl SystemInfo {
    /// Gather system information in a kernel-compatible way
    pub fn gather() -> KResult<Self> {
        Ok(Self {
            architecture: std::env::consts::ARCH.to_string(),
            kernel_version: "unknown".to_string(),
            memory_pages: page_size::get(),
            cpu_cores: num_cpus::get(),
            cache_line_size: 64,
        })
    }
}

/// Performance profiling inspired by kernel instrumentation
pub struct KernelProfiler {
    start: Instant,
    operation: String,
    module: &'static str,
}

impl KernelProfiler {
    pub fn new(operation: impl Into<String>, module: &'static str) -> Self {
        Self {
            start: Instant::now(),
            operation: operation.into(),
            module,
        }
    }
    
    pub fn done(self) {
        let duration = self.start.elapsed();
        log::info!(
            "[KERNEL_PROFILE] {}::{} took {:?}", 
            self.module, 
            self.operation, 
            duration
        );
    }
}

/// Macro for easy profiling
#[macro_export]
macro_rules! profile_operation {
    ($operation:expr, $module:expr) => {
        let _profiler = $crate::KernelProfiler::new($operation, $module);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_info_gathering() {
        let info = SystemInfo::gather().unwrap();
        assert!(!info.architecture.is_empty());
        assert!(info.cpu_cores > 0);
    }

    #[test]
    fn test_error_display() {
        let error = KernelError::HardwareUnsupported {
            feature: "avx512".to_string(),
        };
        assert!(format!("{}", error).contains("avx512"));
    }
}
