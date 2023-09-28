fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let nums = input
        .trim()
        .split('\n')
        .flat_map(|l| l.split(','))
        .flat_map(|l| l.split('-'))
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let total_overlaps = nums.clone().chunks(4).fold(0, |acc, chunk| {
        if (chunk[0] <= chunk[2] && chunk[1] >= chunk[3])
            || (chunk[0] >= chunk[2] && chunk[1] <= chunk[3])
        {
            return acc + 1;
        }

        acc
    });

    println!("Total overlaps: {}", total_overlaps);

    let at_all_overlaps = nums.clone().chunks(4).fold(0, |acc, chunk| {
        if (chunk[0].max(chunk[1]) >= chunk[2].min(chunk[3]))
            && (chunk[0].min(chunk[1]) <= chunk[2].max(chunk[3]))
        {
            return acc + 1;
        }

        acc
    });

    println!("At all overlaps: {}", at_all_overlaps);
}
