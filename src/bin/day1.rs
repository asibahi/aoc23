const INPUT: &str = include_str!("../../input/day1.txt");
const NUMBERS: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];

fn main() {
    let res = solve_1(INPUT);
    println!("Part 1:\t{res}");

    let res = solve_2(INPUT);
    println!("Part 2:\t{res}");
}

fn parse_line(i: &str) -> u32 {
    let mut vals = i.chars().filter_map(|c| c.to_digit(10));

    let fst = vals.next().unwrap();
    let snd = vals.next_back().unwrap_or(fst);

    fst * 10 + snd
}

fn parse_line_2(input: &str) -> usize {
    let mut f_i = usize::MAX;
    let mut f_d = usize::MAX;

    let mut s_i = usize::MIN;
    let mut s_d = usize::MIN;

    for n in NUMBERS {
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

fn _parse_line_2_performant(input: &str) -> usize {
    // from https://github.com/proegssilb/advent-of-code/blob/main/2023/src/day1.rs#L168
    // looked at on Dec2 2023

    let tens = 'tens: {
        for i in 0..input.len() {
            let text = &input[i..];
            if let Some(i) = aoc23::parse_starts_with_digit(text) {
                break 'tens i;
            }
        }
        0
    };
    let ones = 'ones: {
        for i in (0..input.len()).rev() {
            let text = &input[i..];
            if let Some(i) = aoc23::parse_starts_with_digit(text) {
                break 'ones i;
            }
        }
        0
    };

    tens * 10 + ones
}

fn solve_1(i: &str) -> u32 {
    i.lines().map(parse_line).sum()
}

fn solve_2(i: &str) -> usize {
    i.lines().map(parse_line_2).sum()
}

fn _solve_2_performant(i: &str) -> usize {
    i.lines().map(_parse_line_2_performant).sum()
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

    fn test(i: &str) -> usize {
        parse_line_2(i)
    }

    #[test]
    fn bench() {
        use microbench::{self, Options};

        let options = Options::default();
        microbench::bench(&options, "part_1", || solve_1(INPUT));
        microbench::bench(&options, "part_2", || solve_2(INPUT));
        // microbench::bench(&options, "part_2 chemicalivory", || _solve_2_stolen(INPUT));
    }
}
