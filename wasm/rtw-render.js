import "./rtw-timer.js";
import supportsModuleWorkers from "./caniuse-module-worker.js";

export default class RtwRender extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: "open" });
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
                  display: none;
                  margin-bottom: 0;
                }
                .log.active {
                  display: block;
                }
            </style>

            <canvas width="500" height="333"></canvas>
            <div class="controls">
                <button disabled>Render</button>
                <rtw-timer></rtw-timer>
            </div>
            <p class="log"></p>
        `;
    }

    async connectedCallback() {
        this.btn = this.shadowRoot.querySelector("button");
        this.canvas = this.shadowRoot.querySelector("canvas");
        this.timer = this.shadowRoot.querySelector("rtw-timer");
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
            await this.preRender();
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
                if (e.data.data.type === "render") {
                    this.timer.pause();
                    this.log.textContent =
                        "Error occurred in worker during rendering.";
                    this.log.classList.add("active");
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
        this.log.classList.add("active");
    }

    /**
     * Begin the rendering, including starting a timer widget, starting render
     * within the wasm module, and writing the render result into the canvas.
     */
    async preRender() {
        // clearImage();
        this.timer.start();

        // if running on the main thread, pause the timer, we'll update it once at the end.
        if (!supportsModuleWorkers()) {
            // this.timer.pause();
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
