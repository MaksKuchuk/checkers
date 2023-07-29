use super::{draw_rectangle, get_start_position, BLACK_CELL_CL, CELL_SIZE, WHITE_CELL_CL};
use crate::game::{CELL_HORIZONTAL, CELL_VERTICAL};

pub fn draw_board() {
    let start_pos = get_start_position();

    for x in 0..CELL_HORIZONTAL {
        for y in 0..CELL_VERTICAL {
            draw_rectangle(
                start_pos.0 + x as f32 * CELL_SIZE,
                start_pos.1 + y as f32 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                if (x + y) % 2 != 0 {
                    BLACK_CELL_CL
                } else {
                    WHITE_CELL_CL
                },
            )
        }
    }
}
