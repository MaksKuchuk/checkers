use macroquad::{prelude::BLACK, text::draw_text};

use crate::game::{CELL_VERTICAL, FIRST_PLAYER, SECOND_PLAYER};

use super::{get_start_position, CELL_SIZE};

fn get_left_bottom_corner() -> (f32, f32) {
    let (x, y) = get_start_position();
    (x, y + CELL_SIZE * CELL_VERTICAL as f32)
}

fn get_left_top_corner() -> (f32, f32) {
    get_start_position()
}

pub fn draw_result() {
    let fst_res = FIRST_PLAYER.lock().unwrap().result();
    let snd_res = SECOND_PLAYER.lock().unwrap().result();

    let x_shift = 0.;
    let y_shift = 20.;
    let font_size = 60.;

    let lt_y_shift = font_size / 4. - y_shift;
    let lb_y_shift = font_size / 4. + y_shift;

    let lb_cor = get_left_bottom_corner();
    let lt_cor = get_left_top_corner();

    draw_text(
        &fst_res.to_string(),
        lb_cor.0 + x_shift,
        lb_cor.1 + lb_y_shift,
        font_size,
        BLACK,
    );
    draw_text(
        &snd_res.to_string(),
        lt_cor.0 + x_shift,
        lt_cor.1 + lt_y_shift,
        font_size,
        BLACK,
    );
}
