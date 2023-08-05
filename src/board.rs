use crate::{
    checker::Checker,
    game::{BOARD, CELL_HORIZONTAL, CELL_VERTICAL},
    player::PlayerKind,
};

pub fn get_pos_order(pos: (i32, i32)) -> Option<PlayerKind> {
    match BOARD.lock().unwrap()[pos.0 as usize][pos.1 as usize].as_ref() {
        Some(v) => Some(v.get_order()),
        None => None,
    }
}

pub fn get_pos_is_king(pos: (i32, i32)) -> bool {
    match BOARD.lock().unwrap()[pos.0 as usize][pos.1 as usize].as_ref() {
        Some(v) if v.is_king() => true,
        _ => false,
    }
}

pub fn get_pos_checker(pos: (i32, i32)) -> Option<Checker> {
    (BOARD.lock().unwrap()[pos.0 as usize][pos.1 as usize]).take()
}

pub fn set_pos_checker(pos: (i32, i32), checker: Checker) {
    (*BOARD.lock().unwrap())[pos.0 as usize][pos.1 as usize] = Some(checker);
}

pub fn move_pos_checker(pos_from: (i32, i32), pos_to: (i32, i32)) {
    let mut board = BOARD.lock().unwrap();
    (*board)[pos_to.0 as usize][pos_to.1 as usize] =
        (*board)[pos_from.0 as usize][pos_from.1 as usize].take()
}

pub fn remove_pos_checker(pos: (i32, i32)) {
    (*BOARD.lock().unwrap())[pos.0 as usize][pos.1 as usize] = None;
}

pub fn set_pos_king(pos: (i32, i32)) {
    match (*BOARD.lock().unwrap())[pos.0 as usize][pos.1 as usize] {
        Some(ref mut v) => v.set_king(),
        None => (),
    }
}

pub fn is_pos_king(pos: (i32, i32)) -> bool {
    match &(*BOARD.lock().unwrap())[pos.0 as usize][pos.1 as usize] {
        Some(v) => v.is_king(),
        None => false,
    }
}

pub fn is_pos_empty(pos: (i32, i32)) -> bool {
    if pos.0 < 0 || pos.1 < 0 || pos.0 >= CELL_HORIZONTAL || pos.1 >= CELL_VERTICAL {
        return false;
    }

    match (*BOARD.lock().unwrap())[pos.0 as usize][pos.1 as usize] {
        Some(_) => false,
        None => true,
    }
}
