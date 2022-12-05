use std::collections::HashSet;
use utils::aoc_main;

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect()
}

fn priority(c: char) -> u32 {
    match c {
        lowercase if lowercase >= 'a' && lowercase <= 'z' => c as u32 - 96,
        uppercase if uppercase >= 'A' && uppercase <= 'Z' => c as u32 - 38,
        _ => panic!("Invalid character: {}", c),
    }
}

fn split_halves(rucksack: Vec<char>) -> (HashSet<char>, HashSet<char>) {
    let length = rucksack.len();
    if length == 0 || length % 2 != 0 {
        panic!("Invalid rucksack size: {}", rucksack.len())
    }
    let (first, second) = rucksack.split_at(length / 2);
    (
        first.to_owned().into_iter().collect(),
        second.to_owned().into_iter().collect(),
    )
}

fn part1(input: &str) -> u32 {
    parse(input)
        .into_iter()
        .map(|rucksack| {
            let (first, second) = split_halves(rucksack);
            let common = first.intersection(&second);
            common.map(|c| priority(*c)).sum::<u32>()
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let rucksacks: Vec<HashSet<char>> = parse(input)
        .into_iter()
        .map(|rucksack| rucksack.into_iter().collect())
        .collect();

    rucksacks
        .chunks_exact(3)
        .map(|rucksacks| {
            let common = rucksacks
                .iter()
                .cloned()
                .reduce(|a, b| HashSet::from_iter(a.intersection(&b).copied()))
                .unwrap_or(HashSet::new());
            common.into_iter().map(|c| priority(c)).sum::<u32>()
        })
        .sum()
}

fn main() {
    aoc_main!(part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_priority() {
        assert_eq!(priority('p'), 16);
        assert_eq!(priority('L'), 38);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 70);
    }
}
