use std::collections::HashSet;

use crate::{
    board::{get_pos_order, is_pos_empty, is_pos_king, set_pos_king},
    player::PlayerKind,
};

use super::{CELL_HORIZONTAL, CELL_VERTICAL, FIRST_PLAYER, MUST_KILL_CHECKER, ORDER};

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

    match *MUST_KILL_CHECKER.lock().unwrap() {
        Some(v) if v == pos => (),
        None => (),
        _ => return (HashSet::new(), HashSet::new()),
    }

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

    if is_pos_king(pos) {
        let maxx = std::cmp::max(CELL_HORIZONTAL, CELL_VERTICAL);
        for xy in 1..maxx {
            if is_pos_empty((pos.0 + xy, pos.1 + xy)) {
                steps.insert((pos.0 + xy, pos.1 + xy));
            } else {
                break;
            }
        }
        for xy in 1..maxx {
            if is_pos_empty((pos.0 + xy, pos.1 - xy)) {
                steps.insert((pos.0 + xy, pos.1 - xy));
            } else {
                break;
            }
        }
        for xy in 1..maxx {
            if is_pos_empty((pos.0 - xy, pos.1 + xy)) {
                steps.insert((pos.0 - xy, pos.1 + xy));
            } else {
                break;
            }
        }
        for xy in 1..maxx {
            if is_pos_empty((pos.0 - xy, pos.1 - xy)) {
                steps.insert((pos.0 - xy, pos.1 - xy));
            } else {
                break;
            }
        }
    } else {
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
    }

    steps
}

fn get_kill_steps(order: PlayerKind, pos: (i32, i32)) -> HashSet<(i32, i32)> {
    let mut steps = HashSet::new();
    let ways_kill = [(-2, -2), (-2, 2), (2, -2), (2, 2)];

    if is_pos_king(pos) {
        let mut f = false;
        let maxx = std::cmp::max(CELL_HORIZONTAL, CELL_VERTICAL);
        for xy in 1..maxx {
            if is_pos_empty((pos.0 + xy, pos.1 + xy)) {
                if f {
                    steps.insert((pos.0 + xy, pos.1 + xy));
                }
            } else {
                if f {
                    break;
                }
                f = true;
            }
        }
        f = false;
        for xy in 1..maxx {
            if is_pos_empty((pos.0 + xy, pos.1 - xy)) {
                if f {
                    steps.insert((pos.0 + xy, pos.1 - xy));
                }
            } else {
                if f {
                    break;
                }
                f = true;
            }
        }
        f = false;
        for xy in 1..maxx {
            if is_pos_empty((pos.0 - xy, pos.1 + xy)) {
                if f {
                    steps.insert((pos.0 - xy, pos.1 + xy));
                }
            } else {
                if f {
                    break;
                }
                f = true;
            }
        }
        f = false;
        for xy in 1..maxx {
            if is_pos_empty((pos.0 - xy, pos.1 - xy)) {
                if f {
                    steps.insert((pos.0 - xy, pos.1 - xy));
                }
            } else {
                if f {
                    break;
                }
                f = true;
            }
        }
    } else {
        for (x, y) in ways_kill {
            match is_checker_order((pos.0 + x, pos.1 + y)) {
                StepKind::Empty => match is_checker_order((pos.0 + x / 2, pos.1 + y / 2)) {
                    StepKind::Some(v) if v != order => steps.insert((pos.0 + x, pos.1 + y)),
                    _ => false,
                },
                _ => false,
            };
        }
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

pub fn is_game_end() -> bool {
    let order = *ORDER.lock().unwrap();
    for x in 0..CELL_HORIZONTAL {
        for y in 0..CELL_VERTICAL {
            match get_pos_order((x, y)) {
                Some(v) if v == order => (),
                _ => continue,
            }
            let (sp, sk) = get_possible_steps((x, y));
            if !sp.is_empty() || !sk.is_empty() {
                return false;
            }
        }
    }
    true
}
