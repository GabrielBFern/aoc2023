fn extract_digits_around_position(data: &[u8], column: usize) -> (usize, usize) {
    let mut digits_before: Vec<u8> = data
        .iter()
        .take(column)
        .rev()
        .take_while(|&c| c.is_ascii_digit())
        .cloned()
        .collect();

    let digits_after: Vec<u8> = data
        .iter()
        .skip(column + 1)
        .take_while(|&c| c.is_ascii_digit())
        .cloned()
        .collect();

    let start = column - digits_before.len();

    // Combine the two sets of digits
    digits_before.reverse();
    let mut result = digits_before;
    result.push(data[column]);
    result.extend(digits_after);

    (
        start,
        std::str::from_utf8(&result)
            .expect("We only take the digits, so this is fine")
            .parse()
            .unwrap(),
    )
}

fn find_numbers(data: &[Vec<u8>], position: (usize, usize)) -> Vec<usize> {
    let lookup_matrix = [
        (1, 1),
        (1, 0),
        (1, -1),
        (0, 1),
        (0, -1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];

    let mut v: Vec<_> = lookup_matrix
        .into_iter()
        .filter_map(|(l, c)| {
            position
                .0
                .checked_add_signed(l)
                .zip(position.1.checked_add_signed(c))
        })
        .filter(|(l, c)| {
            data.get(*l)
                .and_then(|line| line.get(*c))
                .map(|c| c.is_ascii_digit())
                .unwrap_or_default()
        })
        .map(|(l, c)| extract_digits_around_position(data.get(l).unwrap(), c))
        .collect();

    v.dedup();

    v.into_iter().map(|(_, value)| value).collect()
}

pub(crate) mod part1 {
    use super::*;

    pub fn resolve(input: &str) -> String {
        let data: Vec<Vec<u8>> = input.lines().map(|c| c.as_bytes().to_vec()).collect();

        let mut sum = 0;
        for (line, content) in data.iter().enumerate() {
            for (column, c) in content.iter().enumerate() {
                if !(c.is_ascii_digit() || *c == b'.') {
                    sum += find_numbers(&data, (line, column)).iter().sum::<usize>();
                }
            }
        }

        sum.to_string()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs;

        #[test]
        fn test_example() {
            let input =
                fs::read_to_string("input/day3/example").expect("Need example file to test");
            let result = resolve(&input);
            assert_eq!(result, "4361");
        }
    }
}

pub(crate) mod part2 {
    use super::*;

    pub fn resolve(input: &str) -> String {
        let data: Vec<Vec<u8>> = input.lines().map(|c| c.as_bytes().to_vec()).collect();

        let mut sum = 0;
        for (line, content) in data.iter().enumerate() {
            for (column, c) in content.iter().enumerate() {
                if *c == b'*' {
                    let numbers = find_numbers(&data, (line, column));
                    if numbers.len() == 2 {
                        sum += numbers.iter().product::<usize>();
                    }
                }
            }
        }

        sum.to_string()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs;

        #[test]
        fn test_example() {
            let input =
                fs::read_to_string("input/day3/example").expect("Need example file to test");
            let result = resolve(&input);
            assert_eq!(result, "467835");
        }
    }
}
