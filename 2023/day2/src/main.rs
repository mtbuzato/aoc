#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let test = Round {
        red: 12,
        green: 13,
        blue: 14,
    };

    let games = input.lines().map(Game::from).collect::<Vec<_>>();

    let possible = games
        .iter()
        .filter_map(|x| {
            if x.is_possible(&test) {
                Some(x.id)
            } else {
                None
            }
        })
        .sum::<u32>();

    let powers = games.iter().map(|x| x.power()).sum::<u32>();

    println!("Possible games: {:?}", possible);
    println!("Powers: {:?}", powers);
}

impl From<&str> for Round {
    fn from(value: &str) -> Self {
        let mut instance = Round {
            red: 0,
            green: 0,
            blue: 0,
        };

        value.split(", ").for_each(|x| {
            let parts = x.split(' ').collect::<Vec<_>>();

            let amount = parts.first().unwrap().parse::<u32>().unwrap();

            match *parts.last().unwrap() {
                "red" => instance.red = amount,
                "green" => instance.green = amount,
                "blue" => instance.blue = amount,
                _ => panic!("Unknown color"),
            }
        });

        instance
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let parts = value.split(": ").collect::<Vec<_>>();

        let id = parts
            .first()
            .unwrap()
            .replace("Game ", "")
            .parse::<u32>()
            .unwrap();
        let rounds = parts
            .last()
            .unwrap()
            .split("; ")
            .map(Round::from)
            .collect::<Vec<_>>();

        Game { id, rounds }
    }
}

impl Game {
    fn is_possible(&self, round: &Round) -> bool {
        !self
            .rounds
            .iter()
            .any(|x| x.red > round.red || x.green > round.green || x.blue > round.blue)
    }

    fn power(&self) -> u32 {
        let maxes = self.rounds.iter().fold((0, 0, 0), |acc, round| {
            (
                acc.0.max(round.red),
                acc.1.max(round.green),
                acc.2.max(round.blue),
            )
        });

        maxes.0 * maxes.1 * maxes.2
    }
}
