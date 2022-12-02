use scaffolding::aoc_main;

#[derive(Debug, Eq, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn choice_points(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl std::str::FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            unknown => Err(format!("Invalid move: {}", unknown)),
        }
    }
}

#[derive(Debug)]
struct Round {
    player: Move,
    opponent: Move,
}

impl Round {
    fn score(&self) -> u32 {
        let round_points = match self {
            Round { player, opponent } if player == opponent => 3,
            Round {
                player: Move::Rock,
                opponent: Move::Scissors,
            } => 6,
            Round {
                player: Move::Paper,
                opponent: Move::Rock,
            } => 6,
            Round {
                player: Move::Scissors,
                opponent: Move::Paper,
            } => 6,
            _ => 0,
        };
        round_points + self.player.choice_points()
    }
}

impl std::str::FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s.split_once(" ") {
            Some((opponent, player)) => {
                let parsed_opponent = opponent.parse::<Move>()?;
                let parsed_player = player.parse::<Move>()?;
                Ok(Round {
                    player: parsed_player,
                    opponent: parsed_opponent,
                })
            }
            None => Err(format!("Invalid round: {}", s)),
        }
    }
}

fn parse(input: &str) -> Vec<Round> {
    input
        .trim()
        .split("\n")
        .map(|round| round.parse::<Round>().unwrap())
        .collect()
}

fn part1(input: &str) -> u32 {
    parse(input).iter().map(|round| round.score()).sum()
}

fn part2(input: &str) -> u32 {
    unimplemented!();
}

fn main() {
    aoc_main!(part1);
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

    //#[test]
    //fn test_part2() {
    //    assert_eq!(part2(EXAMPLE_INPUT), 0);
    //}
}
