use super::{draw_circle_in_cell, draw_circle_lines_in_cell, CHECKER_BLACK_CL, CHECKER_WHITE_CL};
use crate::board::{get_pos_is_king, get_pos_order};
use crate::game::{CELL_HORIZONTAL, CELL_VERTICAL, CHECKER_SIZE, HANDELED_CHECKER};

use crate::player::PlayerKind;

pub fn draw_checkers() {
    let handeled_pos = match HANDELED_CHECKER.lock().unwrap().as_ref() {
        Some(v) => *v,
        None => (-1, -1),
    };

    for x in 0..CELL_HORIZONTAL {
        for y in 0..CELL_VERTICAL {
            if (x, y) == handeled_pos {
                continue;
            }

            let col = match get_pos_order((x, y)) {
                None => continue,
                Some(val) => match val {
                    PlayerKind::First => CHECKER_WHITE_CL,
                    PlayerKind::Second => CHECKER_BLACK_CL,
                },
            };

            if get_pos_is_king((x, y)) {
                draw_circle_lines_in_cell((x, y), CHECKER_SIZE, col)
            } else {
                draw_circle_in_cell((x, y), CHECKER_SIZE, col);
            }
        }
    }
}
