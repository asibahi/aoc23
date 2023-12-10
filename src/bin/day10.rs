use std::collections::HashSet;

const INPUT: &str = include_str!("../../input/day10.txt");

fn main() {
    let res = solve_1(INPUT);
    println!("Part 1:\t{res}");

    let res = solve_2(INPUT);
    println!("Part 2:\t{res}");

    assert_eq!(res, solve_2_try_2(INPUT));
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

fn solve_1(input: &str) -> usize {
    let width = input.lines().next().unwrap().len() + 1;

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
                came_by = Some(Dir::Up);
                counter += 1;
                continue;
            }
            // check right
            let (row, col) = (current_loc.0, current_loc.1 + 1);
            let new_pipe = input.as_bytes()[row * width + col];
            if new_pipe == b'7' || new_pipe == b'J' || new_pipe == b'-' {
                current_loc = (row, col);
                current_pipe = new_pipe;
                came_by = Some(Dir::Right);
                counter += 1;
                continue;
            }
            // check down
            let (row, col) = (current_loc.0 + 1, current_loc.1); // OVERFLOW
            let new_pipe = input.as_bytes()[row * width + col];
            if new_pipe == b'J' || new_pipe == b'L' || new_pipe == b'|' {
                current_loc = (row, col);
                current_pipe = new_pipe;
                came_by = Some(Dir::Down);
                counter += 1;
                continue;
            }
            // check left
            let (row, col) = (current_loc.0, current_loc.1 - 1); // OVERFLOW
            let new_pipe = input.as_bytes()[row * width + col];
            if new_pipe == b'L' || new_pipe == b'F' || new_pipe == b'-' {
                current_loc = (row, col);
                current_pipe = new_pipe;
                came_by = Some(Dir::Left);
                counter += 1;
                continue;
            }
            unreachable!()
        };

        counter += 1;
        match (dir, current_pipe) {
            // go right
            (Dir::Up, b'F') | (Dir::Down, b'L') | (Dir::Right, b'-') => {
                let (row, col) = (current_loc.0, current_loc.1 + 1);
                current_loc = (row, col);
                current_pipe = input.as_bytes()[row * width + col];
                came_by = Some(Dir::Right);
            }
            // go up
            (Dir::Right, b'J') | (Dir::Left, b'L') | (Dir::Up, b'|') => {
                let (row, col) = (current_loc.0 - 1, current_loc.1);
                current_loc = (row, col);
                current_pipe = input.as_bytes()[row * width + col];
                came_by = Some(Dir::Up);
            }
            // go down
            (Dir::Right, b'7') | (Dir::Left, b'F') | (Dir::Down, b'|') => {
                let (row, col) = (current_loc.0 + 1, current_loc.1); // OVERFLOW
                current_loc = (row, col);
                current_pipe = input.as_bytes()[row * width + col];
                came_by = Some(Dir::Down);
            }
            // go left
            (Dir::Down, b'J') | (Dir::Up, b'7') | (Dir::Left, b'-') => {
                let (row, col) = (current_loc.0, current_loc.1 - 1); // OVERFLOW
                current_loc = (row, col);
                current_pipe = input.as_bytes()[row * width + col];
                came_by = Some(Dir::Left);
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
    let width = input.lines().next().unwrap().len() + 1;

    let mut current_loc = {
        let start_loc = input.find('S').unwrap();
        (
            start_loc / width, // row
            start_loc % width, // column
        )
    };

    let mut points = vec![];

    let mut current_pipe = input.as_bytes()[current_loc.0 * width + current_loc.1];
    let mut came_by = None;

    loop {
        // This runs once
        let Some(dir) = came_by.as_ref() else {
            // check up
            if current_loc.0 > 0 {
                let (row, col) = (current_loc.0 - 1, current_loc.1);
                let new_pipe = input.as_bytes()[row * width + col];
                if new_pipe == b'7' || new_pipe == b'F' || new_pipe == b'|' {
                    current_loc = (row, col);
                    current_pipe = new_pipe;
                    came_by = Some(Dir::Up);
                    points.push((row, col));
                    continue;
                }
            }
            // check right
            let (row, col) = (current_loc.0, current_loc.1 + 1);
            let new_pipe = input.as_bytes()[row * width + col];
            if new_pipe == b'7' || new_pipe == b'J' || new_pipe == b'-' {
                current_loc = (row, col);
                current_pipe = new_pipe;
                came_by = Some(Dir::Right);
                points.push((row, col));
                continue;
            }
            // check down
            let (row, col) = (current_loc.0 + 1, current_loc.1); // OVERFLOW
            let new_pipe = input.as_bytes()[row * width + col];
            if new_pipe == b'J' || new_pipe == b'L' || new_pipe == b'|' {
                current_loc = (row, col);
                current_pipe = new_pipe;
                came_by = Some(Dir::Down);
                points.push((row, col));
                continue;
            }
            // check left
            let (row, col) = (current_loc.0, current_loc.1 - 1); // OVERFLOW
            let new_pipe = input.as_bytes()[row * width + col];
            if new_pipe == b'L' || new_pipe == b'F' || new_pipe == b'-' {
                current_loc = (row, col);
                current_pipe = new_pipe;
                came_by = Some(Dir::Left);
                points.push((row, col));
                continue;
            }
            unreachable!()
        };

        match (dir, current_pipe) {
            // go right
            (Dir::Up, b'F') | (Dir::Down, b'L') | (Dir::Right, b'-') => {
                let (row, col) = (current_loc.0, current_loc.1 + 1);
                current_loc = (row, col);
                current_pipe = input.as_bytes()[row * width + col];
                came_by = Some(Dir::Right);
                points.push((row, col));
            }
            // go up
            (Dir::Right, b'J') | (Dir::Left, b'L') | (Dir::Up, b'|') => {
                let (row, col) = (current_loc.0 - 1, current_loc.1);
                current_loc = (row, col);
                current_pipe = input.as_bytes()[row * width + col];
                came_by = Some(Dir::Up);
                points.push((row, col));
            }
            // go down
            (Dir::Right, b'7') | (Dir::Left, b'F') | (Dir::Down, b'|') => {
                let (row, col) = (current_loc.0 + 1, current_loc.1); // OVERFLOW
                current_loc = (row, col);
                current_pipe = input.as_bytes()[row * width + col];
                came_by = Some(Dir::Down);
                points.push((row, col));
            }
            // go left
            (Dir::Down, b'J') | (Dir::Up, b'7') | (Dir::Left, b'-') => {
                let (row, col) = (current_loc.0, current_loc.1 - 1); // OVERFLOW
                current_loc = (row, col);
                current_pipe = input.as_bytes()[row * width + col];
                came_by = Some(Dir::Left);
                points.push((row, col));
            }
            (_, b'S') => {
                break;
            }
            _ => eprintln!("----------> DEAD ZONE"),
        }
    }

    #[allow(unused)]
    let count_intersections = |row, col| {
        if points.contains(&(row, col)) {
            return false;
        }
        let sticks = points
            .iter()
            .filter(|(r, c)| *r == row && *c > col)
            .filter(|(r, c)| input.as_bytes()[r * width + c] == b'|')
            .count();
        let effs = points
            .iter()
            .filter(|(r, c)| *r == row && *c > col)
            .filter(|(r, c)| input.as_bytes()[r * width + c] == b'F')
            .count();
        let sevens = points
            .iter()
            .filter(|(r, c)| *r == row && *c > col)
            .filter(|(r, c)| input.as_bytes()[r * width + c] == b'7')
            .count();
        let jays = points
            .iter()
            .filter(|(r, c)| *r == row && *c > col)
            .filter(|(r, c)| input.as_bytes()[r * width + c] == b'J')
            .count();
        let ells = points
            .iter()
            .filter(|(r, c)| *r == row && *c > col)
            .filter(|(r, c)| input.as_bytes()[r * width + c] == b'L')
            .count();

        let count = 100 + sticks + effs - sevens;

        count % 2 == 1
    };

    input
        .bytes()
        .enumerate()
        .filter(|(i, _c)| {
            let loc = (i / width, i % width);
            !points.contains(&loc) && count_intersections(loc.0, loc.1)
        })
        .count()
}

#[allow(dead_code)]
fn solve_2_try_2(input: &str) -> usize {
    let width = input.lines().next().unwrap().len() + 1;

    let mut current_loc = {
        let start_loc = input.find('S').unwrap();
        (
            start_loc / width, // row
            start_loc % width, // column
        )
    };
    let input = input.as_bytes();

    let true_s = {
        // check right
        let right = input[current_loc.0 * width + (current_loc.1 + 1)];
        let conntected_to_right = right == b'-' || right == b'7' || right == b'J';

        // check down
        let down = input[(current_loc.0 + 1) * width + current_loc.1];
        let connected_to_down = down == b'|' || down == b'J' || down == b'L';

        // check left
        let left = input[current_loc.0 * width + (current_loc.1 - 1)];
        let connected_to_left = left == b'F' || left == b'L' || left == b'-';

        // check up
        let connected_to_up = current_loc.0 > 0 && {
            let up = input[(current_loc.0 - 1) * width + current_loc.1];
            up == b'|' || up == b'7' || up == b'F'
        };

        if conntected_to_right && connected_to_down {
            b'F'
        } else if connected_to_left && conntected_to_right {
            b'-'
        } else if connected_to_left && connected_to_down {
            b'7'
        } else if connected_to_left && connected_to_up {
            b'J'
        } else if conntected_to_right && connected_to_up {
            b'L'
        } else {
            b'|'
        }
    };

    let mut points = HashSet::new();

    let mut current_pipe = input[current_loc.0 * width + current_loc.1];
    let mut came_by = None;

    loop {
        // This runs once
        let Some(dir) = came_by.as_ref() else {
            let (row, col, new_pipe) = 'checker: {
                // check right
                let (row, col) = (current_loc.0, current_loc.1 + 1);
                let new_pipe = input[row * width + col];
                if new_pipe == b'7' || new_pipe == b'J' || new_pipe == b'-' {
                    came_by = Some(Dir::Right);
                    break 'checker (row, col, new_pipe);
                }
                // check down
                let (row, col) = (current_loc.0 + 1, current_loc.1);
                let new_pipe = input[row * width + col];
                if new_pipe == b'J' || new_pipe == b'L' || new_pipe == b'|' {
                    came_by = Some(Dir::Down);
                    break 'checker (row, col, new_pipe);
                }
                // check left
                let (row, col) = (current_loc.0, current_loc.1 - 1);
                let new_pipe = input[row * width + col];
                if new_pipe == b'L' || new_pipe == b'F' || new_pipe == b'-' {
                    came_by = Some(Dir::Left);
                    break 'checker (row, col, new_pipe);
                }
                // check up
                let (row, col) = (current_loc.0 - 1, current_loc.1);
                let new_pipe = input[row * width + col];
                if new_pipe == b'7' || new_pipe == b'F' || new_pipe == b'|' {
                    came_by = Some(Dir::Up);
                    break 'checker (row, col, new_pipe);
                }
                unreachable!()
            };
            current_loc = (row, col);
            current_pipe = new_pipe;
            points.insert((row, col));

            continue;
        };

        let (row, col) = match (dir, current_pipe) {
            // go right
            (Dir::Up, b'F') | (Dir::Down, b'L') | (Dir::Right, b'-') => {
                came_by = Some(Dir::Right);
                (current_loc.0, current_loc.1 + 1)
            }
            // go up
            (Dir::Right, b'J') | (Dir::Left, b'L') | (Dir::Up, b'|') => {
                came_by = Some(Dir::Up);
                (current_loc.0 - 1, current_loc.1)
            }
            // go down
            (Dir::Right, b'7') | (Dir::Left, b'F') | (Dir::Down, b'|') => {
                came_by = Some(Dir::Down);
                (current_loc.0 + 1, current_loc.1)
            }
            // go left
            (Dir::Down, b'J') | (Dir::Up, b'7') | (Dir::Left, b'-') => {
                came_by = Some(Dir::Left);
                (current_loc.0, current_loc.1 - 1)
            }
            (_, b'S') => break,
            _ => unreachable!(),
        };
        current_loc = (row, col);
        current_pipe = input[row * width + col];
        points.insert((row, col));
    }

    let mut inner_counter = 0;
    let mut currently_inside = false;
    let mut last_corner = None;

    for (i, c) in input.iter().enumerate() {
        let c = if *c == b'S' { true_s } else { *c };
        let loc = (i / width, i % width);
        if points.contains(&loc) {
            match c {
                b'|' => {
                    // print!("┃");
                    currently_inside = !currently_inside
                }
                b'F' => {
                    // print!("┏");
                    last_corner = Some(b'F')
                }
                b'L' => {
                    // print!("┗");
                    last_corner = Some(b'L')
                }
                b'J' if last_corner.is_some_and(|c| c == b'F') => {
                    // print!("┛");
                    currently_inside = !currently_inside;
                }
                b'7' if last_corner.is_some_and(|c| c == b'L') => {
                    // print!("┓");
                    currently_inside = !currently_inside
                }
                // b'-' => print!("━"),
                // b'J' => print!("┛"),
                // b'7' => print!("┓"),
                _ => {}
            }
        } else if currently_inside {
            // print!("█");
            inner_counter += 1;
        } else {
            // print!("{}", c as char);
        }
    }

    inner_counter
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
        solve_2_try_2(i)
    }

    #[test]
    fn bench() {
        use microbench::{self, Options};

        // use this terminal command
        // cargo test --package aoc23 --bin day10 --release  -- tests::bench --exact --nocapture

        let options = Options::default();
        microbench::bench(&options, "original part 1", || solve_1(INPUT));
        microbench::bench(&options, "original part 2", || solve_2(INPUT));
        microbench::bench(&options, "try 2    part 2", || solve_2_try_2(INPUT));
    }
}
