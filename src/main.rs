use std::fmt::Display;

mod day1;
mod day2;
mod day3;
mod day4;

macro_rules! day {
    ($name:expr, $day:ident) => {
        (
            $name,
            |input| Box::new($day::part1::resolve(input)),
            |input| Box::new($day::part2::resolve(input)),
        )
    };
}

fn main() {
    type SolverFn = fn(&str) -> Box<dyn Display>;

    use std::fs;

    let days: Vec<(&str, SolverFn, SolverFn)> = vec![
        day!("Trebuchet?!", day1),
        day!("Cube Conundrum", day2),
        day!("Gear Ratios", day3),
        day!("Scratchcards", day4),
    ];

    for (num, (name, part1, part2)) in days.into_iter().enumerate() {
        let num = num + 1;
        println!("Day {num} - {name}");
        let input =
            fs::read_to_string(format!("input/day{num}/input")).expect("Need file to run the day");

        let run_solution = |solution: SolverFn| {
            use std::time::Instant;

            let start = Instant::now();
            let result = solution(&input);
            let duration = start.elapsed().as_micros();
            println!("Timing: {duration}us");
            println!("Result:\n {result}");
            println!();
        };

        println!("Part one");
        run_solution(part1);
        println!("Part two");
        run_solution(part2);
    }
}
