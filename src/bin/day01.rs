use utils::aoc_main;

fn part1(input: &str) -> u32 {
    let lines: Vec<u32> = input
        .split('\n')
        .map(|line| {
            let digits: Vec<u32> = line.chars().flat_map(|c| c.to_digit(10)).collect();
            digits.first().unwrap_or(&0) * 10 + digits.last().unwrap_or(&0)
        })
        .collect();
    lines.iter().sum()
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

    static EXAMPLE_INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 142);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(EXAMPLE_INPUT_2), 281);
    // }
}
