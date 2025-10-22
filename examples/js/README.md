This folder demonstrates how to use the WASM build produced by `wasm-pack`.

Steps:
1. cd parflow-wasm
2. wasm-pack build --target web
3. In a web project, import the generated package and call `run_js_par()` which returns a Promise.
