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
