use std::collections::BTreeSet;
use std::fmt::{Debug, Display, Formatter};
use std::iter;
use utils::{aoc_main, parse_obj};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Cube {
    x: usize,
    y: usize,
    z: usize,
}

impl Cube {
    fn new(x: usize, y: usize, z: usize) -> Cube {
        Cube { x, y, z }
    }

    fn x(&self) -> usize {
        self.x
    }

    fn y(&self) -> usize {
        self.y
    }

    fn z(&self) -> usize {
        self.z
    }
}

impl Debug for Cube {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.x, self.y, self.z)
    }
}

fn parse(input: &str) -> Vec<Cube> {
    input
        .lines()
        .map(|line| {
            parse_obj("cube", line, || {
                let coords: Vec<&str> = line.split(',').collect();
                match *coords.as_slice() {
                    [x, y, z] => {
                        let x_coord: usize = x.parse().ok()?;
                        let y_coord: usize = y.parse().ok()?;
                        let z_coord: usize = z.parse().ok()?;
                        Some(Cube::new(x_coord + 1, y_coord + 1, z_coord + 1))
                    }
                    _ => None,
                }
            })
        })
        .collect()
}

fn free_sides(
    cubes: &[Cube],
    coord: fn(&Cube) -> usize,
    rest: fn(&Cube) -> (usize, usize),
) -> usize {
    cubes
        .iter()
        .map(|cube| {
            let (primary_coord, other_coords) = (coord(cube), rest(cube));
            [primary_coord - 1, primary_coord + 1]
                .iter()
                .map(|c| {
                    usize::from(
                        !cubes
                            .iter()
                            .any(|other| rest(other) == other_coords && coord(other) == *c),
                    )
                })
                .sum::<usize>()
        })
        .sum()
}

fn part1(input: &str) -> usize {
    let cubes = parse(input);

    free_sides(&cubes, Cube::x, |c| (c.y, c.z))
        + free_sides(&cubes, Cube::y, |c| (c.x, c.z))
        + free_sides(&cubes, Cube::z, |c| (c.x, c.y))
}

#[derive(Debug)]
struct Matrix3D<T> {
    items: Vec<T>,
    size_x: usize,
    size_y: usize,
    size_z: usize,
}

impl<T: Clone> Matrix3D<T> {
    fn new(initial: T, size_x: usize, size_y: usize, size_z: usize) -> Matrix3D<T> {
        Matrix3D {
            items: iter::repeat(initial)
                .take(size_x * size_y * size_z)
                .collect(),
            size_x,
            size_y,
            size_z,
        }
    }

    fn idx(&self, x: usize, y: usize, z: usize) -> usize {
        if x >= self.size_x || y >= self.size_y || z >= self.size_z {
            panic!("Out of bounds: {x}:{y}:{z}")
        }

        x + y * self.size_x + z * self.size_x * self.size_y
    }

    fn at(&self, cube: &Cube) -> &T {
        self.get(cube.x, cube.y, cube.z)
    }

    fn get(&self, x: usize, y: usize, z: usize) -> &T {
        &self.items[self.idx(x, y, z)]
    }

    fn at_mut(&mut self, cube: &Cube) -> &mut T {
        self.get_mut(cube.x, cube.y, cube.z)
    }

    fn get_mut(&mut self, x: usize, y: usize, z: usize) -> &mut T {
        let idx = self.idx(x, y, z);
        &mut self.items[idx]
    }

    fn neighbours(&self, cube: &Cube) -> Vec<Cube> {
        let (x, y, z) = (cube.x, cube.y, cube.z);

        let mut result = Vec::with_capacity(6);
        if x > 0 {
            result.push(Cube::new(x - 1, y, z));
        }
        if x < self.size_x - 1 {
            result.push(Cube::new(x + 1, y, z));
        }

        if y > 0 {
            result.push(Cube::new(x, y - 1, z));
        }
        if y < self.size_y - 1 {
            result.push(Cube::new(x, y + 1, z));
        }

        if z > 0 {
            result.push(Cube::new(x, y, z - 1));
        }
        if z < self.size_z - 1 {
            result.push(Cube::new(x, y, z + 1));
        }
        result
    }
}

fn part2(input: &str) -> usize {
    let cubes = parse(input);
    let mut grid = Matrix3D::new(
        false,
        cubes.iter().map(Cube::x).max().unwrap() + 2,
        cubes.iter().map(Cube::y).max().unwrap() + 2,
        cubes.iter().map(Cube::z).max().unwrap() + 2,
    );
    for cube in cubes {
        *grid.at_mut(&cube) = true;
    }

    let mut result = 0;
    let mut visited = BTreeSet::new();
    let mut pending = BTreeSet::new();
    pending.insert(Cube::new(0, 0, 0));

    while let Some(cube) = pending.pop_first() {
        visited.insert(cube);

        for neighbour in grid.neighbours(&cube) {
            if visited.contains(&neighbour) {
                continue;
            }

            if *grid.at(&neighbour) {
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
