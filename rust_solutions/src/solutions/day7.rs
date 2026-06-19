use std::collections::HashSet;

pub fn sol(input: &str) -> usize {
    let mut tachyons: HashSet<(usize, usize)> = HashSet::new();
    let mut num_splits = 0;

    for (i, line) in input.lines().enumerate() {
        for (j, sym) in line.char_indices() {
            match sym {
                '^' => {
                    if tachyons.contains(&(i - 1, j)) {
                        num_splits += 1;

                        if j > 0 {
                            tachyons.insert((i, j - 1));
                        }

                        if j < line.len() - 1 {
                            tachyons.insert((i, j + 1));
                        }
                    }
                }
                'S' => {
                    tachyons.insert((i, j));
                }
                _ => {
                    if i > 0 && tachyons.contains(&(i - 1, j)) {
                        tachyons.insert((i, j));
                    }
                }
            }
        }
    }

    num_splits
}
