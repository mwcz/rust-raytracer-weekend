import init, {render} from "./pkg/wasm.js";

async function run() {

    console.log('pre-init');
    await init();
    console.log('post-init');

    console.log('pre-render');
    const pixels = render();
    console.log(pixels);
    window.pixels = pixels;
    console.log('post-render');
}

run();

// init().then(() => {
//     console.log(render);
// });
