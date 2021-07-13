var __defProp = Object.defineProperty;
var __markAsModule = (target) => __defProp(target, "__esModule", { value: true });
var __esm = (fn, res) => function __init() {
  return fn && (res = (0, fn[Object.keys(fn)[0]])(fn = 0)), res;
};
var __export = (target, all) => {
  __markAsModule(target);
  for (var name in all)
    __defProp(target, name, { get: all[name], enumerable: true });
};

// pkg/wasm.js
function getInt32Memory0() {
  if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
    cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
  }
  return cachegetInt32Memory0;
}
function getUint8Memory0() {
  if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
    cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
  }
  return cachegetUint8Memory0;
}
function getArrayU8FromWasm0(ptr, len) {
  return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}
function render() {
  try {
    const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
    wasm.render(retptr);
    var r0 = getInt32Memory0()[retptr / 4 + 0];
    var r1 = getInt32Memory0()[retptr / 4 + 1];
    var v0 = getArrayU8FromWasm0(r0, r1).slice();
    wasm.__wbindgen_free(r0, r1 * 1);
    return v0;
  } finally {
    wasm.__wbindgen_add_to_stack_pointer(16);
  }
}
async function load(module, imports) {
  if (typeof Response === "function" && module instanceof Response) {
    if (typeof WebAssembly.instantiateStreaming === "function") {
      try {
        return await WebAssembly.instantiateStreaming(module, imports);
      } catch (e) {
        if (module.headers.get("Content-Type") != "application/wasm") {
          console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
        } else {
          throw e;
        }
      }
    }
    const bytes = await module.arrayBuffer();
    return await WebAssembly.instantiate(bytes, imports);
  } else {
    const instance = await WebAssembly.instantiate(module, imports);
    if (instance instanceof WebAssembly.Instance) {
      return { instance, module };
    } else {
      return instance;
    }
  }
}
async function init(input) {
  if (typeof input === "undefined") {
    input = new URL("wasm_bg.wasm", import.meta.url);
  }
  const imports = {};
  if (typeof input === "string" || typeof Request === "function" && input instanceof Request || typeof URL === "function" && input instanceof URL) {
    input = fetch(input);
  }
  const { instance, module } = await load(await input, imports);
  wasm = instance.exports;
  init.__wbindgen_wasm_module = module;
  return wasm;
}
var wasm, cachegetInt32Memory0, cachegetUint8Memory0, wasm_default;
var init_wasm = __esm({
  "pkg/wasm.js"() {
    cachegetInt32Memory0 = null;
    cachegetUint8Memory0 = null;
    wasm_default = init;
  }
});

// wasm-render.js
var wasm_render_exports = {};
__export(wasm_render_exports, {
  wasmInit: () => wasmInit,
  wasmRender: () => wasmRender
});
async function wasmRender() {
  console.time("tracing rays");
  const pixels = render();
  console.timeEnd("tracing rays");
  return new ImageData(new Uint8ClampedArray(pixels), 500);
}
async function wasmInit() {
  console.time("init");
  await wasm_default();
  console.timeEnd("init");
}
var init_wasm_render = __esm({
  "wasm-render.js"() {
    init_wasm();
    console.log("wasm-render module");
  }
});

// wasm-worker.js
var wasmRender2;
var initialized = false;
console.log("wasm-worker module");
addEventListener("message", async (e) => {
  if (e.data === "init") {
    try {
      const module = await Promise.resolve().then(() => (init_wasm_render(), wasm_render_exports));
      wasmRender2 = module.wasmRender;
      await module.wasmInit();
      initialized = true;
      postMessage({ status: "success", data: { initialized } });
    } catch (e2) {
      console.error(e2);
      postMessage({ status: "error", message: "error occurred during web worker import" });
    }
  } else if (e.data === "render") {
    try {
      const imageData = await wasmRender2();
      postMessage({ status: "success", data: { initialized, imageData } });
    } catch (e2) {
      console.error(e2);
      postMessage({ status: "error", message: "error occurred during render" });
    }
  }
});
