// import init, {render} from "./pkg/wasm.js";
import "./rtw-timer.js";
import Zooming from "zooming";

const btn = document.querySelector("button");
const canvas = document.querySelector('canvas');
const img = document.querySelector('img');
const timers = document.querySelector("#timers");
let timer = addTimer();

canvas.width = 5 * 100;
canvas.height = canvas.width * 2 / 3;
const ctx = canvas.getContext('2d');
let moduleWorkerSupported = true;

// calculate the URL of the worker as being relative to this file
const workerUrl = new URL(`${import.meta.url}/../wasm-worker.js`);
const worker = new Worker(workerUrl.href, {type: "module"});

let wasmInit;
let wasmRender;

/**
 * Begin the rendering, including starting a timer widget, starting render
 * within the wasm module, and writing the render result into the canvas.
 */
async function preRender() {
    timer = addTimer();
    // clearImage();
    timer.start();
}

async function render() {
    console.log(`starting render ${["ON", "OFF"][~~moduleWorkerSupported]} the main thread`);
    if (moduleWorkerSupported) {
        worker.postMessage('render');
    } else {
        postRender(await wasmRender());
    }
}

function postRender(imageData) {
    console.time("drawing canvas");
    ctx.putImageData(imageData, 0, 0);
    console.timeEnd("drawing canvas");
    console.time("copying canvas to image");
    img.src = canvas.toDataURL();
    console.timeEnd("copying canvas to image");
    timer.step();
    timer.stop()
}


worker.addEventListener('message', async e => {
    if (e.data.status === "success") {
        if (e.data.data.imageData) {
            postRender(e.data.data.imageData);
        } else if (e.data.data.initialized) {
            btn.disabled = false;
        }
    } else if (e.data.status === "error") {
        console.log(`web worker error type: ${e.data.data.type}`);
        if (e.data.data.type === "import") {
            // switch to main thread mode
            moduleWorkerSupported = false;
            // enable the render button
            btn.disabled = false;

            // initialize wasm
            const wasmModule = await import("./wasm-render.js");
            wasmInit = wasmModule.wasmInit;
            wasmRender = wasmModule.wasmRender;
            await wasmInit();

            timer.pause();
            timer.setLabel("Module worker not supported in this browser; running on the main thread (expect lockup during render).");

        }
        if (e.data.data.type === "render") {
            timer.pause();
            timer.setLabel("Error occurred in worker during rendering.");
        }
        // error status probably indicates firefox's lack of support for module
        // workers, so import the renderer directly and run it on the main
        // thread.
        // render({mainThread: true});

        // timer won't tick up anymore, so indicate that things will be block
        // if (timer) {
        //     timer.pause();
        //     timer.setLabel("Web Worker failed, running on the main thread...");
        // }

        // console.log('web worker failed, switching to the main thread');
        // mainThread = true;
        // let renderModule = await import("./wasm-render.js");
        // wasmInit = renderModule.wasmInit;
        // wasmRender = renderModule.wasmRender;
        // await wasmInit();
        // btn.disabled = false;

        // if (timer) {
        //     // restart the timer and step it once to update the total running time
        //     timer.start();
        //     timer.step();
        // }
    }
});

worker.postMessage("init");


function addTimer() {
    const newTimer = document.createElement("rtw-timer");
    const li = document.createElement("li");
    li.appendChild(newTimer);
    timers.prepend(li);
    return newTimer;
}

btn.addEventListener("click", () => {preRender(); render();});

new Zooming({}).listen('.zoomable')
