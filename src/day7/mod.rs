use std::{
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

struct CardHand([char; 5]);

impl FromStr for CardHand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hand: [char; 5] = s
            .chars()
            .collect::<Vec<char>>()
            .as_slice()
            .try_into()
            .map_err(|_| "Hand need the have 5 cards".to_string())?;

        Ok(CardHand(hand))
    }
}

pub(crate) mod part1 {
    use std::collections::BTreeMap;

    use super::*;

    static CARD_ORDER: [char; 13] = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];

    fn decide_hand_kind(hand: &CardHand) -> HandType {
        let card_count = hand.0.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0usize) += 1;
            acc
        });
        let mut card_count: BinaryHeap<_> = card_count.into_iter().map(|c| c.1).collect();

        use HandType as HT;
        match (card_count.pop(), card_count.pop()) {
            (Some(5), _) => HT::FiveKind,
            (Some(4), _) => HT::FourKind,
            (Some(3), Some(2)) => HT::FullHouse,
            (Some(3), _) => HT::ThreeKind,
            (Some(2), Some(2)) => HT::TwoPair,
            (Some(2), _) => HT::OnePair,
            (_, _) => HT::HighCard,
        }
    }

    fn convert_card_rank(hand: &CardHand) -> [usize; 5] {
        let card_score: Vec<_> = hand
            .0
            .iter()
            .map(|c| CARD_ORDER.iter().position(|l| l == c).unwrap())
            .collect();
        card_score.as_slice().try_into().unwrap()
    }

    pub fn resolve(input: &str) -> String {
        let cards: BTreeMap<_, _> = input
            .lines()
            .map(|l| {
                l.split_once(' ')
                    .map(|(card, bet)| {
                        let card = card.parse::<CardHand>().expect("Invalid Card");
                        (
                            (decide_hand_kind(&card), convert_card_rank(&card)),
                            bet.parse::<usize>().expect("Invalid bet"),
                        )
                    })
                    .expect("Invalid line")
            })
            .collect();
        cards
            .into_iter()
            .rev()
            .enumerate()
            .map(|(s, (_, b))| b * (s + 1))
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
                fs::read_to_string("input/day7/example").expect("Need example file to test");
            let result = resolve(&input);
            assert_eq!(result, "6440");
        }
    }
}

pub(crate) mod part2 {

    use std::collections::BTreeMap;

    use super::*;

    static CARD_ORDER: [char; 13] = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];

    fn decide_hand_kind(hand: &CardHand) -> HandType {
        let mut jokers = 0;
        let card_count = hand.0.iter().fold(HashMap::new(), |mut acc, c| {
            if *c == 'J' {
                jokers += 1;
            } else {
                *acc.entry(c).or_insert(0usize) += 1;
            }
            acc
        });

        let mut card_count: BinaryHeap<_> = card_count.into_iter().map(|c| c.1).collect();

        use HandType as HT;
        match (
            card_count.pop().map_or(jokers, |e| e + jokers),
            card_count.pop(),
        ) {
            (5, _) => HT::FiveKind,
            (4, _) => HT::FourKind,
            (3, Some(2)) => HT::FullHouse,
            (3, _) => HT::ThreeKind,
            (2, Some(2)) => HT::TwoPair,
            (2, _) => HT::OnePair,
            (_, _) => HT::HighCard,
        }
    }

    fn convert_card_rank(hand: &CardHand) -> [usize; 5] {
        let card_score: Vec<_> = hand
            .0
            .iter()
            .map(|c| CARD_ORDER.iter().position(|l| l == c).unwrap())
            .collect();
        card_score.as_slice().try_into().unwrap()
    }

    pub fn resolve(input: &str) -> String {
        let cards: BTreeMap<_, _> = input
            .lines()
            .map(|l| {
                l.split_once(' ')
                    .map(|(card, bet)| {
                        let card = card.parse::<CardHand>().expect("Invalid Card");
                        (
                            (decide_hand_kind(&card), convert_card_rank(&card)),
                            bet.parse::<usize>().expect("Invalid bet"),
                        )
                    })
                    .expect("Invalid line")
            })
            .collect();
        cards
            .into_iter()
            .rev()
            .enumerate()
            .map(|(s, (_, b))| b * (s + 1))
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
                fs::read_to_string("input/day7/example").expect("Need example file to test");
            let result = resolve(&input);
            assert_eq!(result, "5905");
        }
    }
}
