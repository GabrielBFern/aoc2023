pub(crate) mod part1 {
    fn parse_digits(input: &str) -> Vec<u32> {
        input
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .filter_map(|n| n.to_digit(10))
            .collect()
    }

    pub fn resolve(input: &str) -> String {
        let digits = input.lines().map(parse_digits);
        let mut sum = 0u32;
        for numbers in digits {
            let first = numbers.first().unwrap();
            let last = numbers.last().unwrap_or(first);
            let line_total = first * 10 + last;
            sum += line_total;
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
                fs::read_to_string("input/day1/example1").expect("Need example file to test");
            let result = resolve(&input);
            assert_eq!(result, "142");
        }
    }
}

pub(crate) mod part2 {

    pub fn resolve(input: &str) -> String {
        let nums: Vec<(usize, &str)> = vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
        .into_iter()
        .enumerate()
        .map(|(n, word)| (n + 1, word))
        .collect();

        input
            .lines()
            .map(|line| {
                let mut numbers_at_line = vec![];
                line.chars().fold("".to_string(), |mut acc, c| {
                    acc.push(c);
                    if let Some(num) = c.to_digit(10) {
                        numbers_at_line.push(num as usize);
                    } else if let Some(num) = nums
                        .iter()
                        .find(|(_, word)| acc.ends_with(word))
                        .map(|(n, _)| n)
                    {
                        numbers_at_line.push(*num);
                    };
                    acc
                });
                let first = numbers_at_line
                    .first()
                    .expect("Wrong file format, cannot find any number on line");
                let last = numbers_at_line.last().unwrap_or(first);
                first * 10 + last
            })
            .sum::<usize>()
            .to_string()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs;

        #[test]
        fn test_example() {
            let input =
                fs::read_to_string("input/day1/example2").expect("Need example file to test");
            let result = resolve(&input);
            assert_eq!(result, "281");
        }
    }
}
