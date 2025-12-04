use crate::util::{logger};

pub fn print_grid(grid: &[Vec<char>], replacements: &[(usize, usize)]) {
    let logger = logger();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if replacements.contains(&(i,j)) {
                logger.log("x");
            } else {
                logger.log(&format!("{}", grid[i][j]));
            }
        }
        logger.logn("");
    }
}