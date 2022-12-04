use scaffolding::aoc_main;
use std::ops::Range;

type Assignment = Range<u32>;
type AssignmentPair = (Assignment, Assignment);

fn parse_range(range: &str) -> Option<Assignment> {
    let parts = range.split_once("-")?;
    let (from_str, to_str) = parts;
    let from = from_str.parse::<u32>().ok()?;
    let to = to_str.parse::<u32>().ok()?;
    Some(Range {
        start: from,
        end: to + 1,
    })
}

fn fully_contains(a: &Assignment, b: &Assignment) -> bool {
    (a.start <= b.start && a.end >= b.end) || (b.start <= a.start && b.end >= a.end)
}

fn overlaps(a: &Assignment, b: &Assignment) -> bool {
    a.start <= b.end - 1 && b.start <= a.end - 1
}

fn parse_line(line: &str) -> Option<AssignmentPair> {
    let parts = line.split_once(",")?;
    let (first, second) = parts;
    Some((parse_range(first)?, parse_range(second)?))
}

fn parse(input: &str) -> Vec<AssignmentPair> {
    input
        .split("\n")
        .map(|line| parse_line(line).unwrap_or_else(|| panic!("Invalid line: {}", line)))
        .collect()
}

fn part1(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|(a, b)| fully_contains(a, b))
        .count()
}

fn part2(input: &str) -> usize {
    parse(input).iter().filter(|(a, b)| overlaps(a, b)).count()
}

fn main() {
    aoc_main!(part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 4);
    }
}
