use crate::util::{Part, logger};

mod input;
mod optimisation;
mod util;

pub fn main_1(grid: Vec<Vec<char>>) {
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

pub fn main_2(mut grid: Vec<Vec<char>>) {
    let mut removed_rolls: Vec<(usize, usize)> = Vec::new();
    let mut iter = 0;

    loop {
        let removed_rolls_before = removed_rolls.len();

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if  grid[i][j] == '@' && optimisation::can_be_accessed_by_forklift(&grid, i, j) {
                    removed_rolls.push((i, j));
                    grid[i][j] = 'x';
                }
            }
        }

        let removed_rolls_after = removed_rolls.len();
        logger().logn(&format!("At the end of iter #{}, total removed rolls = {}", iter, removed_rolls_after));
        if removed_rolls_before == removed_rolls_after {
            // could not remove more rolls: we don't have to keep iterating
            logger().logn(&format!("Could not remove more rolls. Finishing loop."));
            break;
        }
        iter += 1;
    }
    println!("Accessible rolls: {:?}", removed_rolls);
    println!("Num of accessible rolls: {}", removed_rolls.len());
    util::print_grid(&grid, &removed_rolls);
}

pub fn main(part: &Part) {
    let grid = input::parse_input();

    match part {
        Part::One => main_1(grid),
        Part::Two => main_2(grid),
    }
}
