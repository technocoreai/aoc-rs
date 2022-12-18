use std::cmp::max;
use utils::{aoc_main, parse_obj, Matrix, Point2D};

fn parse(input: &str) -> Matrix<u32, 2> {
    let mut result: Matrix<u32, 2> = Matrix::empty();
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

fn step_one(pos: usize, max: usize, delta: isize) -> Option<usize> {
    match delta.cmp(&0) {
        std::cmp::Ordering::Equal => Some(pos),
        std::cmp::Ordering::Less => pos.checked_sub(delta.wrapping_abs() as usize),
        std::cmp::Ordering::Greater => pos.checked_add(delta as usize).filter(|v| *v <= max),
    }
}

fn step<T>(
    grid: &Matrix<T, 2>,
    x: usize,
    y: usize,
    delta_x: isize,
    delta_y: isize,
) -> Option<Point2D> {
    let new_col = step_one(x, grid.width() - 1, delta_x)?;
    let new_row = step_one(y, grid.height() - 1, delta_y)?;
    Some([new_col, new_row])
}

fn mark_visible(
    grid: &Matrix<u32, 2>,
    visibility_grid: &mut Matrix<bool, 2>,
    initial_x: usize,
    initial_y: usize,
    delta_x: isize,
    delta_y: isize,
) {
    let mut max_height: i64 = -1;
    let (mut x, mut y) = (initial_x, initial_y);
    loop {
        let new_height = grid[[x, y]] as i64;
        if new_height > max_height {
            visibility_grid[[x, y]] = true;
        }
        max_height = max(new_height, max_height);

        if let Some(next) = step(grid, x, y, delta_x, delta_y) {
            [x, y] = next;
        } else {
            return;
        }
    }
}

fn part1(input: &str) -> usize {
    let grid = parse(input);
    let mut visibility_grid = Matrix::new(false, [grid.width(), grid.height()]);

    // From the left
    for row in 0..grid.height() {
        mark_visible(&grid, &mut visibility_grid, 0, row, 1, 0);
    }

    // From the right
    for row in 0..grid.height() {
        mark_visible(&grid, &mut visibility_grid, grid.width() - 1, row, -1, 0);
    }

    // From the top
    for col in 0..grid.width() {
        mark_visible(&grid, &mut visibility_grid, col, 0, 0, 1);
    }

    // From the bottom
    for col in 0..grid.width() {
        mark_visible(&grid, &mut visibility_grid, col, grid.height() - 1, 0, -1);
    }

    visibility_grid
        .enumerate()
        .filter(|(_, visible)| **visible)
        .count()
}

fn visible_trees(grid: &Matrix<u32, 2>, x: usize, y: usize, delta_x: isize, delta_y: isize) -> u32 {
    let (mut current_x, mut current_y) = (x, y);
    let initial_height = grid[[current_x, current_y]];
    let mut visible_trees = 0;
    while let Some(next) = step(grid, current_x, current_y, delta_x, delta_y) {
        visible_trees += 1;
        [current_x, current_y] = next;
        if grid[[current_x, current_y]] >= initial_height {
            break;
        }
    }
    visible_trees
}

fn scenic_score(grid: &Matrix<u32, 2>, x: usize, y: usize) -> u32 {
    let up = visible_trees(grid, x, y, 0, -1);
    let down = visible_trees(grid, x, y, 0, 1);
    let left = visible_trees(grid, x, y, 1, 0);
    let right = visible_trees(grid, x, y, -1, 0);
    up * down * left * right
}

fn part2(input: &str) -> u32 {
    let grid = parse(input);
    grid.enumerate()
        .map(|([x, y], _)| scenic_score(&grid, x, y))
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
        assert_eq!(scenic_score(&grid, 2, 1), 4);
        assert_eq!(scenic_score(&grid, 2, 3), 8);
    }
}
