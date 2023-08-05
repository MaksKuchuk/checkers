use crate::{
    board::{get_pos_order, is_pos_empty, move_pos_checker, remove_pos_checker},
    game::{FIRST_PLAYER, SECOND_PLAYER},
    player::PlayerKind,
};

pub fn make_movement(pos_from: (i32, i32), pos_to: (i32, i32)) {
    move_pos_checker(pos_from, pos_to);
}

pub fn make_kill_movement(pos_from: (i32, i32), pos_to: (i32, i32)) {
    let checker_order = match get_pos_order(pos_from) {
        Some(v) => v,
        None => return,
    };

    let fst_ord = FIRST_PLAYER.lock().unwrap().order();
    if fst_ord == checker_order {
        FIRST_PLAYER.lock().unwrap().result_add_one()
    } else {
        SECOND_PLAYER.lock().unwrap().result_add_one()
    };

    for i in 1..((pos_to.0 - pos_from.0).abs()) {
        let x_sign = (pos_to.0 - pos_from.0) / (pos_to.0 - pos_from.0).abs();
        let y_sign = (pos_to.1 - pos_from.1) / (pos_to.1 - pos_from.1).abs();

        if !is_pos_empty((pos_from.0 + i * x_sign, pos_from.1 + i * y_sign)) {
            remove_pos_checker((pos_from.0 + i * x_sign, pos_from.1 + i * y_sign));
        }
    }
    move_pos_checker(pos_from, pos_to);
}
