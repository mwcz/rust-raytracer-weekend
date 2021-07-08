mod utils;

use js_sys::Uint8ClampedArray;
use rtw;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn render() -> Vec<u8> {
    rtw::render()
    // unsafe {
    //     Uint8ClampedArray::from(rtw::render());
    // }
}
