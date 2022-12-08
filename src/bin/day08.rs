use std::cmp::max;
use utils::{aoc_main, parse_obj, Coord, Matrix};

fn parse(input: &str) -> Matrix<u32> {
    let mut result: Matrix<u32> = Matrix::empty();
    for line in input.lines() {
        let heights: Vec<u32> = parse_obj("line", line, || {
            line.chars()
                .map(|c| c.to_digit(10))
                .collect::<Option<Vec<u32>>>()
        });
        result.add_row(&heights);
    }
    result
}

fn mark_visible(
    grid: &Matrix<u32>,
    visibility_grid: &mut Matrix<bool>,
    initial: Coord,
    delta_x: isize,
    delta_y: isize,
) {
    let mut max_height: i64 = -1;
    let mut coord = initial;
    loop {
        let new_height = *grid.elem_at(&coord) as i64;
        if new_height > max_height {
            *visibility_grid.elem_mut_at(&coord) = true;
        }
        max_height = max(new_height, max_height);

        if let Some(next_coord) = grid.step(&coord, delta_x, delta_y) {
            coord = next_coord
        } else {
            return;
        }
    }
}

fn part1(input: &str) -> usize {
    let grid = parse(input);
    let mut visibility_grid = Matrix::fill(false, grid.width(), grid.height());

    // From the left
    for row in 0..grid.height() {
        mark_visible(&grid, &mut visibility_grid, Coord(0, row), 1, 0);
    }

    // From the right
    for row in 0..grid.height() {
        mark_visible(
            &grid,
            &mut visibility_grid,
            Coord(grid.width() - 1, row),
            -1,
            0,
        );
    }

    // From the top
    for col in 0..grid.width() {
        mark_visible(&grid, &mut visibility_grid, Coord(col, 0), 0, 1);
    }

    // From the bottom
    for col in 0..grid.width() {
        mark_visible(
            &grid,
            &mut visibility_grid,
            Coord(col, grid.height() - 1),
            0,
            -1,
        );
    }

    visibility_grid
        .elements()
        .filter(|(_, visible)| **visible)
        .count()
}

fn visible_trees(grid: &Matrix<u32>, coord: &Coord, delta_x: isize, delta_y: isize) -> u32 {
    let mut current_position = coord.clone();
    let initial_height = *grid.elem_at(&current_position);
    let mut visible_trees = 0;
    while let Some(next_position) = grid.step(&current_position, delta_x, delta_y) {
        visible_trees += 1;
        current_position = next_position;
        if *grid.elem_at(&current_position) >= initial_height {
            break;
        }
    }
    visible_trees
}

fn scenic_score(grid: &Matrix<u32>, coord: Coord) -> u32 {
    let up = visible_trees(grid, &coord, 0, -1);
    let down = visible_trees(grid, &coord, 0, 1);
    let left = visible_trees(grid, &coord, 1, 0);
    let right = visible_trees(grid, &coord, -1, 0);
    up * down * left * right
}

fn part2(input: &str) -> u32 {
    let grid = parse(input);
    grid.elements()
        .map(|(pos, _)| scenic_score(&grid, pos))
        .max()
        .unwrap()
}

fn main() {
    aoc_main!(part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 21);
    }

    #[test]
    fn test_part2() {
        let grid = parse(EXAMPLE_INPUT);
        assert_eq!(scenic_score(&grid, Coord(2, 1)), 4);
        assert_eq!(scenic_score(&grid, Coord(2, 3)), 8);
    }
}
