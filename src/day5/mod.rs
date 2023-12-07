use std::{fmt::Debug, ops::Range, str::FromStr};

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    category_conversion: Vec<Vec<(Range<usize>, Range<usize>)>>,
}

#[derive(Debug)]
struct IntersectRange<T> {
    parallel: T,
    remainder_self: Vec<T>,
}

trait Intersect<A = Self>: Sized {
    fn intersect(&self, other: &Self) -> Option<IntersectRange<Self>>;
}

impl<T: Ord + Copy + Debug> Intersect<T> for Range<T> {
    fn intersect(&self, other: &Self) -> Option<IntersectRange<Self>> {
        // If not intersect
        if !(self.start <= other.end && other.start <= self.end) {
            return None;
        }
        // Find the intersection
        let start = std::cmp::max(other.start, self.start);
        let end = std::cmp::min(other.end, self.end);
        let parallel = start..end;

        // Collect the remainder of the self range
        let mut remainder_self = vec![];
        if self.start < parallel.start {
            remainder_self.push(self.start..parallel.start);
        }
        if self.end > parallel.end {
            remainder_self.push(parallel.end..self.end);
        }

        Some(IntersectRange {
            parallel,
            remainder_self,
        })
    }
}

impl FromStr for Almanac {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (seeds, remain) = s.split_once('\n').ok_or("Invalid Format")?;
        let (_prefix, seeds) = seeds.split_once(": ").ok_or("Invalid seed")?;

        let make_range_conversion = |s: &str| {
            let numbers: Vec<usize> = s
                .split_whitespace()
                .map(|s| s.parse::<usize>().expect("Invalid number in category"))
                .collect();
            let &[dest_start, source_start, length] = &numbers[..3] else {
                unreachable!()
            };
            (
                (source_start..source_start + length),
                dest_start..dest_start + length,
            )
        };

        let seeds = seeds
            .split_ascii_whitespace()
            .map(|e| e.parse::<usize>())
            .collect::<Result<_, _>>()
            .map_err(|_| "Invalid seed")?;
        let category_conversion = remain
            .lines()
            .fold(vec![], |mut acc, l| {
                if l.is_empty() {
                    acc.push(vec![]);
                } else {
                    acc.last_mut().unwrap().push(l);
                }
                acc
            })
            .iter()
            .map(|c| {
                c.iter()
                    .skip(1)
                    .map(|&c| make_range_conversion(c))
                    .collect()
            })
            .collect();

        Ok(Almanac {
            seeds,
            category_conversion,
        })
    }
}

pub(crate) mod part1 {
    use super::*;

    pub fn resolve(input: &str) -> String {
        let almanac: Almanac = input.parse().expect("Invalid input");

        let mut last_category = Vec::with_capacity(almanac.seeds.len());
        for &seed in &almanac.seeds {
            let mut previous_category = seed;
            for category in &almanac.category_conversion {
                for conv in category {
                    if conv.0.contains(&previous_category) {
                        previous_category = conv.1.start + (previous_category - conv.0.start);
                        break;
                    }
                }
            }

            last_category.push(previous_category);
        }
        last_category.iter().min().unwrap().to_string()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs;

        #[test]
        fn test_example() {
            let input =
                fs::read_to_string("input/day5/example").expect("Need example file to test");
            let result = resolve(&input);
            assert_eq!(result, "35");
        }
    }
}

pub(crate) mod part2 {

    use super::*;

    // This function presume that origin is contained inside the conversion range
    fn translate_category(
        origin: &Range<usize>,
        conversion_range: &(Range<usize>, Range<usize>),
    ) -> Range<usize> {
        let offset = origin.start - conversion_range.0.start;
        let start = conversion_range.1.start + offset;
        let end = conversion_range.1.start + offset + origin.len();
        start..end
    }

    pub fn resolve(input: &str) -> String {
        let almanac: Almanac = input.parse().expect("Invalid input");
        let true_seeds: Vec<_> = almanac
            .seeds
            .chunks(2)
            .map(|e| (e[0]..e[0] + e[1]))
            .collect();
        let mut last_category = vec![];
        let mut previous_category = true_seeds;
        for category in almanac.category_conversion {
            let mut next_category = vec![];
            for conversion in category {
                let mut remainder = vec![];

                for prev_cat in previous_category {
                    match prev_cat.intersect(&conversion.0) {
                        Some(IntersectRange {
                            parallel,
                            remainder_self,
                        }) => {
                            next_category.push(translate_category(&parallel, &conversion));
                            remainder.extend(remainder_self);
                        }
                        None => {
                            remainder.push(prev_cat);
                        }
                    }
                }
                previous_category = remainder;
            }
            next_category.extend(previous_category.clone());
            previous_category = next_category;
            last_category = previous_category.clone();
        }
        last_category
            .into_iter()
            .map(|r| r.start)
            .min()
            .unwrap()
            .to_string()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::fs;

        #[test]
        fn test_example() {
            let input =
                fs::read_to_string("input/day5/example").expect("Need example file to test");
            let result = resolve(&input);
            assert_eq!(result, "46");
        }
    }
}
