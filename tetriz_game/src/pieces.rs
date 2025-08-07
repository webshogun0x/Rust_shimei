cduse serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PieceType {
    I, O, T, S, Z, J, L
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Piece {
    piece_type: PieceType,
    x: i32,
    y: i32,
    rotation: usize,
    shape: Vec<Vec<bool>>,
}

#[wasm_bindgen]
impl Piece {
    #[wasm_bindgen(constructor)]
    pub fn new(piece_type: PieceType) -> Piece {
        let shape = get_piece_shape(piece_type, 0);
        Piece {
            piece_type,
            x: 4,
            y: 0,
            rotation: 0,
            shape,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> i32 { self.x }
    
    #[wasm_bindgen(getter)]
    pub fn y(&self) -> i32 { self.y }
    
    #[wasm_bindgen(getter)]
    pub fn piece_type(&self) -> PieceType { self.piece_type }

    #[wasm_bindgen]
    pub fn move_left(&mut self) { self.x -= 1; }
    
    #[wasm_bindgen]
    pub fn move_right(&mut self) { self.x += 1; }
    
    #[wasm_bindgen]
    pub fn move_down(&mut self) { self.y += 1; }

    #[wasm_bindgen]
    pub fn rotate(&mut self) {
        self.rotation = (self.rotation + 1) % 4;
        self.shape = get_piece_shape(self.piece_type, self.rotation);
    }

    #[wasm_bindgen]
    pub fn set_x(&mut self, x: i32) { self.x = x; }
    
    #[wasm_bindgen]
    pub fn set_y(&mut self, y: i32) { self.y = y; }

    #[wasm_bindgen]
    pub fn get_blocks(&self) -> JsValue {
        let mut blocks = Vec::new();
        for (row_idx, row) in self.shape.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                if cell {
                    blocks.push((self.x + col_idx as i32, self.y + row_idx as i32));
                }
            }
        }
        serde_wasm_bindgen::to_value(&blocks).unwrap()
    }
}

fn get_piece_shape(piece_type: PieceType, rotation: usize) -> Vec<Vec<bool>> {
    match piece_type {
        PieceType::I => match rotation % 2 {
            0 => vec![
                vec![true, true, true, true]
            ],
            _ => vec![
                vec![true],
                vec![true],
                vec![true],
                vec![true]
            ]
        },
        PieceType::O => vec![
            vec![true, true],
            vec![true, true]
        ],
        PieceType::T => match rotation % 4 {
            0 => vec![
                vec![false, true, false],
                vec![true, true, true]
            ],
            1 => vec![
                vec![true, false],
                vec![true, true],
                vec![true, false]
            ],
            2 => vec![
                vec![true, true, true],
                vec![false, true, false]
            ],
            _ => vec![
                vec![false, true],
                vec![true, true],
                vec![false, true]
            ]
        },
        PieceType::S => match rotation % 2 {
            0 => vec![
                vec![false, true, true],
                vec![true, true, false]
            ],
            _ => vec![
                vec![true, false],
                vec![true, true],
                vec![false, true]
            ]
        },
        PieceType::Z => match rotation % 2 {
            0 => vec![
                vec![true, true, false],
                vec![false, true, true]
            ],
            _ => vec![
                vec![false, true],
                vec![true, true],
                vec![true, false]
            ]
        },
        PieceType::J => match rotation % 4 {
            0 => vec![
                vec![true, false, false],
                vec![true, true, true]
            ],
            1 => vec![
                vec![true, true],
                vec![true, false],
                vec![true, false]
            ],
            2 => vec![
                vec![true, true, true],
                vec![false, false, true]
            ],
            _ => vec![
                vec![false, true],
                vec![false, true],
                vec![true, true]
            ]
        },
        PieceType::L => match rotation % 4 {
            0 => vec![
                vec![false, false, true],
                vec![true, true, true]
            ],
            1 => vec![
                vec![true, false],
                vec![true, false],
                vec![true, true]
            ],
            2 => vec![
                vec![true, true, true],
                vec![true, false, false]
            ],
            _ => vec![
                vec![true, true],
                vec![false, true],
                vec![false, true]
            ]
        }
    }
}

pub fn get_random_piece() -> PieceType {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..7) {
        0 => PieceType::I,
        1 => PieceType::O,
        2 => PieceType::T,
        3 => PieceType::S,
        4 => PieceType::Z,
        5 => PieceType::J,
        _ => PieceType::L,
    }
}

pub fn get_piece_color(piece_type: PieceType) -> &'static str {
    match piece_type {
        PieceType::I => "#00f0f0",
        PieceType::O => "#f0f000",
        PieceType::T => "#a000f0",
        PieceType::S => "#00f000",
        PieceType::Z => "#f00000",
        PieceType::J => "#0000f0",
        PieceType::L => "#f0a000",
    }
}