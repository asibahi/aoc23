//

const INPUT: &str = include_str!("../../input/day6.txt");

fn main() {
    let res = solve_1(INPUT);
    println!("Part 1:\t{res}");

    let res = solve_2(INPUT);
    println!("Part 2:\t{res}");
}

fn solve_1(input: &str) -> usize {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<f64>().unwrap());
    let distances_time = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<f64>().unwrap())
        .zip(times);

    let mut acc = 1;

    for (dist, time) in distances_time {
        let r = roots::find_roots_quadratic(1.0, -time, dist + 0.01);
        let bounds = match r {
            roots::Roots::Two(two) => {
                let bottom = two[0].ceil() as usize;
                let top = two[1].floor() as usize;
                top - bottom + 1
            }
            _ => 0,
        };

        acc *= bounds
    }

    acc
}

fn solve_2(input: &str) -> usize {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split_at(10)
        .1
        .replace(" ", "")
        .parse::<f64>()
        .unwrap();
    let dist = lines
        .next()
        .unwrap()
        .split_at(10)
        .1
        .replace(" ", "")
        .parse::<f64>()
        .unwrap();

    let mut acc = 1;

    let r = roots::find_roots_quadratic(1.0, -time, dist + 0.01);
    let bounds = match r {
        roots::Roots::Two(two) => {
            let bottom = two[0].ceil() as usize;
            let top = two[1].floor() as usize;
            top - bottom + 1
        }
        _ => 0,
    };

    acc *= bounds;

    acc
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
    }
}
