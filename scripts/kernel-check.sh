#!/bin/bash
# Kernel-compatible Rust code quality checks

set -e

echo "🧪 Running kernel-style Rust checks..."

# Format check
echo "📝 Checking formatting..."
cargo fmt --all -- --check  # CHANGED: Using direct command

# Linting
echo "🔍 Running clippy..."
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Documentation
echo "📚 Checking documentation..."
cargo doc --workspace --no-deps

# Tests
echo "🚀 Running tests..."
cargo test --workspace

echo "✅ All kernel-style checks passed!"
