extern crate core;

fn parse(input: &str) -> Vec<u32> {
    return input.split("\n").map(|s| s.parse::<u32>().unwrap()).collect();
}

fn count_increases(measurements: &[u32]) -> u32 {
    return measurements.windows(2).map(|pair| {
        match pair {
            [a, b] if b>a => 1,
            _ => 0,
        }
    }).sum();
}

fn part1(input: &str) -> u32 {
    return count_increases(parse(input).as_slice());
}

fn part2(input: &str) -> u32 {
    let windows: Vec<u32> = parse(input).as_slice().windows(3).map(|items| {
        items.iter().sum()
    }).collect();
    return count_increases(windows.as_slice());
}

static INPUT: &str = include_str!("input/01.txt");

fn main() {
    let solution1 = part1(INPUT);
    let solution2 = part2(INPUT);
    println!("Part 1: {solution1}");
    println!("Part 2: {solution2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 5);
    }
}