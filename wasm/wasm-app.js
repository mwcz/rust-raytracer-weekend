// import init, {render} from "./pkg/wasm.js";
import "./rtw-timer.js";
import Zooming from "zooming";

const btn = document.querySelector("button");
const canvas = document.querySelector('canvas');
const img = document.querySelector('img');
const timers = document.querySelector("#timers");
let timer;

canvas.width = 5 * 100;
canvas.height = canvas.width * 2 / 3;
const ctx = canvas.getContext('2d');

// calculate the URL of the worker as being relative to this file
const workerUrl = new URL(`${import.meta.url}/../wasm-worker.js`);
const worker = new Worker(workerUrl.href, {type: "module"});
worker.postMessage("init");

worker.addEventListener('message', async e => {
    if (e.data.status === "success") {
        if (e.data.data.imageData) {
            drawImage(e.data.data.imageData);
        } else if (e.data.data.initialized) {
            btn.disabled = false;
        }
    } else if (e.data.status === "error") {
        // error status probably indicates firefox's lack of support for module
        // workers, so import the renderer directly and run it on the main
        // thread.

        // timer won't tick up anymore, so indicate that things will be block
        timer.pause();
        timer.setLabel("Web Worker failed, running on the main thread...");

        const {wasmRender} = await import("./wasm-render.js");
        drawImage(await wasmRender());

        // restart the timer and step it once to update the total running time
        timer.start();
        timer.step();
    }
    timer && timer.stop();
});

/**
 * Begin the rendering, including starting a timer widget, starting render
 * within the wasm module, and writing the render result into the canvas.
 */
function startRender() {
    timer = addTimer();
    // clearImage();
    timer.start();
    worker.postMessage('render');
}

function addTimer() {
    const newTimer = document.createElement("rtw-timer");
    const li = document.createElement("li");
    li.appendChild(newTimer);
    timers.prepend(li);
    return newTimer;
}

function drawImage(imageData) {
    console.time("drawing canvas");
    ctx.putImageData(imageData, 0, 0);
    console.timeEnd("drawing canvas");
    img.src = canvas.toDataURL();
}

function clearImage() {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
}

btn.addEventListener("click", startRender);

new Zooming({}).listen('.zoomable')
