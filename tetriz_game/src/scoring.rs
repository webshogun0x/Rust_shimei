use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scoring {
    score: u32,
    level: u32,
    lines_cleared: u32,
}

#[wasm_bindgen]
impl Scoring {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Scoring {
        Scoring {
            score: 0,
            level: 1,
            lines_cleared: 0,
        }
    }

    #[wasm_bindgen]
    pub fn add_lines(&mut self, lines: u32) {
        self.lines_cleared += lines;
        
        let points = match lines {
            1 => 100 * self.level,
            2 => 300 * self.level,
            3 => 500 * self.level,
            4 => 800 * self.level, // Tetris!
            _ => 0,
        };
        
        self.score += points;
        self.level = (self.lines_cleared / 10) + 1;
    }

    #[wasm_bindgen]
    pub fn add_soft_drop_points(&mut self, cells: u32) {
        self.score += cells;
    }

    #[wasm_bindgen]
    pub fn add_hard_drop_points(&mut self, cells: u32) {
        self.score += cells * 2;
    }

    #[wasm_bindgen(getter)]
    pub fn score(&self) -> u32 {
        self.score
    }

    #[wasm_bindgen(getter)]
    pub fn level(&self) -> u32 {
        self.level
    }

    #[wasm_bindgen(getter)]
    pub fn lines_cleared(&self) -> u32 {
        self.lines_cleared
    }

    #[wasm_bindgen]
    pub fn get_drop_speed(&self) -> u32 {
        // Speed increases with level (milliseconds between drops)
        match self.level {
            1 => 1000,
            2 => 900,
            3 => 800,
            4 => 700,
            5 => 600,
            6 => 500,
            7 => 400,
            8 => 300,
            9 => 200,
            _ => 100,
        }
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.score = 0;
        self.level = 1;
        self.lines_cleared = 0;
    }
}