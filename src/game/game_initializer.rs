use super::{Gamemode, BOARD, CELL_HORIZONTAL, CELL_VERTICAL, FIRST_PLAYER, SECOND_PLAYER};
use crate::{
    checker::Checker,
    player::{Player, PlayerKind},
};
use std::{
    net::Ipv4Addr,
    sync::{Arc, Mutex},
};

fn init_first_player(name: String) {
    FIRST_PLAYER
        .lock()
        .unwrap()
        .fill(name, PlayerKind::First, Ipv4Addr::new(127, 0, 0, 1), 7878)
}

fn init_second_player(name: String) {
    SECOND_PLAYER
        .lock()
        .unwrap()
        .fill(name, PlayerKind::Second, Ipv4Addr::new(127, 0, 0, 1), 7878)
}

fn init_board() {
    let mut b: Vec<Vec<Option<Checker>>> = Vec::new();

    for x in 0..(CELL_HORIZONTAL as usize) {
        b.push(Vec::new());
        for _ in 0..(CELL_VERTICAL as usize) {
            b[x].push(None);
        }
    }

    let fst_player = Arc::clone(&(*FIRST_PLAYER));
    let snd_player = Arc::clone(&(*SECOND_PLAYER));

    (*BOARD.lock().unwrap()) = b;

    init_arrangement(snd_player, fst_player);
}

fn init_arrangement(fst: Arc<Mutex<Player>>, snd: Arc<Mutex<Player>>) {
    let mut b = BOARD.lock().unwrap();

    if CELL_VERTICAL % 2 == 0 {
        for x in (0..CELL_HORIZONTAL as usize).step_by(2) {
            b[x][CELL_VERTICAL as usize - 1] = Some(Checker::new(Arc::clone(&fst)));
            b[x][CELL_VERTICAL as usize - 3] = Some(Checker::new(Arc::clone(&fst)));
        }
        for x in (1..CELL_HORIZONTAL as usize).step_by(2) {
            b[x][CELL_VERTICAL as usize - 2] = Some(Checker::new(Arc::clone(&fst)));
        }
    } else {
        for x in (0..CELL_HORIZONTAL as usize).step_by(2) {
            b[x][CELL_VERTICAL as usize - 2] = Some(Checker::new(Arc::clone(&fst)));
        }
        for x in (1..CELL_HORIZONTAL as usize).step_by(2) {
            b[x][CELL_VERTICAL as usize - 1] = Some(Checker::new(Arc::clone(&fst)));
            b[x][CELL_VERTICAL as usize - 3] = Some(Checker::new(Arc::clone(&fst)));
        }
    }

    for x in (0..CELL_HORIZONTAL as usize).step_by(2) {
        b[x][1] = Some(Checker::new(Arc::clone(&snd)));
    }
    for x in (1..CELL_HORIZONTAL as usize).step_by(2) {
        b[x][0] = Some(Checker::new(Arc::clone(&snd)));
        b[x][2] = Some(Checker::new(Arc::clone(&snd)));
    }
}

pub fn init_game(name: String, _gamemode: Gamemode) {
    init_board();
    init_first_player(name);
    init_second_player(String::new());
}
