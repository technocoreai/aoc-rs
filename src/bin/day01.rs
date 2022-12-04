use scaffolding::aoc_main;

fn parse(input: &str) -> Vec<u32> {
    return input
        .split("\n\n")
        .map(|elf| elf.split("\n").map(|s| s.parse::<u32>().unwrap()).sum())
        .collect();
}

fn part1(input: &str) -> u32 {
    let elves = parse(input);
    return *elves.iter().max().unwrap_or(&0);
}

fn part2(input: &str) -> u32 {
    let mut elves = parse(input);
    elves.sort();
    return elves.iter().rev().take(3).sum();
}

fn main() {
    aoc_main!(part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 45000);
    }
}
