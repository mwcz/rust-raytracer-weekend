wasm-pack build --release -t web
esbuild rtw-render.js wasm-worker.js --outdir=dist --bundle --minify --splitting --format=esm --sourcemap=external

cp pkg/wasm_bg.wasm dist
