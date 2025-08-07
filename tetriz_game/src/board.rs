use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use crate::pieces::{Piece, PieceType};

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    grid: Vec<Vec<Option<PieceType>>>,
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Board {
        Board {
            grid: vec![vec![None; BOARD_WIDTH]; BOARD_HEIGHT],
        }
    }

    #[wasm_bindgen]
    pub fn is_valid_position(&self, piece: &Piece) -> bool {
        let blocks = serde_wasm_bindgen::from_value::<Vec<(i32, i32)>>(piece.get_blocks()).unwrap();
        
        for (x, y) in blocks {
            if x < 0 || x >= BOARD_WIDTH as i32 || y >= BOARD_HEIGHT as i32 {
                return false;
            }
            if y >= 0 && self.grid[y as usize][x as usize].is_some() {
                return false;
            }
        }
        true
    }

    #[wasm_bindgen]
    pub fn place_piece(&mut self, piece: &Piece) {
        let blocks = serde_wasm_bindgen::from_value::<Vec<(i32, i32)>>(piece.get_blocks()).unwrap();
        
        for (x, y) in blocks {
            if y >= 0 && y < BOARD_HEIGHT as i32 && x >= 0 && x < BOARD_WIDTH as i32 {
                self.grid[y as usize][x as usize] = Some(piece.piece_type());
            }
        }
    }

    #[wasm_bindgen]
    pub fn clear_lines(&mut self) -> u32 {
        let mut lines_cleared = 0;
        let mut y = BOARD_HEIGHT as i32 - 1;

        while y >= 0 {
            if self.is_line_full(y as usize) {
                self.clear_line(y as usize);
                lines_cleared += 1;
            } else {
                y -= 1;
            }
        }

        lines_cleared
    }

    fn is_line_full(&self, row: usize) -> bool {
        self.grid[row].iter().all(|cell| cell.is_some())
    }

    fn clear_line(&mut self, row: usize) {
        self.grid.remove(row);
        self.grid.insert(0, vec![None; BOARD_WIDTH]);
    }

    #[wasm_bindgen]
    pub fn get_grid(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.grid).unwrap()
    }

    #[wasm_bindgen]
    pub fn is_game_over(&self) -> bool {
        self.grid[0].iter().any(|cell| cell.is_some())
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.grid = vec![vec![None; BOARD_WIDTH]; BOARD_HEIGHT];
    }
}