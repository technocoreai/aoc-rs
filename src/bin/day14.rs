use std::cmp::{max, min};
use std::fmt::{Display, Formatter, Write};
use utils::{aoc_main, parse_obj, Matrix};

const SAND_START_X: usize = 500;

#[derive(Debug, Copy, PartialEq, Eq, Clone)]
enum Cell {
    Air,
    Rock,
    Sand,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Cell::Air => '.',
            Cell::Rock => '#',
            Cell::Sand => 'o',
        })
    }
}

fn parse_line(line: &str) -> Vec<(usize, usize)> {
    parse_obj("line", line, || {
        line.split(" -> ")
            .map(|point| {
                point.split_once(',').and_then(|(x_str, y_str)| {
                    let x = x_str.parse::<usize>().ok()?;
                    let y = y_str.parse::<usize>().ok()?;
                    Some((x, y))
                })
            })
            .collect()
    })
}

#[derive(Debug)]
struct Cave {
    cells: Matrix<Cell>,
    source_x: usize,
    source_y: usize,
}

impl Cave {
    fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.cells.width() && y < self.cells.height()
    }
}

fn parse(input: &str, include_floor: bool) -> Cave {
    let lines: Vec<Vec<(usize, usize)>> = input.lines().map(parse_line).collect();
    let (input_min_x, input_max_x, input_max_y) = lines.iter().flatten().fold(
        (SAND_START_X, SAND_START_X, 0usize),
        |(min_x, max_x, max_y), (x, y)| (min(min_x, *x), max(max_x, *x), max(max_y, *y)),
    );

    let mut min_x = input_min_x - 1;
    let mut max_x = input_max_x + 1;
    let mut max_y = input_max_y;

    if include_floor {
        max_y += 2;
        min_x = min(min_x, SAND_START_X - max_y);
        max_x = max(max_x, SAND_START_X + max_y);
    }

    let mut cells = Matrix::fill(Cell::Air, max_x - min_x + 1, max_y + 1);
    for points in lines {
        for segment in points.windows(2) {
            if let [(x1, y1), (x2, y2)] = segment {
                let x_start = min(x1, x2) - min_x;
                let x_end = max(x1, x2) - min_x;
                let y_start = *min(y1, y2);
                let y_end = *max(y1, y2);
                if x_start == x_end {
                    for y in y_start..=y_end {
                        cells.update(x_start, y, Cell::Rock);
                    }
                } else {
                    for x in x_start..=x_end {
                        cells.update(x, y_start, Cell::Rock);
                    }
                }
            }
        }
    }

    if include_floor {
        for x in 0..cells.width() {
            cells.update(x, max_y, Cell::Rock);
        }
    }

    Cave {
        cells,
        source_x: 500 - min_x,
        source_y: 0,
    }
}

fn advance_sand(cave: &Cave, x: usize, y: usize) -> Option<(usize, usize)> {
    [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)]
        .into_iter()
        .find(|(x, y)| cave.in_bounds(*x, *y) && *cave.cells.elem(*x, *y) == Cell::Air)
}

fn drop_sand(cave: &mut Cave) -> bool {
    let (mut current_x, mut current_y) = (cave.source_x, cave.source_y);
    while let Some((x, y)) = advance_sand(cave, current_x, current_y) {
        if x == 0 || y == cave.cells.width() - 1 {
            return false;
        }
        (current_x, current_y) = (x, y)
    }
    cave.cells.update(current_x, current_y, Cell::Sand);
    true
}

fn part1(input: &str) -> u32 {
    let mut cave = parse(input, false);
    let mut result = 0;
    while drop_sand(&mut cave) {
        result += 1;
        println!("{}", cave.cells);
        println!();
    }
    result
}

fn part2(input: &str) -> u32 {
    let mut cave = parse(input, true);
    let mut result = 0;
    while *cave.cells.elem(cave.source_x, cave.source_y) == Cell::Air {
        drop_sand(&mut cave);
        result += 1;
        if cave.cells.width() < 50 {
            println!("{}", cave.cells);
            println!();
        }
    }
    result
}

fn main() {
    aoc_main!(part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 93);
    }
}
