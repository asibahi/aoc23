use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../input/day3.txt");

fn main() {
    let res = solve_1(INPUT);
    println!("Part 1 Try 1:\t{res}");

    let res = solve_2(INPUT);
    println!("Part 2:\t{res}");
}

#[derive(PartialEq, Eq, Hash)]
struct NumberLocation {
    line: usize,
    leftmost_col: usize,
    rightmost_col: usize,
}

impl NumberLocation {
    fn new(line: usize, leftmost_col: usize, rightmost_col: usize) -> Self {
        Self {
            line,
            leftmost_col,
            rightmost_col,
        }
    }

    fn expand(&self) -> Vec<(usize, usize)> {
        let mut res = Vec::new();

        for col in self.leftmost_col.saturating_sub(1)..=self.rightmost_col + 1 {
            for row in self.line.saturating_sub(1)..=self.line + 1 {
                if !(row == self.line && col >= self.leftmost_col && col <= self.rightmost_col) {
                    res.push((row, col))
                }
            }
        }
        res
    }
}

fn solve_1(input: &str) -> usize {
    let mut num_map = HashMap::new();
    let mut sym_map = HashSet::new();

    input
        .lines()
        .enumerate()
        .map(|(line_number, line)| parse_line_1(line_number, line))
        .for_each(|(x, y)| {
            num_map.extend(x);
            sym_map.extend(y);
        });

    num_map
        .into_iter()
        .filter(|(k, _)| k.expand().iter().any(|k| sym_map.contains(k)))
        .map(|(_, v)| v)
        .sum()
}

fn parse_line_1(
    line_number: usize,
    input: &str,
) -> (HashMap<NumberLocation, usize>, HashSet<(usize, usize)>) {
    // where to store the buffers
    let mut buffer = String::new();
    // keys are the location of the digit , values are the digits and symbols
    let mut num_map = HashMap::new();
    let mut sym_map = HashSet::new();

    for (i, c) in input.char_indices() {
        if c.is_ascii_digit() {
            buffer.push(c)
        } else {
            // either a '.' or a symbol.
            if !buffer.is_empty() {
                // if we have digits in the buffer
                let num = buffer.parse::<usize>().unwrap();
                num_map.insert(
                    NumberLocation::new(line_number, i - buffer.len(), i - 1),
                    num,
                );
            }
            buffer.clear();
            if c != '.' {
                // if it is a symbol
                sym_map.insert((line_number, i));
            }
        }
    }
    if !buffer.is_empty() {
        // if we have digits in the buffer
        let num = buffer.parse::<usize>().unwrap();
        num_map.insert(
            NumberLocation::new(line_number, input.len() - buffer.len(), input.len() - 1),
            num,
        );
    }

    (num_map, sym_map)
}

#[allow(dead_code)]
fn solve_1_try2(input: &str) -> usize {
    let mut num_map = HashMap::new();
    let mut sym_map = HashSet::new();
    let mut buffer = String::new();

    input.lines().enumerate().for_each(|(line_number, line)| {
        parse_line_1_try2(&mut buffer, &mut num_map, &mut sym_map, line_number, line)
    });

    num_map
        .into_iter()
        .filter(|(k, _)| k.expand().iter().any(|k| sym_map.contains(k)))
        .map(|(_, v)| v)
        .sum()
}

fn parse_line_1_try2(
    buffer: &mut String,
    num_map: &mut HashMap<NumberLocation, usize>,
    sym_map: &mut HashSet<(usize, usize)>,
    line_number: usize,
    input: &str,
) {
    buffer.clear();

    for (i, c) in input.char_indices() {
        if c.is_ascii_digit() {
            buffer.push(c)
        } else {
            // either a '.' or a symbol.
            if !buffer.is_empty() {
                // if we have digits in the buffer
                let num = buffer.parse::<usize>().unwrap();
                num_map.insert(
                    NumberLocation::new(line_number, i - buffer.len(), i - 1),
                    num,
                );
            }
            buffer.clear();
            if c != '.' {
                // if it is a symbol
                sym_map.insert((line_number, i));
            }
        }
    }
    if !buffer.is_empty() {
        // if we have digits in the buffer
        let num = buffer.parse::<usize>().unwrap();
        num_map.insert(
            NumberLocation::new(line_number, input.len() - buffer.len(), input.len() - 1),
            num,
        );
    }
}

#[allow(dead_code)]
fn solve_1_try3(input: &str) -> usize {
    let width = input.find('\n').unwrap() + 1;
    let chars = input.chars().collect::<Vec<_>>();

    let mut running_sum = 0;

    for (line_number, line) in input.lines().enumerate() {
        let mut digit_line = 0;

        for (i, c) in line.char_indices() {
            if c.is_ascii_digit() {
                digit_line += 1;
            } else {
                // either a '.' or a symbol.
                if digit_line > 0
                    && (/* cols */(i - digit_line).saturating_sub(1)..=i.min(width - 1))
                        .cartesian_product(
                            /* rows */ line_number.saturating_sub(1)..=line_number + 1,
                        )
                        .filter(|(col, row)| {
                            !(*row == line_number && *col >= (i - digit_line) && *col <= (i - 1))
                        })
                        .map(|(row, col)| width * row + col)
                        .any(|idx| chars[idx] != '.' || !chars[idx].is_ascii_digit())
                {
                    running_sum += &line[i - digit_line..i].parse::<usize>().unwrap();
                }
                digit_line = 0;
            }
        }

        if digit_line > 0
            && (/* cols */(line.len() - digit_line).saturating_sub(1)..=line.len().min(width - 1))
                .cartesian_product(
                    /* rows */ line_number.saturating_sub(1)..=line_number + 1,
                )
                .filter(|(col, row)| {
                    !(*row == line_number
                        && *col >= (line.len() - digit_line)
                        && *col <= (line.len() - 1))
                })
                .map(|(row, col)| width * row + col)
                .any(|idx| chars[idx] != ',' || !chars[idx].is_ascii_digit())
        {
            running_sum += &line[line.len() - digit_line..line.len()]
                .parse::<usize>()
                .unwrap();
        }
    }

    running_sum
}

