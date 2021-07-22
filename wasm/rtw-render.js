import "./rtw-timer.js";
import Zooming from "zooming";

export default class RtwRender extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: "open" });
        fetch("./wasm_bg.wasm");
        this.shadowRoot.innerHTML = `
            <style>
                canvas {
                    display: none;
                    aspect-ratio: 5/3.33;
                }
                img {
                    width: 100%;
                    aspect-ratio: 5/3.33;
                    /* height:  calc(66px * 3); */
                    image-rendering: -moz-crisp-edges;
                    image-rendering: -webkit-crisp-edges;
                    image-rendering: pixelated;
                    image-rendering: crisp-edges;
                }
                .controls {
                    display: grid;
                    grid-template-columns: 1fr 2fr;
                    overflow-y: hidden;
                }
                /*
                #timers { display: none }
                */
            </style>

            <canvas width="500" height="333"></canvas>
            <img width="500" height="333" class="zoomable" />
            <div class="controls">
                <button id="do-render" disabled>Render</button>
                <ol reversed id="timers"></ol>
            </div>
        `;
    }

    connectedCallback() {
        this.btnRender = this.shadowRoot.querySelector("button#do-render");
        this.canvas = this.shadowRoot.querySelector("canvas");
        this.img = this.shadowRoot.querySelector("img");
        this.timersCollapse = this.shadowRoot.querySelector("pfe-collapse");
        this.timers = this.shadowRoot.querySelector("#timers");
        this.timer = this.addTimer();

        this.ctx = this.canvas.getContext("2d");
        this.moduleWorkerSupported = true;

        this.worker = this.createWorker();

        this.wasmInit = null;
        this.wasmRender = null;

        this.btnRender.addEventListener("click", () => {
            this.preRender();
            this.render();
        });

        new Zooming({}).listen(".zoomable");
    }

    createWorker() {
        const workerUrl = new URL(`${import.meta.url}/../wasm-worker.js`);
        const worker = new Worker(workerUrl.href, { type: "module" });
        worker.addEventListener("message", async (e) => {
            if (e.data.status === "success") {
                if (e.data.data.imageData) {
                    this.postRender(e.data.data.imageData);
                } else if (e.data.data.initialized) {
                    this.btnRender.disabled = false;
                }
            } else if (e.data.status === "error") {
                console.log(`web worker error type: ${e.data.data.type}`);
                if (e.data.data.type === "import") {
                    // switch to main thread mode
                    this.moduleWorkerSupported = false;
                    // enable the render button
                    this.btnRender.disabled = false;

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
        this.timer = this.addTimer();
        // clearImage();
        this.timer.start();
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
        console.time("copying canvas to image");
        this.img.src = this.canvas.toDataURL();
        console.timeEnd("copying canvas to image");
        this.timer.step();
        this.timer.stop();
        this.btnRender.innerText = "Re-render";
    }

    addTimer() {
        const newTimer = document.createElement("rtw-timer");
        const li = document.createElement("li");
        li.appendChild(newTimer);
        this.timers.prepend(li);
        return newTimer;
    }
}

customElements.define("rtw-render", RtwRender);
