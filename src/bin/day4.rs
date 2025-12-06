fn main() {
    let input = include_str!("../../inputs/day4_test.txt");

    let mut grid: Vec<Vec<i32>> = input
        .lines()
        .map(|row| {
            row.chars()
                .map(|c| match c {
                    '@' => 1,
                    '.' => 0,
                    _ => panic!("Faulty input!"),
                })
                .collect()
        })
        .collect();

    let n_rows = grid.len();
    let n_cols = grid[0].len();
    let n_rolls = part1(&grid, n_rows, n_cols);
    println!("Part 1: {n_rolls}");

    let mut done = false;
    let mut total_removed = 0;
    while !done {
        let deletable = find_deletable(&grid, n_rows, n_cols);

        if deletable.len() == 0 {
            done = true;
        } else {
            for (r, c) in deletable {
                grid[r][c] = 0;
                total_removed += 1;
            }
        }
    }
    println!("{total_removed}")
}

fn find_deletable(grid: &Vec<Vec<i32>>, n_rows: usize, n_cols: usize) -> Vec<(usize, usize)> {
    let mut deletable: Vec<(usize, usize)> = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if *value == 1 {
                let n_adj = count_adjacent(&grid, i, j, n_rows, n_cols);
                if n_adj < 4 {
                    deletable.push((i, j));
                }
            }
        }
    }
    deletable
}

fn part1(grid: &Vec<Vec<i32>>, n_rows: usize, n_cols: usize) -> i32 {
    let mut count = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if *value == 1 {
                let n_adj = count_adjacent(&grid, i, j, n_rows, n_cols);
                if n_adj < 4 {
                    count += 1;
                }
            }
        }
    }
    count
}
fn count_adjacent(grid: &Vec<Vec<i32>>, r: usize, c: usize, n_rows: usize, n_cols: usize) -> i32 {
    let mut n_adj = 0;

    // top left
    if (r > 0) && (c > 0) {
        n_adj += grid[r - 1][c - 1];
    }
    // top
    if r > 0 {
        n_adj += grid[r - 1][c];
    }
    // top right
    if (r > 0) && (c + 1 < n_cols) {
        n_adj += grid[r - 1][c + 1];
    }
    // right
    if c + 1 < n_cols {
        n_adj += grid[r][c + 1];
    }
    // bot right
    if (r + 1 < n_rows) && (c + 1 < n_cols) {
        n_adj += grid[r + 1][c + 1];
    }
    // bot
    if r + 1 < n_rows {
        n_adj += grid[r + 1][c];
    }
    // bot left
    if (r + 1 < n_rows) && (c > 0) {
        n_adj += grid[r + 1][c - 1];
    }
    // left
    if c > 0 {
        n_adj += grid[r][c - 1];
    }
    n_adj
}
