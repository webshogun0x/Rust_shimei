use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use crate::board::Board;
use crate::pieces::{Piece, get_random_piece};
use crate::scoring::Scoring;

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameState {
    Playing,
    Paused,
    GameOver,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TetrisGame {
    board: Board,
    current_piece: Option<Piece>,
    next_piece: Piece,
    scoring: Scoring,
    state: GameState,
    last_drop_time: f64,
}

#[wasm_bindgen]
impl TetrisGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> TetrisGame {
        let mut game = TetrisGame {
            board: Board::new(),
            current_piece: None,
            next_piece: Piece::new(get_random_piece()),
            scoring: Scoring::new(),
            state: GameState::Playing,
            last_drop_time: 0.0,
        };
        game.spawn_piece();
        game
    }

    #[wasm_bindgen]
    pub fn spawn_piece(&mut self) {
        if self.current_piece.is_none() {
            self.current_piece = Some(self.next_piece.clone());
            self.next_piece = Piece::new(get_random_piece());
            
            if let Some(ref piece) = self.current_piece {
                if !self.board.is_valid_position(piece) {
                    self.state = GameState::GameOver;
                }
            }
        }
    }

    #[wasm_bindgen]
    pub fn move_left(&mut self) -> bool {
        if let Some(ref mut piece) = self.current_piece {
            piece.move_left();
            if !self.board.is_valid_position(piece) {
                piece.move_right();
                return false;
            }
            true
        } else {
            false
        }
    }

    #[wasm_bindgen]
    pub fn move_right(&mut self) -> bool {
        if let Some(ref mut piece) = self.current_piece {
            piece.move_right();
            if !self.board.is_valid_position(piece) {
                piece.move_left();
                return false;
            }
            true
        } else {
            false
        }
    }

    #[wasm_bindgen]
    pub fn rotate(&mut self) -> bool {
        if let Some(ref mut piece) = self.current_piece {
            piece.rotate();
            if !self.board.is_valid_position(piece) {
                // Try wall kicks
                for &offset in &[-1, 1, -2, 2] {
                    let new_x = piece.x() + offset;
                    piece.set_x(new_x);
                    if self.board.is_valid_position(piece) {
                        return true;
                    }
                    piece.set_x(piece.x() - offset);
                }
                // Rotate back if no valid position found
                piece.rotate();
                piece.rotate();
                piece.rotate();
                return false;
            }
            true
        } else {
            false
        }
    }

    #[wasm_bindgen]
    pub fn soft_drop(&mut self) -> bool {
        if let Some(ref mut piece) = self.current_piece {
            let old_y = piece.y();
            piece.move_down();
            if !self.board.is_valid_position(piece) {
                piece.set_y(old_y);
                self.lock_piece();
                return false;
            }
            self.scoring.add_soft_drop_points(1);
            true
        } else {
            false
        }
    }

    #[wasm_bindgen]
    pub fn hard_drop(&mut self) -> u32 {
        let mut cells_dropped = 0;
        if let Some(ref mut piece) = self.current_piece {
            while self.board.is_valid_position(piece) {
                piece.move_down();
                cells_dropped += 1;
            }
            piece.set_y(piece.y() - 1);
            cells_dropped -= 1;
            self.scoring.add_hard_drop_points(cells_dropped);
            self.lock_piece();
        }
        cells_dropped
    }

    fn lock_piece(&mut self) {
        if let Some(piece) = self.current_piece.take() {
            self.board.place_piece(&piece);
            let lines_cleared = self.board.clear_lines();
            if lines_cleared > 0 {
                self.scoring.add_lines(lines_cleared);
            }
            self.spawn_piece();
        }
    }

    #[wasm_bindgen]
    pub fn update(&mut self, current_time: f64) -> bool {
        if matches!(self.state, GameState::Playing) {
            let drop_speed = self.scoring.get_drop_speed() as f64;
            if current_time - self.last_drop_time >= drop_speed {
                self.last_drop_time = current_time;
                return self.soft_drop();
            }
        }
        true
    }

    #[wasm_bindgen]
    pub fn pause(&mut self) {
        match self.state {
            GameState::Playing => self.state = GameState::Paused,
            GameState::Paused => self.state = GameState::Playing,
            _ => {}
        }
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.board.reset();
        self.scoring.reset();
        self.current_piece = None;
        self.next_piece = Piece::new(get_random_piece());
        self.state = GameState::Playing;
        self.last_drop_time = 0.0;
        self.spawn_piece();
    }

    #[wasm_bindgen]
    pub fn get_board(&self) -> Board {
        self.board.clone()
    }

    #[wasm_bindgen]
    pub fn get_current_piece(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.current_piece).unwrap()
    }

    #[wasm_bindgen]
    pub fn get_next_piece(&self) -> Piece {
        self.next_piece.clone()
    }

    #[wasm_bindgen]
    pub fn get_scoring(&self) -> Scoring {
        self.scoring.clone()
    }

    #[wasm_bindgen]
    pub fn get_state(&self) -> GameState {
        self.state.clone()
    }

    #[wasm_bindgen]
    pub fn is_game_over(&self) -> bool {
        matches!(self.state, GameState::GameOver)
    }
}