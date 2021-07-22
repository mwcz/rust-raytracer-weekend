wasm-pack build --release -t web
# esbuild wasm-app.js wasm-worker.js --outdir=out --bundle --minify --splitting --format=esm --sourcemap=external && cp pkg/wasm_bg.wasm out
esbuild rtw-render.js wasm-worker.js --outdir=out --minify --bundle --splitting --format=esm --sourcemap=external && cp pkg/wasm_bg.wasm index.html out
