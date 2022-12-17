extern crate core;

use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use utils::aoc_main;

#[derive(Debug)]
struct Chamber {
    rows: BTreeMap<isize, [bool; 7]>,
    truncated_height: isize,
}

impl Chamber {
    fn new() -> Chamber {
        Chamber {
            rows: BTreeMap::new(),
            truncated_height: 0,
        }
    }

    fn max_row(&self) -> isize {
        *self.rows.keys().max().unwrap_or(&self.truncated_height)
    }

    fn mark_rock(&mut self, x: isize, y: isize) {
        let row_values = self.rows.entry(y).or_insert([false; 7]);
        row_values[x as usize] = true;
    }

    fn is_rock(&self, x: isize, y: isize) -> bool {
        if !(0..=6).contains(&x) {
            return true;
        }
        if y <= self.truncated_height {
            return true;
        }

        self.rows
            .get(&y)
            .map(|row| row[x as usize])
            .unwrap_or_default()
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (idx, row) in self.rows.iter().rev().take(50) {
            write!(f, "{idx} |")?;
            for c in row.iter() {
                write!(f, "{}", if *c { "#" } else { "." })?;
            }
            writeln!(f, "|")?;
        }
        Ok(())
    }
}

const HLINE: &[(isize, isize)] = &[(0, 0), (1, 0), (2, 0), (3, 0)];
const CROSS: &[(isize, isize)] = &[(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)];
const LSHAPE: &[(isize, isize)] = &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)];
const VLINE: &[(isize, isize)] = &[(0, 0), (0, 1), (0, 2), (0, 3)];
const BLOCK: &[(isize, isize)] = &[(0, 0), (0, 1), (1, 0), (1, 1)];

const SHAPES: &[&[(isize, isize)]] = &[HLINE, CROSS, LSHAPE, VLINE, BLOCK];

#[derive(Debug)]
struct Figure {
    blocks: &'static [(isize, isize)],
    x_pos: isize,
    y_pos: isize,
}

impl Figure {
    fn new(max_row: isize, blocks: &'static [(isize, isize)]) -> Figure {
        let y_pos = *blocks.iter().map(|(_, y)| y).min().unwrap() + max_row + 4;

        Figure {
            blocks,
            x_pos: 2,
            y_pos,
        }
    }

    fn update_position(&self, delta_x: isize, delta_y: isize) -> Self {
        Figure {
            blocks: self.blocks,
            x_pos: self.x_pos + delta_x,
            y_pos: self.y_pos + delta_y,
        }
    }

    fn blocks(&self) -> impl Iterator<Item = (isize, isize)> + '_ {
        self.blocks
            .iter()
            .map(|(x, y)| (self.x_pos + *x, self.y_pos + *y))
    }

    fn position_valid(&self, chamber: &Chamber) -> bool {
        for (x, y) in self.blocks() {
            if chamber.is_rock(x, y) {
                return false;
            }
        }
        true
    }

    fn freeze(&self, chamber: &mut Chamber) {
        for (x, y) in self.blocks() {
            chamber.mark_rock(x, y);
        }
    }
}

fn parse_input(input: &str) -> Vec<isize> {
    input
        .chars()
        .map(|c| match c {
            '<' => -1,
            '>' => 1,
            _ => panic!("Invalid char: {c}"),
        })
        .collect()
}

fn drop_figure<F: FnMut() -> isize>(
    chamber: &mut Chamber,
    pattern: &'static [(isize, isize)],
    mut next_direction: F,
) {
    let mut figure = Figure::new(chamber.max_row(), pattern);
    loop {
        let direction = next_direction();
        let moved_horizontally = figure.update_position(direction, 0);
        if moved_horizontally.position_valid(chamber) {
            figure = moved_horizontally;
        }

        let moved_vertically = figure.update_position(0, -1);
        if moved_vertically.position_valid(chamber) {
            figure = moved_vertically;
        } else {
            figure.freeze(chamber);
            return;
        }
    }
}

fn truncate_index(chamber: &Chamber) -> Option<isize> {
    for (idx, row) in chamber.rows.iter().rev().take(4) {
        if row.iter().all(|v| *v) {
            return Some(*idx);
        }

        if let Some(next) = chamber.rows.get(&(*idx + 1)) {
            if row.iter().zip(next).all(|(c1, c2)| *c1 || *c2) {
                return Some(*idx);
            }
        }
    }
    None
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct TruncationEventKey {
    remaining_rows: Vec<[bool; 7]>,
    shape_idx: usize,
    direction_idx: usize,
}

fn solve(input: &str, iterations: usize) -> isize {
    let directions = parse_input(input);

    let mut i = 0;
    let mut chamber = Chamber::new();
    let mut direction_idx = 0usize;

    let mut truncations: BTreeMap<TruncationEventKey, (usize, isize)> = BTreeMap::new();

    while i < iterations {
        let shape_idx = i % SHAPES.len();

        if i % 10000 == 0 {
            println!(
                "Done: {i} ({:.1}%); {} / {} / {direction_idx} / {shape_idx}",
                (i as f64 * 100.0) / (iterations as f64),
                chamber.truncated_height,
                chamber.rows.len()
            );
        }

        let pattern = SHAPES[shape_idx];
        drop_figure(&mut chamber, pattern, || {
            let result = directions[direction_idx];
            direction_idx = (direction_idx + 1) % directions.len();
            result
        });

        if let Some(row) = truncate_index(&chamber) {
            let remaining_rows: Vec<(isize, [bool; 7])> = chamber
                .rows
                .range((row + 1)..)
                .map(|(idx, row)| (*idx, *row))
                .collect();

            let event = TruncationEventKey {
                remaining_rows: remaining_rows.iter().map(|(_, row)| *row).collect(),
                shape_idx,
                direction_idx,
            };
            println!("Truncating at {row}, iteration {i}: {event:?}");

            let (time_skip, height_skip) = match truncations.get(&event) {
                Some((previous_iteration, previous_row)) => {
                    let time_delta = i - previous_iteration;
                    let height_delta = row - previous_row;
                    let skips = (iterations - i) / time_delta;

                    let time_skip = time_delta * skips;
                    let height_skip = height_delta * (skips as isize);

                    (time_skip, height_skip)
                }
                None => (0, 0),
            };

            if time_skip > 0 {
                println!("Fast forwarding {time_skip} iterations ({height_skip} rows)");
            } else {
                truncations.insert(event, (i, row));
            }

            chamber.rows.clear();
            chamber.truncated_height = row + height_skip;
            for (idx, row) in remaining_rows {
                chamber.rows.insert(idx + height_skip, row);
            }
            i += time_skip;
        }
        i += 1
    }

    chamber.max_row()
}

fn part1(input: &str) -> isize {
    solve(input, 2022)
}

fn part2(input: &str) -> isize {
    solve(input, 1000000000000)
}

fn main() {
    aoc_main!(part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 3068);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 1514285714288);
    }
}
