use std::io;

const WIDTH: usize = 16;
const HEIGHT: usize = 12;

/// Print our grid. Only one necessary is
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

fn display_grid_coords(grid: &[[bool;WIDTH]]) {
    for (y, &row) in grid.iter().enumerate() {
        for (x,&cell) in row.iter().enumerate() {
            print!("[{},{}]", x, y);
        }
        println!("");
    }
}

/// Return the number of neighbors a given cell has. World is a torus (wrap around)
/// For now, we don't need to know the location of the neighbors.
fn neighbors(grid: &[[bool;WIDTH]], y: usize, x: usize) -> u8 {
    let mut ncount: u8 = 0;
    for y_off in (0..3) {
        // Ugly-ass hack because the (x..y) range does not tolerate negative values
        //println!("Evaluating row {}", y+y_off-1);
        for x_off in (0..3) {
            //println!("Evaluating column {}", x+x_off-1);
            println!("Grid position [{}][{}] is {}", y+y_off-1, x+x_off-1, grid[(x + x_off - 1) % WIDTH][(y + y_off-1) % HEIGHT]);
            if (grid[(y + y_off-1) % HEIGHT][(x + x_off - 1) % WIDTH]){
                ncount += 1;
            }
        }
    }
    if grid[y][x] {
        ncount - 1
    } else {
        ncount
    }

}

fn main() {

    let mut grid_now = [[false;WIDTH];HEIGHT];
    let mut grid_neighbors = [[0;WIDTH];HEIGHT];


    // let mut grid_prev = [[false;WIDTH];HEIGHT]; // Draw the current grid. Generate the current grid from the previous
    // grid[y][x] FOR THE LOVE OF GOD THIS IS IMPORTANT. WE ACCESS SHIT BACKWARDS YOLO.
    grid_now[3][9] = true;
    grid_now[4][9] = true;
    grid_now[5][9] = true;
    grid_now[6][9] = true;

    let n39 = neighbors(&grid_now, 3, 9);
    println!("Cell[3][9] should have 1 neighbor. It has {}.", n39);

    display_grid(&grid_now);
    //display_grid_coords(&grid_now);


}
