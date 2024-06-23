#[derive(Clone)]
pub enum Square {
    Empty,
    Boat,
    Water,
    Hit,
    Chosen,
}

pub mod grid {
    use std::io;

    use rand::Rng;

    use crate::Square;

    #[derive(Clone)]
    pub struct Grid {
        pub height: usize,
        pub width: usize,
        pub grid: Vec<Vec<Square>>,
        pub n_boats: usize,
    }
    impl Grid {
        pub fn set_up_boats(&mut self) -> Grid {
            let self_grid = self;
            let mut _i = 0;
            while _i < self_grid.n_boats {
                let mut input: String = String::new();
                self_grid.print();
                println!("choose where to place the boats:   (use standard notation (<row, in letter><column, in number>)");
                io::stdin()
                    .read_line(&mut input)
                    .expect("failed to read line");
                let info = match self_grid.validate_input(&input.trim().to_string()) {
                    Ok(a) => a,
                    Err(e) => {
                        println!("an error occurred: {}", e);
                        continue;
                    }
                };
                self_grid.grid[info.0][info.1] = match self_grid.grid[info.0][info.1] {
                    Square::Empty => Square::Chosen,
                    Square::Chosen => {
                        println!();
                        continue;
                    }
                    _ => {
                        _i -= 1;
                        continue;
                    }
                };

                _i += 1;
            }
            self_grid.clone()
        }

        pub fn player_new(height: &usize, width: &usize, n_boats: &usize) -> Grid {
            Grid {
                height: *height,
                width: *width,
                grid: vec![vec![Square::Empty; *width]; *height],
                n_boats: *n_boats,
            }
        }

        pub fn sel_random(height: &usize, width: &usize) -> (usize, usize) {
            (
                rand::thread_rng().gen_range(0, *height),
                rand::thread_rng().gen_range(0, *width),
            )
        }

        pub fn make_grid(rows: &usize, cols: &usize, n_boats: usize) -> Vec<Vec<Square>> {
            let mut grid: Vec<Vec<Square>> = vec![vec![Square::Empty; *cols]; *rows];

            for mut _i in 0..=n_boats {
                let sel = Grid::sel_random(&rows, &cols);
                match grid[sel.0][sel.1] {
                    Square::Boat => {
                        _i -= 1;
                        continue;
                    }
                    Square::Empty | _ => grid[sel.0][sel.1] = Square::Boat,
                }
            }

            return grid;
        }

        pub fn print(&self) {
            use super::reverse_log;
            let mut char1: char = 'A';
            let mut num = 1;
            print!("     {}", num);
            for _i in 0..self.width - 1 {
                for _j in 0..reverse_log(_i+1){
                    print!(" ");
                }
                num += 1;
                print!("{}", num);
            }
            println!();
            for i in 0..self.height {
                print!("{}  ", char1);
                char1 = (char1 as u8 + 1) as char;
                for j in 0..self.width {
                    match self.grid[i][j] {
                        Square::Boat => print!(" [ ]"),
                        Square::Water => print!(" [0]"),
                        Square::Hit => print!(" [X]"),
                        Square::Empty => print!(" [ ]"),
                        Square::Chosen => print!(" [£]"),
                    }
                }
                println!();
            }
        }

        pub fn upd_grid(&mut self, usr_in: String) -> Result<(Self, bool), String> {
            let sel = self.validate_input(&usr_in)?;
            drop(usr_in);
            // Initialize the hit variable
            let hit: bool;

            // Update the grid based on the selection
            self.grid[sel.0][sel.1] = match self.grid[sel.0][sel.1] {
                Square::Boat | Square::Chosen => {
                    hit = true;
                    Square::Hit
                }
                Square::Empty | _ => {
                    hit = false;
                    Square::Water
                }
            };

            // Create the new grid to return
            let new_grid = Grid {
                height: self.height,
                width: self.width,
                grid: self.grid.clone(),
                n_boats: self.n_boats,
            };

            // Return the result
            Ok((new_grid, hit))
        }

