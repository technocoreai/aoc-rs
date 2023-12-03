use std::collections::HashMap;
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

fn neighbours(map: &Matrix<char, 2>, num: &Number) -> Vec<[usize; 2]> {
    let mut result = Vec::new();
    if num.columns.start > 0 {
        result.push([num.columns.start - 1, num.row]);
    }
    if num.columns.end < map.width() - 2 {
        result.push([num.columns.end, num.row]);
    }
    if num.row > 0 {
        result.extend(neighbour_range(map, num.columns.clone()).map(|col| [col, num.row - 1]));
    }
    if num.row < map.height() - 2 {
        result.extend(neighbour_range(map, num.columns.clone()).map(|col| [col, num.row + 1]));
    }
    result
}

fn part1(input: &str) -> u32 {
    let map = Matrix::read(input);
    find_numbers(&map)
        .into_iter()
        .filter(|num| neighbours(&map, num).iter().any(|c| is_symbol(map[*c])))
        .map(|num| num.value)
        .sum()
}

fn part2(input: &str) -> u32 {
    let map = Matrix::read(input);
    let mut gears: HashMap<[usize; 2], Vec<u32>> = HashMap::new();

    for num in find_numbers(&map) {
        for neighbour in neighbours(&map, &num) {
            if map[neighbour] == '*' {
                gears
                    .entry(neighbour)
                    .and_modify(|v| v.push(num.value))
                    .or_insert(vec![num.value]);
            }
        }
    }
    gears
        .values()
        .map(|v| if v.len() == 2 { v.iter().product() } else { 0 })
        .sum()
}

fn main() {
    aoc_main!(part1);
    aoc_main!(part2);
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 467835);
    }
}
