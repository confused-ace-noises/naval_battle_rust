use std::{io, usize};
use naval_battle::{self, grid::*, usr_input};
fn main() {
    let mut _won = false;
    let mut grid_p: Grid;
    let mut grid_o: Grid;
    let rows: usize;
    let cols: usize;
    let n_boats: usize;

    loop {
        (rows, cols, n_boats) = match usr_input::start(){
            Ok((a, b, c)) => (a,b,c),
            Err(e) => {println!("an error occurred: {e}"); println!(); continue;}
        };
        break
    }
    
    grid_p = Grid::player_new(&rows, &cols, &n_boats);

    grid_p = grid_p.set_up_boats();


    grid_o = Grid::opponent_new(&grid_p);

    let mut boats_hit_o  = 0;
    let mut boats_hit_p = 0;
    'main_loop: loop {
        let mut usr_in: String = String::new();

        println!("opponent's field: \n");
        grid_o.print();
        
        println!("your field: \n:");
        grid_p.print();

        println!("\n input the row and column, using standard notation (<row, in letter><column, in number>)");
        match io::stdin().read_line(&mut usr_in) {
            Ok(_) => usr_in = String::from(usr_in.trim()),
            Err(_) => {
                println!("an error occurred");
                continue;
            }
        };

        let info_o = match grid_o.upd_grid(usr_in) {
            Ok(info) => info,
            Err(e) => {
                println!("an error occurred: {}", e);
                continue;
            }
        };

        grid_o = info_o.0;
        if info_o.1 {
            boats_hit_o += 1;
            if boats_hit_o >= n_boats {
                grid_o.print();
                println!("You hit all the ships! You won!");
                break 'main_loop;
            }
        }

        let info_p = Grid::attack(&mut grid_p);

        grid_p = info_p.0;
        if info_p.1{
            boats_hit_p += 1;
            if boats_hit_p >= n_boats {
                grid_p.print();
                println!("Your opponent hit all the ships! You lost!");
                break 'main_loop;
            }
        }
    }
}