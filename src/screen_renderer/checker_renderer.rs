use super::{draw_circle_in_cell, draw_circle_lines_in_cell, CHECKER_BLACK_CL, CHECKER_WHITE_CL};
use crate::game::{CELL_HORIZONTAL, CELL_VERTICAL, CHECKER_SIZE};

use crate::game::BOARD;
use crate::player::PlayerKind;

pub fn draw_checkers() {
    let board = BOARD.lock().unwrap();

    for x in 0..CELL_HORIZONTAL {
        for y in 0..CELL_VERTICAL {
            let col = match &board[x as usize][y as usize] {
                None => continue,
                Some(val) => match val.player().lock().unwrap().order() {
                    PlayerKind::First => CHECKER_WHITE_CL,
                    PlayerKind::Second => CHECKER_BLACK_CL,
                },
            };

            if let None = board[x as usize][y as usize] {
                continue;
            }

            if board[x as usize][y as usize].as_ref().unwrap().is_king() {
                draw_circle_lines_in_cell((x, y), CHECKER_SIZE, col)
            } else {
                draw_circle_in_cell((x, y), CHECKER_SIZE, col);
            }
        }
    }
}
