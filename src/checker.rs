use std::sync::{Arc, Mutex};

use crate::player::Player;

pub struct Checker {
    player: Arc<Mutex<Player>>,
    is_king: bool,
}

pub struct SelectedChecker {
    checker: Checker,
    place: (i32, i32),
}

impl SelectedChecker {
    pub fn create(checker: Checker, place: (i32, i32)) -> SelectedChecker {
        SelectedChecker { checker, place }
    }

    pub fn checker(&self) -> &Checker {
        &self.checker
    }

    pub fn move_checker(self) -> Checker {
        self.checker
    }

    pub fn place(&self) -> (i32, i32) {
        self.place
    }
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
