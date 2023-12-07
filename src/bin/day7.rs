#![allow(dead_code)]

use std::collections::BTreeMap;

const INPUT: &str = include_str!("../../input/day7.txt");

mod part_1 {
    use counter::Counter;
    use itertools::Itertools;

    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    enum Rank {
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Jack,
        Queen,
        King,
        Ace,
    }

    impl From<u8> for Rank {
        fn from(value: u8) -> Self {
            match value {
                b'A' => Self::Ace,
                b'K' => Self::King,
                b'Q' => Self::Queen,
                b'J' => Self::Jack,
                b'T' => Self::Ten,
                b'9' => Self::Nine,
                b'8' => Self::Eight,
                b'7' => Self::Seven,
                b'6' => Self::Six,
                b'5' => Self::Five,
                b'4' => Self::Four,
                b'3' => Self::Three,
                _ => Self::Two,
            }
        }
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    enum HandType {
        HighCard,
        OnePair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    pub(crate) struct Hand {
        hand_type: HandType,
        literal: (Rank, Rank, Rank, Rank, Rank),
    }

    impl Hand {
        pub fn parse(input: &str) -> Self {
            // for example: 32T3K

            let counts = input.bytes().collect::<Counter<_>>();
            let counts = counts
                .most_common()
                .iter()
                .map(|i| i.1)
                .pad_using(2, |_| 0)
                .next_tuple()
                .unwrap();

            let hand_type = match counts {
                (5, _) => HandType::FiveOfAKind,
                (4, _) => HandType::FourOfAKind,
                (3, 2) => HandType::FullHouse,
                (3, _) => HandType::ThreeOfAKind,
                (2, 2) => HandType::TwoPair,
                (2, _) => HandType::OnePair,
                _ => HandType::HighCard,
            };

            let literal = input.bytes().map(Rank::from).collect_tuple().unwrap();

            Self { hand_type, literal }
        }
    }
}

mod part_2 {
    use counter::Counter;
    use itertools::Itertools;

    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    enum Rank {
        Joker,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Queen,
        King,
        Ace,
    }

    impl From<u8> for Rank {
        fn from(value: u8) -> Self {
            match value {
                b'A' => Self::Ace,
                b'K' => Self::King,
                b'Q' => Self::Queen,
                b'J' => Self::Joker,
                b'T' => Self::Ten,
                b'9' => Self::Nine,
                b'8' => Self::Eight,
                b'7' => Self::Seven,
                b'6' => Self::Six,
                b'5' => Self::Five,
                b'4' => Self::Four,
                b'3' => Self::Three,
                _ => Self::Two,
            }
        }
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    enum HandType {
        HighCard,
        OnePair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    pub(crate) struct Hand {
        hand_type: HandType,
        literal: (Rank, Rank, Rank, Rank, Rank),
    }

    impl Hand {
        pub fn parse(input: &str) -> Self {
            // for example: 32T3K

            let mut counts = input.bytes().collect::<Counter<_>>();
            let jokers = counts.remove(&b'J').unwrap_or_default();
            let counts = counts
                .most_common()
                .iter()
                .map(|i| i.1)
                .pad_using(2, |_| 0)
                .next_tuple()
                .unwrap();

            let hand_type = match counts {
                (x, _) if x + jokers == 5 => HandType::FiveOfAKind,
                (x, _) if x + jokers == 4 => HandType::FourOfAKind,
                (x, 2) if x + jokers == 3 => HandType::FullHouse,
                (x, _) if x + jokers == 3 => HandType::ThreeOfAKind,
                (2, x) if x + jokers == 2 => HandType::TwoPair,
                (x, _) if x + jokers == 2 => HandType::OnePair,
                _ => HandType::HighCard,
            };

            let literal = input.bytes().map(Rank::from).collect_tuple().unwrap();

            Self { hand_type, literal }
        }
    }
}

fn main() {
    let res = solve_1(INPUT);
    println!("Part 1:\t{res}");

    let res = solve_2(INPUT);
    println!("Part 2:\t{res}");
}

fn solve_1(input: &str) -> usize {
    let mut map = BTreeMap::new();

    for line in input.lines() {
        let (hand, bid) = line.split_at(5);
        let bid = bid.trim().parse::<usize>().unwrap();
        let hand = part_1::Hand::parse(hand);

        map.insert(hand, bid);
    }

    map.values().enumerate().map(|(i, bid)| (i + 1) * bid).sum()
}

fn solve_2(input: &str) -> usize {
    let mut map = BTreeMap::new();

    for line in input.lines() {
        let (hand, bid) = line.split_at(5);
        let bid = bid.trim().parse::<usize>().unwrap();
        let hand = part_2::Hand::parse(hand);

        map.insert(hand, bid);
    }

    map.values().enumerate().map(|(i, bid)| (i + 1) * bid).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE: &str = include_str!("../../input/day7example.txt");

    #[test_case(EXAMPLE => 6440)]
    fn test_part_1(i: &str) -> usize {
        solve_1(i)
    }

    #[test_case(EXAMPLE => 5905)]
    fn test_part_2(i: &str) -> usize {
        solve_2(i)
    }

    #[test]
    fn bench() {
        use microbench::{self, Options};

        // use this terminal command
        // cargo test --package aoc23 --bin day7 --release  -- tests::bench --exact --nocapture

        let options = Options::default();
        microbench::bench(&options, "original part 1", || solve_1(INPUT));
        microbench::bench(&options, "original part 2", || solve_2(INPUT));
    }
}
