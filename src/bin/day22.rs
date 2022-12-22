use crate::Facing::*;
use crate::Transition::*;
use enum_map::{enum_map, Enum, EnumMap};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter, Write};
use utils::{aoc_main, parse_peg, Matrix, Point2D};

peg::parser! {
    grammar input_parser() for str {
        rule tile() -> char
            = c:[' ' | '.' | '#'] { c }

        rule row() -> Vec<char>
            = v:(tile()+) { v }

        rule map() -> Vec<Vec<char>>
            = v:(row() ++ "\n") { v }

        rule command() -> Command
            = ['L'] { Command::TurnLeft }
            / ['R'] { Command::TurnRight }
            / n:$(['0'..='9']+) { ? n.parse::<usize>().or(Err("usize")).map(Command::Move) }

        pub rule input() -> (Vec<Vec<char>>, Vec<Command>)
            = map:(row() ++ "\n")
              "\n\n"
              commands:(command()+) { (map, commands) }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Open,
    Wall,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Tile::Open => '.',
            Tile::Wall => '#',
        })
    }
}

#[derive(Debug)]
pub enum Command {
    TurnLeft,
    TurnRight,
    Move(usize),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Enum)]
pub enum Facing {
    Right,
    Down,
    Left,
    Up,
}

impl Facing {
    fn rotate_left(&self) -> Facing {
        match self {
            Right => Up,
            Down => Right,
            Left => Down,
            Up => Left,
        }
    }

    fn rotate_right(&self) -> Facing {
        match self {
            Right => Down,
            Down => Left,
            Left => Up,
            Up => Right,
        }
    }

    fn flip(&self) -> Facing {
        match self {
            Right => Left,
            Down => Up,
            Left => Right,
            Up => Down,
        }
    }

    fn score(&self) -> usize {
        match self {
            Right => 0,
            Down => 1,
            Left => 2,
            Up => 3,
        }
    }
}

#[derive(Debug)]
pub enum Transition {
    Warp(usize),
    CCW(usize),
    CW(usize),
    WarpFlip(usize),
    Missing(usize),
}

impl Transition {
    fn transform(
        &self,
        position: Point2D,
        facing: Facing,
        region_size: usize,
    ) -> (usize, Point2D, Facing) {
        match self {
            Warp(region) => {
                let [x, y] = position;
                let new_position = match facing {
                    Right => [0, y],
                    Down => [x, 0],
                    Left => [region_size - 1, y],
                    Up => [x, region_size - 1],
                };
                (*region, new_position, facing)
            }
            CCW(region) => {
                let [x, y] = position;
                let new_position = match facing {
                    Right => [y, region_size - 1],
                    Down => [0, region_size - x - 1],
                    Left => [y, 0],
                    Up => [region_size - 1, region_size - y - 1],
                };
                (*region, new_position, facing.rotate_left())
            }
            CW(region) => {
                let [x, y] = position;
                let new_position = match facing {
                    Right => [region_size - y - 1, 0],
                    Down => [region_size - 1, x],
                    Left => [region_size - y - 1, region_size - 1],
                    Up => [0, x],
                };
                (*region, new_position, facing.rotate_right())
            }
            WarpFlip(region) => {
                let [x, y] = position;
                let new_position = match facing {
                    Right => [region_size - 1, region_size - y - 1],
                    Down => [region_size - x - 1, region_size - 1],
                    Left => [0, region_size - y - 1],
                    Up => [region_size - x - 1, 0],
                };
                (*region, new_position, facing.flip())
            }
            Missing(source_region) => {
                panic!("No transition for {facing:?} from {source_region}");
            }
        }
    }
}

#[derive(Debug)]
struct Region {
    tiles: Matrix<Tile, 2>,
    transitions: EnumMap<Facing, Transition>,
    origin: Point2D,
}

#[derive(Debug)]
struct Map {
    regions: Vec<Region>,
    width: usize,
    height: usize,
}

impl Map {
    fn region_size(&self) -> usize {
        self.regions[0].tiles.width()
    }
}

fn parse(input: &str, region_size: usize) -> (Map, Vec<Command>) {
    let (tiles, commands) = parse_peg(input, input_parser::input);
    let mut regions = Vec::new();

    let (mut origin_x, mut origin_y) = (0, 0);
    while origin_y < tiles.len() {
        while origin_x < tiles[origin_y].len() {
            if tiles[origin_y][origin_x] != ' ' {
                let mut region_tiles = Matrix::new(Tile::Open, [region_size, region_size]);

                for ty in 0..region_size {
                    for tx in 0..region_size {
                        region_tiles[[tx, ty]] = match tiles[origin_y + ty][origin_x + tx] {
                            '.' => Tile::Open,
                            '#' => Tile::Wall,
                            _ => unimplemented!(),
                        }
                    }
                }

                regions.push(Region {
                    tiles: region_tiles,
                    transitions: enum_map! {_ => Missing(regions.len())},
                    origin: [origin_x, origin_y],
                })
            }

            origin_x += region_size;
        }
        origin_x = 0;
        origin_y += region_size;
    }

    (
        Map {
            regions,
            width: tiles[0].len(),
            height: tiles.len(),
        },
        commands,
    )
}

