use std::sync::{Arc, Mutex};

use crate::player::Player;

pub struct Checker {
    player: Arc<Mutex<Player>>,
    is_king: bool,
}

impl Checker {
    pub fn new(player: Arc<Mutex<Player>>) -> Checker {
        Checker {
            player,
            is_king: false,
        }
    }

    pub fn player(&self) -> Arc<Mutex<Player>> {
        Arc::clone(&self.player)
    }

    pub fn is_king(&self) -> bool {
        self.is_king
    }

    pub fn set_king(&mut self) {
        self.is_king = true;
    }
}
