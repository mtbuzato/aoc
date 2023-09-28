enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn score(&self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Draw => 3,
        }
    }
}

impl From<&str> for Outcome {
    fn from(value: &str) -> Self {
        match value {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Invalid result: '{}'", value),
        }
    }
}

impl From<&str> for Play {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Invalid play: '{}'", value),
        }
    }
}

impl Play {
    fn result_against(&self, other: &Self) -> Outcome {
        match (self, other) {
            (Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper) => Outcome::Win,
            (Self::Rock, Self::Paper)
            | (Self::Paper, Self::Scissors)
            | (Self::Scissors, Self::Rock) => Outcome::Loss,
            _ => Outcome::Draw,
        }
    }

    fn score(&self) -> usize {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }

    fn get_play_for_outcome(&self, outcome: Outcome) -> Play {
        match (self, outcome) {
            (Play::Rock, Outcome::Draw)
            | (Play::Paper, Outcome::Loss)
            | (Play::Scissors, Outcome::Win) => Play::Rock,
            (Play::Rock, Outcome::Win)
            | (Play::Paper, Outcome::Draw)
            | (Play::Scissors, Outcome::Loss) => Play::Paper,
            (Play::Rock, Outcome::Loss)
            | (Play::Paper, Outcome::Win)
            | (Play::Scissors, Outcome::Draw) => Play::Scissors,
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let total = input
        .split('\n')
        .take_while(|line| !line.is_empty())
        .map(|round| {
            let plays = round.split(' ').map(Play::from).collect::<Vec<Play>>();

            let enemy_play = plays.get(0).unwrap();
            let my_play = plays.get(1).unwrap();

            my_play.score() + my_play.result_against(enemy_play).score()
        })
        .sum::<usize>();

    println!("Total score: {}", total);

    let total_from_results = input
        .split('\n')
        .take_while(|line| !line.is_empty())
        .map(|round| {
            let plays_str = round.split(' ').collect::<Vec<&str>>();

            let enemy_play = Play::from(*plays_str.first().unwrap());
            let result = Outcome::from(*plays_str.get(1).unwrap());

            let my_play = enemy_play.get_play_for_outcome(result.clone());

            my_play.score() + result.score()
        })
        .sum::<usize>();

    println!("Total score from results: {}", total_from_results);
}
