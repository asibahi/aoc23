fn main() {
    let input = include_str!("../input/day3.txt");

    let res = solve_1(input);
    println!("Part 1:\t{res}");

    let res = solve_2(input);
    println!("Part 2:\t{res}");
}

fn parse_line_1(input: &str) -> usize {
    input.len()
}

fn parse_line_2(input: &str) -> usize {
    input.len()
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

    #[test_case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green" => 1)]
    fn test_part_1(i: &str) -> usize {
        parse_line_1(i)
    }

    #[test_case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green" => 48)]
    fn test_part_2(i: &str) -> usize {
        parse_line_2(i)
    }
}
