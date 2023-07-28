use super::{draw_rectangle, get_screen_center, BLACK_CELL_CL, CELL_SIZE, WHITE_CELL_CL};
use crate::game::{CELL_HORIZONTAL, CELL_VERTICAL};

fn get_start_position(center: &(f32, f32)) -> (f32, f32) {
    (
        center.0 - CELL_SIZE * CELL_HORIZONTAL as f32 / 2.,
        center.1 - CELL_SIZE * CELL_VERTICAL as f32 / 2.,
    )
}

pub fn draw_board() {
    let center = get_screen_center();
    let start_pos = get_start_position(&center);

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
