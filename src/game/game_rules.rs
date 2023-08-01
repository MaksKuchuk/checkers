use crate::{checker::SelectedChecker, player::PlayerKind};

use super::{
    game_mouse_control::CellError, BOARD, CELL_HORIZONTAL, CELL_VERTICAL, FIRST_PLAYER, ORDER,
};

pub enum StepKind<T> {
    Some(T),
    Empty,
    OutOfBoard,
}

pub fn get_possible_steps(checker: &SelectedChecker) -> (Vec<(i32, i32)>, Vec<(i32, i32)>) {
    let mut steps: Vec<(i32, i32)> = Vec::new();
    let mut steps_kill: Vec<(i32, i32)> = Vec::new();
    let pos = checker.place();
    let ways_top = [(-1, -1), (1, -1)];
    let ways_down = [(-1, 1), (1, 1)];
    let ways_kill = [(-2, -2), (-2, 2), (2, -2), (2, 2)];
    let order = &checker.checker().player().lock().unwrap().order();
    let fst_player_order = &(*FIRST_PLAYER.lock().unwrap()).order();

    for (x, y) in if order == fst_player_order {
        ways_top
    } else {
        ways_down
    } {
        match is_checker_order((pos.0 + x, pos.1 + y)) {
            StepKind::Empty => steps.push((pos.0 + x, pos.1 + y)),
            _ => (),
        }
    }

    for (x, y) in ways_kill {
        match is_checker_order((pos.0 + x, pos.1 + y)) {
            StepKind::Empty => match is_checker_order((pos.0 + x / 2, pos.1 + y / 2)) {
                StepKind::Some(v) if v != checker.checker().player().lock().unwrap().order() => {
                    steps_kill.push((pos.0 + x, pos.1 + y))
                }
                _ => (),
            },
            _ => (),
        }
    }

    (steps, steps_kill)
}

fn is_checker_order(pos: (i32, i32)) -> StepKind<PlayerKind> {
    if pos.0 < 0 || pos.0 >= CELL_HORIZONTAL || pos.1 < 0 || pos.1 >= CELL_VERTICAL {
        return StepKind::OutOfBoard;
    }

    match &(*BOARD.lock().unwrap())[pos.0 as usize][pos.1 as usize] {
        Some(v) => StepKind::Some((*v.player().lock().unwrap()).order()),
        None => StepKind::Empty,
    }
}

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
