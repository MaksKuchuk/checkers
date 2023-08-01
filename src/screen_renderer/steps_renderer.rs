use crate::game::game_rules::get_possible_steps;
use crate::game::{HANDELED_CHECKER, STEP_SIZE};

use super::{draw_circle_in_cell, STEP_CL, STEP_KILL_CL};

pub fn draw_possible_steps() {
    let checker = &*HANDELED_CHECKER.lock().unwrap();
    if let None = checker {
        return;
    }
    let (steps, steps_kill) = get_possible_steps(checker.as_ref().unwrap());

    for pos in steps {
        draw_circle_in_cell(pos, STEP_SIZE, STEP_CL);
    }
    for pos in steps_kill {
        draw_circle_in_cell(pos, STEP_SIZE, STEP_KILL_CL);
    }
}
