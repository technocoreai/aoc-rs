use regex::Regex;
use utils::aoc_main;

struct Matchers(Regex, Regex);

fn build_matchers(re: &str) -> Matchers {
    let first = Regex::new(format!("^.*?({})", re).as_str()).unwrap();
    let last = Regex::new(format!(".*({}).*?$", re).as_str()).unwrap();
    Matchers(first, last)
}

fn parse_digit(token: &str) -> Option<u32> {
    match token {
        "1" | "one" => Some(1),
        "2" | "two" => Some(2),
        "3" | "three" => Some(3),
        "4" | "four" => Some(4),
        "5" | "five" => Some(5),
        "6" | "six" => Some(6),
        "7" | "seven" => Some(7),
        "8" | "eight" => Some(8),
        "9" | "nine" => Some(9),
        _ => None,
    }
}

fn line_value(line: &str, Matchers(first, last): &Matchers) -> Option<u32> {
    let first = first.captures(line).and_then(|c| parse_digit(&c[1]));
    let last = last.captures(line).and_then(|c| parse_digit(&c[1]));
    Some(first? * 10 + last?)
}

fn solve(input: &str, matchers: &Matchers) -> u32 {
    input
        .split('\n')
        .map(|line| {
            line_value(line, matchers).unwrap_or_else(|| panic!("No digits found in {}", line))
        })
        .sum()
}

fn part1(input: &str) -> u32 {
    solve(input, &build_matchers(r"\d"))
}

fn part2(input: &str) -> u32 {
    solve(
        input,
        &build_matchers(r"\d|one|two|three|four|five|six|seven|eight|nine"),
    )
}

fn main() {
    aoc_main!(part1);
    aoc_main!(part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    static EXAMPLE_INPUT_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 142);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT_2), 281);
    }
}
