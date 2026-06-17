use std::vec;

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

pub fn sol2(input: &str) -> usize {
    let mut lines = input.lines();
    let operations: Vec<char> = lines.next_back().unwrap().chars().collect();
    let mat: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let mut op = operations.first().unwrap();
    let mut nums: Vec<usize> = vec![];
    let mut grand_total = 0;

    for col in 0..operations.len() {
        let joined: String = mat.iter().filter_map(|v| v.get(col)).collect();
        let joined = joined.trim();

        if !joined.is_empty() {
            nums.push(joined.trim().parse::<usize>().unwrap());
        }

        if operations.get(col + 1).is_none_or(|op| !op.is_whitespace()) {
            grand_total += match op {
                '*' => nums.iter().product(),
                '+' => nums.iter().sum(),
                _ => 0,
            };

            nums.clear();
        }

        if !operations[col].is_whitespace() {
            op = &operations[col];
        }
    }

    grand_total
}
