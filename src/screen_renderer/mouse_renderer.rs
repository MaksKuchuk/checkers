use macroquad::{
    prelude::mouse_position,
    shapes::{draw_circle, draw_circle_lines},
};

use crate::{
    board::{get_pos_is_king, get_pos_order},
    game::{CHECKER_SIZE, HANDELED_CHECKER},
    player::PlayerKind,
};

use super::{CHECKER_BLACK_CL, CHECKER_WHITE_CL};

pub fn draw_mouse() {
    let selected_checker_pos = match *HANDELED_CHECKER.lock().unwrap() {
        Some(v) => v,
        None => return,
    };

    let order = match get_pos_order(selected_checker_pos) {
        Some(v) => v,
        None => return,
    };

    let pos = mouse_position();
    let col = if PlayerKind::First == order {
        CHECKER_WHITE_CL
    } else {
        CHECKER_BLACK_CL
    };

    if get_pos_is_king(selected_checker_pos) {
        draw_circle_lines(
            pos.0,
            pos.1,
            CHECKER_SIZE as f32 / 2.,
            CHECKER_SIZE as f32 / 4.,
            col,
        );
    } else {
        draw_circle(pos.0, pos.1, CHECKER_SIZE as f32 / 2., col);
    }
}
