use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Display, Formatter, Write};

use utils::aoc_main;

type Point2D = (isize, isize);

#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn checked_neighbours(&self) -> [usize; 3] {
        match self {
            Direction::North => [0, 1, 2],
            Direction::South => [5, 6, 7],
            Direction::West => [0, 3, 5],
            Direction::East => [2, 4, 7],
        }
    }

    fn move_destination(&self, (x, y): Point2D) -> Point2D {
        match self {
            Direction::North => (x, y - 1),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
            Direction::East => (x + 1, y),
        }
    }
}

struct Map {
    data: BTreeMap<isize, BTreeSet<isize>>,
}

impl Map {
    fn new() -> Map {
        Map {
            data: BTreeMap::new(),
        }
    }

    fn y_range(&self) -> std::ops::Range<isize> {
        self.data
            .first_key_value()
            .zip(self.data.last_key_value())
            .map(|((first, _), (last, _))| *first..*last + 1)
            .unwrap_or(0..0)
    }

    fn x_range(&self) -> std::ops::Range<isize> {
        let mut x_min: Option<isize> = None;
        let mut x_max: Option<isize> = None;
        for row in self.data.values() {
            let row_min = row.first().copied();
            let row_max = row.last().copied();
            x_min = match (x_min, row_min) {
                (Some(a), Some(b)) => Some(a.min(b)),
                (a, None) => a,
                (_, b) => b,
            };
            x_max = match (x_max, row_max) {
                (Some(a), Some(b)) => Some(a.max(b)),
                (a, None) => a,
                (_, b) => b,
            };
        }

        x_min
            .zip(x_max)
            .map(|(first, last)| first..last + 1)
            .unwrap_or(0..0)
    }

    fn get(&self, (x, y): Point2D) -> bool {
        self.data
            .get(&y)
            .map(|row| row.contains(&x))
            .unwrap_or(false)
    }

    fn set(&mut self, (x, y): Point2D) {
        self.data
            .entry(y)
            .and_modify(|v| {
                v.insert(x);
            })
            .or_insert_with(|| std::iter::once(x).collect());
    }

    fn items(&self) -> impl Iterator<Item = Point2D> + '_ {
        self.data
            .iter()
            .flat_map(|(y, row)| row.iter().map(|x| (*x, *y)))
    }

    fn neighbours(&self, (x, y): Point2D) -> [bool; 8] {
        [
            self.get((x - 1, y - 1)),
            self.get((x, y - 1)),
            self.get((x + 1, y - 1)),
            self.get((x - 1, y)),
            self.get((x + 1, y)),
            self.get((x - 1, y + 1)),
            self.get((x, y + 1)),
            self.get((x + 1, y + 1)),
        ]
    }

    fn len(&self) -> usize {
        self.data.values().map(|row| row.len()).sum()
    }

    fn area(&self) -> usize {
        self.x_range().count() * self.y_range().count()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in self.y_range() {
            for x in self.x_range() {
                f.write_char(if self.get((x, y)) { '#' } else { '.' })?
            }
            f.write_char('\n')?
        }
        Ok(())
    }
}

fn parse(input: &str) -> Map {
    let mut result = Map::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '.' => {}
                '#' => result.set((x as isize, y as isize)),
                other => panic!("Invalid character: {}", other),
            }
        }
    }
    result
}

fn move_destination(point: Point2D, neighbours: &[bool; 8], directions: &[Direction]) -> Point2D {
    for direction in directions {
        let can_move = direction
            .checked_neighbours()
            .into_iter()
            .all(|idx| !neighbours[idx]);
        if can_move {
            return direction.move_destination(point);
        }
    }
    point
}

fn update(map: &Map, directions: &[Direction]) -> Map {
    let mut moves = BTreeMap::new();
    let mut move_destinations = BTreeMap::new();

    for elf in map.items() {
        let neighbours = map.neighbours(elf);
        let destination = if !neighbours.contains(&true) {
            elf
        } else {
            move_destination(elf, &neighbours, directions)
        };

        moves.insert(elf, destination);
        move_destinations
            .entry(destination)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    let mut result = Map::new();
    for (src, dest) in moves {
        match move_destinations.get(&dest) {
            Some(v) if *v == 1 => result.set(dest),
            _ => result.set(src),
        }
    }
    result
}

fn part1(input: &str) -> usize {
    let mut map = parse(input);
    let mut directions = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];
    for _ in 0..10 {
        map = update(&map, &directions);
        directions.rotate_left(1);
    }

    map.area() - map.len()
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

    static EXAMPLE_INPUT: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 110);
    }

    //#[test]
    //fn test_part2() {
    //    assert_eq!(part2(EXAMPLE_INPUT), 0);
    //}
}
