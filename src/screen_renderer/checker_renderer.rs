use macroquad::shapes::draw_circle;

use super::{
    get_start_position, BLACK_CELL_CL, CELL_SIZE, CHECKER_BLACK_CL, CHECKER_SIZE, CHECKER_WHITE_CL,
    WHITE_CELL_CL,
};
use crate::game::{CELL_HORIZONTAL, CELL_VERTICAL};

use crate::game::BOARD;
use crate::player::PlayerKind;

pub fn draw_checkers() {
    let start_pos = get_start_position();
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

            if let Some(val) = &board[x as usize][y as usize] {
                if val.is_king() {
                    draw_circle(
                        start_pos.0 + xpos,
                        start_pos.1 + ypos,
                        CHECKER_SIZE / 4.,
                        if (x + y) % 2 != 0 {
                            BLACK_CELL_CL
                        } else {
                            WHITE_CELL_CL
                        },
                    );
                }
            }
        }
    }
}
