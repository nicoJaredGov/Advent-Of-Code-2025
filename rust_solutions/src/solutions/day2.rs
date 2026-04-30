fn split_and_parse(id: &str) -> (usize, usize) {
    let x = id.split_at(id.len() / 2);
    (x.0.parse().unwrap(), x.1.parse().unwrap())
}

fn get_doubles_sum(i: usize, j: usize, range_len: usize) -> usize {
    let arithmetic_sum = ((i + j) * (j - i + 1)) / 2;
    let multiple = 10_usize.pow((range_len as u32) / 2) + 1;
    arithmetic_sum * multiple
}

fn process_range(id_range: (&str, &str)) -> usize {
    let (first, last) = id_range;

    let mut length_range = (first.len()..=last.len()).peekable();
    if length_range.peek().is_none() {
        return 0;
    }

    let mut first_iter = true;
    if length_range.peek().unwrap() % 2 != 0 {
        first_iter = false;
        length_range.next(); // skip over odd length
    }

    let mut compute_left_bound = |len| -> usize {
        if first_iter {
            first_iter = false;
            let (a, b) = split_and_parse(first);
            if a < b { a + 1 } else { a }
        } else {
            10_usize.pow(len as u32 / 2 - 1)
        }
    };

    let compute_right_bound = |last_iter: bool, len| -> usize {
        if last_iter {
            let (c, d) = split_and_parse(last);
            if c > d { c - 1 } else { c }
        } else {
            10_usize.pow(len as u32 / 2) - 1
        }
    };

    let mut total = 0;
    while let Some(len) = length_range.next() {
        let i = compute_left_bound(len);
        let j = compute_right_bound(length_range.peek().is_none(), len);

        if i <= j {
            total += get_doubles_sum(i, j, len);
        }
        length_range.next();
    }

    total
}

pub fn sol(input: &str) -> usize {
    input
        .split(',')
        .map(|r| r.split_once('-').expect("Input string not in valid format"))
        .fold(0, |total, range| total + process_range(range))
}
