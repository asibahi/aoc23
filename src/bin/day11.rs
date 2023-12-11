// Template

use std::collections::HashSet;

use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day11.txt");

fn main() {
    let res = solve_1(INPUT);
    println!("Part 1:\t{res}");

    let res = solve_2(INPUT);
    println!("Part 2:\t{res}");
}

fn solve_1(input: &str) -> usize {
    let width = input.lines().next().unwrap().len() + 1;

    let mut cols = (0..width).collect::<HashSet<_>>();
    let mut rows = (0..input.lines().count()).collect::<HashSet<_>>();

    let locations = input
        .as_bytes()
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == b'#')
        .map(|(i, _)| {
            let row = i / width;
            rows.remove(&row);

            let col = i % width;
            cols.remove(&col);

            (row, col)
        })
        .collect_vec();

    locations
        .into_iter()
        .map(|(row, col)| {
            let row_offset = rows.iter().filter(|r| row > **r).count();
            let col_offset = cols.iter().filter(|c| col > **c).count();

            (row + row_offset, col + col_offset)
        })
        .tuple_combinations()
        .map(|(g1, g2)| g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1))
        .sum()
}

fn solve_2(input: &str) -> usize {
    let width = input.lines().next().unwrap().len() + 1;

    let mut cols = (0..width).collect::<HashSet<_>>();
    let mut rows = (0..input.lines().count()).collect::<HashSet<_>>();

    let locations = input
        .as_bytes()
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == b'#')
        .map(|(i, _)| {
            let row = i / width;
            rows.remove(&row);

            let col = i % width;
            cols.remove(&col);

            (row, col)
        })
        .collect_vec();

    locations
        .into_iter()
        .map(|(row, col)| {
            let row_offset = rows.iter().filter(|r| row > **r).count();
            let col_offset = cols.iter().filter(|c| col > **c).count();

            (row + 999_999 * row_offset, col + 999_999 * col_offset)
        })
        .tuple_combinations()
        .map(|(g1, g2)| g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE: &str = include_str!("../../tests/day11.txt");

    #[test_case(EXAMPLE => 374)]
    fn test_part_1(i: &str) -> usize {
        solve_1(i)
    }

    #[test]
    fn bench() {
        use microbench::{self, Options};

        // use this terminal command
        // cargo test --package aoc23 --bin day11 --release  -- tests::bench --exact --nocapture

        let options = Options::default();
        microbench::bench(&options, "original part 1", || solve_1(INPUT));
        microbench::bench(&options, "original part 2", || solve_2(INPUT));
    }
}
