fn main() {
    let input = include_str!("../input/day1.txt");

    let res = solve_1(input);
    println!("Part 1:\t{res}");

    let res = solve_2(input);
    println!("Part 2:\t{res}");
}

fn solve_1(i: &str) -> u32 {
    0
}

fn solve_2(i: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("two1nine" => 29)]
    fn test(i: &str) -> u32 {
        solve_1(i)
    }
}
