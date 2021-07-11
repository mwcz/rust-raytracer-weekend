esbuild wasm-app.js wasm-worker.js --outdir=out --bundle --minify --splitting --format=esm --sourcemap=external --summary && cp pkg/wasm_bg.wasm out
