use std::collections::HashMap;

use bstr::{ByteSlice, B};
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day4.txt");

fn main() {
    let res = solve_1(INPUT);
    println!("Part 1:\t{res}");
    let res = solve_1_try_2(INPUT);
    println!("Part 1, try 2:\t{res}");

    let res = solve_2(INPUT);
    println!("Part 2:\t{res}");
    let res = solve_2_try_2(INPUT);
    println!("Part 2, try 2:\t{res}");
    let res = solve_2_try_3(INPUT);
    println!("Part 2, try 3:\t{res}");
}

fn solve_1(input: &str) -> u32 {
    input.lines().map(parse_line_1).sum()
}

fn parse_line_1(input: &str) -> u32 {
    input
        .split_once(':')
        .and_then(|(_, info)| {
            info.split_once('|').and_then(|(winners, havers)| {
                let answers = winners
                    .split_ascii_whitespace()
                    .cartesian_product(havers.split_ascii_whitespace())
                    .filter(|(x, y)| x.eq(y))
                    .count() as u32;

                (answers != 0).then(|| 1 << (answers - 1))
            })
        })
        .unwrap_or_default()
}

#[allow(dead_code)]
fn solve_1_try_2(input: &str) -> usize {
    input.lines().map(parse_line_1_try_2).sum()
}

fn parse_line_1_try_2(input: &str) -> usize {
    input
        .split_once(':')
        .and_then(|(_, info)| {
            info.split_once('|').and_then(|(winners, havers)| {
                let answers = winners
                    .as_bytes()
                    .chunks_exact(3)
                    .filter(|chunk| B(havers).contains_str(B(chunk)))
                    .count();

                (answers != 0).then(|| 1 << (answers - 1))
            })
        })
        .unwrap_or_default()
}

/*
 * Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
 * Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
 * Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
 * Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
 * Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
 * Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
 */

fn solve_2(input: &str) -> usize {
    let numbered_lines = input
        .lines()
        .map(|line| line.split_once(':').unwrap().1)
        .enumerate();

    // key is card number. value is (whether the card exists, how many copies given by previous cards should the card exist.)
    let mut cards_set: HashMap<usize, (bool, usize)> = HashMap::new();

    for (idx, line) in numbered_lines {
        let (winners, havers) = line.split_once('|').unwrap();

        let matches = winners
            .split_ascii_whitespace()
            .cartesian_product(havers.split_ascii_whitespace())
            .filter(|(x, y)| x.eq(y))
            .count();

        cards_set
            .entry(idx)
            .and_modify(|(seen, copies)| {
                *seen |= true;
                *copies += 1;
            })
            .or_insert((true, 1));

        let copies_of_this_card = cards_set.get(&idx).unwrap().1;

        for i in 0..matches {
            cards_set
                .entry(idx + i + 1)
                .and_modify(|(_, copies)| {
                    *copies += 1 * copies_of_this_card;
                })
                .or_insert((false, 1 * copies_of_this_card));
        }
    }

    cards_set
        .values()
        .filter(|(seen, _)| *seen)
        .map(|(_, count)| count)
        .sum()
}

#[allow(dead_code)]
fn solve_2_try_2(input: &str) -> usize {
    let numbered_lines = input
        .lines()
        .map(|line| line.split_once(':').unwrap().1)
        .enumerate();

    let mut cards_set = HashMap::new();
    let mut running_sum = 0;

    for (idx, line) in numbered_lines {
        let (winners, havers) = line.split_once('|').unwrap();

        cards_set
            .entry(idx)
            .and_modify(|copies| *copies += 1)
            .or_insert(1);

        let copies_of_this_card = *cards_set.get(&idx).unwrap();
        running_sum += copies_of_this_card;

        let matches = B(winners)
            .chunks_exact(3)
            .filter(|chunk| B(havers).contains_str(B(chunk)))
            .enumerate();

        for (i, _) in matches {
            cards_set
                .entry(idx + i + 1)
                .and_modify(|copies| *copies += copies_of_this_card)
                .or_insert(copies_of_this_card);
        }
    }

    running_sum
}

#[allow(dead_code)]
fn solve_2_try_3(input: &str) -> usize {
    let numbered_lines = input
        .lines()
        // hard coded for available input
        .map(|line| line.split_at(9).1)
        .enumerate();

    let mut cards_set = vec![0; 500];
    let mut running_sum = 0;

    for (idx, line) in numbered_lines {
        // hard coded for available input
        let (winners, havers) = line.split_at(32);

        cards_set[idx] += 1;

        let copies_of_this_card = cards_set[idx];
        running_sum += copies_of_this_card;

        let matches = B(winners)
            .chunks_exact(3)
            .filter(|chunk| B(havers).contains_str(B(chunk)))
            .enumerate();

        for (i, _) in matches {
            cards_set[idx + i + 1] += copies_of_this_card;
        }
    }

    running_sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53" => 8)]
    #[test_case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19" => 2)]
    #[test_case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1" => 2)]
    #[test_case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83" => 1)]
    #[test_case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36" => 0)]
    #[test_case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11" => 0)]
    fn test_part_1(i: &str) -> usize {
        parse_line_1_try_2(i)
    }

    const EXAMPLE: &str = indoc::indoc! {"
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"};

    #[test_case(EXAMPLE => 30)]
    fn test_part_2(i: &str) -> usize {
        solve_2(i)
    }

    #[test]
    fn bench() {
        use microbench::{self, Options};

        // use this terminal command
        // cargo test --package aoc23 --bin day4 --release -- tests::bench --exact --nocapture

        let options = Options::default();
        microbench::bench(&options, "original part 1", || solve_1(INPUT));
        microbench::bench(&options, "try 2    part 1", || solve_1_try_2(INPUT));
        microbench::bench(&options, "original part 2", || solve_2(INPUT));
        microbench::bench(&options, "try 2    part 2", || solve_2_try_2(INPUT));
        microbench::bench(&options, "try 3    part 2", || solve_2_try_3(INPUT));
    }
}
