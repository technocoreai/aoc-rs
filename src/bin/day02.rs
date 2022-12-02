use crate::Move::{Paper, Rock, Scissors};
use scaffolding::aoc_main;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn choice_points(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

fn parse_move(s: &str) -> Move {
    match s {
        "A" | "X" => Rock,
        "B" | "Y" => Paper,
        "C" | "Z" => Scissors,
        unknown => panic!("Invalid move: {}", unknown),
    }
}

#[derive(Debug, Copy, Clone)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn determine_move(&self, opponent: Move) -> Move {
        match (opponent, self) {
            (x, Outcome::Draw) => x,
            (Rock, Outcome::Win) => Paper,
            (Paper, Outcome::Win) => Scissors,
            (Scissors, Outcome::Win) => Rock,
            (Rock, Outcome::Lose) => Scissors,
            (Paper, Outcome::Lose) => Rock,
            (Scissors, Outcome::Lose) => Paper,
        }
    }
}

fn parse_outcome(s: &str) -> Outcome {
    match s {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        unknown => panic!("Invalid outcome: {}", unknown),
    }
}

fn round_score(player: Move, opponent: Move) -> u32 {
    let outcome_points = match (&player, &opponent) {
        (a, b) if a == b => 3,
        (Rock, Scissors) => 6,
        (Paper, Rock) => 6,
        (Scissors, Paper) => 6,
        _ => 0,
    };
    outcome_points + player.choice_points()
}

fn score_part1(round: &str) -> u32 {
    match round.split_once(" ") {
        Some((opponent, player)) => {
            let opponent_move = parse_move(opponent);
            let player_move = parse_move(player);
            round_score(player_move, opponent_move)
        }
        None => panic!("Invalid round: {}", round),
    }
}

fn part1(input: &str) -> u32 {
    input
        .trim()
        .split("\n")
        .map(|round| score_part1(round))
        .sum()
}

fn score_part2(round: &str) -> u32 {
    match round.split_once(" ") {
        Some((opponent, expected_outcome)) => {
            let opponent_move = parse_move(opponent);
            let outcome = parse_outcome(expected_outcome);
            let player_move = outcome.determine_move(opponent_move);
            round_score(player_move, opponent_move)
        }
        None => panic!("Invalid round: {}", round),
    }
}

fn part2(input: &str) -> u32 {
    input
        .trim()
        .split("\n")
        .map(|round| score_part2(round))
        .sum()
}

fn main() {
    aoc_main!(part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 12);
    }
}
