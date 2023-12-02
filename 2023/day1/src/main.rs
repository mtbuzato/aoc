fn main() {
    first_part();
    second_part();
}

fn first_part() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let value: u32 = input
        .lines()
        .map(|line| {
            let chars = line.chars();

            format!(
                "{}{}",
                chars.clone().find(|c| c.is_numeric()).unwrap(),
                chars.rev().find(|c| c.is_numeric()).unwrap()
            )
            .parse::<u32>()
            .unwrap()
        })
        .sum();

    println!("{}", value);
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn second_part() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let value: u32 = input
        .lines()
        .map(|line| {
            let mut matches = line
                .match_indices(char::is_numeric)
                .chain(DIGITS.iter().flat_map(|d| line.match_indices(d)))
                .collect::<Vec<_>>();

            matches.sort_by_key(|m| m.0);

            format!(
                "{}{}",
                digit_to_number(matches.first().unwrap().1),
                digit_to_number(matches.last().unwrap().1),
            )
            .parse::<u32>()
            .unwrap()
        })
        .sum();

    println!("{:?}", value);
}

fn digit_to_number(digit: &str) -> u32 {
    if digit.chars().next().unwrap().is_numeric() {
        return digit.parse::<u32>().unwrap();
    }

    match digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("Unknown digit: {}", digit),
    }
}
