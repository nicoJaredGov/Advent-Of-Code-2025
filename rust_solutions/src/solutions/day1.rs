pub fn sol(input: &str) -> usize {
    let rotations = input.lines().filter_map(|line| {
        if line.is_empty() {
            return None;
        }

        let direction = line.chars().nth(0).unwrap();
        let range: isize = line[1..]
            .parse()
            .expect("Error parsing rotation number value.");

        Some((direction, range))
    });

    let mut current = 50;
    let num_zeros: usize = rotations.fold(0, |acc, rot| {
        let mut updated = match rot.0 {
            'L' => current - rot.1,
            'R' => current + rot.1,
            _ => panic!("Invalid rotation operation!"),
        };

        updated %= 100;
        if updated < 0 {
            updated += 100;
        }
        current = updated;

        match updated {
            0 => acc + 1,
            _ => acc,
        }
    });

    num_zeros
}
