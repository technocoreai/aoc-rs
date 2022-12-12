use std::collections::{BTreeSet, HashMap};
use utils::{aoc_main, Matrix};

type Coord = (usize, usize);

#[derive(Debug)]
struct Heightmap {
    starting_pos: Coord,
    target_pos: Coord,
    elevations: Matrix<i64>,
}

fn parse(input: &str) -> Heightmap {
    let mut elevations = Matrix::empty();
    let mut current_pos = (0, 0);
    let mut target_pos = (0, 0);

    for (row, line) in input.lines().enumerate() {
        let elevation_row: Vec<i64> = line
            .chars()
            .enumerate()
            .map(|(col, c)| match c {
                'S' => {
                    current_pos = (col, row);
                    0
                }
                'E' => {
                    target_pos = (col, row);
                    25
                }
                'a'..='z' => (c as i64) - ('a' as i64),
                _ => panic!("Invalid character: {}", c),
            })
            .collect();
        elevations.add_row(elevation_row.as_slice());
    }

    Heightmap {
        starting_pos: current_pos,
        target_pos,
        elevations,
    }
}

fn pop(visit_queue: &mut BTreeSet<(u64, Coord)>) -> Option<(u64, Coord)> {
    let result = visit_queue.iter().next().copied();
    if let Some(elem) = result {
        visit_queue.remove(&elem);
    }
    result
}

fn fill_movement_costs(
    heightmap: &Matrix<i64>,
    starting_pos: Coord,
    can_move: fn(i64, i64) -> bool,
) -> HashMap<Coord, u64> {
    let mut best_costs: HashMap<Coord, u64> = HashMap::new();
    let mut visit_queue: BTreeSet<(u64, Coord)> = BTreeSet::new();

    best_costs.insert(starting_pos, 0);
    visit_queue.insert((0, starting_pos));

    while let Some((cost, (x, y))) = pop(&mut visit_queue) {
        let height = *heightmap.elem(x, y);
        println!("Checking {x}:{y}, height {height}, current cost {cost}");
        for (nx, ny) in heightmap.neighbours(x, y) {
            let nheight = *heightmap.elem(nx, ny);
            if !can_move(height, nheight) {
                continue;
            }

            let new_best_cost = cost + 1;
            let current_best_cost = best_costs.get(&(nx, ny)).copied();
            if let Some(current_best) = current_best_cost {
                if current_best <= new_best_cost {
                    continue;
                }
            }

            if let Some(current_best) = current_best_cost {
                visit_queue.remove(&(current_best, (nx, ny)));
            }

            println!(" - Updating cost for {nx}:{ny} to {new_best_cost}");
            best_costs.insert((nx, ny), new_best_cost);
            visit_queue.insert((new_best_cost, (nx, ny)));
        }
    }

    best_costs
}

fn part1(input: &str) -> u64 {
    let heightmap = parse(input);
    let costs = fill_movement_costs(
        &heightmap.elevations,
        heightmap.starting_pos,
        |elev, nelev| nelev <= elev + 1,
    );
    *costs.get(&heightmap.target_pos).unwrap()
}

fn part2(input: &str) -> u64 {
    let heightmap = parse(input);
    let scores = fill_movement_costs(
        &heightmap.elevations,
        heightmap.target_pos,
        |elev, nelev| elev <= nelev + 1,
    );
    *heightmap
        .elevations
        .elements()
        .flat_map(|(x, y, elev)| match *elev {
            0 => scores.get(&(x, y)),
            _ => None,
        })
        .min()
        .unwrap()
}

fn main() {
    aoc_main!(part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 29);
    }
}
