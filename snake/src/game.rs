use piston_window::types::Color;
use piston_window::*;

use rand::Rng;

use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Direction, Snake};

const FOOD_COLOR: Color = [0.80, 0.0, 0.0, 1.0];
const BOARDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const GAME_OVER_COLOR: Color = [0.70, 0.50, 0.0, 1.0];

const MOVING_PERIOD: f64 = 0.20;
const RESTART_TIME: f64 = 2.0;

#[derive(PartialEq)]
pub enum GameState {
    Playing,
    Paused,
    GameOver,
}

pub struct Game {
    snake: Snake,
    food_exits: bool,
    food_x: u32,
    food_y: u32,
    width: u32,
    height: u32,
    state: GameState,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        Game {
            snake: Snake::new(2, 2),
            food_exits: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            state: GameState::Playing,
            waiting_time: 0.0,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        match key {
            Key::Space => {
                if self.state == GameState::Playing {
                    self.state = GameState::Paused;
                } else if self.state == GameState::Paused {
                    self.state = GameState::Playing;
                }
                return;
            }
            _ => {}
        }

        if self.state != GameState::Playing {
            return;
        }

        let new_direction = match key {
            Key::Up => Direction::Up,
            Key::Down => Direction::Down,
            Key::Left => Direction::Left,
            Key::Right => Direction::Right,
            _ => return,
        };

        if new_direction == self.snake.head_direction().opposite() {
            return;
        }

        let (next_x, next_y) = self.snake.next_head(Some(new_direction));
        if self.snake.overlap_tail(next_x, next_y) {
            return;
        }

        self.update_snake(Some(new_direction));
    }

    pub fn draw(&self, con: &Context, g: &mut G2d, glyphs: &mut Glyphs) {
        self.snake.draw(con, g);

        if self.food_exits {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        // Draw border
        let w = self.width as f64;
        let h = self.height as f64;
        draw_rectangle(BOARDER_COLOR, 0.0, 0.0, w, 1.0, con, g);
        draw_rectangle(BOARDER_COLOR, 0.0, 0.0, 1.0, h, con, g);
        draw_rectangle(BOARDER_COLOR, w - 1.0, 0.0, 1.0, h, con, g);
        draw_rectangle(BOARDER_COLOR, 0.0, h - 1.0, w, 1.0, con, g);

        if self.state == GameState::Paused {
            let pause_text = "PAUSED - Press SPACE to continue";
            let transform = con.transform.trans(self.width as f64 * 5.0, self.height as f64 * 12.0);
            Text::new_color([1.0, 1.0, 1.0, 1.0], 24)
                .draw(pause_text, glyphs, &con.draw_state, transform, g).ok();
        }

        if self.state == GameState::GameOver {
            draw_rectangle(
                GAME_OVER_COLOR,
                0.0,
                0.0,
                self.width as f64,
                self.height as f64,
                con,
                g,
            );

            let game_over_text = "GAME IS OVER";
            let score_text = format!("Score: {}", self.snake.body.len() - 3);

            let transform = con.transform.trans(
                self.width as f64 * 10.0,
                self.height as f64 * 12.0,
            );

            let score_transform = con
                .transform
                .trans(self.width as f64 * 10.0, self.height as f64 * 16.0);

            Text::new_color([1.0, 1.0, 1.0, 1.0], 48)
                .draw(game_over_text, glyphs, &con.draw_state, transform, g)
                .ok();

            Text::new_color([1.0, 1.0, 1.0, 1.0], 32)
                .draw(&score_text, glyphs, &con.draw_state, score_transform, g)
                .ok();
        }
    }
    pub fn update(&mut self, dt: f64) {
        if self.state != GameState::Playing {
            if self.state == GameState::GameOver {
                self.waiting_time += dt;
                if self.waiting_time > RESTART_TIME {
                    self.restart();
                }
            }
            return;
        }

        self.waiting_time += dt;
        if !self.food_exits {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
            self.waiting_time = 0.0;
        }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y) = self.snake.head_position();
        if self.food_exits && head_x == self.food_x && head_y == self.food_y {
            self.snake.restore_tail();
            self.food_exits = false;
        }
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }
        if next_x >= self.width || next_y >= self.height {
            return false;
        }
        true
    }

    fn add_food(&mut self) {
        let mut rng = rand::rng();
        loop {
            let new_x = rng.random_range(1..self.width - 1);
            let new_y = rng.random_range(1..self.height - 1);
            if !self.snake.overlap_tail(new_x, new_y) {
                self.food_x = new_x;
                self.food_y = new_y;
                self.food_exits = true;
                break;
            }
        }
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.state = GameState::GameOver;
            self.waiting_time = 0.0;
        }
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.food_exits = false;
        self.state = GameState::Playing;
        self.waiting_time = 0.0;
        self.food_y = 6;
        self.food_x = 4;
    }

    pub fn get_score(&self) -> u32 {
        (self.snake.body.len() - 3) as u32
    }

    pub fn is_game_over(&self) -> bool {
        self.state == GameState::GameOver
    }
}
