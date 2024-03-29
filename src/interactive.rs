use std::io::{self, Write};
use std::time::Duration;
use std::env;
use std::thread;
use std::fs;

use crate::game::{Game, GameError};

pub fn create_game() -> Game {
    let args: Vec<String> = env::args().collect();


    if args.len() <= 1 {
        return new_empty_game();
    }

    let filepath = String::from("./game-files/") + &args[1];
    read_game_from_file(&filepath)
}

pub fn maybe_save_game(game: &Game) {
    loop {
        print!("Do you want to save this game to a file? (y/n) ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");


        match input.trim() {
            "y" => break,
            "n" => return,
            _ => continue
        }
    }

    let mut filename = String::new();

    print!("Input the filename (it will be saved in game-files directory): ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut filename)
        .expect("Failed to read line.");

    if let Err(s) = game.to_file(&((String::from("./game-files/")) + filename.trim())) {
        println!("{s}");
        thread::sleep(Duration::from_secs(2));
    }
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
            match Game::from_string(s) {
                Ok(mut game) => {
                    game.set_cursor(0, 0).unwrap();
                    game
                },
                Err(GameError::FormatError) => {
                    println!("Error reading file, wrong format.");
                    new_empty_game()
                },
                Err(GameError::ZeroDimension) => {
                    println!("Board cannot have zero as one dimension.");
                    new_empty_game()
                },
                Err(GameError::OutOfBounds) => {
                    println!("File contains a cell out of bounds.");
                    new_empty_game()
                },
                _ => {
                    println!("Error reading game from file.");
                    new_empty_game()
                }
            }
        },
        Err(_) => {
            println!("There is no such file.");
            new_empty_game()
        }
    }
}
