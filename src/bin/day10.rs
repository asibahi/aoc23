// use itertools::Itertools;

// use std::{thread::sleep, time::Duration};

const INPUT: &str = include_str!("../../input/day10.txt");

fn main() {
    let res = solve_1(INPUT);
    println!("Part 1:\t{res}");

    let res = solve_2(INPUT);
    println!("Part 2:\t{res}");
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn solve_1(input: &str) -> usize {
    let width = input.lines().next_back().unwrap().len() + 1;

    let mut current_loc = {
        let start_loc = input.find('S').unwrap();
        (
            start_loc / width, // row
            start_loc % width, // column
        )
    };

    let mut current_pipe = input.as_bytes()[current_loc.0 * width + current_loc.1];
    let mut came_by = None;
    let mut counter = 0;

    loop {
        // This runs once
        let Some(dir) = came_by.as_ref() else {
            // check up
            let (row, col) = (current_loc.0 - 1, current_loc.1);
            let new_pipe = input.as_bytes()[row * width + col];
            if new_pipe == b'7' || new_pipe == b'F' || new_pipe == b'|' {
                current_loc = (row, col);
                current_pipe = new_pipe;
                came_by = Some(Direction::Up);
                counter += 1;
                continue;
            }
            // check right
            let (row, col) = (current_loc.0, current_loc.1 + 1);
            let new_pipe = input.as_bytes()[row * width + col];
            if new_pipe == b'7' || new_pipe == b'J' || new_pipe == b'_' {
                current_loc = (row, col);
                current_pipe = new_pipe;
                came_by = Some(Direction::Right);
                counter += 1;
                continue;
            }
            // check down
            let (row, col) = (current_loc.0 + 1, current_loc.1); // OVERFLOW
            let new_pipe = input.as_bytes()[row * width + col];
            if new_pipe == b'J' || new_pipe == b'L' || new_pipe == b'|' {
                current_loc = (row, col);
                current_pipe = new_pipe;
                came_by = Some(Direction::Down);
                counter += 1;
                continue;
            }
            // check left
            let (row, col) = (current_loc.0, current_loc.1 - 1); // OVERFLOW
            let new_pipe = input.as_bytes()[row * width + col];
            if new_pipe == b'L' || new_pipe == b'F' || new_pipe == b'_' {
                current_loc = (row, col);
                current_pipe = new_pipe;
                came_by = Some(Direction::Left);
                counter += 1;
                continue;
            }
            unreachable!()
        };

        counter += 1;
        match (dir, current_pipe) {
            // go right
            (Direction::Up, b'F') | (Direction::Down, b'L') | (Direction::Right, b'-') => {
                let (row, col) = (current_loc.0, current_loc.1 + 1);
                current_loc = (row, col);
                current_pipe = input.as_bytes()[row * width + col];
                came_by = Some(Direction::Right);
            }
            // go up
            (Direction::Right, b'J') | (Direction::Left, b'L') | (Direction::Up, b'|') => {
                let (row, col) = (current_loc.0 - 1, current_loc.1);
                current_loc = (row, col);
                current_pipe = input.as_bytes()[row * width + col];
                came_by = Some(Direction::Up);
            }
            // go down
            (Direction::Right, b'7') | (Direction::Left, b'F') | (Direction::Down, b'|') => {
                let (row, col) = (current_loc.0 + 1, current_loc.1); // OVERFLOW
                current_loc = (row, col);
                current_pipe = input.as_bytes()[row * width + col];
                came_by = Some(Direction::Down);
            }
            // go left
            (Direction::Down, b'J') | (Direction::Up, b'7') | (Direction::Left, b'-') => {
                let (row, col) = (current_loc.0, current_loc.1 - 1); // OVERFLOW
                current_loc = (row, col);
                current_pipe = input.as_bytes()[row * width + col];
                came_by = Some(Direction::Left);
            }
            (_, b'S') => {
                break;
            }
            _ => eprintln!("----------> DEAD ZONE"),
        }
    }

    counter / 2
}

fn solve_2(input: &str) -> usize {
    input.lines().map(|i| i.len()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE1: &str = include_str!("../../tests/day10_1.txt");
    const EXAMPLE2: &str = include_str!("../../tests/day10_2.txt");

    #[test_case(EXAMPLE1 => 4)]
    #[test_case(EXAMPLE2 => 8)]
    fn test_part_1(i: &str) -> usize {
        solve_1(i)
    }

    const EXAMPLE3: &str = include_str!("../../tests/day10_3.txt");
    const EXAMPLE4: &str = include_str!("../../tests/day10_4.txt");
    const EXAMPLE5: &str = include_str!("../../tests/day10_5.txt");
    const EXAMPLE6: &str = include_str!("../../tests/day10_6.txt");

    #[test_case(EXAMPLE3 => 4)]
    #[test_case(EXAMPLE4 => 4)]
    #[test_case(EXAMPLE5 => 8)]
    #[test_case(EXAMPLE6 => 10)]
    fn test_part_2(i: &str) -> usize {
        solve_2(i)
    }

    // #[test]
    // fn bench() {
    //     use microbench::{self, Options};

    //     // use this terminal command
    //     // cargo test --package aoc23 --bin day2 --release  -- tests::bench --exact --nocapture

    //     let options = Options::default();
    //     microbench::bench(&options, "original part 1", || solve_1(INPUT));
    //     microbench::bench(&options, "original part 2", || solve_2(INPUT));
    // }
}
