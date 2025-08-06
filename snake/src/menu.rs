use piston_window::*;
use std::fs;

#[derive(PartialEq, Copy, Clone)]
pub enum MenuOption {
    NewGame,
    Continue,
    HighScore,
}

#[derive(PartialEq)]
pub enum MenuState {
    Main,
    HighScore,
}

pub struct Menu {
    selected: MenuOption,
    state: MenuState,
    high_score: u32,
}

impl Menu {
    pub fn new() -> Self {
        Menu {
            selected: MenuOption::NewGame,
            state: MenuState::Main,
            high_score: Self::load_high_score(),
        }
    }

    pub fn key_pressed(&mut self, key: Key) -> Option<MenuOption> {
        match self.state {
            MenuState::Main => {
                match key {
                    Key::Up => {
                        self.selected = match self.selected {
                            MenuOption::NewGame => MenuOption::HighScore,
                            MenuOption::Continue => MenuOption::NewGame,
                            MenuOption::HighScore => MenuOption::Continue,
                        };
                    }
                    Key::Down => {
                        self.selected = match self.selected {
                            MenuOption::NewGame => MenuOption::Continue,
                            MenuOption::Continue => MenuOption::HighScore,
                            MenuOption::HighScore => MenuOption::NewGame,
                        };
                    }
                    Key::Return => {
                        if self.selected == MenuOption::HighScore {
                            self.state = MenuState::HighScore;
                        } else {
                            return Some(self.selected);
                        }
                    }
                    _ => {}
                }
            }
            MenuState::HighScore => {
                if key == Key::Escape || key == Key::Return {
                    self.state = MenuState::Main;
                }
            }
        }
        None
    }

    pub fn draw(&self, con: &Context, g: &mut G2d, glyphs: &mut Glyphs, width: u32, height: u32) {
        match self.state {
            MenuState::Main => self.draw_main_menu(con, g, glyphs, width, height),
            MenuState::HighScore => self.draw_high_score(con, g, glyphs, width, height),
        }
    }

    fn draw_main_menu(&self, con: &Context, g: &mut G2d, glyphs: &mut Glyphs, width: u32, height: u32) {
        let center_x = width as f64 * 12.5;
        let start_y = height as f64 * 8.0;

        let title_transform = con.transform.trans(center_x - 50.0, start_y);
        Text::new_color([1.0, 1.0, 1.0, 1.0], 32)
            .draw("SNAKE GAME", glyphs, &con.draw_state, title_transform, g).ok();

        let options = [
            ("New Game", MenuOption::NewGame),
            ("Continue", MenuOption::Continue),
            ("High Score", MenuOption::HighScore),
        ];

        for (i, (text, option)) in options.iter().enumerate() {
            let color = if *option == self.selected { [1.0, 1.0, 0.0, 1.0] } else { [1.0, 1.0, 1.0, 1.0] };
            let transform = con.transform.trans(center_x - 30.0, start_y + 80.0 + i as f64 * 40.0);
            Text::new_color(color, 24)
                .draw(text, glyphs, &con.draw_state, transform, g).ok();
        }
    }

    fn draw_high_score(&self, con: &Context, g: &mut G2d, glyphs: &mut Glyphs, width: u32, height: u32) {
        let center_x = width as f64 * 12.5;
        let center_y = height as f64 * 12.0;

        let title_transform = con.transform.trans(center_x - 60.0, center_y - 40.0);
        Text::new_color([1.0, 1.0, 1.0, 1.0], 32)
            .draw("HIGH SCORE", glyphs, &con.draw_state, title_transform, g).ok();

        let score_text = format!("{}", self.high_score);
        let score_transform = con.transform.trans(center_x - 20.0, center_y + 20.0);
        Text::new_color([1.0, 1.0, 0.0, 1.0], 48)
            .draw(&score_text, glyphs, &con.draw_state, score_transform, g).ok();

        let back_transform = con.transform.trans(center_x - 40.0, center_y + 80.0);
        Text::new_color([0.8, 0.8, 0.8, 1.0], 20)
            .draw("Press ESC to go back", glyphs, &con.draw_state, back_transform, g).ok();
    }

    pub fn update_high_score(&mut self, score: u32) {
        if score > self.high_score {
            self.high_score = score;
            self.save_high_score();
        }
    }

    fn load_high_score() -> u32 {
        if let Ok(content) = fs::read_to_string("high_score.txt") {
            content.trim().parse().unwrap_or(0)
        } else {
            0
        }
    }

    fn save_high_score(&self) {
        fs::write("high_score.txt", self.high_score.to_string()).ok();
    }
}