// import wasmRender from "./wasm-render.js";

addEventListener('message', async e => {
    if (e.data === 'render') {
        try {
            const {wasmRender} = await import("./wasm-render.js");
            postMessage({status: "success", data: await wasmRender()});
        } catch (e) {
            postMessage({status: "error", message: "error occurred during import or render"});
        }
    }
});
