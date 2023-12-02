#[derive(Debug, PartialEq)]
enum Colour {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq)]
struct Game {
    id: usize,
    draws: Vec<DrawInfo>,
}

#[derive(Debug, PartialEq)]
struct DrawInfo {
    sequence: Vec<Draw>,
}

#[derive(Debug, PartialEq)]
struct Draw {
    qty: usize,
    colour: Colour,
}

#[derive(Debug)]
enum ParseError {
    InvalidFormat,
    InvalidKeyFormat,
    GameIdIsNotNumeric,
    InvalidDraw(DrawInfoParseError),
}

#[derive(Debug)]
enum DrawInfoParseError {
    InvalidDraw(DrawParseError),
}

#[derive(Debug)]
enum DrawParseError {
    InvalidFormat,
    QuantityIsNotNumeric,
    InvalidColour,
}

struct InvalidColour;

impl TryFrom<&str> for Game {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (game, sequence) = value.split_once(": ").ok_or(ParseError::InvalidFormat)?;
        let (_, id) = game.split_once(' ').ok_or(ParseError::InvalidKeyFormat)?;
        let id = id.parse().map_err(|_| ParseError::GameIdIsNotNumeric)?;
        let sequence: Result<_, _> = sequence
            .split_terminator("; ")
            .map(|draw| draw.try_into())
            .collect();
        let sequence = sequence.map_err(ParseError::InvalidDraw)?;
        Ok(Game {
            id,
            draws: sequence,
        })
    }
}

impl TryFrom<&str> for DrawInfo {
    type Error = DrawInfoParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let sequence: Result<_, _> = value
            .split_terminator(", ")
            .map(|draw| draw.try_into())
            .collect();
        let sequence = sequence.map_err(DrawInfoParseError::InvalidDraw)?;
        Ok(DrawInfo { sequence })
    }
}

impl TryFrom<&str> for Draw {
    type Error = DrawParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (qty, colour) = value.split_once(' ').ok_or(DrawParseError::InvalidFormat)?;
        let qty = qty
            .parse()
            .map_err(|_| DrawParseError::QuantityIsNotNumeric)?;
        let colour = colour
            .try_into()
            .map_err(|_| DrawParseError::InvalidColour)?;
        Ok(Draw { qty, colour })
    }
}

impl TryFrom<&str> for Colour {
    type Error = InvalidColour;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Colour::*;
        match value {
            "red" => Ok(Red),
            "blue" => Ok(Blue),
            "green" => Ok(Green),
            _ => Err(InvalidColour),
        }
    }
}

struct FilterRule {
    max: usize,
}

struct Filter {
    colours_rules: Vec<(Colour, FilterRule)>,
}

impl Filter {
    fn valid_game(&self, game: &Game) -> bool {
        for draw_seq in game.draws.iter() {
            for draw in draw_seq.sequence.iter() {
                if let Some((_, rule)) = self
                    .colours_rules
                    .iter()
                    .find(|(colour, _)| *colour == draw.colour)
                {
                    if draw.qty > rule.max {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        self.colours_rules.iter().any(|(c, _)| {
            game.draws
                .iter()
                .any(|di| di.sequence.iter().any(|d| d.colour == *c))
        })
    }
}

impl Game {
    fn lowest_possible_set(self) -> [usize; 3] {
        self.draws
            .into_iter()
            .flat_map(|d| d.sequence)
            .fold([0usize; 3], |mut acc, d| {
                let i = d.colour as usize;
                acc[i] = acc[i].max(d.qty);
                acc
            })
    }
}

pub(crate) mod part1 {
    use super::*;

    pub fn resolve(input: &str) -> String {
        let games: Vec<Game> = input
            .lines()
            .map(|l| l.try_into())
            .collect::<Result<_, _>>()
            .expect("Invalid input");
        let filter = Filter {
            colours_rules: vec![
                (Colour::Red, FilterRule { max: 12 }),
                (Colour::Green, FilterRule { max: 13 }),
                (Colour::Blue, FilterRule { max: 14 }),
            ],
        };

        games
            .iter()
            .filter(|g| filter.valid_game(g))
            .map(|g| g.id)
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
                fs::read_to_string("input/day2/example").expect("Need example file to test");
            let result = resolve(&input);
            assert_eq!(result, "8");
        }
    }
}

pub(crate) mod part2 {
    use super::*;

    pub fn resolve(input: &str) -> String {
        let games: Vec<Game> = input
            .lines()
            .map(|l| l.try_into())
            .collect::<Result<_, _>>()
            .expect("Invalid input");
        games
            .into_iter()
            .map(Game::lowest_possible_set)
            .map(|d| d.iter().product::<usize>())
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
                fs::read_to_string("input/day2/example").expect("Need example file to test");
            let result = resolve(&input);
            assert_eq!(result, "2286");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_draw() {
        let draw: Draw = "1 red".try_into().unwrap();
        assert_eq!(
            draw,
            Draw {
                qty: 1,
                colour: Colour::Red
            }
        );
    }

    #[test]
    fn test_parse_drawinfo() {
        let drawinfo: DrawInfo = "1 red, 2 blue".try_into().unwrap();
        assert_eq!(
            drawinfo,
            DrawInfo {
                sequence: vec![
                    Draw {
                        qty: 1,
                        colour: Colour::Red
                    },
                    Draw {
                        qty: 2,
                        colour: Colour::Blue
                    }
                ]
            }
        );
    }

    #[test]
    fn test_parse_game() {
        let game: Game = "Game 111: 1 red, 2 blue".try_into().unwrap();
        assert_eq!(
            game,
            Game {
                id: 111,
                draws: vec!(DrawInfo {
                    sequence: vec!(
                        Draw {
                            qty: 1,
                            colour: Colour::Red
                        },
                        Draw {
                            qty: 2,
                            colour: Colour::Blue
                        }
                    )
                })
            }
        );
    }
}
