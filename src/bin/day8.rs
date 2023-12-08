// Template

use std::collections::HashMap;

const INPUT: &str = include_str!("../../input/day8.txt");

fn main() {
    let res = solve_1(INPUT);
    println!("Part 1:\t{res}");

    let res = solve_2(INPUT);
    println!("Part 2:\t{res}");
}

fn solve_1(input: &str) -> usize {
    let (instructions, input) = input.split_once("\n\n").unwrap();
    let instructions = instructions.as_bytes().iter().cycle().enumerate();

    let mut maps = HashMap::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let name = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];

        maps.insert(name, (left, right));
    }

    let mut current_loc = "AAA";

    for (i, dir) in instructions {
        if current_loc == "ZZZ" {
            return i;
        }
        let map = maps.get(current_loc).unwrap();

        current_loc = match dir {
            b'L' => map.0,
            _ => map.1,
        }
    }

    0
}

fn solve_2(input: &str) -> usize {
    let (instructions, input) = input.split_once("\n\n").unwrap();
    let instructions = instructions.as_bytes().iter().cycle().enumerate();

    let mut maps = HashMap::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let name = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];

        maps.insert(name, (left, right));
    }

    let maps = maps;

    let current_locs = maps.keys().cloned().filter(|s| s.ends_with('A'));

    current_locs
        .map(|mut loc| {
            for (i, dir) in instructions.clone() {
                if loc.ends_with('Z') {
                    return i;
                }
                let map = maps.get(loc).unwrap();

                loc = match dir {
                    b'L' => map.0,
                    _ => map.1,
                }
            }
            1
        })
        .fold(1, |acc, x| num::integer::lcm(acc, x))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE_1: &str = indoc::indoc! {"
    RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)"};

    const EXAMPLE_2: &str = indoc::indoc! {"
    LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)"};

    const EXAMPLE_3: &str = indoc::indoc! {"
    LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)"};

    #[test_case(EXAMPLE_1 => 2)]
    #[test_case(EXAMPLE_2 => 6)]
    fn test_part_1(i: &str) -> usize {
        solve_1(i)
    }

    #[test_case(EXAMPLE_3 => 6)]
    fn test_part_2(i: &str) -> usize {
        solve_2(i)
    }

    #[test]
    fn bench() {
        use microbench::{self, Options};

        // use this terminal command
        // cargo test --package aoc23 --bin day8 --release  -- tests::bench --exact --nocapture

        let options = Options::default();
        microbench::bench(&options, "original part 1", || solve_1(INPUT));
        microbench::bench(&options, "original part 2", || solve_2(INPUT));
    }
}
