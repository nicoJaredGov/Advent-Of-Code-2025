use std::collections::{HashMap, HashSet};

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

pub fn sol2(input: &str) -> usize {
    let mut tachyons: HashMap<(usize, usize), usize> = HashMap::new();
    let mut num_timelines = 1;

    for (i, line) in input.lines().enumerate() {
        for (j, sym) in line.char_indices() {
            match sym {
                '^' => {
                    if let Some(count) = tachyons.get(&(i - 1, j)).copied() {
                        let mut new_count = 0;

                        if j > 0 {
                            let tachyon_count = tachyons.entry((i, j - 1)).or_insert(0);
                            *tachyon_count += count;
                            new_count = *tachyon_count;
                        }

                        if j < line.len() - 1 {
                            let tachyon_count = tachyons.entry((i, j + 1)).or_insert(0);
                            *tachyon_count += count;
                            new_count = *tachyon_count;
                        }

                        if j > 0 && j < line.len() - 1 {
                            num_timelines += new_count;
                        }
                    }
                }
                'S' => {
                    tachyons.insert((i, j), 1);
                }
                _ => {
                    if i > 0
                        && let Some(count) = tachyons.get(&(i - 1, j)).copied()
                    {
                        let tachyon_count = tachyons.entry((i, j)).or_insert(0);
                        *tachyon_count += count;
                    }
                }
            }
        }
    }

    num_timelines
}
