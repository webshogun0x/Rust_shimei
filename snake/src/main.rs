extern crate piston_window;
extern crate rand;

mod draw;
mod game;
mod menu;
mod snake;

use piston_window::types::Color;
use piston_window::*;

use draw::to_coord_u32;
use game::Game;
use menu::{Menu, MenuOption};

const BACK_COLOR: Color = [0.0, 0.8, 0.0, 1.0];

#[derive(PartialEq)]
enum AppState {
    Menu,
    Playing,
}

fn main() {
    let (width, height) = (20, 20);
    let mut window: PistonWindow =
        WindowSettings::new("Snake Game", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let assets = "./assets/dejavu-fonts-ttf-2.37/ttf";
    let font = format!("{}/DejaVuSans.ttf", assets);
    let mut glyphs = window.load_font(font).unwrap();

    let mut app_state = AppState::Menu;
    let mut menu = Menu::new();
    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match app_state {
                AppState::Menu => {
                    if let Some(option) = menu.key_pressed(key) {
                        match option {
                            MenuOption::NewGame => {
                                game = Game::new(width, height);
                                app_state = AppState::Playing;
                            }
                            MenuOption::Continue => {
                                app_state = AppState::Playing;
                            }
                            _ => {}
                        }
                    }
                }
                AppState::Playing => {
                    if key == Key::Escape {
                        app_state = AppState::Menu;
                    } else {
                        game.key_pressed(key);
                        if game.is_game_over() {
                            menu.update_high_score(game.get_score());
                        }
                    }
                }
            }
        }

        window.draw_2d(&event, |c, g, device| {
            clear(BACK_COLOR, g);
            match app_state {
                AppState::Menu => menu.draw(&c, g, &mut glyphs, width, height),
                AppState::Playing => game.draw(&c, g, &mut glyphs),
            }
            glyphs.factory.encoder.flush(device);
        });

        event.update(|args| {
            if app_state == AppState::Playing {
                game.update(args.dt);
            }
        });
    }
}
