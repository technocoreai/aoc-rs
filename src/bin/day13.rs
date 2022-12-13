use std::cmp::{max, Ordering};
use utils::{aoc_main, parse_peg};

peg::parser! {
    grammar value_parser() for str {
        rule newline() = "\n"

        rule integer() -> Value
            = n:$(['0'..='9']+) { ? n.parse::<i64>().map(Value::Integer).or(Err("integer")) }

        rule list() -> Value
            = "[" v:(value() ** ",") "]" { Value::List(v) }

        rule value() -> Value
            = integer()
            / list()

        rule pair() -> (Value, Value)
            = first:value() newline() second:value() { (first, second) }

        pub rule input() -> Vec<(Value, Value)>
            = v:(pair() ** (newline()+)) { v }

    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Value {
    Integer(i64),
    List(Vec<Value>),
}

impl Value {
    fn singleton_list(v: Value) -> Value {
        Value::List(vec![v])
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn compare_lists(a: &Vec<Value>, b: &Vec<Value>) -> Ordering {
    (0..max(a.len(), b.len()))
        .map(|i| match (a.get(i), b.get(i)) {
            (Some(va), Some(vb)) => va.cmp(vb),
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            _ => Ordering::Equal,
        })
        .find(|v| *v != Ordering::Equal)
        .unwrap_or(Ordering::Equal)
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a.cmp(b),
            (Value::List(a), Value::List(b)) => compare_lists(a, b),
            (Value::List(a), Value::Integer(i)) => compare_lists(a, &vec![Value::Integer(*i)]),
            (Value::Integer(i), Value::List(b)) => compare_lists(&vec![Value::Integer(*i)], b),
        }
    }
}

fn parse(input: &str) -> Vec<(Value, Value)> {
    parse_peg(input, value_parser::input)
}

fn part1(input: &str) -> usize {
    parse(input)
        .iter()
        .enumerate()
        .map(|(idx, (a, b))| if a <= b { idx + 1 } else { 0 })
        .sum()
}

fn part2(input: &str) -> usize {
    let divider1 = Value::singleton_list(Value::singleton_list(Value::Integer(2)));
    let divider2 = Value::singleton_list(Value::singleton_list(Value::Integer(6)));

    let source_packets = parse(input);
    let mut all_packets: Vec<&Value> = source_packets
        .iter()
        .flat_map(|(a, b)| vec![a, b])
        .chain(vec![&divider1, &divider2])
        .collect();

    all_packets.sort();
    all_packets
        .into_iter()
        .enumerate()
        .map(|(idx, packet)| {
            if *packet == divider1 || *packet == divider2 {
                idx + 1
            } else {
                1
            }
        })
        .product()
}

fn main() {
    aoc_main!(part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 140);
    }
}
