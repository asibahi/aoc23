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
        .rfold(0, |inc, seq| inc + seq.last().unwrap())
}

#[allow(dead_code)]
fn solve_1_recursion(input: &str) -> isize {
    input.lines().map(parse_line_1_recursion).sum()
}

fn parse_line_1_recursion(input: &str) -> isize {
    let nums = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<isize>().unwrap())
        .collect_vec();

    fn next_num(seq: &[isize]) -> isize {
        if let Ok(v) = seq.iter().all_equal_value() {
            return *v;
        }

        let sub_seq = seq.iter().tuple_windows().map(|(x, y)| y - x).collect_vec();

        seq.last().unwrap() + next_num(&sub_seq)
    }

    next_num(&nums)
}

#[allow(dead_code)]
fn solve_1_recursion_2(input: &str) -> isize {
    input.lines().map(parse_line_1_recursion_2).sum()
}

fn parse_line_1_recursion_2(input: &str) -> isize {
    let mut nums = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<isize>().unwrap())
        .collect_vec();

    fn next_num(seq: &mut [isize]) -> isize {
        if let Ok(v) = seq.iter().all_equal_value() {
            return *v;
        }

        let len = seq.len();
        let last = seq[len - 1];

        for i in 0..len - 1 {
            seq[i] = seq[i + 1] - seq[i];
        }

        last + next_num(&mut seq[0..len - 1])
    }

    next_num(nums.as_mut_slice())
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
        .rfold(0, |dec, seq| seq.first().unwrap() - dec)
}

#[allow(dead_code)]
fn solve_2_recursion(input: &str) -> isize {
    input.lines().map(parse_line_2_recursion).sum()
}

fn parse_line_2_recursion(input: &str) -> isize {
    let mut nums = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<isize>().unwrap())
        .collect_vec();

    fn prev_num(seq: &mut [isize]) -> isize {
        if let Ok(v) = seq.iter().all_equal_value() {
            return *v;
        }

        let len = seq.len();
        let first = seq[0];

        for i in (1..len).rev() {
            seq[i] -= -seq[i - 1];
        }

        first - prev_num(&mut seq[1..len])
    }

    prev_num(nums.as_mut_slice())
}

#[allow(dead_code)]
fn solve_2_recursion_2(input: &str) -> isize {
    let mut my_vec = Vec::with_capacity(21);
    input
        .lines()
        .map(|s| parse_line_2_recursion_2(s, &mut my_vec))
        .sum()
}

fn parse_line_2_recursion_2(input: &str, nums: &mut Vec<isize>) -> isize {
    nums.clear();
    nums.extend(
        input
            .split_ascii_whitespace()
            .map(|s| s.parse::<isize>().unwrap()),
    );

    fn prev_num(seq: &mut [isize]) -> isize {
        if let Ok(v) = seq.iter().all_equal_value() {
            return *v;
        }

        let len = seq.len();
        let first = seq[0];

        for i in (1..len).rev() {
            seq[i] -= seq[i - 1];
        }

        first - prev_num(&mut seq[1..len])
    }

    prev_num(nums.as_mut_slice())
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
        solve_1_recursion(i)
    }

    #[test_case(EXAMPLE => 2)]
    fn test_part_2(i: &str) -> isize {
        solve_2_recursion(i)
    }

    #[test_case("0 3 6 9 12 15" => 18)]
    #[test_case("1 3 6 10 15 21" => 28)]
    #[test_case("10 13 16 21 30 45" => 68)]
    fn test_parse_1(i: &str) -> isize {
        parse_line_1_recursion(i)
    }

    #[test]
    fn bench() {
        use microbench::{self, Options};

        // use this terminal command
        // cargo test --package aoc23 --bin day9 --release  -- tests::bench --exact --nocapture

        let options = Options::default();
        microbench::bench(&options, "Vec<Vec<_>>   part 1", || solve_1(INPUT));
        microbench::bench(&options, "recursion     part 1", || {
            solve_1_recursion(INPUT)
        });
        microbench::bench(&options, "mut recursion part 1", || {
            solve_1_recursion_2(INPUT)
        });
        microbench::bench(&options, "Vec<Vec<_>>   part 2", || solve_2(INPUT));
        microbench::bench(&options, "mut recursion part 2", || {
            solve_2_recursion(INPUT)
        });
        microbench::bench(&options, "mut rec 2     part 2", || {
            solve_2_recursion_2(INPUT)
        });
    }
}
