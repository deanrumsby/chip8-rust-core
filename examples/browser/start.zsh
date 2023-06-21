#!/bin/zsh
# Build the wasm module
wasm-pack build ../.. --out-dir examples/browser/pkg --target web -- --features wasm &&
# Run a local server and open the browser
python3 -m http.server 8000 &
open http://localhost:8000