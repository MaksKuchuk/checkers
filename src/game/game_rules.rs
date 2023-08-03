use std::collections::HashSet;

use crate::{
    board::{get_pos_order, set_pos_king},
    checker::Checker,
    player::PlayerKind,
};

use super::{CELL_HORIZONTAL, CELL_VERTICAL, FIRST_PLAYER, ORDER};

pub enum StepKind<T> {
    Some(T),
    Empty,
    OutOfBoard,
}

pub fn get_possible_steps(pos: (i32, i32)) -> (HashSet<(i32, i32)>, HashSet<(i32, i32)>) {
    let order = match get_pos_order(pos) {
        Some(v) => v,
        None => return (HashSet::new(), HashSet::new()),
    };

    let mut steps: HashSet<(i32, i32)> = HashSet::new();

    let all_kill_steps = get_kill_steps_for_all(order);

    if all_kill_steps.is_empty() {
        steps.extend(get_general_steps(order, pos));
    }

    let steps_kill = get_kill_steps(order, pos);

    (steps, steps_kill)
}

fn get_general_steps(order: PlayerKind, pos: (i32, i32)) -> HashSet<(i32, i32)> {
    let mut steps: HashSet<(i32, i32)> = HashSet::new();
    let ways_top = [(-1, -1), (1, -1)];
    let ways_down = [(-1, 1), (1, 1)];
    let fst_player_order = FIRST_PLAYER.lock().unwrap().order();

    for (x, y) in if order == fst_player_order {
        ways_top
    } else {
        ways_down
    } {
        match is_checker_order((pos.0 + x, pos.1 + y)) {
            StepKind::Empty => steps.insert((pos.0 + x, pos.1 + y)),
            _ => false,
        };
    }

    steps
}

fn get_kill_steps(order: PlayerKind, pos: (i32, i32)) -> HashSet<(i32, i32)> {
    let mut steps = HashSet::new();
    let ways_kill = [(-2, -2), (-2, 2), (2, -2), (2, 2)];

    for (x, y) in ways_kill {
        match is_checker_order((pos.0 + x, pos.1 + y)) {
            StepKind::Empty => match is_checker_order((pos.0 + x / 2, pos.1 + y / 2)) {
                StepKind::Some(v) if v != order => steps.insert((pos.0 + x, pos.1 + y)),
                _ => false,
            },
            _ => false,
        };
    }

    steps
}

fn get_kill_steps_for_all(order: PlayerKind) -> HashSet<(i32, i32)> {
    let mut steps: HashSet<(i32, i32)> = HashSet::new();

    for x in 0..CELL_HORIZONTAL {
        for y in 0..CELL_VERTICAL {
            match get_pos_order((x, y)) {
                Some(c) if c == order => (),
                _ => continue,
            }
            steps.extend(get_kill_steps(order, (x as i32, y as i32)))
        }
    }

    steps
}

fn is_checker_order(pos: (i32, i32)) -> StepKind<PlayerKind> {
    if pos.0 < 0 || pos.0 >= CELL_HORIZONTAL || pos.1 < 0 || pos.1 >= CELL_VERTICAL {
        return StepKind::OutOfBoard;
    }

    match get_pos_order(pos) {
        Some(v) => StepKind::Some(v),
        None => StepKind::Empty,
    }
}

pub fn set_checker_pos_king(pos: (i32, i32)) {
    match get_pos_order(pos) {
        Some(v) => match v {
            PlayerKind::First if pos.1 == 0 => set_pos_king(pos),
            PlayerKind::Second if pos.1 == CELL_VERTICAL - 1 => set_pos_king(pos),
            _ => (),
        },
        None => (),
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
