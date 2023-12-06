use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day6.txt");

fn main() {
    let res = solve_1(INPUT);
    println!("Part 1:\t{res}");

    let res = solve_2(INPUT);
    println!("Part 2:\t{res}");
}

fn solve_1(input: &str) -> usize {
    let (ts, ds) = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .skip(1)
                .map(|s| s.parse::<f64>().unwrap())
        })
        .collect_tuple()
        .unwrap();

    ts.zip(ds)
        .map(|(time, dist)| {
            let roots::Roots::Two([bottom, top]) =
                roots::find_roots_quadratic(1.0, -time, dist + 0.01)
            else {
                panic!();
            };
            (top.floor() as usize) - (bottom.ceil() as usize) + 1
        })
        .product()
}

fn solve_2(input: &str) -> usize {
    let (time, dist) = input
        .lines()
        .map(|line| line.split_at(10).1.replace(" ", "").parse::<f64>().unwrap())
        .collect_tuple()
        .unwrap();

    let roots::Roots::Two([bottom, top]) = roots::find_roots_quadratic(1.0, -time, dist + 0.01)
    else {
        panic!();
    };

    (top.floor() as usize) - (bottom.ceil() as usize) + 1
}

#[allow(dead_code)]
fn solve_2_try_2(input: &str) -> usize {
    let (time, dist) = input
        .lines()
        .map(|line| {
            line.split_at(10)
                .1
                .split_ascii_whitespace()
                .collect::<String>()
                .parse::<f64>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

    let roots::Roots::Two([bottom, top]) = roots::find_roots_quadratic(1.0, -time, dist + 0.01)
    else {
        panic!();
    };

    (top.floor() as usize) - (bottom.ceil() as usize) + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE: &str = indoc::indoc! {"
    Time:      7  15   30
    Distance:  9  40  200"};

    #[test_case(EXAMPLE => 288)]
    fn test_part_1(i: &str) -> usize {
        solve_1(i)
    }
    #[test_case(EXAMPLE => 71503)]
    fn test_part_2(i: &str) -> usize {
        solve_2(i)
    }

    #[test]
    fn bench() {
        use microbench::{self, Options};

        // use this terminal command
        // cargo test --package aoc23 --bin day2 --release  -- tests::bench --exact --nocapture

        let options = Options::default();
        microbench::bench(&options, "original part 1", || solve_1(INPUT));
        microbench::bench(&options, "original part 2", || solve_2(INPUT));
        microbench::bench(&options, "split ws part 2", || solve_2_try_2(INPUT));
    }
}
