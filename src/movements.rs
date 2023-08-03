use crate::board::{move_pos_checker, remove_pos_checker};

pub fn make_movement(pos_from: (i32, i32), pos_to: (i32, i32)) {
    move_pos_checker(pos_from, pos_to);
}

pub fn make_kill_movement(pos_from: (i32, i32), pos_to: (i32, i32)) {
    remove_pos_checker(((pos_from.0 + pos_to.0) / 2, (pos_from.1 + pos_to.1) / 2));
    move_pos_checker(pos_from, pos_to);
}
