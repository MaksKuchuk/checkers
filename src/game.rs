use crate::checker::Checker;
use crate::player::Player;
use crate::screen_renderer::draw;
use game_initializer::init_game;
use macroquad::prelude::*;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

mod game_initializer;

pub enum Gamemode {
    Online,
    Offline,
}

pub static CELL_HORIZONTAL: i32 = 8;
pub static CELL_VERTICAL: i32 = 8;

pub static BOARD: Lazy<Arc<Mutex<Vec<Vec<Option<Checker>>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(Vec::new())));
pub static FIRST_PLAYER: Lazy<Arc<Mutex<Player>>> =
    Lazy::new(|| Arc::new(Mutex::new(Player::default())));
pub static SECOND_PLAYER: Lazy<Arc<Mutex<Player>>> =
    Lazy::new(|| Arc::new(Mutex::new(Player::default())));

pub async fn run_game(name: String, gamemode: Gamemode) {
    init_game(name, gamemode);

    loop {
        draw();

        next_frame().await;
    }
}
