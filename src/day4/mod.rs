use std::{collections::HashSet, str::FromStr};

struct ScratchCard {
    _id: usize,
    winning_numbers: HashSet<usize>,
    ticket_numbers: HashSet<usize>,
}

#[derive(Debug)]
enum ScratchCardParseError {
    InvalidFormat,
    IdNotNumeric,
    InvalidWinningNumbers,
    InvalidTicketNumbers,
}

impl FromStr for ScratchCard {
    type Err = ScratchCardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, remain) = s
            .split_once(": ")
            .ok_or(ScratchCardParseError::InvalidFormat)?;
        let (_, id) = id
            .split_once(' ')
            .ok_or(ScratchCardParseError::InvalidFormat)?;
        let id = id
            .trim()
            .parse()
            .map_err(|_| ScratchCardParseError::IdNotNumeric)?;
        let (winning_numbers, ticket_numbers) = remain
            .split_once(" | ")
            .ok_or(ScratchCardParseError::InvalidFormat)?;

        let winning_numbers: HashSet<usize> = winning_numbers
            .split_ascii_whitespace()
            .map(|e| e.parse::<usize>())
            .collect::<Result<_, _>>()
            .map_err(|_| ScratchCardParseError::InvalidWinningNumbers)?;

        let ticket_numbers: HashSet<usize> = ticket_numbers
            .split_ascii_whitespace()
            .map(|e| e.parse::<usize>())
            .collect::<Result<_, _>>()
            .map_err(|_| ScratchCardParseError::InvalidTicketNumbers)?;

        Ok(ScratchCard {
            _id: id,
            winning_numbers,
            ticket_numbers,
        })
    }
}

pub(crate) mod part1 {
    use super::*;

    pub fn resolve(input: &str) -> String {
        input
            .lines()
            .map(|l| l.parse::<ScratchCard>().expect("invalid input"))
            .map(|s| {
                s.winning_numbers
                    .intersection(&s.ticket_numbers)
                    .fold(0usize, |acc, _| acc + 1)
            })
            .filter(|&len| len > 0)
            .map(|len| 2usize.pow((len - 1) as u32))
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
                fs::read_to_string("input/day4/example").expect("Need example file to test");
            let result = resolve(&input);
            assert_eq!(result, "13");
        }
    }
}

pub(crate) mod part2 {
    use super::*;

    pub fn resolve(input: &str) -> String {
        let lines: Vec<_> = input.lines().collect();
        let mut tickets = vec![1; lines.len()];

        input
            .lines()
            .map(|l| l.parse::<ScratchCard>().expect("invalid input"))
            .map(|s| {
                s.winning_numbers
                    .intersection(&s.ticket_numbers)
                    .fold(0usize, |acc, _| acc + 1)
            })
            .enumerate()
            .for_each(|(i, winning_times)| {
                if winning_times != 0 {
                    for next in i + 1..=i + winning_times {
                        tickets[next] += tickets[i];
                    }
                }
            });

        tickets.iter().sum::<usize>().to_string()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs;

        #[test]
        fn test_example() {
            let input =
                fs::read_to_string("input/day4/example").expect("Need example file to test");
            let result = resolve(&input);
            assert_eq!(result, "30");
        }
    }
}
