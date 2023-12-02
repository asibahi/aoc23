fn main() {
    let input = include_str!("../input/day1.txt");

    let res = solve_1(input);
    println!("Part 1:\t{res}");

    let res = solve_2(input);
    println!("Part 2:\t{res}");
}

fn parse_line(i: &str) -> u32 {
    let vals = i
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    vals[0] * 10 + vals.last().unwrap()
}

fn parse_line_2(input: &str) -> u32 {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];

    let mut f_i = usize::MAX;
    let mut f_d = u32::MAX;

    let mut s_i = usize::MIN;
    let mut s_d = u32::MIN;

    for n in numbers {
        if let Some(idx) = input.find(n).filter(|idx| idx < &f_i) {
            f_i = idx;
            f_d = aoc23::parse_digit(n);
        }

        if let Some(idx) = input.rfind(n).filter(|idx| idx >= &s_i) {
            s_i = idx;
            s_d = aoc23::parse_digit(n);
        }
    }

    f_d * 10 + s_d
}

fn solve_1(i: &str) -> u32 {
    i.lines().map(parse_line).sum()
}

fn solve_2(i: &str) -> u32 {
    i.lines().map(parse_line_2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("two1nine" => 29)]
    #[test_case("eightwothree" => 83)]
    #[test_case("abcone2threexyz" => 13)]
    #[test_case("xtwone3four" => 24)]
    #[test_case("sevenine" => 79)]
    #[test_case("one"=> 11)]

    fn test(i: &str) -> u32 {
        parse_line_2(i)
    }
}
