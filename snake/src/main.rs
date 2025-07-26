extern crate rand;
extern crate piston_window;

mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;

use game::Game;
use draw::to_coord_u32;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (30, 30);
    let mut window: PistonWindow = WindowSettings::new("Snake Game", [to_coord_u32(width), to_coord_u32(height)],)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Load font
    let assets = "./assets/dejavu-fonts-ttf-2.37/ttf";
    let font = format!("{}/DejaVuSans.ttf", assets);
    let mut glyphs = window.load_font(font).unwrap();

    let mut game = Game::new(width, height);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, device| {
            clear(BACK_COLOR, g);
            game.draw(&c, g, &mut glyphs);
            glyphs.factory.encoder.flush(device);
        });

        event.update(|args| {
            game.update(args.dt);
        });
    }
}
