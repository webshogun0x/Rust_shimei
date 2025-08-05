use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: u32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: u32) -> u32 {
    (game_coord as f64 * BLOCK_SIZE) as u32
}

pub fn draw_block(color: Color, x: u32, y: u32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

pub fn draw_rectangle(
    color: Color,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    con: &Context,
    g: &mut G2d,
) {
    rectangle(
        color,
        [x, y, BLOCK_SIZE * width, BLOCK_SIZE * height],
        con.transform,
        g,
    );
}
