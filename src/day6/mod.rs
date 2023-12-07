#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

struct CorrectRace {
    time: usize,
    distance: usize,
}

fn parse_races(input: &str) -> Option<Vec<Race>> {
    let (time, distance) = input.split_once('\n')?;
    let times = time
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(str::parse::<usize>);
    let distance = distance
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(str::parse::<usize>);
    Some(
        times
            .zip(distance)
            .map(|(time, distance)| Race { time, distance })
            .collect(),
    )
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn parse_correct_race(input: &str) -> Option<CorrectRace> {
    let (time, distance) = input.split_once('\n')?;
    let (_header, time) = time.split_once(' ')?;
    let (_header, distance) = distance.split_once(' ')?;
    let time: usize = remove_whitespace(time).parse().ok()?;
    let distance: usize = remove_whitespace(distance).parse().ok()?;
    Some(CorrectRace { time, distance })
}

pub(crate) mod part1 {
    use super::*;

    pub fn resolve(input: &str) -> String {
        let races = parse_races(input).expect("invalid input");

        races
            .into_iter()
            .map(|race| {
                let winning = |charge: &usize| (charge * (race.time - charge)) > race.distance;
                let first_time = (0..race.time).find(winning).unwrap();
                race.time - first_time * 2 + 1
            })
            .product::<usize>()
            .to_string()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs;

        #[test]
        fn test_example() {
            let input =
                fs::read_to_string("input/day6/example").expect("Need example file to test");
            let result = resolve(&input);
            assert_eq!(result, "288");
        }
    }
}

pub(crate) mod part2 {

    use super::*;

    pub fn resolve(input: &str) -> String {
        let race = parse_correct_race(input).expect("Expected valid input");
        let winning = |charge: &usize| (charge * (race.time - charge)) > race.distance;
        let first_time = (0..race.time)
            .find(winning)
            .expect("Expected a winnable race");
        let possibilities = race.time - first_time * 2 + 1;

        possibilities.to_string()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs;

        #[test]
        fn test_example() {
            let input =
                fs::read_to_string("input/day6/example").expect("Need example file to test");
            let result = resolve(&input);
            assert_eq!(result, "71503");
        }
    }
}
