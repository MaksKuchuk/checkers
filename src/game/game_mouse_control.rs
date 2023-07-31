use std::sync::Arc;

use macroquad::prelude::{is_mouse_button_pressed, mouse_position, MouseButton};

use crate::checker::{self, Checker, SelectedChecker};
use crate::game::CELL_SIZE;
use crate::screen_renderer::get_start_position;

use super::{
    BOARD, CELL_HORIZONTAL, CELL_VERTICAL, FIRST_PLAYER, HANDELED_CHECKER, ORDER, SECOND_PLAYER,
};

pub enum CellError {
    OutOfBoard,
}

pub fn is_taken() -> bool {
    if let Some(_) = *HANDELED_CHECKER.lock().unwrap() {
        true
    } else {
        false
    }
}

pub fn take_checker() {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return;
    }

    let pos = get_cell_by_pixel(mouse_position());

    let place = match pos {
        Ok(val) => val,
        Err(_) => {
            return;
        }
    };

    let order = &*ORDER.lock().unwrap();

    let checker = (*BOARD.lock().unwrap())[place.0 as usize][place.1 as usize].take();

    let checker = match checker {
        Some(v) => v,
        None => return,
    };

    *HANDELED_CHECKER.lock().unwrap() = Some(SelectedChecker::create(checker, place));
}

fn get_cell_by_pixel(pos: (f32, f32)) -> Result<(i32, i32), CellError> {
    let (x, y) = pos;
    let board_start = get_start_position();
    let x_p = ((x - board_start.0) / CELL_SIZE as f32) as i32;
    let y_p = ((y - board_start.1) / CELL_SIZE as f32) as i32;

    if x_p < 0 || x_p >= CELL_HORIZONTAL || y_p < 0 || y_p >= CELL_VERTICAL {
        return Err(CellError::OutOfBoard);
    }

    Ok((x_p, y_p))
}

pub fn select_place() -> Result<(i32, i32), CellError> {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return Err(CellError::OutOfBoard);
    }
    get_cell_by_pixel(mouse_position())
}

pub fn place_checker(place: Result<(i32, i32), CellError>) {
    let pos = match place {
        Ok(v) => v,
        Err(_) => return,
    };

    let checker = match (*BOARD.lock().unwrap())[pos.0 as usize][pos.1 as usize] {
        Some(_) => return,
        None => (*HANDELED_CHECKER.lock().unwrap()).take(),
    };
    let checker = checker.unwrap().move_checker();

    (*BOARD.lock().unwrap())[pos.0 as usize][pos.1 as usize] = Some(checker);
}
