use wasm_bindgen::prelude::*;

mod game;
mod board;
mod pieces;
mod scoring;

pub use game::TetrisGame;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(start)]
pub fn main() {
    init_panic_hook();
    console_log!("Tetris WASM module loaded!");
}