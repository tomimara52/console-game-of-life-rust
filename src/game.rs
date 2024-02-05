use std::{fs::File, io::Write};

const WHITE_BG: &str = "\x1b[47m";
const BLACK_FG: &str = "\x1b[30m";
const RED_BG: &str   = "\x1b[41m";
const RESET: &str    = "\x1b[0m";

#[derive(Debug)]
pub enum GameError {
    OutOfBounds,
    CursorIsNone,
    InvalidDirection,
    ZeroDimension,
    FormatError
}

pub struct Game {
    board: Vec<Vec<bool>>,
    dim_x: usize,
    dim_y: usize,
    cursor: Option<(usize, usize)>
}

impl Game {
    pub fn new(dim_x: usize, dim_y: usize) -> Self {
        let mut game = Game {
            board: Vec::with_capacity(dim_x),
            dim_x,
            dim_y,
            cursor: None
        };

        for _ in 0..dim_x { 
            game.board.push(vec![false; dim_y]);
        }

        game
    }

    pub fn print_game(&self) {
        for y in 0..self.dim_y {
            for x in 0..self.dim_x {
                let cell = self.board[x][y];

                match self.cursor {
                    Some(coor) if coor == (x, y)  => {
                        print!("{RED_BG}");

                        if cell {
                            print!("X");
                        } else {
                            print!("O");
                        }
                    },
                    _ => {
                        if cell {
                            print!("{WHITE_BG}{BLACK_FG}X");
                        } else {
                            print!("O");
                        }
                    }
                };

                print!("{RESET} ");
            }
            println!("");
        }
    }

    fn set_cell(&mut self, x: usize, y: usize) -> Result<(), GameError> {
        if x >= self.dim_x || y >= self.dim_y {
            return Err(GameError::OutOfBounds);
        }

        self.board[x][y] = true;
        Ok(())
    }

    pub fn step_game(&mut self) {
        let board = self.board.clone();

        let bool_to_int = |b: bool| -> u32 {
            if b { 1 } else { 0 }
        };

        for x in 0..self.dim_x {
            for y in 0..self.dim_y {
                let mut neigh = 0;

                if x > 0 {
                    if y > 0 {
                        neigh += bool_to_int(board[x-1][y-1]);
                    }

                    neigh += bool_to_int(board[x-1][y]);

                    if y < self.dim_y - 1 {
                        neigh += bool_to_int(board[x-1][y+1]);
                    }                        
                }

                if y > 0 {
                    neigh += bool_to_int(board[x][y-1]);
                }

                if y < self.dim_y - 1 {
                    neigh += bool_to_int(board[x][y+1]);
                }                        

                if x < self.dim_x - 1 {
                    if y > 0 {
                        neigh += bool_to_int(board[x+1][y-1]);
                    }

                    neigh += bool_to_int(board[x+1][y]);

                    if y < self.dim_y - 1 {
                        neigh += bool_to_int(board[x+1][y+1]);
                    }                        
                }

                if !board[x][y] && neigh == 3 {
                    self.board[x][y] = true;
                } else if board[x][y] && (neigh < 2 || neigh > 3) {
                    self.board[x][y] = false;
                }
            }
        }
    }

    pub fn set_cursor(&mut self, x: usize, y: usize) -> Result<(), GameError> {
        if x >= self.dim_x || y >= self.dim_y {
            return Err(GameError::OutOfBounds);
        }

        self.cursor = Some((x, y));

        Ok(())
    }

    pub fn move_cursor(&mut self, dir: char) -> Result<(), GameError> {
        if let None = self.cursor {
            return Err(GameError::CursorIsNone);
        }

        let mut cursor = self.cursor.unwrap();
        
        match dir {
            'w' => {
                if cursor.1 > 0 {
                    cursor.1 -= 1;
                }
            },
            'a' => {
                if cursor.0 > 0 {
                    cursor.0 -= 1;
                }
            },
            's' => {
                if cursor.1 < self.dim_y - 1 {
                    cursor.1 += 1;
                }
            },
            'd' => {
                if cursor.0 < self.dim_x - 1 {
                    cursor.0 += 1;
                }
            },
            _ => {
                return Err(GameError::InvalidDirection);
            }
        }

        self.cursor = Some(cursor);

        Ok(())
    }

    pub fn remove_cursor(&mut self) {
        self.cursor = None;
    }

    pub fn swap_cell(&mut self) -> Result<(), GameError> {
        if let None = self.cursor {
            return Err(GameError::CursorIsNone);
        }

        let (x, y) = self.cursor.unwrap();

        self.board[x][y] = !self.board[x][y];

        Ok(())
    }

    pub fn from_string(s: String) -> Result<Self, GameError> {
        let mut lines = s.trim().lines();

        let (dim_x, dim_y) = match lines.next() {
            Some(s) => {
                match read_pair(&s, "x") {
                    Some((x, y)) if x == 0 || y == 0 => return Err(GameError::ZeroDimension),
                    Some(t) => t,
                    None => return Err(GameError::FormatError)
                }
            },
            None => return Err(GameError::FormatError)
        };

        let mut game = Game::new(dim_x, dim_y);

        for line in lines {

            let (x, y) = match read_pair(&line, ",") {
                Some(t) => t,
                None => return Err(GameError::FormatError)
            };

            if let Err(_) = game.set_cell(x, y) {
                return Err(GameError::OutOfBounds);
            }
        }

        Ok(game)
    }

    pub fn to_file(&self, filepath: &str) {
        let mut file = match File::create(filepath) {
            Ok(f) => f,
            Err(_) => {
                println!("Error creating file.");
                return;
            }
        };

        let mut game_str = String::new();

        game_str += &self.dim_x.to_string();
        game_str += "x";
        game_str += &self.dim_y.to_string();
        game_str += "\n";

        for (x, col) in self.board.iter().enumerate() {
            for (y, &cell) in col.iter().enumerate() {
                if cell {
                    game_str += &x.to_string();
                    game_str += ",";
                    game_str += &y.to_string();
                    game_str += "\n";
                }
            }
        }

        if let Err(_) = file.write(game_str.as_bytes()) {
            println!("Error when writing to file");
        }
    }
}

fn read_pair(s: &str, sep: &str) -> Option<(usize, usize)> {
    let pair_vec: Vec<&str> = s.split(sep).collect();

    if pair_vec.len() != 2 {
        return None;
    }

    let first: usize = match pair_vec[0].trim().parse() {
        Ok(n) => n,
        Err(_) => return None
    };

    let second: usize = match pair_vec[1].trim().parse() {
        Ok(n) => n,
        Err(_) => return None
    };

    Some((first, second))
}
