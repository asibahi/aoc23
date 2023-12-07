#![allow(dead_code)]

use std::collections::BTreeMap;

use counter::Counter;
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day7.txt");

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
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
struct Hand {
    hand_type: HandType,
    literal: (Rank, Rank, Rank, Rank, Rank),
}

impl Hand {
    fn parse(input: &str) -> Self {
        // for example: 32T3K

        let counts = input.bytes().map(|b| Rank::from(b)).collect::<Counter<_>>();
        // .fold(BTreeMap::new(), |mut map, card| {
        //     *map.entry(card).or_default() += 1;
        //     map
        // });

        let counts = counts.values().sorted().rev().collect_vec();

        let hand_type = if *counts[0] == 5 {
            HandType::FiveOfAKind
        } else if *counts[0] == 4 {
            HandType::FourOfAKind
        } else if *counts[0] == 3 && *counts[1] == 2 {
            HandType::FullHouse
        } else if *counts[0] == 3 {
            HandType::ThreeOfAKind
        } else if *counts[0] == 2 && *counts[1] == 2 {
            HandType::TwoPair
        } else if *counts[0] == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        };

        let literal = input
            .bytes()
            // .inspect(|b| {
            //     dbg!(*b as char);
            // })
            .map(|b| Rank::from(b))
            .collect_tuple()
            .unwrap();

        Self { hand_type, literal }
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
        let hand = Hand::parse(hand);

        map.insert(hand, bid);
    }

    map.values().enumerate().map(|(i, bid)| (i + 1) * bid).sum()
}

fn solve_2(input: &str) -> usize {
    input.lines().map(|i| i.len()).sum()
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

    /* #[test]
    fn bench() {
        use microbench::{self, Options};

        // use this terminal command
        // cargo test --package aoc23 --bin day7 --release  -- tests::bench --exact --nocapture

        let options = Options::default();
        microbench::bench(&options, "original part 1", || solve_1(INPUT));
        microbench::bench(&options, "original part 2", || solve_2(INPUT));
    } */
}
