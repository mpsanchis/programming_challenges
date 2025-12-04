const ADJACENT_POSITIONS: [[isize;2]; 8] = [
    [-1,-1],
    [-1,0],
    [-1,1],
    [0,-1],
    [0,1],
    [1,-1],
    [1,0],
    [1,1],
];

fn outta_bounds(grid: &[Vec<char>], x: &isize, y: &isize) -> bool {
    *x < 0 || *y < 0 || *x > (grid.len() as isize - 1) || *y > (grid[0].len() as isize - 1)
}

pub fn can_be_accessed_by_forklift(grid: &[Vec<char>], i: usize, j: usize) -> bool {
    let mut adjacent_rolls = 0;

    for k in 0..8 {
        let [x, y] = ADJACENT_POSITIONS[k];
        let [i_adj, j_adj] = [i as isize + x, j as isize + y];

        if outta_bounds(grid, &i_adj, &j_adj) {
            continue;
        }
        let adjacent_element = grid[i_adj as usize][j_adj as usize];

        if adjacent_element == '@' {
            adjacent_rolls += 1;
            if adjacent_rolls > 3 {
                return false;
            }
        }
    }
    true
}