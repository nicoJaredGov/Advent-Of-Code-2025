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

pub fn sol2(input: &str) -> isize {
    let mut current = 50;
    let mut password = 0;

    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .for_each(|line| {
            let direction = line.chars().nth(0).unwrap();
            let mut range: isize = line[1..]
                .parse()
                .expect("Error parsing rotation number value.");

            // Skip if no change
            if range == 0 {
                return;
            }

            // Calculate number of revolutions then simplify range to number without revs
            let num_revs = range / 100;
            range %= 100;

            let mut updated = match direction {
                'L' => current - range,
                'R' => current + range,
                _ => panic!("Invalid rotation operation!"),
            };

            let passed_zero = updated < 0 || updated > 99;
            let landed_on_zero = updated == 0;
            if current != 0 && (passed_zero || landed_on_zero) {
                password += 1;
            }

            // get updated value within 0-99 range
            updated %= 100;
            if updated < 0 {
                updated += 100;
            }

            current = updated;
            password += num_revs;
        });

    password
}
