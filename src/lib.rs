fn _run() {
    println!("Hello, world!");
}

pub fn parse_digit(input: &str) -> usize {
    match input {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => input.parse::<usize>().unwrap(),
    }
}

pub fn parse_starts_with_digit(input: &str) -> Option<usize> {
    // from https://github.com/proegssilb/advent-of-code/blob/main/2023/src/day1.rs#L168
    // looked at on Dec2 2023
    const FIND: [&str; 18] = [
        "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
        "eight", "8", "nine", "9",
    ];

    for (idx, f) in FIND.iter().enumerate() {
        if input.starts_with(f) {
            return Some((idx >> 1) + 1);
        }
    }

    None
}
