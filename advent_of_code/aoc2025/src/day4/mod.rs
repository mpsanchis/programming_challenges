use crate::util::{Part, logger};

mod input;
mod optimisation;
mod util;

pub fn main(part: &Part) {
    let logger = logger();
    let grid = input::parse_input();

    util::print_grid(&grid, &Vec::new());

    let mut accessible_rolls: Vec<(usize, usize)> = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if  grid[i][j] == '@' &&
                optimisation::can_be_accessed_by_forklift(&grid, i, j) {
                accessible_rolls.push((i, j));
            }
        }
    }

    println!("Accessible rolls: {:?}", accessible_rolls);
    println!("Num of accessible rolls: {}", accessible_rolls.len());
    util::print_grid(&grid, &accessible_rolls);
}