fn solve_2(input: &str) -> usize {
    let mut num_map = HashMap::new();
    let mut gear_mp = HashMap::new();

    input
        .lines()
        .enumerate()
        .map(|(line_number, line)| parse_line_2(line_number, line))
        .for_each(|(x, y)| {
            num_map.extend(x);
            gear_mp.extend(y);
        });

    for (loc, n) in num_map {
        let potentials = loc.expand();

        for pot in potentials {
            if let Some(nums) = gear_mp.get_mut(&pot) {
                match *nums {
                    (None, None) => *nums = (Some(n), None),
                    (None, s) => *nums = (Some(n), s),
                    (s, None) => *nums = (s, Some(n)),
                    s => *nums = s,
                }
            }
        }
    }

    gear_mp
        .into_values()
        .filter_map(|(fst, snd)| fst.and_then(|fst| snd.map(|snd| snd * fst)))
        .sum()
}

type GearMap = HashMap<(usize, usize), (Option<usize>, Option<usize>)>;
type ParseLine2Return = (HashMap<NumberLocation, usize>, GearMap);

fn parse_line_2(line_number: usize, input: &str) -> ParseLine2Return {
    // where to store the buffers
    let mut buffer = String::new();
    // keys are the location of the digit , values are the digits and symbols
    let mut num_map = HashMap::new();
    let mut gear_mp = HashMap::new();

    for (i, c) in input.char_indices() {
        if c.is_ascii_digit() {
            buffer.push(c)
        } else {
            // either a '.' or a symbol.
            if !buffer.is_empty() {
                // if we have digits in the buffer
                let num = buffer.parse::<usize>().unwrap();
                num_map.insert(
                    NumberLocation::new(line_number, i - buffer.len(), i - 1),
                    num,
                );
            }
            buffer.clear();
            if c == '*' {
                // if it is a gear
                gear_mp.insert((line_number, i), (None, None));
            }
        }
    }
    if !buffer.is_empty() {
        // if we have digits in the buffer
        let num = buffer.parse::<usize>().unwrap();
        num_map.insert(
            NumberLocation::new(line_number, input.len() - buffer.len(), input.len() - 1),
            num,
        );
    }

    (num_map, gear_mp)
}

#[allow(dead_code)]
fn solve_2_try2(input: &str) -> usize {
    let mut num_map = HashMap::new();
    let mut gear_mp = HashMap::new();
    let mut buffer = String::new();

    input.lines().enumerate().for_each(|(line_number, line)| {
        parse_line_2_try2(&mut buffer, &mut num_map, &mut gear_mp, line_number, line)
    });

    for (loc, n) in num_map {
        let potentials = loc.expand();

        for pot in potentials {
            if let Some(nums) = gear_mp.get_mut(&pot) {
                match *nums {
                    (None, None) => *nums = (Some(n), None),
                    (None, s) => *nums = (Some(n), s),
                    (s, None) => *nums = (s, Some(n)),
                    s => *nums = s,
                }
            }
        }
    }

    gear_mp
        .into_values()
        .filter_map(|(fst, snd)| fst.and_then(|fst| snd.map(|snd| snd * fst)))
        .sum()
}

fn parse_line_2_try2(
    buffer: &mut String,
    num_map: &mut HashMap<NumberLocation, usize>,
    gear_mp: &mut GearMap,
    line_number: usize,
    input: &str,
) {
    for (i, c) in input.char_indices() {
        if c.is_ascii_digit() {
            buffer.push(c)
        } else {
            // either a '.' or a symbol.
            if !buffer.is_empty() {
                // if we have digits in the buffer
                let num = buffer.parse::<usize>().unwrap();
                num_map.insert(
                    NumberLocation::new(line_number, i - buffer.len(), i - 1),
                    num,
                );
            }
            buffer.clear();
            if c == '*' {
                // if it is a gear
                gear_mp.insert((line_number, i), (None, None));
            }
        }
    }
    if !buffer.is_empty() {
        // if we have digits in the buffer
        let num = buffer.parse::<usize>().unwrap();
        num_map.insert(
            NumberLocation::new(line_number, input.len() - buffer.len(), input.len() - 1),
            num,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE1: &str = indoc::indoc! {"
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598.."};
    const EXAMPLE2: &str = indoc::indoc! {"
    467..114.
    ...*.....
    ..35..633
    ......#..
    617*.....
    .....+.58
    ..592....
    ......755
    ...$.*...
    .664.598."};

    #[test_case(EXAMPLE1 => 4361)]
    #[test_case(EXAMPLE2 => 4361)]
    fn test_part_1(i: &str) -> usize {
        solve_1(i)
    }

    #[test_case(EXAMPLE1 => 467835)]
    #[test_case(EXAMPLE2 => 467835)]
    fn test_part_2(i: &str) -> usize {
        solve_2(i)
    }

    #[test]
    fn bench() {
        use microbench::{self, Options};

        let options = Options::default();

        // Part 1
        microbench::bench(&options, "part_1", || solve_1(INPUT));
        microbench::bench(&options, "part_1 try 2", || solve_1_try2(INPUT));
        microbench::bench(&options, "part_1 try 3", || solve_1_try3(INPUT));

        // Part 2
        // microbench::bench(&options, "part_2", || solve_2(INPUT));
        // microbench::bench(&options, "part_2 try 2", || solve_2_try2(INPUT));
    }
}
