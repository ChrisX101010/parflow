# ParFlow

ParFlow is a Rust-based cross-language async task orchestrator providing:
- Core async runner (`par`, `seq`)
- REST API (Axum)
- gRPC server (Tonic)
- WebAssembly module (wasm-bindgen)
- C FFI dynamic library (for Python/other languages)

This repository is a scaffold. See individual crate READMEs for usage.

## Quick local steps

1. Install Rust toolchain (stable).
2. Build and run tests for workspace:
```bash
cargo test --workspace
```

3. Run the REST server:
```bash
cargo run -p parflow-rest
# then: curl http://127.0.0.1:3000/par
```

4. Build the C library and call it from Python (Linux example):
```bash
# from repo root
cargo build -p parflow-c --release
# produce: target/release/libparflow_c.so (Linux)
python3 examples/python/orchestrator_ctypes.py
```

5. Build WASM (requires wasm-pack):
```bash
rustup target add wasm32-unknown-unknown
cd parflow-wasm
wasm-pack build --target web
```

## License
MIT
