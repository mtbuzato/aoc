fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let splits = input.split("\n\n").collect::<Vec<&str>>();

    let initial_state = splits
        .first()
        .unwrap()
        .split('\n')
        .take_while(|l| !l.starts_with('1'))
        .map(|l| l.chars().skip(1).step_by(4).collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let rows = initial_state.len();
    let cols = initial_state.first().unwrap().len();

    let crates = (0..cols)
        .map(|col| {
            (0..rows)
                .filter_map(|row| initial_state.get(row).unwrap().get(col))
                .copied()
                .filter(|c| !c.is_whitespace())
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let moves = splits
        .get(1)
        .unwrap()
        .split('\n')
        .map(|l| {
            l.split(' ')
                .filter(|s| s.chars().all(|c| c.is_numeric()))
                .filter_map(|s| s.parse::<u32>().ok())
                .collect::<Vec<u32>>()
        })
        .filter(|l| l.len() > 1)
        .collect::<Vec<Vec<u32>>>();

    for (_, m) in moves.iter().enumerate() {
        let amount = m.get(0).unwrap();
        let from = m.get(1).unwrap();
        let to = m.get(2).unwrap();

        let from_crates = crates.get(*from as usize).unwrap();
    }

    println!("crates: {:?}", crates);
    println!("moves: {:?}", moves);
}
