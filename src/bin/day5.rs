#![allow(dead_code)]

use std::time::Instant;

use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day5.txt");

// (initial value for source, difference for dist, range)
struct Op(i64, i64, i64);
struct Map(Vec<Op>);

impl Op {
    fn apply_to(&self, input: i64) -> Option<i64> {
        (self.0..self.0 + self.2)
            .contains(&input)
            .then_some(input + self.1)
    }
}

impl Map {
    fn apply_to(&self, input: i64) -> i64 {
        self.0
            .iter()
            .find_map(|o| o.apply_to(input))
            .unwrap_or(input)
    }
}

fn solve_1(input: &str) -> i64 {
    let mut sections = input.split("\n\n");
    let seeds = sections
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap());

    let maps: Vec<Map> = sections
        .map(|section| {
            let map = section
                .lines()
                .skip(1)
                .filter(|l| !l.is_empty())
                .map(|line| {
                    let (dist, source, range) = line
                        .split_ascii_whitespace()
                        .map(|s| s.parse::<i64>().unwrap())
                        .collect_tuple()
                        .unwrap_or_default();

                    Op(source, dist - source, range)
                })
                .collect();

            Map(map)
        })
        .collect();

    seeds
        .map(|mut seed| {
            for map in maps.iter() {
                seed = map.apply_to(seed);
            }
            seed
        })
        .min()
        .unwrap()
}

fn solve_2(input: &str) -> i64 {
    let mut sections = input.split("\n\n");
    let seeds = sections
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .tuples()
        .flat_map(|(x, y)| (x..x + y));

    let maps: Vec<Map> = sections
        .map(|section| {
            let map = section
                .lines()
                .skip(1)
                .filter(|l| !l.is_empty())
                .map(|line| {
                    let (dist, source, range) = line
                        .split_ascii_whitespace()
                        .map(|s| s.parse::<i64>().unwrap())
                        .collect_tuple()
                        .unwrap_or_default();

                    Op(source, dist - source, range)
                })
                .collect();

            Map(map)
        })
        .collect();

    seeds
        .map(|mut seed| {
            for map in maps.iter() {
                seed = map.apply_to(seed);
            }
            seed
        })
        .min()
        .unwrap()
}

fn main() {
    let res = solve_1(INPUT);
    println!("Part 1:\t{res}");

    let now = Instant::now();

    let res = solve_2(INPUT);
    println!("Part 2:\t{res}");

    let p = now.elapsed().as_secs();
    println!("{p}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE: &str = include_str!("../../tests/day5.txt");

    #[test_case(EXAMPLE => 35)]
    fn test_part_1(i: &str) -> i64 {
        solve_1(i)
    }

    #[test_case(EXAMPLE => 46)]
    fn test_part_2(i: &str) -> i64 {
        solve_2(i)
    }

    // #[test]
    // fn bench() {
    //     use microbench::{self, Options};

    //     // use this terminal command
    //     // cargo test --package aoc23 --bin day2 --release  -- tests::bench --exact --nocapture

    //     let options = Options::default();
    //     microbench::bench(&options, "original part 1", || solve_1(INPUT));
    //     microbench::bench(&options, "original part 2", || solve_2(INPUT));
    // }
}
