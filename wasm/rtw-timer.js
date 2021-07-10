export default class Timer extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({mode: 'open'});
        this.active = false;
        this.paused = false;

        this.shadowRoot.innerHTML = `
        <label>
            <meter low=30 high=60 value=0 max=100></meter>
            <span id="label-text"></span>
        </label>
        `;

        this.labelText = this.shadowRoot.querySelector("#label-text");
        this.meter = this.shadowRoot.querySelector("meter");

        this.step = this.step.bind(this);
    }

    start() {
        if (!this.paused) {
            this.startTime = performance.now();
        } else {
            this.paused = false;
        }
        this.active = true;
        requestAnimationFrame(this.step);
    }

    pause() {
        this.paused = true;
    }

    step() {
        if (this.active && !this.paused) {
            requestAnimationFrame(this.step);
            const diff = performance.now() - this.startTime;
            this.setLabel(`${diff.toFixed(2)}ms`);
            this.meter.value = diff / 10;
        }
    }

    stop() {
        this.active = false;
    }

    setLabel(msg) {
        this.labelText.innerText = msg;
    }
}
customElements.define("rtw-timer", Timer);
