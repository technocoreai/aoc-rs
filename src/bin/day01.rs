fn parse(input: &str) -> Vec<u32> {
    return input
        .trim()
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

static INPUT: &str = include_str!("input/01.txt");

fn main() {
    for (i, fun) in [part1, part2].iter().enumerate() {
        let solution = fun(INPUT);
        let part_num = i + 1;
        println!("Part {part_num}: {solution}");
    }
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
