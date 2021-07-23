import "./rtw-timer.js";

export default class RtwRender extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: "open" });
        fetch(`${import.meta.url}/../wasm_bg.wasm`);
        this.shadowRoot.innerHTML = `
            <style>
                :host {
                    display: inline-block;
                    background-color: var(--rtw-background-color, grey);
                    padding: 14px;
                }
                canvas {
                    aspect-ratio: 5/3.33;
                    image-rendering: -moz-crisp-edges;
                    image-rendering: -webkit-crisp-edges;
                    image-rendering: pixelated;
                    image-rendering: crisp-edges;
                }
                .controls {
                    margin-top: 8px;
                    width: 500px;
                    display: grid;
                    grid-template-columns: 1fr 1fr;
                    grid-gap: 14px;
                    overflow-y: hidden;
                }
            </style>

            <canvas width="500" height="333"></canvas>
            <div class="controls">
                <button disabled>Render</button>
                <rtw-timer></rtw-timer>
            </div>
        `;
    }

    connectedCallback() {
        this.btn = this.shadowRoot.querySelector("button");
        this.canvas = this.shadowRoot.querySelector("canvas");
        this.timer = this.shadowRoot.querySelector("rtw-timer");

        this.ctx = this.canvas.getContext("2d");
        this.moduleWorkerSupported = true;

        this.worker = this.createWorker();

        this.wasmInit = null;
        this.wasmRender = null;

        this.btn.addEventListener("click", () => {
            this.preRender();
            this.render();
        });
    }

    createWorker() {
        const workerUrl = new URL(`${import.meta.url}/../wasm-worker.js`);
        const worker = new Worker(workerUrl.href, { type: "module" });
        worker.addEventListener("message", async (e) => {
            if (e.data.status === "success") {
                if (e.data.data.imageData) {
                    this.postRender(e.data.data.imageData);
                } else if (e.data.data.initialized) {
                    this.btn.disabled = false;
                }
            } else if (e.data.status === "error") {
                console.log(`web worker error type: ${e.data.data.type}`);
                if (e.data.data.type === "import") {
                    // switch to main thread mode
                    this.moduleWorkerSupported = false;
                    // enable the render button
                    this.btn.disabled = false;

                    // initialize wasm
                    const wasmModule = await import("./wasm-render.js");
                    this.wasmInit = wasmModule.wasmInit;
                    this.wasmRender = wasmModule.wasmRender;
                    await wasmInit();

                    this.timer.pause();
                    this.timer.setLabel(
                        "Module worker not supported in this browser; running on the main thread (expect lockup during render)."
                    );
                }
                if (e.data.data.type === "render") {
                    this.timer.pause();
                    this.timer.setLabel(
                        "Error occurred in worker during rendering."
                    );
                }
            }
        });

        worker.postMessage("init");
        return worker;
    }

    /**
     * Begin the rendering, including starting a timer widget, starting render
     * within the wasm module, and writing the render result into the canvas.
     */
    async preRender() {
        // clearImage();
        this.timer.start();
        this.btn.disabled = true;
    }

    async render() {
        console.log(
            `starting render ${
                ["ON", "OFF"][~~this.moduleWorkerSupported]
            } the main thread`
        );
        if (this.moduleWorkerSupported) {
            this.worker.postMessage("render");
        } else {
            this.postRender(await this.wasmRender());
        }
    }
    postRender(imageData) {
        console.time("drawing canvas");
        this.ctx.putImageData(imageData, 0, 0);
        console.timeEnd("drawing canvas");
        this.timer.step();
        this.timer.stop();
        this.btn.innerText = "Re-render";
        this.btn.disabled = false;
    }
}

customElements.define("rtw-render", RtwRender);
