// import init, {render} from "./pkg/wasm.js";
import "./rtw-timer.js";

const btn = document.querySelector("button");
const canvas = document.querySelector('canvas');
const timers = document.querySelector("#timers");
let timer;

canvas.width = 3 * 100;
canvas.height = 3 * 66;
const ctx = canvas.getContext('2d');

const worker = new Worker('wasm-worker.js', {type: "module"});
worker.addEventListener('message', async e => {
    if (e.data.status === "success") {
        drawImage(e.data.data);
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
    timer.stop();
});

/**
 * Begin the rendering, including starting a timer widget, starting render
 * within the wasm module, and writing the render result into the canvas.
 */
function startRender() {
    timer = addTimer();
    clearImage();
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
}

function clearImage() {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
}

btn.addEventListener("click", startRender);
