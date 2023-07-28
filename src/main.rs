use checkers::game::run_game;
use checkers::game::Gamemode;
use std::io;

#[macroquad::main("Checkers")]
async fn main() {
    let debug = true;

    let mut _name = String::new();
    let mut _gamemode = Gamemode::Offline;
    if debug {
        _name = String::from("fst pl");
        _gamemode = Gamemode::Offline;
    } else {
        println!("Enter player name: ");
        _name = io::read_to_string(io::stdin()).unwrap();
        _gamemode = Gamemode::Offline;

        loop {
            println!("Enter game mode(number): \r\n\t 1. offline \r\n\t 2. online");

            _gamemode = match io::read_to_string(io::stdin()).unwrap().trim() {
                x if x == "1" => Gamemode::Offline,
                x if x == "2" => Gamemode::Online,
                _ => {
                    println!("Wrong input, try again...");
                    continue;
                }
            };

            break;
        }
    }

    run_game(_name, _gamemode).await;
}
