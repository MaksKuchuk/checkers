use macroquad::{
    prelude::mouse_position,
    shapes::{draw_circle, draw_circle_lines},
};

use crate::{
    game::{CHECKER_SIZE, HANDELED_CHECKER},
    player::PlayerKind,
};

use super::{CHECKER_BLACK_CL, CHECKER_WHITE_CL};

pub fn draw_mouse() {
    if let None = *HANDELED_CHECKER.lock().unwrap() {
        return;
    }

    let selected_checker = &*HANDELED_CHECKER.lock().unwrap();
    let checker = selected_checker.as_ref().unwrap();
    let order = checker.checker().player().as_ref().lock().unwrap().order();
    let pos = mouse_position();
    let col = if PlayerKind::First == order {
        CHECKER_WHITE_CL
    } else {
        CHECKER_BLACK_CL
    };

    if checker.checker().is_king() {
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
