pub fn sol(input: &str) -> usize {
    let (id_ranges, ingredients) = input.split_once("\n\n").unwrap();

    let id_ranges: Vec<(usize, usize)> = id_ranges
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }

            let (a, b) = line.split_once('-')?;
            Some((a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        })
        .collect();

    ingredients.lines().fold(0, |count, id| {
        let id = id.trim();
        if id.is_empty() {
            return count;
        }

        let id = match id.parse::<usize>() {
            Ok(id) => id,
            Err(_) => return count,
        };

        for (min, max) in &id_ranges {
            if id >= *min && id <= *max {
                return count + 1;
            }
        }

        return count;
    })
}

pub fn sol2(input: &str) -> usize {
    let id_ranges = input.split_once("\n\n").unwrap().0;

    let mut id_ranges: Vec<(usize, usize)> = id_ranges
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }

            let (a, b) = line.split_once('-')?;
            Some((a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        })
        .collect();

    id_ranges.sort();

    let mut num_ids = 0;
    for i in 0..id_ranges.len() {
        let (a, mut b) = id_ranges[i];

        if let Some((c, d)) = id_ranges.get(i + 1) {
            if b >= *c {
                let right = c - 1;

                if b > *d {
                    id_ranges[i + 1].1 = b;
                }

                b = right;
            }
        }

        if b >= a {
            num_ids += b - a + 1;
        }
    }

    num_ids
}
