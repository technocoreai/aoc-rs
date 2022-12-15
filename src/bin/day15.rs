use std::collections::{HashMap, HashSet};
use utils::{aoc_main, parse_peg};

type Point = (i64, i64);

peg::parser! {
    grammar input_parser() for str {
        rule newline() = "\n";

        rule integer() -> i64
            = n:$("-"? ['0'..='9']+) { ? n.parse::<i64>().or(Err("integer")) }

        rule coords() -> Point
            = "x=" x:integer() ", y=" y:integer() { (x, y) }

        rule line() -> (Point, Point)
            = "Sensor at " sensor:coords() ": closest beacon is at " beacon:coords() {
            (sensor, beacon)
        }

        pub rule input() -> Vec<(Point, Point)>
            = v:(line() ** newline()) { v }
    }
}

#[derive(Debug, Copy, Clone)]
struct Range(i64, i64);

impl Range {
    fn start(&self) -> i64 {
        self.0
    }

    fn end(&self) -> i64 {
        self.1
    }

    fn contains(&self, point: i64) -> bool {
        point >= self.start() && point <= self.end()
    }
}

fn manhattan_distance(p1: Point, p2: Point) -> i64 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    (x2 - x1).abs() + (y2 - y1).abs()
}

fn covered_ranges(map: &[(Point, Point)], row: i64) -> Vec<Range> {
    // Collect ranges
    let mut ranges: Vec<Range> = map
        .iter()
        .filter_map(|(sensor, beacon)| {
            let (sx, sy) = sensor;
            let distance = manhattan_distance(*sensor, *beacon);

            let x_distance = distance - (row - sy).abs();
            if x_distance > 0 {
                Some(Range(sx - x_distance, sx + x_distance))
            } else {
                None
            }
        })
        .collect();

    // Collapse
    ranges.sort_by(|Range(a_start, _), Range(b_start, _)| a_start.cmp(b_start));
    if ranges.len() <= 1 {
        ranges
    } else {
        let mut collapsed = vec![];
        let mut current = ranges[0];
        for elem in ranges.iter().skip(1) {
            if elem.start() <= current.end() {
                current = Range(current.start(), elem.end().max(current.end()))
            } else {
                collapsed.push(current);
                current = *elem;
            }
        }
        collapsed.push(current);
        collapsed
    }
}

fn beacons_by_row(map: &[(Point, Point)]) -> HashMap<i64, HashSet<i64>> {
    let mut result: HashMap<i64, HashSet<i64>> = HashMap::new();
    for (_, (bx, by)) in map {
        let current = result.entry(*by).or_default();
        current.insert(*bx);
    }
    result
}

fn solve_part1(input: &str, row: i64) -> i64 {
    let map = parse_peg(input, input_parser::input);
    let all_beacons = beacons_by_row(&map);
    let ranges = covered_ranges(&map, row);

    println!("Ranges: {ranges:?}");
    println!("Beacon positions: {all_beacons:?}");

    ranges
        .iter()
        .map(|range| {
            let length = range.end() - range.start() + 1;
            let beacon_count = match all_beacons.get(&row) {
                None => 0,
                Some(beacons) => beacons.iter().filter(|bx| range.contains(**bx)).count(),
            };
            length - (beacon_count as i64)
        })
        .sum()
}

fn solve_part2(input: &str, max_coord: i64) -> i64 {
    let map = parse_peg(input, input_parser::input);
    let all_beacons = beacons_by_row(&map);
    let empty = HashSet::new();

    for row in 0..=max_coord {
        if row % 100000 == 0 {
            println!("Row: {row}");
        }
        let ranges = covered_ranges(&map, row);
        let beacons = all_beacons.get(&row).unwrap_or(&empty);

        let mut candidate_x = 0;
        for range in &ranges {
            if range.contains(candidate_x) {
                candidate_x = range.end();
            } else if beacons.contains(&candidate_x) {
                candidate_x += 1
            }
        }

        if candidate_x <= max_coord {
            candidate_x += 1;
            println!("Found at {candidate_x}:{row}");
            return (candidate_x * 4000000) + row;
        }
    }
    panic!("Unable to find the beacon");
}

fn part1(input: &str) -> i64 {
    solve_part1(input, 2000000)
}

fn part2(input: &str) -> i64 {
    solve_part2(input, 4000000)
}

fn main() {
    aoc_main!(part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(EXAMPLE_INPUT, 10), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(EXAMPLE_INPUT, 20), 56000011);
    }
}
