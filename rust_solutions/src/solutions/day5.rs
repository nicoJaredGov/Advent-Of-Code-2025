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
