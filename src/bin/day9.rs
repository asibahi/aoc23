use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day9.txt");

fn main() {
    let res = solve_1(INPUT);
    println!("Part 1:\t{res}");

    let res = solve_2(INPUT);
    println!("Part 2:\t{res}");
}

fn solve_1(input: &str) -> isize {
    input.lines().map(parse_line_1).sum()
}

fn parse_line_1(input: &str) -> isize {
    let nums = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<isize>().unwrap())
        .collect_vec();

    let mut sequences = vec![nums];

    while !sequences.last().unwrap().iter().all_equal() {
        let windows = sequences
            .last()
            .unwrap()
            .iter()
            .tuple_windows()
            .map(|(x, y)| y - x)
            .collect_vec();

        sequences.push(windows);
    }

    sequences
        .into_iter()
        .rev()
        .fold(0, |inc, seq| inc + seq.last().unwrap())
}

fn solve_2(input: &str) -> isize {
    input.lines().map(parse_line_2).sum()
}

fn parse_line_2(input: &str) -> isize {
    let nums = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<isize>().unwrap())
        .collect_vec();

    let mut sequences = vec![nums];

    while !sequences.last().unwrap().iter().all_equal() {
        let windows = sequences
            .last()
            .unwrap()
            .iter()
            .tuple_windows()
            .map(|(x, y)| y - x)
            .collect_vec();

        sequences.push(windows);
    }

    sequences
        .into_iter()
        .rev()
        .fold(0, |dec, seq| seq.first().unwrap() - dec)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE: &str = indoc::indoc! {"
    0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45"};

    #[test_case(EXAMPLE => 114)]
    fn test_part_1(i: &str) -> isize {
        solve_1(i)
    }

    #[test_case(EXAMPLE => 2)]
    fn test_part_2(i: &str) -> isize {
        solve_2(i)
    }

    #[test_case("0 3 6 9 12 15" => 18)]
    #[test_case("1 3 6 10 15 21" => 28)]
    #[test_case("10 13 16 21 30 45" => 68)]
    fn test_parse_1(i: &str) -> isize {
        parse_line_1(i)
    }

    #[test]
    fn bench() {
        use microbench::{self, Options};

        // use this terminal command
        // cargo test --package aoc23 --bin day9 --release  -- tests::bench --exact --nocapture

        let options = Options::default();
        microbench::bench(&options, "original part 1", || solve_1(INPUT));
        microbench::bench(&options, "original part 2", || solve_2(INPUT));
    }
}
