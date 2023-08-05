mod board_renderer;
mod checker_renderer;
mod mouse_renderer;
mod name_renderer;
mod result_renderer;
mod steps_renderer;

use macroquad::{
    prelude::Color,
    shapes::{draw_circle, draw_circle_lines, draw_rectangle},
    window::{clear_background, screen_height, screen_width},
};

use board_renderer::draw_board;
use checker_renderer::draw_checkers;
use mouse_renderer::draw_mouse;
use result_renderer::draw_result;
use steps_renderer::draw_possible_steps;

use crate::game::{CELL_HORIZONTAL, CELL_SIZE, CELL_VERTICAL};

use self::name_renderer::draw_name;

static BACKGROUND_CL: Color = Color::new(0.974, 0.974, 0.974, 1.);

static BLACK_CELL_CL: Color = Color::new(0.772, 0.392, 0.203, 1.);
static WHITE_CELL_CL: Color = Color::new(0.913, 0.796, 0.647, 1.);

static STEP_CL: Color = Color::new(0.56, 0.807, 0., 1.);
static STEP_KILL_CL: Color = Color::new(0.956, 0.262, 0.211, 1.);

static CHECKER_BLACK_CL: Color = Color::new(0.16, 0.16, 0.16, 1.);
static CHECKER_WHITE_CL: Color = Color::new(0.882, 0.854, 0.741, 1.);

pub fn draw() {
    clear_background(BACKGROUND_CL);
    draw_board();
    draw_checkers();
    draw_possible_steps();
    draw_mouse();
    draw_result();
    draw_name();
}

fn get_screen_center() -> (f32, f32) {
    (screen_width() / 2., screen_height() / 2.)
}

pub fn get_start_position() -> (f32, f32) {
    let center = get_screen_center();
    (
        center.0 - CELL_SIZE * CELL_HORIZONTAL as f32 / 2.,
        center.1 - CELL_SIZE * CELL_VERTICAL as f32 / 2.,
    )
}

fn draw_circle_in_cell(pos: (i32, i32), size: i32, col: Color) {
    let start_pos = get_start_position();
    let xpos = pos.0 as f32 * CELL_SIZE + CELL_SIZE / 2.;
    let ypos = pos.1 as f32 * CELL_SIZE + CELL_SIZE / 2.;
    draw_circle(
        start_pos.0 + xpos,
        start_pos.1 + ypos,
        size as f32 / 2.,
        col,
    );
}
fn draw_circle_lines_in_cell(pos: (i32, i32), size: i32, col: Color) {
    let start_pos = get_start_position();
    let xpos = pos.0 as f32 * CELL_SIZE + CELL_SIZE / 2.;
    let ypos = pos.1 as f32 * CELL_SIZE + CELL_SIZE / 2.;
    draw_circle_lines(
        start_pos.0 + xpos,
        start_pos.1 + ypos,
        size as f32 / 2.,
        size as f32 / 4.,
        col,
    );
}
