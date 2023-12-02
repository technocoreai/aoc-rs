use utils::{aoc_main, parse_peg};

peg::parser! {
  grammar input_parser() for str {
    rule number() -> i32
      = n:$(['0'..='9']+) {? n.parse().or(Err("i32")) }

    rule ball_count() -> BallSet
      =  n:number() " " "red" { BallSet(-n, 0, 0) }
      /  n:number() " " "green" { BallSet(0, -n, 0) }
      /  n:number() " " "blue" { BallSet(0, 0, -n) }

    rule turn() -> BallSet
      = bc:ball_count() ++ ", " { bc.into_iter().fold(BallSet(0, 0, 0), |a, b| a+b) }

    rule turns() -> Vec<BallSet>
      = t:turn() ++ "; " { t }

    rule game() -> (i32, Vec<BallSet>)
      = "Game " n:number() ": " t:turns() { (n, t) }

    pub rule input() -> Vec<(i32, Vec<BallSet>)>
      = g:game() ++ "\n" { g }
  }
}

#[derive(Debug, Copy, Clone)]
pub struct BallSet(i32, i32, i32);

impl std::ops::Add for BallSet {
    type Output = BallSet;

    fn add(self, rhs: Self) -> Self::Output {
        BallSet(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

fn part1(input: &str) -> i32 {
    let games = parse_peg(input, input_parser::input);
    let available = BallSet(12, 13, 14);
    games
        .into_iter()
        .filter_map(|(id, turns)| {
            let valid = turns.into_iter().all(|turn| match available + turn {
                BallSet(r, g, b) => r >= 0 && g >= 0 && b >= 0,
            });
            if valid {
                Some(id)
            } else {
                None
            }
        })
        .sum()
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

    static EXAMPLE_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 8);
    }

    //#[test]
    //fn test_part2() {
    //    assert_eq!(part2(EXAMPLE_INPUT), 0);
    //}
}
