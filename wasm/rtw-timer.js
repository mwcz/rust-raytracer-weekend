import "@mwcz/pbp-loading";

export default class Timer extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: "open" });

        this.active = false;
        this.paused = false;

        this.step = this.step.bind(this);
    }

    connectedCallback() {
        this.shadowRoot.innerHTML = `
        <style>
            pbp-loading {
                width: 100%;
            }
            label {
                display: grid;
                grid-template-columns: 1fr;
                grid-gap: 6px;
            }
        </style>
        <label>
            <pbp-loading paused duration=0.8 box-count=8></pbp-loading>
            <span id="label-text">&nbsp;</span>
        </label>
        `;

        this.labelText = this.shadowRoot.querySelector("#label-text");
        this.loading = this.shadowRoot.querySelector("pbp-loading");
    }

    resetSpinner() {
        this.loading.style.removeProperty("--play-state");
        this.labelText.parentNode.removeChild(this.loading);
        this.labelText.parentNode.insertBefore(this.loading, this.labelText);
    }

    start() {
        if (!this.paused) {
            this.startTime = performance.now();
        } else {
            this.paused = false;
        }
        this.active = true;
        this.loading.play();
        requestAnimationFrame(this.step);
    }

    pause() {
        this.paused = true;
        this.loading.pause();
    }

    step() {
        if (this.active && !this.paused) {
            this.duration = performance.now() - this.startTime;
            this._updateLabel(this.duration);
            requestAnimationFrame(this.step);
        }
    }

    _updateLabel(duration) {
        this.setLabel(`${duration.toFixed(1)}ms`);
    }

    stop() {
        this.resetSpinner();
        this.active = false;
    }

    setLabel(msg) {
        this.labelText.innerText = msg;
    }
}
customElements.define("rtw-timer", Timer);
