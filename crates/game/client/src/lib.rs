pub mod game;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init() {
    use crate::game::*;
    console_error_panic_hook::set_once();
    run();
}
