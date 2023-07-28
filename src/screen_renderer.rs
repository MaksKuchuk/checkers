mod board_renderer;
mod checker_renderer;

use macroquad::{
    prelude::{Color, BLACK, GRAY, WHITE},
    shapes::draw_rectangle,
    window::{clear_background, screen_height, screen_width},
};

use board_renderer::draw_board;
use checker_renderer::draw_checkers;

static BACKGROUND_CL: Color = GRAY;

static CELL_SIZE: f32 = 50.;
static BLACK_CELL_CL: Color = BLACK;
static WHITE_CELL_CL: Color = WHITE;

static CHECKER_SIZE: f32 = 40.;
static CHECKER_BLACK_CL: Color = Color::new(0.35, 0.35, 0.35, 1.);
static CHECKER_WHITE_CL: Color = WHITE;

pub fn draw() {
    clear_background(BACKGROUND_CL);
    draw_board();
    draw_checkers();
}

fn get_screen_center() -> (f32, f32) {
    (screen_width() / 2., screen_height() / 2.)
}
