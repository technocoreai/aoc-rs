use std::ops::Range;
use utils::{aoc_main, Matrix};

#[derive(Debug)]
struct Number {
    value: u32,
    row: usize,
    columns: Range<usize>,
}

fn find_numbers(map: &Matrix<char, 2>) -> Vec<Number> {
    let mut result: Vec<Number> = Vec::new();
    let mut current: Option<(u32, usize)> = None;

    for ([col, row], elem) in map.enumerate() {
        if col == 0 {
            if let Some((value, starting_column)) = current {
                result.push(Number {
                    value,
                    row: row - 1,
                    columns: starting_column..map.width(),
                });
            }
            current = None;
        }

        match elem {
            '0'..='9' => {
                let digit = elem.to_digit(10).unwrap();
                current = Some(if let Some((value, starting_column)) = current {
                    (value * 10 + digit, starting_column)
                } else {
                    (digit, col)
                })
            }
            _ => {
                if let Some((value, starting_column)) = current {
                    result.push(Number {
                        value,
                        row,
                        columns: starting_column..col,
                    });
                    current = None
                }
            }
        }
    }

    result
}

fn is_symbol(elem: char) -> bool {
    !matches!(elem, '0'..='9' | '.')
}

fn neighbour_range(map: &Matrix<char, 2>, range: Range<usize>) -> Range<usize> {
    let start = if range.start > 0 {
        range.start - 1
    } else {
        range.start
    };
    let end = if range.end < map.width() - 1 {
        range.end + 1
    } else {
        range.end
    };
    start..end
}

fn adjacent_to_symbol(map: &Matrix<char, 2>, num: &Number) -> bool {
    if num.columns.start > 0 && is_symbol(map[[num.columns.start - 1, num.row]]) {
        return true;
    }
    if num.columns.end < map.width() - 2 && is_symbol(map[[num.columns.end, num.row]]) {
        return true;
    }
    if num.row > 0
        && neighbour_range(map, num.columns.clone()).any(|col| is_symbol(map[[col, num.row - 1]]))
    {
        return true;
    }
    if num.row < map.height() - 2
        && neighbour_range(map, num.columns.clone()).any(|col| is_symbol(map[[col, num.row + 1]]))
    {
        return true;
    }
    false
}

fn part1(input: &str) -> u32 {
    let map = Matrix::read(input);
    find_numbers(&map)
        .into_iter()
        .filter(|num| adjacent_to_symbol(&map, num))
        .map(|num| num.value)
        .sum()
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

    static EXAMPLE_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 4361);
    }

    //#[test]
    //fn test_part2() {
    //    assert_eq!(part2(EXAMPLE_INPUT), 0);
    //}
}
