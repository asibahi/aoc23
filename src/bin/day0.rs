// Template

const INPUT: &str = include_str!("../../input/day0.txt");

fn main() {
    let res = solve_1(INPUT);
    println!("Part 1:\t{res}");

    let res = solve_2(INPUT);
    println!("Part 2:\t{res}");
}

fn solve_1(input: &str) -> usize {
    input.lines().map(|i| i.len()).sum()
}

fn solve_2(input: &str) -> usize {
    input.lines().map(|i| i.len()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE: &str = "insert example here";

    #[test_case(EXAMPLE => 0)]
    fn test_part_1(i: &str) -> usize {
        solve_1(i)
    }

    #[test]
    fn bench() {
        use microbench::{self, Options};

        // use this terminal command
        // cargo test --package aoc23 --bin day2 --release  -- tests::bench --exact --nocapture

        let options = Options::default();
        microbench::bench(&options, "original part 1", || solve_1(INPUT));
        microbench::bench(&options, "original part 2", || solve_2(INPUT));
    }
}
