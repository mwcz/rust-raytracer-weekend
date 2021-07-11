import init, {render} from "./pkg/wasm.js";

console.log("wasm-render module");

/**
 * Initialize wasm module and run the render function.  Embeds the render
 * result (flat RGBA array of u8's) into a Uint8ClampedArray view and embeds
 * that into an ImageData object suitable to draw into a <canvas>.
 *
 * ImageData is a supported type to pass to/from Web Workers:
 * https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Structured_clone_algorithm
 */
export async function wasmRender() {
    console.time("tracing rays");
    const pixels = render();
    console.timeEnd("tracing rays");

    return new ImageData(new Uint8ClampedArray(pixels), 300);
}

/**
 * Initialize the WASM module.
 */
export async function wasmInit() {
    console.time("init");
    await init();
    console.timeEnd("init");
}
