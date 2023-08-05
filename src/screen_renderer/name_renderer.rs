use macroquad::{prelude::BLACK, text::draw_text};

use crate::game::{CELL_HORIZONTAL, CELL_SIZE, CELL_VERTICAL, FIRST_PLAYER, SECOND_PLAYER};

use super::get_start_position;

fn get_top_center() -> (f32, f32) {
    let (x, y) = get_start_position();
    (x + CELL_SIZE * (CELL_HORIZONTAL / 2) as f32, y)
}

fn get_bottom_center() -> (f32, f32) {
    let (x, y) = get_start_position();
    (
        x + CELL_SIZE * (CELL_HORIZONTAL / 2) as f32,
        y + CELL_SIZE * CELL_VERTICAL as f32,
    )
}

pub fn draw_name() {
    let fst = FIRST_PLAYER.lock().unwrap();
    let snd = SECOND_PLAYER.lock().unwrap();

    let fst_name = fst.name();
    let snd_name = snd.name();

    let x_shift = 0.;
    let y_shift = 20.;
    let font_size = 36.;

    let rt_y_shift = font_size / 4. - y_shift;
    let rb_y_shift = font_size / 4. + y_shift;

    let rb_cor = get_bottom_center();
    let rt_cor = get_top_center();

    match fst_name {
        Some(v) => {
            draw_text(
                v,
                rb_cor.0 + x_shift,
                rb_cor.1 + rb_y_shift,
                font_size,
                BLACK,
            );
        }
        _ => (),
    }

    match snd_name {
        Some(v) => {
            draw_text(
                v,
                rt_cor.0 + x_shift,
                rt_cor.1 + rt_y_shift,
                font_size,
                BLACK,
            );
        }
        _ => (),
    }
}
