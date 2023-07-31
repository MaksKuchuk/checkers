use checkers::game::run_game;
use checkers::game::Gamemode;
use std::io;

#[macroquad::main("Checkers")]
async fn main() {
    let debug = true;

    let mut name = String::new();
    let mut gamemode = Gamemode::Offline;
    if debug {
        name = String::from("fst pl");
    } else {
        println!("Enter player name: ");
        name = io::read_to_string(io::stdin()).unwrap();

        loop {
            println!("Enter game mode(number): \r\n\t 1. offline \r\n\t 2. online");

            gamemode = match io::read_to_string(io::stdin()).unwrap().trim() {
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

    run_game(name, gamemode).await;
}
