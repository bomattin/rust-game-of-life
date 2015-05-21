//use std::io;

const WIDTH: usize = 16;
const HEIGHT: usize = 12;
const RULE_NEW_CELL_BORN: [u8; 1] = [3]; // How many neighbors can a dead cell have to be born?
const RULE_LIVE_CELL_LIVES: [u8; 2] = [2,3]; // Making these arrays for ease of adjustment.

/// Print our grid. 
fn display_grid(grid: &[[bool;WIDTH]]) {
    for &row in grid.iter() {
        for &cell in row.iter() {
            if(cell) {
                print!(" O");
            } else {
                print!(" .");
            }

        }
        println!("");
    }
}

/// Return the number of neighbors a given cell has. World is a torus (wrap around)
/// For now, we don't need to know the location of the neighbors.
fn neighbors(grid: &[[bool;WIDTH]], y: usize, x: usize) -> u8 {
    let mut ncount: u8 = 0;
    for y_off in (0..3) {
        let y_test: usize = (y+y_off+HEIGHT-1) % HEIGHT; // Ugly-ass hack because the (x..y) range does not tolerate negative values
        for x_off in (0..3) {
            let x_test: usize = (x+x_off+WIDTH-1) % WIDTH; // Order of ops, matters. Add constant so our unsigned int usize will never go negative
            if (grid[y_test][x_test]) { // EVEN UGLIER HACK FUCK YEAH
                ncount += 1;
            }
        }
    }

    if grid[y][x] { // Don't count ourselves
        ncount - 1
    } else {
        ncount
    }

}

fn live_or_die(grid: &[[bool;WIDTH]], y: usize, x: usize) -> bool {
    let n = neighbors(&grid, y, x);
    if !grid[y][x] {
        for &i in RULE_NEW_CELL_BORN.iter() {
            if n == i {
                return true
            }
        }
    } else {
        for &i in RULE_LIVE_CELL_LIVES.iter() {
            if n == i {
                return true
            }
        }
    }

    false
}

fn main() {

    let mut grid_now = [[false;WIDTH];HEIGHT];
    let mut grid_next = [[false;WIDTH];HEIGHT];


    // let mut grid_prev = [[false;WIDTH];HEIGHT]; // Draw the current grid. Generate the current grid from the previous
    // grid[y][x] FOR THE LOVE OF GOD THIS IS IMPORTANT. WE ACCESS SHIT BACKWARDS YOLO.
    grid_now[3][9] = true;
    grid_now[4][9] = true;
    grid_now[5][9] = true;
    grid_now[6][9] = true;
    grid_now[6][8] = true;

    let n39 = neighbors(&grid_now, 3, 9);
    let n49 = neighbors(&grid_now, 4, 9);
    let n58 = neighbors(&grid_now, 5, 8);
    println!("Cell[3][9] should have 1 neighbor. It has {}.", n39);
    println!("Cell[4][9] should have 2 neighbors. It has {}.", n49);
    println!("Cell[5][8] should have 4 neighbors. It has {}", n58);

    display_grid(&grid_now);
    //display_grid_coords(&grid_now);

    println!("-1 % 16 = {}", -1%16);

    for (y, &row) in grid_now.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            grid_next[y][x] = live_or_die(&grid_now, y, x);
        }
    }

    display_grid(&grid_next);


}
