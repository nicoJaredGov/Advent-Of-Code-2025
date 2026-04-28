fn split_and_parse(id: &str) -> (usize, usize) {
    let x = id.split_at(id.len() / 2);
    (x.0.parse().unwrap(), x.1.parse().unwrap())
}

fn get_max_for_length(len: usize) -> usize {
    10_usize.pow(len as u32 / 2) - 1
}

fn get_doubles_sum(i: usize, j: usize, range_len: usize) -> usize {
    let arithmetic_sum = ((i + j) * (j - i + 1)) / 2;
    let multiple = 10_usize.pow((range_len as u32) / 2) + 1;
    arithmetic_sum * multiple
}

fn process_range(id_range: (&str, &str)) -> usize {
    let (first, last) = id_range;

    // halves of each id
    let (c, d) = split_and_parse(last);

    let mut length_range = (first.len()..=last.len()).peekable();
    if length_range.peek().is_none() {
        return 0;
    }

    let compute_right_bound = |has_next_value: bool, len| -> usize {
        if !has_next_value {
            if c > d { c - 1 } else { c }
        } else {
            get_max_for_length(len)
        }
    };

    let mut total = 0;

    // handle first length
    if let Some(len) = length_range.next()
        && len % 2 == 0
    {
        let (a, b) = split_and_parse(first);

        let i = if a < b { a + 1 } else { a };
        let j = compute_right_bound(length_range.peek().is_some(), len);

        if i <= j {
            total += get_doubles_sum(i, j, len);
        }
        length_range.next(); // skip over next odd length
    }

    while length_range.peek().is_some() {
        let len = length_range.next().unwrap();

        let i = 10_usize.pow(len as u32 / 2 - 1);
        let j = compute_right_bound(length_range.peek().is_some(), len);

        if i <= j {
            total += get_doubles_sum(i, j, len);
        }
        length_range.next(); // skip over next odd length
    }

    total
}

pub fn sol(input: &str) -> usize {
    let ranges: Vec<_> = input
        .split(',')
        .map(|r| r.split_once('-').expect("Input string not in valid format"))
        .collect();

    ranges
        .iter()
        .fold(0, |total, range| total + process_range(*range))
}
