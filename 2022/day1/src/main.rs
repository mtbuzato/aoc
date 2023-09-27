fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let calories = input.split("\n\n").map(|group| {
        group
            .split('\n')
            .filter_map(|line| match line.len() {
                0 => None,
                _ => Some(line.parse::<usize>().unwrap()),
            })
            .sum::<usize>()
    });

    let max_calories = calories.clone().max().unwrap();

    let mut calories = calories.collect::<Vec<usize>>();

    calories.sort();

    let top_3_calories = calories.iter().rev().take(3).sum::<usize>();

    println!("Max calories: {}", max_calories);
    println!("Top 3 calories: {}", top_3_calories);
}
