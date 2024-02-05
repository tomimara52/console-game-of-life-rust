use std::io::{self, Write};
use std::env;
use std::fs;

use crate::game::Game;

pub fn create_game() -> Game {
    let args: Vec<String> = env::args().collect();


    if args.len() <= 1 {
        return new_empty_game();
    }

    let filepath = String::from("./game-files/") + &args[1];
    read_game_from_file(&filepath)
}

fn read_usize(msg: &str, error_msg: &str) -> usize {
    loop {
        print!("{msg}");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        if let Ok(n) = input.trim().parse() {
            if n > 0 {
                break n;
            }
        };
        println!("{error_msg}");
    }
}

fn new_empty_game() -> Game {
    let dim_x = read_usize("Put the width you want the board to be: ", "Invalid number.");
    let dim_y = read_usize("Put the height you want the board to be: ", "Invalid number.");

    let mut game = Game::new(dim_x, dim_y);
    game.set_cursor(0, 0).unwrap();

    game
}

fn read_game_from_file(filepath: &String) -> Game {
    match fs::read_to_string(filepath) {
        Ok(s) => {
            if let Some(mut game) = Game::from_string(s) {
                game.set_cursor(0, 0).unwrap();
                game
            } else {
                println!("Error reading file, it is probably a wrong format.");
                new_empty_game()
            }
        },
        Err(_) => {
            println!("There is no such file.");
            new_empty_game()
        }
    }
}
