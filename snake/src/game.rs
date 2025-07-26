use piston_window::*;
use piston_window::types::Color;

use rand::Rng;

use crate::snake::{Direction, Snake};
use crate::draw::{draw_block, draw_rectangle};

const FOOD_COLOR: Color = [0.80, 0.0, 0.0, 1.0];
const BOARDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const GAME_OVER_COLOR: Color = [0.70, 0.50, 0.0, 1.0];

const MOVING_PERIOD: f64 = 0.20;
const RESTART_TIME: f64 = 2.0;

pub struct Game {
    snake: Snake,

    food_exits: bool,
    food_x: u32,
    food_y: u32,
    
    width: u32,
    height: u32,
    
    game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(width:u32 , height:u32 ) -> Self {
        Game {
            snake: Snake::new(2, 2),
            food_exits: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false,
            waiting_time: 0.0,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
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

        // Check if the next head position would overlap the body
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

        draw_rectangle(BOARDER_COLOR, 0.0, 0.0, self.width as f64, 1.0, con, g);
        draw_rectangle(BOARDER_COLOR, 0.0, 0.0, 1.0, self.height as f64, con, g);
        draw_rectangle(BOARDER_COLOR, (self.width - 1) as f64, 0.0, 1.0, self.height as f64, con, g);
        draw_rectangle(BOARDER_COLOR, 0.0, (self.height - 1) as f64, self.width as f64, 1.0, con, g);

        if self.game_over {
            draw_rectangle(GAME_OVER_COLOR, 0.0, 0.0, self.width as f64, self.height as f64, con, g);

            // Draw "GAME IS OVER"
            let game_over_text = "GAME IS OVER";
            let score_text = format!("Score: {}", self.snake.body.len() - 3); // assuming initial length is 3

            let transform = con.transform.trans(
                self.width as f64 * 10.0, // center horizontally
                self.height as f64 * 12.0, // position vertically
            );
            
            let score_transform = con.transform.trans(
                self.width as f64 * 10.0,
                self.height as f64 * 16.0,
            );

            Text::new_color([1.0, 1.0, 1.0, 1.0], 48).draw(
                game_over_text,
                glyphs,
                &con.draw_state,
                transform,
                g,
            ).ok();

            Text::new_color([1.0, 1.0, 1.0, 1.0], 32).draw(
                &score_text,
                glyphs,
                &con.draw_state,
                score_transform,
                g,
            ).ok();
        }
    }
    pub fn update(&mut self, dt: f64) {
        self.waiting_time += dt;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exits {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
            self.waiting_time = 0.0; // <-- Reset after move
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

        let mut new_x = rng.random_range(1..self.width - 1);
        let mut new_y = rng.random_range(1..self.height - 1);

        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.random_range(1..self.width - 1);
            new_y = rng.random_range(1..self.height - 1);
        }

        self.food_exits = true;
        self.food_x = new_x;
        self.food_y = new_y;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } 
        else {
            self.game_over = true;
            self.waiting_time = 0.0;
        }
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2 ,2);
        self.food_exits = false;
        self.game_over = false;
        self.waiting_time = 0.0;
        self.food_y = 6;
        self.food_x = 4;
    }
    
}