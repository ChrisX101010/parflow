#!/bin/bash
# Kernel-style performance profiling

set -e

echo "📊 Running kernel-style performance profiling..."

# Build with profiling features
cargo build --release

# Run benchmarks with detailed output
echo "🏃 Running benchmarks..."
./target/release/parflow-cli benchmark --benchmark fibonacci
./target/release/parflow-cli benchmark --benchmark simple

# System analysis
echo "🖥️  System analysis..."
./target/release/parflow-cli system-analyze --format json

echo "✅ Performance profiling complete!"
