fn get_aoc_value(c: char) -> u32 {
    if c.is_ascii_uppercase() {
        return Into::<u32>::into(c) - 38;
    }

    if c.is_ascii_lowercase() {
        return Into::<u32>::into(c) - 96;
    }

    0
}

fn get_value(s: &str) -> u64 {
    s.chars()
        .map(get_aoc_value)
        .fold(0, |acc, v| acc | 2_u64.pow(Into::<u32>::into(v)))
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let sacks = input
        .clone()
        .split('\n')
        .take_while(|l| !l.is_empty())
        .map(|l| l.split_at(l.len() / 2))
        .map(|(l, r)| get_value(l) & get_value(r))
        .map(|v| (v as f64).log2() as u64)
        .sum::<u64>();

    println!("Total: {}", sacks);

    let badges = input
        .split('\n')
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|c| {
            c.iter()
                .map(|s| get_value(s))
                .reduce(|acc, v| acc & v)
                .unwrap()
        })
        .map(|v| (v as f64).log2() as u64)
        .sum::<u64>();

    println!("Badges total: {:?}", badges);
}
