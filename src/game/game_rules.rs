use crate::player::PlayerKind;

use super::{game_mouse_control::CellError, ORDER};

pub fn is_correct_move(place: &Result<(i32, i32), CellError>) -> bool {
    match place {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn next_order() {
    let mut order = ORDER.lock().unwrap();
    *order = if let PlayerKind::First = *order {
        PlayerKind::Second
    } else {
        PlayerKind::First
    };
}
