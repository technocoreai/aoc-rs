use std::fmt::Debug;
use utils::{aoc_main, parse_obj};

fn parse_input(input: &str) -> Vec<isize> {
    input
        .lines()
        .map(|line| parse_obj("number", line, || line.parse::<isize>().ok()))
        .collect()
}

fn move_item<T: Debug>(items: &mut Vec<T>, source_index: usize, amount: isize) {
    let target_index =
        (source_index as isize + amount).rem_euclid(items.len() as isize - 1) as usize;
    let old = items.remove(source_index);
    items.insert(target_index, old)
}

fn mix(input: Vec<isize>, iterations: usize) -> Vec<isize> {
    let length = input.len();
    let mut result: Vec<(usize, isize)> = input.into_iter().enumerate().collect();
    for _ in 0..iterations {
        for i in 0..length {
            let (source_index, _) = result
                .iter()
                .enumerate()
                .find(|(_, (order, _))| *order == i)
                .unwrap();
            let (_, amount) = result[source_index];
            move_item(&mut result, source_index, amount);
        }
    }
    result.into_iter().map(|(_, elem)| elem).collect()
}

fn solve(input: &str, key: isize, iterations: usize) -> isize {
    let input = parse_input(input).iter().map(|v| v * key).collect();
    let mixed = mix(input, iterations);

    let (zero_idx, _) = mixed
        .iter()
        .enumerate()
        .find(|(_, elem)| **elem == 0)
        .unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|i| mixed[(zero_idx + i) % mixed.len()])
        .sum()
}

fn part1(input: &str) -> isize {
    solve(input, 1, 1)
}

fn part2(input: &str) -> isize {
    solve(input, 811589153, 10)
}

fn main() {
    aoc_main!(part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 1623178306);
    }
}
