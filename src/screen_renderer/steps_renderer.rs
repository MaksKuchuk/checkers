use crate::game::game_rules::get_possible_steps;
use crate::game::{HANDELED_CHECKER, MUST_KILL_CHECKER, STEP_SIZE};

use super::{draw_circle_in_cell, STEP_CL, STEP_KILL_CL};

pub fn draw_possible_steps() {
    let checker_pos = match *HANDELED_CHECKER.lock().unwrap() {
        Some(v) => v,
        None => return,
    };

    let (steps, steps_kill) = get_possible_steps(checker_pos);

    for pos in steps {
        draw_circle_in_cell(pos, STEP_SIZE, STEP_CL);
    }
    for pos in steps_kill {
        draw_circle_in_cell(pos, STEP_SIZE, STEP_KILL_CL);
    }
}
