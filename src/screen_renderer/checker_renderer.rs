use macroquad::shapes::draw_circle;

use super::{get_screen_center, CELL_SIZE, CHECKER_BLACK_CL, CHECKER_SIZE, CHECKER_WHITE_CL};
use crate::game::{CELL_HORIZONTAL, CELL_VERTICAL};

use crate::game::BOARD;
use crate::player::PlayerKind;

fn get_start_position(center: &(f32, f32)) -> (f32, f32) {
    (
        center.0 - CELL_SIZE * CELL_HORIZONTAL as f32 / 2.,
        center.1 - CELL_SIZE * CELL_VERTICAL as f32 / 2.,
    )
}

pub fn draw_checkers() {
    let center = get_screen_center();
    let start_pos = get_start_position(&center);
    let board = BOARD.lock().unwrap();

    for x in 0..CELL_HORIZONTAL {
        for y in 0..CELL_VERTICAL {
            let col = match &board[x as usize][y as usize] {
                None => continue,
                Some(val) => match val.player().lock().unwrap().order() {
                    &PlayerKind::First => CHECKER_WHITE_CL,
                    &PlayerKind::Second => CHECKER_BLACK_CL,
                },
            };

            if let None = board[x as usize][y as usize] {
                continue;
            }

            let xpos = x as f32 * CELL_SIZE + CELL_SIZE / 2.;
            let ypos = y as f32 * CELL_SIZE + CELL_SIZE / 2.;
            draw_circle(
                start_pos.0 + xpos,
                start_pos.1 + ypos,
                CHECKER_SIZE / 2.,
                col,
            );
        }
    }
}
