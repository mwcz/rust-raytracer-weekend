import("./out/wasm-app.js");

export default class RtwRender extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({mode: 'open'});
        fetch("./out/wasm_bg.wasm");
        this.shadowRoot.innerHTML = `
            <style>
            canvas { display: none; }
            img {
                width: 500px;
                /* height:  calc(66px * 3); */
                image-rendering: -moz-crisp-edges;
                image-rendering: -webkit-crisp-edges;
                image-rendering: pixelated;
                image-rendering: crisp-edges;
            }
            </style>

            <canvas width=500 height=333></canvas>
            <img width=500 height=333 class="zoomable">
            <br>
            <button disabled>Render</button>
            <br>
            <ol reversed id="timers"></ol>
            <script type=module>
                fetch("./out/wasm-worker.js");
                fetch("./out/wasm-render.js");
            </script>
        `;
    }
}
customElements.define("rtw-render", RtwRender);
