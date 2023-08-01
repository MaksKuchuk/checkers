mod board_renderer;
mod checker_renderer;
mod mouse_renderer;
mod result_renderer;
mod steps_renderer;

use macroquad::{
    prelude::{Color, BLACK, GRAY, GREEN, RED, WHITE},
    shapes::{draw_circle, draw_rectangle},
    window::{clear_background, screen_height, screen_width},
};

use board_renderer::draw_board;
use checker_renderer::draw_checkers;
use mouse_renderer::draw_mouse;
use result_renderer::draw_result;
use steps_renderer::draw_possible_steps;

use crate::game::{CELL_HORIZONTAL, CELL_SIZE, CELL_VERTICAL};

static BACKGROUND_CL: Color = GRAY;

static BLACK_CELL_CL: Color = BLACK;
static WHITE_CELL_CL: Color = WHITE;

static STEP_CL: Color = GREEN;
static STEP_KILL_CL: Color = RED;

static CHECKER_BLACK_CL: Color = Color::new(0.35, 0.35, 0.35, 1.);
static CHECKER_WHITE_CL: Color = WHITE;

pub fn draw() {
    clear_background(BACKGROUND_CL);
    draw_board();
    draw_checkers();
    draw_mouse();
    draw_possible_steps();
    draw_result();
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
