// try importing wasm-render.  if the import fails, it probably indicates a
// browser that doesn't support module imports from web workers, like Firefox,
// so report an error back to the main thread, which will respond by running
// the rendering on the main thread.

let wasmRender;
let initialized = false;

console.log("wasm-worker module");

addEventListener("message", async (e) => {
    if (e.data === "init") {
        try {
            const module = await import(`./wasm-render.js`);
            wasmRender = module.wasmRender;
            await module.wasmInit();
            initialized = true;
            postMessage({status: "success", data: {initialized}});
        } catch (e) {
            console.error(e);
            postMessage({
                status: "error",
                data: {type: "import"},
                message: "error occurred during web worker import",
            });
        }
    } else if (e.data === "render") {
        try {
            const renderResult = await wasmRender();
            postMessage({
                status: "success",
                data: {
                    initialized,
                    renderResult: {
                        // directly reference each property returned from wasm,
                        // otherwise wasm-bindgen seems to clear the pointers
                        // and the properties cann't be referenced later. TODO
                        // figure out more about why this happens.
                        pixels: renderResult.pixels,
                        total_rays: renderResult.total_rays,
                        width: renderResult.width,
                        height: renderResult.height,
                        samples_per_pixel: renderResult.samples_per_pixel,
                        duration: renderResult.duration,
                    },
                },
            });
        } catch (error) {
            console.error(error);
            postMessage({
                status: "error",
                data: {type: "render", error},
                message: "error occurred during render",
            });
        }
    }
});

// // import wasmRender from "./wasm-render.js";

// try {
//     console.log('importing');
//     import("./wasm-render.js").then(({wasmRender}) => {
//         console.log('imported');
//         addEventListener('message', () => {
//             postMessage({status: "success", data: await wasmRender()});
//         });
//     });
// } catch (e) {
//     postMessage({status: "error", message: "error occurred during import or render"});
// }
