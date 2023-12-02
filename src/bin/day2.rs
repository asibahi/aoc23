const RED: usize = 12;
const GREEN: usize = 13;
const BLUE: usize = 14;

fn main() {
    let input = include_str!("../input/day2.txt");

    let res = solve_1(input);
    println!("Part 1:\t{res}");

    let res = solve_2(input);
    println!("Part 2:\t{res}");
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
                        .reduce(std::ops::BitOr::bitor)
                        .unwrap_or(false)
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
                        .unwrap_or((0, 0, 0))
                })
                .reduce(|x, y| (x.0.max(y.0), x.1.max(y.1), x.2.max(y.2)))
                .map(|(x, y, z)| x * y * z)
                .unwrap_or_default()
        })
        .unwrap_or(0)
}

fn solve_1(input: &str) -> usize {
    input.lines().map(parse_line_1).sum()
}

fn solve_2(input: &str) -> usize {
    input.lines().map(parse_line_2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green" => 48)]
    #[test_case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue" => 12)]
    #[test_case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red" => 1560)]
    #[test_case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red" => 630)]
    #[test_case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green" => 36)]
    fn test(i: &str) -> usize {
        parse_line_2(i)
    }
}
