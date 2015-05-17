use std::io;

const WIDTH: usize = 32;
const HEIGHT: usize = 16;

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

fn main() {

    let mut grid = [[false;WIDTH];HEIGHT];
    grid[3][9] = true;
    grid[4][9] = true;
    grid[5][9] = true;
    grid[6][9] = true;

    display_grid(&grid);
}