        fn validate_input(&self, usr_in: &String) -> Result<(usize, usize), String> {
            let usr_in = usr_in.trim().trim_end().to_string();

            let lenght = 2 + ((self.width as f32).log10().floor()) as usize;
            match usr_in.len() {
                len if len <= lenght && len >=2 => {
                    let row_u: usize = match usr_in[0..1]
                        .to_ascii_uppercase()
                        .chars()
                        .next()
                        .unwrap_or('A') as isize
                        - 65
                    {
                        num1 if num1 > self.width as isize => {
                            drop(usr_in);
                            return Err(String::from("the spot selected doesn't exist."));
                        }
                        num1 => num1 as usize,
                    }; //returns 0 if A, 1 if B, etc

                    let col_u: usize = match Grid::validate_2nd_part(&usr_in[1..]) {
                        Ok(num) => match num {
                            0 => {
                                drop(usr_in);
                                return Err(String::from("that coordinate doesn't exist."));
                            }
                            _ => num - 1,
                        },
                        Err(_) => {
                            drop(usr_in);
                            return Err(String::from(
                                "syntax error. please enter a correct coordinate",
                            ));
                        }
                    };

                    match self.grid.get(row_u) {
                        Some(row) => match row.get(col_u) {
                            Some(col) => match col {
                                Square::Water | Square::Hit => {
                                    drop(usr_in);
                                    return Err(String::from("enter a new coordinate"));
                                }
                                _ => {
                                    drop(usr_in);
                                    return Ok((row_u, col_u));
                                }
                            },
                            None => {
                                drop(usr_in);
                                return Err(String::from("that coordinate is out of bounds"));
                            }
                        },
                        None => {
                            drop(usr_in);
                            return Err(String::from("that coordinate is out of bounds"));
                        }
                    }
                }
                _ => {
                    drop(usr_in);
                    return Err(String::from("the spot selected doesn't exist."));
                }
            };
        }

        fn validate_2nd_part(input: &str) -> Result<usize, String> {
            if input.chars().all(|c| c.is_digit(10)) {
                // Convert the slice to a usize
                input.parse::<usize>().map_err(|e| e.to_string())
            } else {
                Err("Slice contains non-numeric characters".to_string())
            }
        }
    }

    pub mod opponent {

        use crate::grid;
        use crate::{grid::Grid, Square};

        impl grid::Grid {
            pub fn opponent_new(grid: &grid::Grid) -> Grid {
                grid::Grid {
                    height: grid.height,
                    width: grid.width,
                    grid: grid::Grid::make_grid(&grid.height, &grid.width, grid.n_boats),
                    n_boats: grid.n_boats,
                }
            }

            pub fn attack(grid_p: &mut Grid) -> (Grid, bool) {
                let a = grid::Grid::sel_random(&grid_p.height, &grid_p.width);
                let mut hit = false;
                for mut _i in 0..1 {
                    grid_p.grid[a.0][a.1] = match grid_p.grid[a.0][a.1] {
                        Square::Empty => Square::Water,
                        Square::Boat | Square::Chosen => {
                            hit = true;
                            Square::Hit
                        }
                        _ => {
                            _i -= 1;
                            continue;
                        }
                    }
                }
                return (grid_p.clone(), hit);
            }
        }
    }
}

pub mod usr_input {

    use std::{io, usize};

    pub fn start() -> Result<(usize, usize, usize), String> {
        let mut rows_temp = String::new();
        let mut cols_temp = String::new();
        let mut boats_n_temp = String::new();
        let cols: usize;
        let rows: usize;
        let n_boats: usize;

        println!("choose number of cols: ");
        io::stdin()
            .read_line(&mut cols_temp)
            .expect("reading the line failed");
        cols = match cols_temp.trim().parse() {
            Ok(usize) => usize,
            Err(_) => return Err(String::from("please insert a number")),
        };

        println!("choose number of rows: ");
        io::stdin()
            .read_line(&mut rows_temp)
            .expect("reading the line failed");

        rows = match rows_temp.trim().parse() {
            Ok(usize) => usize,
            Err(_) => return Err(String::from("please insert a number")),
        };

        println!("insert a number of boats:");
        io::stdin()
            .read_line(&mut boats_n_temp)
            .expect("reading the line failed");
        n_boats = match boats_n_temp.trim().parse() {
            Ok(x) => match x {
                x if x == 0 => return Err(String::from("The value must be different than 0")),

                x if x > rows * cols => {
                    return Err(String::from("Value too big to fit un the selected grid"))
                }

                x => x,
            },
            Err(_) => return Err(String::from("please insert a number")),
        };

        return Ok((rows, cols, n_boats));
    }
}

fn reverse_log(n: usize) -> usize {
    if n >= 1 && n < 9 {
        3
    } else if n >= 9 && n < 99 {
        2
    } else if n >= 99 {
        0
    } else {
        0
    }
}