wasm-pack build --release -t web
# esbuild wasm-app.js wasm-worker.js --outdir=out --bundle --minify --splitting --format=esm --sourcemap=external && cp pkg/wasm_bg.wasm out
esbuild wasm-app.js wasm-worker.js --outdir=out --bundle --format=esm --sourcemap=external && cp pkg/wasm_bg.wasm out
