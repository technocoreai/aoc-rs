use std::fmt::Debug;
use utils::{aoc_main, Matrix, Point2D};

#[derive(Debug)]
struct Map {
    occupied_points: Matrix<bool, 3>,
}

fn advance(current: usize, delta: isize, size: usize) -> usize {
    match current.saturating_add_signed(delta) {
        i if i == 0 => size - 2,
        i if i == size - 1 => 1,
        i => i,
    }
}

impl Map {
    fn width(&self) -> usize {
        self.occupied_points.width()
    }

    fn height(&self) -> usize {
        self.occupied_points.height()
    }

    fn period(&self) -> usize {
        self.occupied_points.depth()
    }

    fn read(input: &str) -> Map {
        let map = Matrix::read(input);
        let period = num_integer::lcm(map.width() - 2, map.height() - 2);
        let mut blizzard_positions: Vec<(Point2D, (isize, isize))> = map
            .enumerate()
            .filter_map(|([x, y], c)| {
                let deltas: Option<(isize, isize)> = match c {
                    '>' => Some((1, 0)),
                    '<' => Some((-1, 0)),
                    'v' => Some((0, 1)),
                    '^' => Some((0, -1)),
                    _ => None,
                };
                deltas.map(|d| ([x, y], d))
            })
            .collect();

        let mut occupied_points = Matrix::new(false, [map.width(), map.height(), period]);
        for i in 0..period {
            let mut next_blizzard_positions = Vec::with_capacity(blizzard_positions.len());

            for x in 0..map.width() {
                if x != 1 {
                    occupied_points[[x, 0, i]] = true;
                }

                if x != map.width() - 2 {
                    occupied_points[[x, map.height() - 1, i]] = true;
                }
            }

            for y in 0..map.height() {
                occupied_points[[0, y, i]] = true;
                occupied_points[[map.width() - 1, y, i]] = true;
            }

            for ([x, y], (dx, dy)) in &blizzard_positions {
                occupied_points[[*x, *y, i]] = true;

                let next_x = advance(*x, *dx, map.width());
                let next_y = advance(*y, *dy, map.height());
                next_blizzard_positions.push(([next_x, next_y], (*dx, *dy)));
            }

            blizzard_positions = next_blizzard_positions;
        }
        Map { occupied_points }
    }

    fn occupied(&self, [x, y]: &Point2D, turn: usize) -> bool {
        self.occupied_points[[*x, *y, turn % self.period()]]
    }
}

fn solve(
    map: &Map,
    initial_turn: usize,
    initial_position: Point2D,
    target_position: Point2D,
) -> usize {
    let mut best_score = usize::MAX;
    let mut pending = Vec::with_capacity(1000000);
    let mut best_cell_score = Matrix::new(usize::MAX, [map.width(), map.height(), map.period()]);

    let going_down = target_position[1] > initial_position[1];

    pending.push((initial_turn, initial_position));
    while let Some((turn, current_position)) = pending.pop() {
        if turn >= best_score {
            continue;
        }

        let best_cell_score = &mut best_cell_score[[
            current_position[0],
            current_position[1],
            turn % map.period(),
        ]];
        if turn >= *best_cell_score {
            continue;
        }
        *best_cell_score = turn;

        if current_position == target_position {
            best_score = turn;
            continue;
        }

        if map.occupied(&current_position, turn) {
            continue;
        }

        let [x, y] = current_position;
        pending.push((turn + 1, current_position));
        if going_down {
            if y > 0 {
                pending.push((turn + 1, [x, y - 1]));
            }
            pending.push((turn + 1, [x - 1, y]));
            pending.push((turn + 1, [x + 1, y]));
            if y < map.height() - 1 {
                pending.push((turn + 1, [x, y + 1]));
            }
        } else {
            pending.push((turn + 1, [x + 1, y]));
            if y < map.height() - 1 {
                pending.push((turn + 1, [x, y + 1]));
            }
            if y > 0 {
                pending.push((turn + 1, [x, y - 1]));
            }
            pending.push((turn + 1, [x - 1, y]));
        }
    }
    best_score
}

fn part1(input: &str) -> usize {
    let map = Map::read(input);
    solve(&map, 0, [1, 0], [map.width() - 2, map.height() - 1])
}

fn part2(input: &str) -> usize {
    let map = Map::read(input);
    let start = [1, 0];
    let end = [map.width() - 2, map.height() - 1];
    let points = [(start, end), (end, start), (start, end)];

    points.into_iter().fold(0, |turn, (initial, target)| {
        println!("[{turn}] {initial:?} -> {target:?}");
        solve(&map, turn, initial, target)
    })
}

fn main() {
    aoc_main!(part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 54);
    }
}
