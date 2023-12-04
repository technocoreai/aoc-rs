use std::collections::HashSet;
use utils::{aoc_main, parse_peg};

peg::parser! {
  grammar input_parser() for str {
    rule number() -> u32
      = n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }

    rule ws() = " "+

    rule numbers() -> Vec<u32>
      = n:number() ++ ws()

    rule card() -> Card
      = "Card" ws() number() ":" ws() wins:numbers() ws() "|" ws() player:numbers() {
            Card {
                winning_numbers: wins.into_iter().collect(),
                player_numbers: player.into_iter().collect(),
            }
        }

    pub rule input() -> Vec<Card>
      = c:card() ++ "\n" { c }
  }
}

#[derive(Debug)]
pub struct Card {
    winning_numbers: HashSet<u32>,
    player_numbers: HashSet<u32>,
}

fn part1(input: &str) -> usize {
    let cards = parse_peg(input, input_parser::input);
    cards
        .iter()
        .map(|card| {
            let win_count = card
                .player_numbers
                .intersection(&card.winning_numbers)
                .count();
            if win_count > 0 {
                2usize.pow(u32::try_from(win_count).unwrap() - 1)
            } else {
                0
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

    static EXAMPLE_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 13);
    }

    //#[test]
    //fn test_part2() {
    //    assert_eq!(part2(EXAMPLE_INPUT), 0);
    //}
}
