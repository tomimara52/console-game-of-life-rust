mod game;
mod interactive;

use std::thread;
use std::sync::mpsc::{self, Receiver};
use std::io::{self, Read};
use std::time::Duration;
use termios::*;

use interactive::create_game;

const SLEEP_DURATION: Duration = Duration::from_millis(100);

fn spawn_input_thread(quit_char: char) -> Receiver<char> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut input: [u8; 1] = [0;1];

        while input[0] != (quit_char as u8) {
            
            if let Err(e) = io::stdin().read_exact(&mut input) {
                println!("Error reading character: {}", e);
            } else {
                tx.send(input[0] as char).expect("Failed to send input character.");
            }
        }
    });

    rx
}

fn set_termios_lflag(t: &mut Termios, flag: u32) {
    t.c_lflag = flag;

    tcsetattr(0, TCSANOW, t).expect("Failed to set termios lflags.");
}

fn main() {
    let mut game = create_game();

    let mut termios = Termios::from_fd(0).expect("Failed to create termios structure for fd 0.");

    let start_lflag = termios.c_lflag;
    let input_lflag = termios.c_lflag & !(ICANON | ECHO);

    set_termios_lflag(&mut termios, input_lflag);
    
    let rx = spawn_input_thread('r');

    loop {
        let input = rx.try_recv().unwrap_or('0');

        match input {
            'w'|'a'|'s'|'d' => game.move_cursor(input).unwrap(),
            ' ' => game.swap_cell().unwrap(),
            'r' => break,
            _ => {}
        };

        print!("{}[2J", 27 as char);

        game.print_game();

        thread::sleep(SLEEP_DURATION);
    }

    set_termios_lflag(&mut termios, start_lflag);
    interactive::maybe_save_game(&game);
    set_termios_lflag(&mut termios, input_lflag);
    

    game.remove_cursor();

    let mut pause = false;

    let rx = spawn_input_thread('q');
    
    loop {
        let input = rx.try_recv().unwrap_or('0');

        if input == 'q' {
            break;
        }

        if input == 'p' {
            pause = !pause;
        }

        if !pause || input == 'n' {
            game.step_game();
        }

        print!("{}[2J", 27 as char);

        game.print_game();

        thread::sleep(SLEEP_DURATION);

    }

    set_termios_lflag(&mut termios, start_lflag);
}
