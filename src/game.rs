mod game_initializer;
mod game_mouse_control;
pub mod game_rules;

use crate::checker::SelectedChecker;
use crate::player::Player;
use crate::screen_renderer::draw;
use crate::{checker::Checker, player::PlayerKind};
use game_initializer::init_game;
use macroquad::prelude::*;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

use game_mouse_control::take_checker;

use self::game_mouse_control::{is_taken, place_checker, select_place};
use self::game_rules::{get_possible_steps, is_correct_move, next_order};

pub enum Gamemode {
    Online,
    Offline,
}

pub static CELL_SIZE: f32 = 50.;
pub static CELL_HORIZONTAL: i32 = 8;
pub static CELL_VERTICAL: i32 = 8;
pub static CHECKER_SIZE: i32 = 40;
pub static STEP_SIZE: i32 = 20;

pub static BOARD: Lazy<Arc<Mutex<Vec<Vec<Option<Checker>>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(Vec::new())));
pub static FIRST_PLAYER: Lazy<Arc<Mutex<Player>>> =
    Lazy::new(|| Arc::new(Mutex::new(Player::default())));
pub static SECOND_PLAYER: Lazy<Arc<Mutex<Player>>> =
    Lazy::new(|| Arc::new(Mutex::new(Player::default())));
pub static ORDER: Mutex<PlayerKind> = Mutex::new(PlayerKind::First);
pub static HANDELED_CHECKER: Mutex<Option<SelectedChecker>> = Mutex::new(None);

pub async fn run_game(name: String, gamemode: Gamemode) {
    init_game(name, gamemode);

    loop {
        draw();
        make_step();

        next_frame().await;
    }
}

fn make_step() {
    if is_taken() {
        let place = select_place();
        if is_correct_move(&place) {
            if place_checker(place) {
                next_order();
            }
        }
    } else {
        take_checker();
    }
}
