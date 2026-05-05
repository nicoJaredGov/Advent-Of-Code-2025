pub fn sol(input: &str) -> usize {
    input.lines().fold(0, |total, bank| {
        let &(idx, biggest) = &bank
            .char_indices()
            .max_by(|(i, a), (j, b)| a.cmp(b).then(i.cmp(j).reverse()))
            .unwrap();

        let result = if idx == bank.len() - 1 {
            let second_biggest = &bank[..bank.len() - 1].chars().max().unwrap();
            format!("{}{}", second_biggest, biggest)
        } else {
            let second_biggest = &bank[idx + 1..bank.len()].chars().max().unwrap();
            format!("{}{}", biggest, second_biggest)
        };

        total + result.parse::<usize>().unwrap()
    })
}

fn find_max_digit(sub_string: &str) -> (usize, char) {
    sub_string
        .char_indices()
        .max_by(|(i, a), (j, b)| a.cmp(b).then(i.cmp(j).reverse()))
        .unwrap()
}

const NUM_BATTERIES: usize = 12;

pub fn sol2(input: &str) -> usize {
    input.lines().fold(0, |total, bank| {
        if bank.len() < NUM_BATTERIES {
            println!("Unable to form a {NUM_BATTERIES}-digit number. Skipping...");
            return 0;
        }

        let mut result = String::new();
        let mut sub_string = bank;
        let (mut idx, mut max_digit) = find_max_digit(sub_string);

        while result.len() != NUM_BATTERIES {
            if sub_string.len() - idx >= NUM_BATTERIES - result.len() {
                result.push(max_digit);
                sub_string = &sub_string[idx + 1..];

                if sub_string.is_empty() {
                    break;
                }

                (idx, max_digit) = find_max_digit(sub_string);
            } else {
                (idx, max_digit) = find_max_digit(&sub_string[..idx]);
            }
        }

        total + result.parse::<usize>().unwrap()
    })
}
