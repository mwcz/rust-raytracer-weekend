// rtw-timer.js
var Timer = class extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({mode: "open"});
    this.active = false;
    this.paused = false;
    this.shadowRoot.innerHTML = `
        <label>
            <meter optimum=0 low=500 high=1000 value=0 max=1500></meter>
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
      this._updateLabel();
      requestAnimationFrame(this.step);
    }
  }
  _updateLabel() {
    const diff = performance.now() - this.startTime;
    this.setLabel(`${diff.toFixed(2)}ms`);
    this.meter.value = diff;
  }
  stop() {
    this._updateLabel();
    this.active = false;
  }
  setLabel(msg) {
    this.labelText.innerText = msg;
  }
};
var rtw_timer_default = Timer;
customElements.define("rtw-timer", Timer);

// node_modules/zooming/build/zooming.module.js
var cursor = {
  default: "auto",
  zoomIn: "zoom-in",
  zoomOut: "zoom-out",
  grab: "grab",
  move: "move"
};
function listen(el, event, handler2) {
  var add = arguments.length > 3 && arguments[3] !== void 0 ? arguments[3] : true;
  var options = {passive: false};
  if (add) {
    el.addEventListener(event, handler2, options);
  } else {
    el.removeEventListener(event, handler2, options);
  }
}
function loadImage(src, cb) {
  if (src) {
    var img2 = new Image();
    img2.onload = function onImageLoad() {
      if (cb)
        cb(img2);
    };
    img2.src = src;
  }
}
function getOriginalSource(el) {
  if (el.dataset.original) {
    return el.dataset.original;
  } else if (el.parentNode.tagName === "A") {
    return el.parentNode.getAttribute("href");
  } else {
    return null;
  }
}
function setStyle(el, styles, remember) {
  checkTrans(styles);
  var s = el.style;
  var original = {};
  for (var key in styles) {
    if (remember) {
      original[key] = s[key] || "";
    }
    s[key] = styles[key];
  }
  return original;
}
function bindAll(_this, that) {
  var methods = Object.getOwnPropertyNames(Object.getPrototypeOf(_this));
  methods.forEach(function bindOne(method) {
    _this[method] = _this[method].bind(that);
  });
}
var trans = {
  transitionProp: "transition",
  transEndEvent: "transitionend",
  transformProp: "transform",
  transformCssProp: "transform"
};
var transformCssProp = trans.transformCssProp;
var transEndEvent = trans.transEndEvent;
function checkTrans(styles) {
  var transitionProp = trans.transitionProp, transformProp = trans.transformProp;
  if (styles.transition) {
    var value = styles.transition;
    delete styles.transition;
    styles[transitionProp] = value;
  }
  if (styles.transform) {
    var _value = styles.transform;
    delete styles.transform;
    styles[transformProp] = _value;
  }
}
var noop = function noop2() {
};
var DEFAULT_OPTIONS = {
  enableGrab: true,
  preloadImage: false,
  closeOnWindowResize: true,
  transitionDuration: 0.4,
  transitionTimingFunction: "cubic-bezier(0.4, 0, 0, 1)",
  bgColor: "rgb(255, 255, 255)",
  bgOpacity: 1,
  scaleBase: 1,
  scaleExtra: 0.5,
  scrollThreshold: 40,
  zIndex: 998,
  customSize: null,
  onOpen: noop,
  onClose: noop,
  onGrab: noop,
  onMove: noop,
  onRelease: noop,
  onBeforeOpen: noop,
  onBeforeClose: noop,
  onBeforeGrab: noop,
  onBeforeRelease: noop,
  onImageLoading: noop,
  onImageLoaded: noop
};
var PRESS_DELAY = 200;
var handler = {
  init: function init(instance) {
    bindAll(this, instance);
  },
  click: function click(e) {
    e.preventDefault();
    if (isPressingMetaKey(e)) {
      return window.open(this.target.srcOriginal || e.currentTarget.src, "_blank");
    } else {
      if (this.shown) {
        if (this.released) {
          this.close();
        } else {
          this.release();
        }
      } else {
        this.open(e.currentTarget);
      }
    }
  },
  scroll: function scroll() {
    var el = document.documentElement || document.body.parentNode || document.body;
    var scrollLeft = window.pageXOffset || el.scrollLeft;
    var scrollTop = window.pageYOffset || el.scrollTop;
    if (this.lastScrollPosition === null) {
      this.lastScrollPosition = {
        x: scrollLeft,
        y: scrollTop
      };
    }
    var deltaX = this.lastScrollPosition.x - scrollLeft;
    var deltaY = this.lastScrollPosition.y - scrollTop;
    var threshold = this.options.scrollThreshold;
    if (Math.abs(deltaY) >= threshold || Math.abs(deltaX) >= threshold) {
      this.lastScrollPosition = null;
      this.close();
    }
  },
  keydown: function keydown(e) {
    if (isEscape(e)) {
      if (this.released) {
        this.close();
      } else {
        this.release(this.close);
      }
    }
  },
  mousedown: function mousedown(e) {
    if (!isLeftButton(e) || isPressingMetaKey(e))
      return;
    e.preventDefault();
    var clientX = e.clientX, clientY = e.clientY;
    this.pressTimer = setTimeout(function grabOnMouseDown() {
      this.grab(clientX, clientY);
    }.bind(this), PRESS_DELAY);
  },
  mousemove: function mousemove(e) {
    if (this.released)
      return;
    this.move(e.clientX, e.clientY);
  },
  mouseup: function mouseup(e) {
    if (!isLeftButton(e) || isPressingMetaKey(e))
      return;
    clearTimeout(this.pressTimer);
    if (this.released) {
      this.close();
    } else {
      this.release();
    }
  },
  touchstart: function touchstart(e) {
    e.preventDefault();
    var _e$touches$ = e.touches[0], clientX = _e$touches$.clientX, clientY = _e$touches$.clientY;
    this.pressTimer = setTimeout(function grabOnTouchStart() {
      this.grab(clientX, clientY);
    }.bind(this), PRESS_DELAY);
  },
  touchmove: function touchmove(e) {
    if (this.released)
      return;
    var _e$touches$2 = e.touches[0], clientX = _e$touches$2.clientX, clientY = _e$touches$2.clientY;
    this.move(clientX, clientY);
  },
  touchend: function touchend(e) {
    if (isTouching(e))
      return;
    clearTimeout(this.pressTimer);
    if (this.released) {
      this.close();
    } else {
      this.release();
    }
  },
  clickOverlay: function clickOverlay() {
    this.close();
  },
  resizeWindow: function resizeWindow() {
    this.close();
  }
};
function isLeftButton(e) {
  return e.button === 0;
}
function isPressingMetaKey(e) {
  return e.metaKey || e.ctrlKey;
}
function isTouching(e) {
  e.targetTouches.length > 0;
}
function isEscape(e) {
  var code = e.key || e.code;
  return code === "Escape" || e.keyCode === 27;
}
var overlay = {
  init: function init2(instance) {
    this.el = document.createElement("div");
    this.instance = instance;
    this.parent = document.body;
    setStyle(this.el, {
      position: "fixed",
      top: 0,
      left: 0,
      right: 0,
      bottom: 0,
      opacity: 0
    });
    this.updateStyle(instance.options);
    listen(this.el, "click", instance.handler.clickOverlay.bind(instance));
  },
  updateStyle: function updateStyle(options) {
    setStyle(this.el, {
      zIndex: options.zIndex,
      backgroundColor: options.bgColor,
      transition: "opacity\n        " + options.transitionDuration + "s\n        " + options.transitionTimingFunction
    });
  },
  insert: function insert() {
    this.parent.appendChild(this.el);
  },
  remove: function remove() {
    this.parent.removeChild(this.el);
  },
  fadeIn: function fadeIn() {
    this.el.offsetWidth;
    this.el.style.opacity = this.instance.options.bgOpacity;
  },
  fadeOut: function fadeOut() {
    this.el.style.opacity = 0;
  }
};
var _typeof = typeof Symbol === "function" && typeof Symbol.iterator === "symbol" ? function(obj) {
  return typeof obj;
} : function(obj) {
  return obj && typeof Symbol === "function" && obj.constructor === Symbol && obj !== Symbol.prototype ? "symbol" : typeof obj;
};
var classCallCheck = function(instance, Constructor) {
  if (!(instance instanceof Constructor)) {
    throw new TypeError("Cannot call a class as a function");
  }
};
var createClass = function() {
  function defineProperties(target2, props) {
    for (var i = 0; i < props.length; i++) {
      var descriptor = props[i];
      descriptor.enumerable = descriptor.enumerable || false;
      descriptor.configurable = true;
      if ("value" in descriptor)
        descriptor.writable = true;
      Object.defineProperty(target2, descriptor.key, descriptor);
    }
  }
  return function(Constructor, protoProps, staticProps) {
    if (protoProps)
      defineProperties(Constructor.prototype, protoProps);
    if (staticProps)
      defineProperties(Constructor, staticProps);
    return Constructor;
  };
}();
var _extends = Object.assign || function(target2) {
  for (var i = 1; i < arguments.length; i++) {
    var source = arguments[i];
    for (var key in source) {
      if (Object.prototype.hasOwnProperty.call(source, key)) {
        target2[key] = source[key];
      }
    }
  }
  return target2;
};
var TRANSLATE_Z = 0;
var target = {
  init: function init3(el, instance) {
    this.el = el;
    this.instance = instance;
    this.srcThumbnail = this.el.getAttribute("src");
    this.srcset = this.el.getAttribute("srcset");
    this.srcOriginal = getOriginalSource(this.el);
    this.rect = this.el.getBoundingClientRect();
    this.translate = null;
    this.scale = null;
    this.styleOpen = null;
    this.styleClose = null;
  },
  zoomIn: function zoomIn() {
    var _instance$options = this.instance.options, zIndex = _instance$options.zIndex, enableGrab = _instance$options.enableGrab, transitionDuration = _instance$options.transitionDuration, transitionTimingFunction = _instance$options.transitionTimingFunction;
    this.translate = this.calculateTranslate();
    this.scale = this.calculateScale();
    this.styleOpen = {
      position: "relative",
      zIndex: zIndex + 1,
      cursor: enableGrab ? cursor.grab : cursor.zoomOut,
      transition: transformCssProp + "\n        " + transitionDuration + "s\n        " + transitionTimingFunction,
      transform: "translate3d(" + this.translate.x + "px, " + this.translate.y + "px, " + TRANSLATE_Z + "px)\n        scale(" + this.scale.x + "," + this.scale.y + ")",
      height: this.rect.height + "px",
      width: this.rect.width + "px"
    };
    this.el.offsetWidth;
    this.styleClose = setStyle(this.el, this.styleOpen, true);
  },
  zoomOut: function zoomOut() {
    this.el.offsetWidth;
    setStyle(this.el, {transform: "none"});
  },
  grab: function grab(x, y, scaleExtra) {
    var windowCenter = getWindowCenter();
    var dx = windowCenter.x - x, dy = windowCenter.y - y;
    setStyle(this.el, {
      cursor: cursor.move,
      transform: "translate3d(\n        " + (this.translate.x + dx) + "px, " + (this.translate.y + dy) + "px, " + TRANSLATE_Z + "px)\n        scale(" + (this.scale.x + scaleExtra) + "," + (this.scale.y + scaleExtra) + ")"
    });
  },
  move: function move(x, y, scaleExtra) {
    var windowCenter = getWindowCenter();
    var dx = windowCenter.x - x, dy = windowCenter.y - y;
    setStyle(this.el, {
      transition: transformCssProp,
      transform: "translate3d(\n        " + (this.translate.x + dx) + "px, " + (this.translate.y + dy) + "px, " + TRANSLATE_Z + "px)\n        scale(" + (this.scale.x + scaleExtra) + "," + (this.scale.y + scaleExtra) + ")"
    });
  },
  restoreCloseStyle: function restoreCloseStyle() {
    setStyle(this.el, this.styleClose);
  },
  restoreOpenStyle: function restoreOpenStyle() {
    setStyle(this.el, this.styleOpen);
  },
  upgradeSource: function upgradeSource() {
    if (this.srcOriginal) {
      var parentNode = this.el.parentNode;
      if (this.srcset) {
        this.el.removeAttribute("srcset");
      }
      var temp = this.el.cloneNode(false);
      temp.setAttribute("src", this.srcOriginal);
      temp.style.position = "fixed";
      temp.style.visibility = "hidden";
      parentNode.appendChild(temp);
      setTimeout(function updateSrc() {
        this.el.setAttribute("src", this.srcOriginal);
        parentNode.removeChild(temp);
      }.bind(this), 50);
    }
  },
  downgradeSource: function downgradeSource() {
    if (this.srcOriginal) {
      if (this.srcset) {
        this.el.setAttribute("srcset", this.srcset);
      }
      this.el.setAttribute("src", this.srcThumbnail);
    }
  },
  calculateTranslate: function calculateTranslate() {
    var windowCenter = getWindowCenter();
    var targetCenter = {
      x: this.rect.left + this.rect.width / 2,
      y: this.rect.top + this.rect.height / 2
    };
    return {
      x: windowCenter.x - targetCenter.x,
      y: windowCenter.y - targetCenter.y
    };
  },
  calculateScale: function calculateScale() {
    var _el$dataset = this.el.dataset, zoomingHeight = _el$dataset.zoomingHeight, zoomingWidth = _el$dataset.zoomingWidth;
    var _instance$options2 = this.instance.options, customSize = _instance$options2.customSize, scaleBase = _instance$options2.scaleBase;
    if (!customSize && zoomingHeight && zoomingWidth) {
      return {
        x: zoomingWidth / this.rect.width,
        y: zoomingHeight / this.rect.height
      };
    } else if (customSize && (typeof customSize === "undefined" ? "undefined" : _typeof(customSize)) === "object") {
      return {
        x: customSize.width / this.rect.width,
        y: customSize.height / this.rect.height
      };
    } else {
      var targetHalfWidth = this.rect.width / 2;
      var targetHalfHeight = this.rect.height / 2;
      var windowCenter = getWindowCenter();
      var targetEdgeToWindowEdge = {
        x: windowCenter.x - targetHalfWidth,
        y: windowCenter.y - targetHalfHeight
      };
      var scaleHorizontally = targetEdgeToWindowEdge.x / targetHalfWidth;
      var scaleVertically = targetEdgeToWindowEdge.y / targetHalfHeight;
      var scale = scaleBase + Math.min(scaleHorizontally, scaleVertically);
      if (customSize && typeof customSize === "string") {
        var naturalWidth = zoomingWidth || this.el.naturalWidth;
        var naturalHeight = zoomingHeight || this.el.naturalHeight;
        var maxZoomingWidth = parseFloat(customSize) * naturalWidth / (100 * this.rect.width);
        var maxZoomingHeight = parseFloat(customSize) * naturalHeight / (100 * this.rect.height);
        if (scale > maxZoomingWidth || scale > maxZoomingHeight) {
          return {
            x: maxZoomingWidth,
            y: maxZoomingHeight
          };
        }
      }
      return {
        x: scale,
        y: scale
      };
    }
  }
};
function getWindowCenter() {
  var docEl = document.documentElement;
  var windowWidth = Math.min(docEl.clientWidth, window.innerWidth);
  var windowHeight = Math.min(docEl.clientHeight, window.innerHeight);
  return {
    x: windowWidth / 2,
    y: windowHeight / 2
  };
}
var Zooming = function() {
  function Zooming2(options) {
    classCallCheck(this, Zooming2);
    this.target = Object.create(target);
    this.overlay = Object.create(overlay);
    this.handler = Object.create(handler);
    this.body = document.body;
    this.shown = false;
    this.lock = false;
    this.released = true;
    this.lastScrollPosition = null;
    this.pressTimer = null;
    this.options = _extends({}, DEFAULT_OPTIONS, options);
    this.overlay.init(this);
    this.handler.init(this);
  }
  createClass(Zooming2, [{
    key: "listen",
    value: function listen$$1(el) {
      if (typeof el === "string") {
        var els = document.querySelectorAll(el);
        var i = els.length;
        while (i--) {
          this.listen(els[i]);
        }
      } else if (el.tagName === "IMG") {
        el.style.cursor = cursor.zoomIn;
        listen(el, "click", this.handler.click);
        if (this.options.preloadImage) {
          loadImage(getOriginalSource(el));
        }
      }
      return this;
    }
  }, {
    key: "config",
    value: function config(options) {
      if (options) {
        _extends(this.options, options);
        this.overlay.updateStyle(this.options);
        return this;
      } else {
        return this.options;
      }
    }
  }, {
    key: "open",
    value: function open(el) {
      var _this = this;
      var cb = arguments.length > 1 && arguments[1] !== void 0 ? arguments[1] : this.options.onOpen;
      if (this.shown || this.lock)
        return;
      var target$$1 = typeof el === "string" ? document.querySelector(el) : el;
      if (target$$1.tagName !== "IMG")
        return;
      this.options.onBeforeOpen(target$$1);
      this.target.init(target$$1, this);
      if (!this.options.preloadImage) {
        var srcOriginal = this.target.srcOriginal;
        if (srcOriginal != null) {
          this.options.onImageLoading(target$$1);
          loadImage(srcOriginal, this.options.onImageLoaded);
        }
      }
      this.shown = true;
      this.lock = true;
      this.target.zoomIn();
      this.overlay.insert();
      this.overlay.fadeIn();
      listen(document, "scroll", this.handler.scroll);
      listen(document, "keydown", this.handler.keydown);
      if (this.options.closeOnWindowResize) {
        listen(window, "resize", this.handler.resizeWindow);
      }
      var onOpenEnd = function onOpenEnd2() {
        listen(target$$1, transEndEvent, onOpenEnd2, false);
        _this.lock = false;
        _this.target.upgradeSource();
        if (_this.options.enableGrab) {
          toggleGrabListeners(document, _this.handler, true);
        }
        cb(target$$1);
      };
      listen(target$$1, transEndEvent, onOpenEnd);
      return this;
    }
  }, {
    key: "close",
    value: function close() {
      var _this2 = this;
      var cb = arguments.length > 0 && arguments[0] !== void 0 ? arguments[0] : this.options.onClose;
      if (!this.shown || this.lock)
        return;
      var target$$1 = this.target.el;
      this.options.onBeforeClose(target$$1);
      this.lock = true;
      this.body.style.cursor = cursor.default;
      this.overlay.fadeOut();
      this.target.zoomOut();
      listen(document, "scroll", this.handler.scroll, false);
      listen(document, "keydown", this.handler.keydown, false);
      if (this.options.closeOnWindowResize) {
        listen(window, "resize", this.handler.resizeWindow, false);
      }
      var onCloseEnd = function onCloseEnd2() {
        listen(target$$1, transEndEvent, onCloseEnd2, false);
        _this2.shown = false;
        _this2.lock = false;
        _this2.target.downgradeSource();
        if (_this2.options.enableGrab) {
          toggleGrabListeners(document, _this2.handler, false);
        }
        _this2.target.restoreCloseStyle();
        _this2.overlay.remove();
        cb(target$$1);
      };
      listen(target$$1, transEndEvent, onCloseEnd);
      return this;
    }
  }, {
    key: "grab",
    value: function grab2(x, y) {
      var scaleExtra = arguments.length > 2 && arguments[2] !== void 0 ? arguments[2] : this.options.scaleExtra;
      var cb = arguments.length > 3 && arguments[3] !== void 0 ? arguments[3] : this.options.onGrab;
      if (!this.shown || this.lock)
        return;
      var target$$1 = this.target.el;
      this.options.onBeforeGrab(target$$1);
      this.released = false;
      this.target.grab(x, y, scaleExtra);
      var onGrabEnd = function onGrabEnd2() {
        listen(target$$1, transEndEvent, onGrabEnd2, false);
        cb(target$$1);
      };
      listen(target$$1, transEndEvent, onGrabEnd);
      return this;
    }
  }, {
    key: "move",
    value: function move2(x, y) {
      var scaleExtra = arguments.length > 2 && arguments[2] !== void 0 ? arguments[2] : this.options.scaleExtra;
      var cb = arguments.length > 3 && arguments[3] !== void 0 ? arguments[3] : this.options.onMove;
      if (!this.shown || this.lock)
        return;
      this.released = false;
      this.body.style.cursor = cursor.move;
      this.target.move(x, y, scaleExtra);
      var target$$1 = this.target.el;
      var onMoveEnd = function onMoveEnd2() {
        listen(target$$1, transEndEvent, onMoveEnd2, false);
        cb(target$$1);
      };
      listen(target$$1, transEndEvent, onMoveEnd);
      return this;
    }
  }, {
    key: "release",
    value: function release() {
      var _this3 = this;
      var cb = arguments.length > 0 && arguments[0] !== void 0 ? arguments[0] : this.options.onRelease;
      if (!this.shown || this.lock)
        return;
      var target$$1 = this.target.el;
      this.options.onBeforeRelease(target$$1);
      this.lock = true;
      this.body.style.cursor = cursor.default;
      this.target.restoreOpenStyle();
      var onReleaseEnd = function onReleaseEnd2() {
        listen(target$$1, transEndEvent, onReleaseEnd2, false);
        _this3.lock = false;
        _this3.released = true;
        cb(target$$1);
      };
      listen(target$$1, transEndEvent, onReleaseEnd);
      return this;
    }
  }]);
  return Zooming2;
}();
function toggleGrabListeners(el, handler$$1, add) {
  var types = ["mousedown", "mousemove", "mouseup", "touchstart", "touchmove", "touchend"];
  types.forEach(function toggleListener(type) {
    listen(el, type, handler$$1[type], add);
  });
}
var zooming_module_default = Zooming;

// wasm-app.js
var btn = document.querySelector("button");
var canvas = document.querySelector("canvas");
var img = document.querySelector("img");
var timers = document.querySelector("#timers");
var timer = addTimer();
canvas.width = 5 * 100;
canvas.height = canvas.width * 2 / 3;
var ctx = canvas.getContext("2d");
var moduleWorkerSupported = true;
var workerUrl = new URL(`${import.meta.url}/../wasm-worker.js`);
var worker = new Worker(workerUrl.href, {type: "module"});
var wasmInit;
var wasmRender;
async function preRender() {
  timer = addTimer();
  timer.start();
}
async function render() {
  console.log(`starting render ${["ON", "OFF"][~~moduleWorkerSupported]} the main thread`);
  if (moduleWorkerSupported) {
    worker.postMessage("render");
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
  timer.stop();
}
worker.addEventListener("message", async (e) => {
  if (e.data.status === "success") {
    if (e.data.data.imageData) {
      postRender(e.data.data.imageData);
    } else if (e.data.data.initialized) {
      btn.disabled = false;
    }
  } else if (e.data.status === "error") {
    console.log(`web worker error type: ${e.data.data.type}`);
    if (e.data.data.type === "import") {
      moduleWorkerSupported = false;
      btn.disabled = false;
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
btn.addEventListener("click", () => {
  preRender();
  render();
});
new zooming_module_default({}).listen(".zoomable");
