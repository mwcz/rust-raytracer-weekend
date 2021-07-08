import init, {render} from "./pkg/wasm.js";

const btn = document.querySelector("button");

function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

async function run() {

    const canvas = document.querySelector('canvas');
    canvas.width = 3 * 100;
    canvas.height = 3 * 66;
    const ctx = canvas.getContext('2d');

    console.time("init");
    await init();
    console.timeEnd("init");

    console.time("tracing rays");
    const pixels = new ImageData(new Uint8ClampedArray(render()), 300);
    window.pixels = pixels;
    // const pixels = render();
    console.timeEnd("tracing rays");

    console.time("drawing canvas");
    ctx.putImageData(pixels, 0, 0);
    // let x = 0;
    // let y = 0;
    // var i4 = 0;
    // for (var i = 0; i < pixels.length; i += 1, i4 += 4) {
    //     y = Math.floor((i4) / 100);
    //     x = Math.floor((i4) % 100);
    //     ctx.fillStyle = `rgba(${pixels[i4]}, ${pixels[i4 + 1]}, ${pixels[i4 + 2]})`;
    //     ctx.fillRect(x * dpr, y * dpr, dpr, dpr);
    // }
    console.timeEnd("drawing canvas");

}

btn.addEventListener("click", run);