fn move_once(
    map: &Map,
    current_region: usize,
    position: Point2D,
    facing: Facing,
) -> (usize, Point2D, Facing) {
    match (facing, position) {
        (Right, [x, y]) if x < map.region_size() - 1 => (current_region, [x + 1, y], facing),
        (Left, [x, y]) if x > 0 => (current_region, [x - 1, y], facing),
        (Up, [x, y]) if y > 0 => (current_region, [x, y - 1], facing),
        (Down, [x, y]) if y < map.region_size() - 1 => (current_region, [x, y + 1], facing),
        _ => {
            let transition = &map.regions[current_region].transitions[facing];
            transition.transform(position, facing, map.region_size())
        }
    }
}

fn solve(input: &str, region_size: usize, fill_transitions: fn(&mut Map)) -> usize {
    let (mut map, commands) = parse(input, region_size);
    fill_transitions(&mut map);

    for (idx, region) in map.regions.iter().enumerate() {
        println!("Region {idx}, origin {:?}", region.origin);
        println!("Transitions: {:?}", region.transitions);
        println!();
    }

    let mut current_region = 0;
    let mut current_position = [0, 0];
    let mut current_facing = Right;

    for command in commands {
        match command {
            Command::TurnLeft => {
                current_facing = current_facing.rotate_left();
            }
            Command::TurnRight => {
                current_facing = current_facing.rotate_right();
            }
            Command::Move(amount) => {
                for _ in 0..amount {
                    let (next_region, next_position, next_facing) =
                        move_once(&map, current_region, current_position, current_facing);
                    if map.regions[next_region].tiles[next_position] == Tile::Open {
                        (current_region, current_position, current_facing) =
                            (next_region, next_position, next_facing)
                    } else {
                        break;
                    }
                }
            }
        }
    }

    let [x, y] = current_position;
    let [region_x, region_y] = map.regions[current_region].origin;

    println!(
        "Done: {current_region} [{region_x}, {region_y}], {current_position:?}, {current_facing:?}"
    );

    (region_y + y + 1) * 1000 + (region_x + x + 1) * 4 + current_facing.score()
}

fn fill_warp_transitions(map: &mut Map) {
    let by_origin: BTreeMap<Point2D, usize> = map
        .regions
        .iter()
        .enumerate()
        .map(|(idx, region)| (region.origin, idx))
        .collect();
    let region_size = map.region_size() as isize;

    for idx in 0..map.regions.len() {
        for facing in [Right, Down, Left, Up] {
            let (dx, dy) = match facing {
                Right => (1, 0),
                Down => (0, 1),
                Left => (-1, 0),
                Up => (0, -1),
            };
            let [mut origin_x, mut origin_y] = map.regions[idx].origin;
            loop {
                origin_x = (origin_x as isize + dx * region_size)
                    .wrapping_rem_euclid(map.width as isize) as usize;
                origin_y = (origin_y as isize + dy * region_size)
                    .wrapping_rem_euclid(map.height as isize) as usize;
                if let Some(target_region) = by_origin.get(&[origin_x, origin_y]) {
                    map.regions[idx].transitions[facing] = Warp(*target_region);
                    break;
                }
            }
        }
    }
}

fn part1(input: &str) -> usize {
    solve(input, 50, fill_warp_transitions)
}

fn part2(input: &str) -> usize {
    let fill_transitions: for<'a> fn(&'a mut Map) = |map| {
        map.regions[0].transitions = enum_map! {
            Down => Warp(2),
            Right => Warp(1),
            Up => CW(5),
            Left => WarpFlip(3),
        };
        map.regions[1].transitions = enum_map! {
            Left => Warp(0),
            Up => Warp(5),
            Right => WarpFlip(4),
            Down => CW(2),
        };
        map.regions[2].transitions = enum_map! {
            Up => Warp(0),
            Down => Warp(4),
            Left => CCW(3),
            Right => CCW(1),
        };
        map.regions[3].transitions = enum_map! {
            Right => Warp(4),
            Down => Warp(5),
            Left => WarpFlip(0),
            Up => CW(2),
        };
        map.regions[4].transitions = enum_map! {
            Up => Warp(2),
            Left => Warp(3),
            Right => WarpFlip(1),
            Down => CW(5),
        };
        map.regions[5].transitions = enum_map! {
            Up => Warp(3),
            Down => Warp(1),
            Left => CCW(0),
            Right => CCW(4),
        };
    };
    solve(input, 50, fill_transitions)
}

fn main() {
    aoc_main!(part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test_part1() {
        assert_eq!(solve(EXAMPLE_INPUT, 4, fill_warp_transitions), 6032);
    }

    #[test]
    fn test_part2() {
        let fill_transitions: for<'a> fn(&'a mut Map) = |map| {
            // I'm not doing the whole cube
            map.regions[0].transitions[Down] = Warp(3);
            map.regions[3].transitions[Right] = CW(5);
            map.regions[5].transitions[Left] = Warp(4);
            map.regions[4].transitions[Down] = WarpFlip(1);
            map.regions[1].transitions[Right] = Warp(2);
            map.regions[2].transitions[Up] = CW(0);
        };
        assert_eq!(solve(EXAMPLE_INPUT, 4, fill_transitions), 5031);
    }
}
