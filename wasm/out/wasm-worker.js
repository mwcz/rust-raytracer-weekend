// wasm-worker.js
var wasmRender;
var initialized = false;
console.log("wasm-worker module");
addEventListener("message", async (e) => {
  if (e.data === "init") {
    try {
      const module = await import("./wasm-render.js");
      wasmRender = module.wasmRender;
      await module.wasmInit();
      initialized = true;
      postMessage({ status: "success", data: { initialized } });
    } catch (e2) {
      console.error(e2);
      postMessage({ status: "error", data: { type: "import" }, message: "error occurred during web worker import" });
    }
  } else if (e.data === "render") {
    try {
      const imageData = await wasmRender();
      postMessage({ status: "success", data: { initialized, imageData } });
    } catch (error) {
      console.error(error);
      postMessage({ status: "error", data: { type: "render", error }, message: "error occurred during render" });
    }
  }
});
