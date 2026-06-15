pub fn sol(input: &str) -> usize {
    let mut lines = input.lines();
    let operations: Vec<&str> = lines.next_back().unwrap().split_whitespace().collect();

    let matrix: Vec<Vec<usize>> = lines
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let num_columns = matrix[0].len();
    let mut grand_total = 0;

    for col in 0..num_columns {
        let column = matrix.iter().map(|row| row[col]);

        grand_total += match operations.get(col) {
            Some(&"*") => column.product(),
            Some(&"+") => column.sum(),
            _ => 0,
        };
    }

    grand_total
}
