use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

use crate::draw::draw_block;

const SNAKE_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
#[derive(Debug, Clone)]

pub struct Block {
    x: u32,
    y: u32,
}

pub struct Snake {
    pub body: LinkedList<Block>,
    direction: Direction,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: u32, y: u32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x, y });

        Snake {
            body,
            direction: Direction::Right,
            tail: None,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g)
        }
    }

    pub fn head_position(&self) -> (u32, u32) {
        let head_block = self.body.front().unwrap(); // head_block: &Block
        (head_block.x, head_block.y)
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }

        let (last_x, last_y): (u32, u32) = self.head_position();

        let new_block = match self.direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1,
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
        };
        self.body.push_front(new_block);
        let remove_block = self.body.pop_back().unwrap();
        self.tail = Some(remove_block);
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (u32, u32) {
        let (head_x, head_y) = self.head_position();

        let moving_dir = dir.unwrap_or(self.direction);
        match moving_dir {
            Direction::Down => (head_x, head_y + 1),
            Direction::Up => (head_x, head_y - 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }

    pub fn restore_tail(&mut self) {
        if let Some(blk) = self.tail.take() {
            self.body.push_back(blk);
        }
    }

    pub fn overlap_tail(&self, x: u32, y: u32) -> bool {
        self.body.iter().skip(1).any(|block| block.x == x && block.y == y)
    }
}
