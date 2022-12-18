use std::collections::BTreeSet;
use utils::{aoc_main, parse_obj, Matrix, Point3D};

fn parse(input: &str) -> Vec<Point3D> {
    input
        .lines()
        .map(|line| {
            parse_obj("location", line, || {
                let coords: Vec<&str> = line.split(',').collect();
                match *coords.as_slice() {
                    [x, y, z] => {
                        let x_adj: usize = x.parse::<usize>().ok()? + 1;
                        let y_adj: usize = y.parse::<usize>().ok()? + 1;
                        let z_adj: usize = z.parse::<usize>().ok()? + 1;
                        Some([x_adj, y_adj, z_adj])
                    }
                    _ => None,
                }
            })
        })
        .collect()
}

fn free_sides(
    lava_locations: &[Point3D],
    coord: fn(&Point3D) -> usize,
    rest: fn(&Point3D) -> (usize, usize),
) -> usize {
    lava_locations
        .iter()
        .map(|location| {
            let (primary_coord, other_coords) = (coord(location), rest(location));
            [primary_coord - 1, primary_coord + 1]
                .iter()
                .map(|c| {
                    usize::from(
                        !lava_locations
                            .iter()
                            .any(|other| rest(other) == other_coords && coord(other) == *c),
                    )
                })
                .sum::<usize>()
        })
        .sum()
}

fn part1(input: &str) -> usize {
    let lava_locations = parse(input);

    free_sides(&lava_locations, |c| c[0], |c| (c[1], c[2]))
        + free_sides(&lava_locations, |c| c[1], |c| (c[0], c[2]))
        + free_sides(&lava_locations, |c| c[2], |c| (c[0], c[1]))
}

fn part2(input: &str) -> usize {
    let lava_locations = parse(input);
    let mut grid = Matrix::new(
        false,
        [
            lava_locations.iter().map(|c| c[0]).max().unwrap() + 2,
            lava_locations.iter().map(|c| c[1]).max().unwrap() + 2,
            lava_locations.iter().map(|c| c[2]).max().unwrap() + 2,
        ],
    );
    for lava_location in lava_locations {
        grid[lava_location] = true;
    }

    let mut result = 0;
    let mut visited = BTreeSet::new();
    let mut pending = BTreeSet::new();
    pending.insert([0usize, 0usize, 0usize]);

    while let Some(cell) = pending.pop_first() {
        visited.insert(cell);

        for neighbour in grid.neighbours(cell) {
            if visited.contains(&neighbour) {
                continue;
            }

            if grid[neighbour] {
                result += 1;
            } else {
                pending.insert(neighbour);
            }
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

    static EXAMPLE_INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 64);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 58);
    }
}
