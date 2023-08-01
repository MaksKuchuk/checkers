use macroquad::prelude::{
    is_mouse_button_down, is_mouse_button_released, mouse_position, MouseButton,
};

use crate::game::CELL_SIZE;
use crate::screen_renderer::get_start_position;
use crate::{checker::SelectedChecker, player::PlayerKind};

use super::{
    game_rules::{get_possible_steps, next_order},
    BOARD, CELL_HORIZONTAL, CELL_VERTICAL, HANDELED_CHECKER, ORDER,
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
    let mut board = BOARD.lock().unwrap();

    match &board[place.0 as usize][place.1 as usize] {
        Some(v) if v.player().as_ref().lock().unwrap().order() == *order => (),
        _ => return,
    };

    let checker = board[place.0 as usize][place.1 as usize].take().unwrap();

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
    if !is_mouse_button_released(MouseButton::Left) {
        return Err(CellError::WrongStep);
    }
    get_cell_by_pixel(mouse_position())
}

pub fn place_checker(place: Result<(i32, i32), CellError>) -> bool {
    let pos = match place {
        Ok(v) => v,
        Err(_) => return false,
    };

    let mut selected_checker = HANDELED_CHECKER.lock().unwrap();

    let (steps, steps_kill) = get_possible_steps(selected_checker.as_ref().unwrap());

    let start_pos = selected_checker.as_ref().unwrap().place();

    if !steps.contains(&pos) && !steps_kill.contains(&pos) && pos != start_pos {
        return false;
    }

    let mut board = BOARD.lock().unwrap();

    let checker = match board[pos.0 as usize][pos.1 as usize] {
        Some(_) => return false,
        None => (*selected_checker).take(),
    };

    let mut checker = checker.unwrap().move_checker();
    let player_ref = checker.player();
    let player = player_ref.as_ref().lock().unwrap();

    match player.order() {
        PlayerKind::First if pos.1 == 0 => checker.set_king(),
        PlayerKind::Second if pos.1 == CELL_VERTICAL - 1 => checker.set_king(),
        _ => (),
    }

    (*board)[pos.0 as usize][pos.1 as usize] = Some(checker);
    if pos == start_pos {
        next_order();
    }
    true
}
