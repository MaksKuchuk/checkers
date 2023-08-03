use macroquad::prelude::{
    is_mouse_button_down, is_mouse_button_released, mouse_position, MouseButton,
};

use crate::board::get_pos_order;
use crate::movements::{make_kill_movement, make_movement};
use crate::screen_renderer::get_start_position;

use super::game_rules::{next_order, set_checker_pos_king};
use super::{
    game_rules::get_possible_steps, CELL_HORIZONTAL, CELL_SIZE, CELL_VERTICAL, HANDELED_CHECKER,
    ORDER,
};

pub enum CellError {
    OutOfBoard,
    WrongStep,
}

pub fn is_taken() -> bool {
    if let Some(_) = *HANDELED_CHECKER.lock().unwrap() {
        true
    } else {
        false
    }
}

pub fn take_checker() {
    if !is_mouse_button_down(MouseButton::Left) {
        return;
    }

    let pos = get_cell_by_pixel(mouse_position());

    let place = match pos {
        Ok(val) => val,
        Err(_) => {
            return;
        }
    };

    let order = ORDER.lock().unwrap();

    match get_pos_order(place) {
        Some(v) if v == *order => (),
        _ => return,
    }

    *HANDELED_CHECKER.lock().unwrap() = Some(place);
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
    if !is_mouse_button_released(MouseButton::Left) {
        return Err(CellError::WrongStep);
    }
    get_cell_by_pixel(mouse_position())
}

pub fn place_checker(pos: (i32, i32)) -> bool {
    let mut selected_checker = HANDELED_CHECKER.lock().unwrap();
    let start_pos = *selected_checker.as_ref().unwrap();

    let (steps, steps_kill) = get_possible_steps(start_pos);

    if !steps.contains(&pos) && !steps_kill.contains(&pos) && pos != start_pos {
        return false;
    }

    if pos == start_pos {
        next_order();
    } else if steps.contains(&pos) {
        make_movement(start_pos, pos);
        set_checker_pos_king(pos);
    } else if steps_kill.contains(&pos) {
        make_kill_movement(start_pos, pos);
        set_checker_pos_king(pos);
    }

    *selected_checker = None;
    true
}
