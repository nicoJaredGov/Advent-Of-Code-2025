use std::collections::HashSet;

const ROLL: char = '@';

pub fn sol(input: &str) -> usize {
    let grid: Vec<_> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    grid.iter().enumerate().fold(0, |total, (i, row)| {
        let has_top = i > 0;
        let has_bottom = i < grid.len() - 1;

        total
            + row.iter().enumerate().fold(0, |subtotal, (j, &cell)| {
                if cell != ROLL {
                    return subtotal;
                }

                let mut count = -1; // offset to avoid counting itself
                let has_left = j > 0;
                let has_right = j < grid.first().unwrap().len() - 1;

                let mut eval_row = |row_index: usize| {
                    if has_left && grid[row_index][j - 1].eq(&ROLL) {
                        count += 1;
                    }
                    if grid[row_index][j].eq(&ROLL) {
                        count += 1;
                    }
                    if has_right && grid[row_index][j + 1].eq(&ROLL) {
                        count += 1;
                    }
                };

                if has_top {
                    eval_row(i - 1);
                }

                eval_row(i);

                if has_bottom {
                    eval_row(i + 1);
                }

                subtotal + if count < 4 { 1 } else { 0 }
            })
    })
}

pub fn sol2(input: &str) -> usize {
    let grid: Vec<_> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut removed: HashSet<(usize, usize)> = HashSet::new();
    let height = grid.len();
    let width = grid[0].len();

    let count_neighbours =
        |grid: &[Vec<char>], i: usize, j: usize, removed: &HashSet<(usize, usize)>| {
            let mut count = -1; // offset to avoid counting itself

            for di in 0..=2 {
                let row_idx = i.wrapping_add(di).wrapping_sub(1);
                if row_idx >= height {
                    continue;
                }

                for dj in 0..=2 {
                    let col_idx = j.wrapping_add(dj).wrapping_sub(1);
                    if col_idx >= width {
                        continue;
                    }

                    if grid[row_idx][col_idx] == ROLL && !removed.contains(&(row_idx, col_idx)) {
                        count += 1;
                    }
                }
            }

            count
        };

    loop {
        let mut has_removed = false;

        for (i, row) in grid.iter().enumerate() {
            for (j, &col) in row.iter().enumerate() {
                if col != ROLL || removed.contains(&(i, j)) {
                    continue;
                }

                let num_neighbours = count_neighbours(&grid, i, j, &removed);

                if num_neighbours < 4 {
                    removed.insert((i, j));
                    has_removed = true;
                }
            }
        }

        if !has_removed {
            break;
        }
    }

    removed.len()
}
