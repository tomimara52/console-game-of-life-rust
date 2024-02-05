use std::io::{self, Write};
use std::env;
use crate::game::Game;

pub fn create_game() -> Game {
    let args: Vec<String> = env::args().collect();


    if args.len() == 1 {
        return new_empty_game();
    }

    Game::new(1,1)
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

    Game::new(dim_x, dim_y)
}
