import supportsModuleWorkers from "./caniuse-module-worker.js";

function sleep(ms) {
    return new Promise( (resolve) => setTimeout(resolve, ms));
}

export default class RtwRender extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: "open" });
        this.renderCount = 0;
        this.active = false;
        fetch(`${import.meta.url}/../wasm_bg.wasm`);
        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: inline-block;
                    background-color: var(--rtw-background-color, black);
                    padding: 14px;
                }
                canvas {
                    image-rendering: -moz-crisp-edges;
                    image-rendering: -webkit-crisp-edges;
                    image-rendering: pixelated;
                    image-rendering: crisp-edges;
                }
                .controls {
                    margin-top: 8px;
                    width: 100%;
                    display: grid;
                    grid-template-columns: 1fr 1fr;
                    grid-gap: 14px;
                }
                button {
                    border: var(--rtw-button-border, 1px solid white);
                    background: var(--rtw-button-background, black);
                    color: var(--rtw-button-color, white);
                }
                button:hover {
                    background: var(--rtw-button-background-hover, #1f1f1f);
                }
                button:active, button:focus {
                    background: var(--rtw-button-background-active, #3f3f3f);
                }
                .log {
                  margin-bottom: 0;
                  font-family: monospace;
                  white-space: pre;
                  line-height: 1.4;
                }
            </style>

            <canvas width=500 height=333></canvas>
            <div class=controls>
                <button disabled>Start</button>
            </div>
            <p class="log">Total rays        = -
Total duration    = -
Time per ray      = -
Rays per time     = -
Image width       = -
Image height      = -
Samples per pixel = -</p>
        `;
    }

    async connectedCallback() {
        this.btn = this.shadowRoot.querySelector("button");
        this.canvas = this.shadowRoot.querySelector("canvas");
        this.log = this.shadowRoot.querySelector(".log");

        this.ctx = this.canvas.getContext("2d");

        if (supportsModuleWorkers()) {
            console.log("module workers supported, creating worker");
            this.worker = this.createWorker();
        } else {
            console.log(
                "module workers NOT supported, will render on the main thread"
            );
            await this.initMainThreadRendering();
        }

        this.wasmInit = null;
        this.wasmRender = null;

        this.btn.addEventListener("click", async () => {
            if (this.active) {
                this.pauseRenderLoop();
            } else {
                this.startRenderLoop();
            }
        });
    }

    async startRenderLoop() {
        await this.preRender();

        this.active = true;

        this.render();

        while (this.active) {
            await sleep(1100);
            this.render();
        }
    }

    async pauseRenderLoop() {
        this.active = false;
    }

    createWorker() {
        const workerUrl = new URL(`${import.meta.url}/../wasm-worker.js`);
        const worker = new Worker(workerUrl.href, { type: "module" });
        worker.addEventListener("message", async (e) => {
            if (e.data.status === "success") {
                if (e.data.data.renderResult) {
                    this.postRender(e.data.data.renderResult);
                } else if (e.data.data.initialized) {
                    this.btn.disabled = false;
                }
            } else if (e.data.status === "error") {
                if (e.data.data.type === "render") {
                    this.log.textContent =
                        "Error occurred in worker during rendering.";
                }
            }
        });

        worker.postMessage("init");
        return worker;
    }

    async initMainThreadRendering() {
        // initialize wasm
        const wasmModule = await import("./wasm-render.js");
        this.wasmInit = wasmModule.wasmInit;
        this.wasmRender = wasmModule.wasmRender;
        await this.wasmInit();

        this.btn.disabled = false;

        this.log.textContent =
            "Rendering will run on the main thread because Module Workers are not supported in this browser.  Expect lock-up during rendering.";
    }

    /**
     * Begin the rendering, including starting a timer widget, starting render
     * within the wasm module, and writing the render result into the canvas.
     */
    async preRender() {
        // clearImage();

        // if running on the main thread, pause the timer, we'll update it once at the end.
        if (!supportsModuleWorkers()) {
        }
        this.btn.disabled = true;
    }

    async render() {
        if (supportsModuleWorkers()) {
            console.log("starting render in a module worker");
            this.worker.postMessage("render");
        } else {
            console.log("starting render on the main thread");
            if (!this.wasmInit) {
                await this.initMainThreadRendering();
            }
            const imageData = await this.wasmRender();
            this.postRender(imageData);
        }
    }

    writeStats(renderResult) {
        const total_rays = Number(renderResult.total_rays);
        this.log.textContent = `Total rays        = ${total_rays.toLocaleString(
            "en-US"
        )}
Total duration    = ${renderResult.duration.toFixed(1)} ms
Time per ray      = ${((renderResult.duration / total_rays) * 1000).toFixed(
            4
        )} microseconds/ray
Ray rate          = ${(total_rays / renderResult.duration / 1000).toFixed(
            4
        )} rays/microsecond
Image width       = ${renderResult.width}
Image height      = ${renderResult.height}
Samples per pixel = ${renderResult.samples_per_pixel}`;
    }

    postRender(renderResult) {
        if (this.renderCount === 0) {
            this.imageData = new ImageData(renderResult.pixels, renderResult.width);
        }
        else {
            const pixels = this.imageData.data.map( (c, i) => c * this.renderCount / (this.renderCount + 1) + renderResult.pixels[i] * 1 / (this.renderCount + 1) );
            this.imageData = new ImageData(pixels, renderResult.width);
        }

        console.time("drawing canvas");
        this.canvas.width = renderResult.width;
        this.canvas.height = renderResult.height;
        this.ctx.putImageData(
            this.imageData,
            0,
            0
        );
        console.timeEnd("drawing canvas");

        this.writeStats(renderResult);

        this.btn.innerText = "Pause";
        this.btn.disabled = false;

        this.renderCount++;
    }
}

customElements.define("rtw-render", RtwRender);
