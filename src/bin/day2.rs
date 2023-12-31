const RED: usize = 12;
const GREEN: usize = 13;
const BLUE: usize = 14;

const INPUT: &str = include_str!("../../input/day2.txt");

fn main() {
    let res = solve_1(INPUT);
    println!("Part 1:\t{res}");

    let res = solve_2(INPUT);
    println!("Part 2:\t{res}");
}

fn solve_1(input: &str) -> usize {
    input.lines().map(parse_line_1).sum()
}

fn parse_line_1(input: &str) -> usize {
    input
        .split_once(':')
        .map(|(group, hands)| {
            hands
                .split(';')
                .map(|hand| {
                    hand.split(',')
                        .map(|c| {
                            let (count, color) = c.trim().split_once(' ').unwrap();
                            match color {
                                "red" => count.parse::<usize>().unwrap() > RED,
                                "green" => count.parse::<usize>().unwrap() > GREEN,
                                _ => count.parse::<usize>().unwrap() > BLUE,
                            }
                        })
                        .reduce(|x, y| x || y)
                        .unwrap_or_default()
                })
                .all(|x| !x)
                .then(|| {
                    group
                        .split_once(' ')
                        .map(|(_, n)| n.parse::<usize>().unwrap())
                        .unwrap()
                })
                .unwrap_or_default()
        })
        .unwrap_or_default()
}

#[allow(unused)]
fn solve_1_try2(input: &str) -> usize {
    input
        .lines()
        .enumerate()
        .filter_map(|(x, y)| parse_line_1_try2(y).then_some(x))
        .sum()
}

fn parse_line_1_try2(input: &str) -> bool {
    input
        .split_once(':')
        .map(|(_, hands)| {
            hands
                .split([';', ','])
                .map(|c| {
                    let (count, color) = c.trim().split_once(' ').unwrap();
                    match color {
                        "red" => count.parse::<usize>().unwrap() > RED,
                        "green" => count.parse::<usize>().unwrap() > GREEN,
                        _ => count.parse::<usize>().unwrap() > BLUE,
                    }
                })
                .all(|x| !x)
        })
        .unwrap_or_default()
}

#[allow(unused)]
fn solve_1_try3(input: &str) -> usize {
    let mut buffer = String::new();
    input
        .lines()
        .enumerate()
        .filter_map(|(x, y)| parse_line_1_try3(y, &mut buffer).then_some(x))
        .sum()
}

fn parse_line_1_try3(input: &str, buffer: &mut String) -> bool {
    const SEPS: [char; 3] = [':', ';', ','];

    for ch in input.chars() {
        if !SEPS.contains(&ch) {
            buffer.push(ch);
        } else if ch == ':' {
            buffer.clear();
        } else {
            let (count, color) = buffer.trim().split_once(' ').unwrap();
            let cond = count.len() > 2
                || (count.len() == 2 && !count.starts_with('1'))
                || match color {
                    "red" => count.parse::<usize>().unwrap() > RED,
                    "green" => count.parse::<usize>().unwrap() > GREEN,
                    _ => count.parse::<usize>().unwrap() > BLUE,
                };
            if cond {
                return true;
            }
            buffer.clear();
        }
    }

    false
}

fn solve_2(input: &str) -> usize {
    input.lines().map(parse_line_2).sum()
}

fn parse_line_2(input: &str) -> usize {
    input
        .split_once(':')
        .map(|(_, hands)| {
            hands
                .split(';')
                .map(|hand| {
                    hand.split(',')
                        .map(|c| {
                            let (count, color) = c.trim().split_once(' ').unwrap();
                            match color {
                                "red" => (count.parse::<usize>().unwrap(), 0, 0),
                                "green" => (0, count.parse::<usize>().unwrap(), 0),
                                _ => (0, 0, count.parse::<usize>().unwrap()),
                            }
                        })
                        .reduce(|x, y| (x.0.max(y.0), x.1.max(y.1), x.2.max(y.2)))
                        .unwrap_or_default()
                })
                .reduce(|x, y| (x.0.max(y.0), x.1.max(y.1), x.2.max(y.2)))
                .map(|(x, y, z)| x * y * z)
                .unwrap_or_default()
        })
        .unwrap_or_default()
}

#[allow(unused)]
fn solve_2_try2(input: &str) -> usize {
    input.lines().map(parse_line_2_try2).sum()
}

fn parse_line_2_try2(input: &str) -> usize {
    input
        .split_once(':')
        .and_then(|(_, hands)| {
            hands
                .split([';', ','])
                .map(|c| {
                    let (count, color) = c.trim().split_once(' ').unwrap();
                    match color {
                        "red" => (count.parse::<usize>().unwrap(), 0, 0),
                        "green" => (0, count.parse::<usize>().unwrap(), 0),
                        _ => (0, 0, count.parse::<usize>().unwrap()),
                    }
                })
                .reduce(|x, y| (x.0.max(y.0), x.1.max(y.1), x.2.max(y.2)))
                .map(|(x, y, z)| x * y * z)
        })
        .unwrap_or_default()
}

#[allow(unused)]
fn solve_2_try3(input: &str) -> usize {
    input.lines().map(parse_line_2_try3).sum()
}

fn parse_line_2_try3(input: &str) -> usize {
    input
        .split_once(':')
        .map(|(_, hands)| {
            hands
                .split([';', ','])
                .fold([0; 3], |mut acc, x| {
                    let (count, color) = x.trim().split_once(' ').unwrap();
                    let count = count.parse::<usize>().unwrap();
                    let i = &mut acc[color.as_bytes()[0] as usize % 3];
                    *i = (*i).max(count);
                    acc
                })
                .iter()
                .product()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green" => 1)]
    #[test_case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue" => 2)]
    #[test_case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red" => 0)]
    #[test_case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red" => 0)]
    #[test_case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green" => 5)]
    fn test_part_1(i: &str) -> usize {
        parse_line_1(i)
    }

    #[test_case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green" => 48)]
    #[test_case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue" => 12)]
    #[test_case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red" => 1560)]
    #[test_case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red" => 630)]
    #[test_case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green" => 36)]
    fn test_part_2(i: &str) -> usize {
        parse_line_2(i)
    }

    #[test]
    fn bench() {
        use microbench::{self, Options};

        // use this terminal command
        // cargo test --package aoc23 --bin day2 --release  -- tests::bench --exact --nocapture

        let options = Options::default();
        microbench::bench(&options, "original part 1", || solve_1(INPUT));
        microbench::bench(&options, "try 2    part 1", || solve_1_try2(INPUT));
        microbench::bench(&options, "try 3    part 1", || solve_1_try3(INPUT));
        microbench::bench(&options, "original part 2", || solve_2(INPUT));
        microbench::bench(&options, "try 2    part 2", || solve_2_try2(INPUT));
        microbench::bench(&options, "try 3    part 2", || solve_2_try3(INPUT));
    }
}
