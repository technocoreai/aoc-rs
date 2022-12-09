use std::collections::HashSet;
use utils::{aoc_main, parse_obj};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_move(line: &str) -> (Direction, u32) {
    parse_obj("move", line, || {
        let (direction, length) = line.split_once(' ')?;
        let parsed_direction: Direction = match direction {
            "U" => Some(Direction::Up),
            "D" => Some(Direction::Down),
            "L" => Some(Direction::Left),
            "R" => Some(Direction::Right),
            _ => None,
        }?;
        let parsed_length: u32 = length.parse().ok()?;
        Some((parsed_direction, parsed_length))
    })
}

fn move_head(position: (i32, i32), direction: Direction) -> (i32, i32) {
    let (x, y) = position;
    match direction {
        Direction::Up => (x, y + 1),
        Direction::Down => (x, y - 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
    }
}

fn move_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let (head_x, head_y) = head;
    let (tail_x, tail_y) = tail;

    let delta_x = head_x - tail_x;
    let delta_y = head_y - tail_y;

    if delta_x.abs() > 1 || delta_y.abs() > 1 {
        (tail_x + delta_x.signum(), tail_y + delta_y.signum())
    } else {
        tail
    }
}

fn simulate(input: &str, tail_count: usize) -> usize {
    let mut head: (i32, i32) = (0, 0);
    let mut tails: Vec<(i32, i32)> = std::iter::repeat(head).take(tail_count).collect();
    let mut visited: HashSet<(i32, i32)> = HashSet::from([head]);

    for line in input.lines() {
        let (direction, steps) = parse_move(line);
        for _ in 0..steps {
            head = move_head(head, direction);
            let last = tails.iter_mut().fold(head, |prev, tail| {
                let updated = move_tail(prev, *tail);
                *tail = updated;
                updated
            });
            visited.insert(last);
        }
    }

    visited.len()
}

fn part1(input: &str) -> usize {
    simulate(input, 1)
}

fn part2(input: &str) -> usize {
    simulate(input, 9)
}

fn main() {
    aoc_main!(part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    static EXAMPLE_INPUT_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT_2), 36);
    }
}
