use aoc_runner_derive::aoc;

#[derive(Clone, Copy)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn for_opponent(value: &str) -> Self {
        match value {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
            _ => panic!("Invalid input"),
        }
    }

    fn for_player(value: &str) -> Self {
        match value {
            "X" => Play::Rock,
            "Y" => Play::Paper,
            "Z" => Play::Scissors,
            _ => panic!("Invalid input"),
        }
    }

    fn for_player_result(value: &str, opponent: Play) -> Self {
        match (value, opponent) {
            ("X", Play::Rock) => Play::Scissors,
            ("X", Play::Paper) => Play::Rock,
            ("X", Play::Scissors) => Play::Paper,
            ("Y", Play::Rock) => Play::Rock,
            ("Y", Play::Paper) => Play::Paper,
            ("Y", Play::Scissors) => Play::Scissors,
            ("Z", Play::Rock) => Play::Paper,
            ("Z", Play::Paper) => Play::Scissors,
            ("Z", Play::Scissors) => Play::Rock,
            _ => panic!("Invalid input"),
        }
    }

    fn value(self) -> usize {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }

    fn outcome(self, opponent: Play) -> usize {
        match (self, opponent) {
            (Play::Rock, Play::Rock) => 3,
            (Play::Rock, Play::Paper) => 0,
            (Play::Rock, Play::Scissors) => 6,
            (Play::Paper, Play::Rock) => 6,
            (Play::Paper, Play::Paper) => 3,
            (Play::Paper, Play::Scissors) => 0,
            (Play::Scissors, Play::Rock) => 0,
            (Play::Scissors, Play::Paper) => 6,
            (Play::Scissors, Play::Scissors) => 3,
        }
    }
}

#[aoc(day2, part1)]
fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (opponent, player) = line.split_once(' ').unwrap();
            let opponent_play = Play::for_opponent(opponent);
            let player_play = Play::for_player(player);

            player_play.value() + player_play.outcome(opponent_play)
        })
        .sum()
}

#[aoc(day2, part2)]
fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (opponent, result) = line.split_once(' ').unwrap();
            let opponent_play = Play::for_opponent(opponent);
            let player_play = Play::for_player_result(result, opponent_play);

            player_play.value() + player_play.outcome(opponent_play)
        })
        .sum()
}
