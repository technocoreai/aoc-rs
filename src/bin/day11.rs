use utils::{aoc_main, parse_obj};

#[derive(Debug)]
enum Value {
    Const(i64),
    OldValue,
}

impl Value {
    fn parse(token: &str) -> Option<Value> {
        match token {
            "old" => Some(Value::OldValue),
            num => num.parse().ok().map(Value::Const),
        }
    }

    fn evaluate(&self, old_value: i64) -> i64 {
        match self {
            Value::Const(value) => *value,
            Value::OldValue => old_value,
        }
    }
}

#[derive(Debug)]
enum Operation {
    Add(Value, Value),
    Multiply(Value, Value),
}

impl Operation {
    fn parse(expr: &str) -> Option<Operation> {
        match tokens(expr).as_slice() {
            [a, "+", b] => Some(Operation::Add(Value::parse(a)?, Value::parse(b)?)),
            [a, "*", b] => Some(Operation::Multiply(Value::parse(a)?, Value::parse(b)?)),
            _ => None,
        }
    }

    fn evaluate(&self, old_value: i64) -> i64 {
        match self {
            Operation::Add(a, b) => a.evaluate(old_value) + b.evaluate(old_value),
            Operation::Multiply(a, b) => a.evaluate(old_value) * b.evaluate(old_value),
        }
    }
}

fn tokens(string: &str) -> Vec<&str> {
    string.split_whitespace().collect()
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    divide_by: i64,
    throw_if_true: usize,
    throw_if_false: usize,
}

impl Monkey {
    fn parse_items(line: &str) -> Option<Vec<i64>> {
        let (_, value) = line.split_once(": ")?;
        value
            .trim()
            .split(", ")
            .map(|v| v.parse::<i64>().ok())
            .collect()
    }

    fn parse_divide_by(line: &str) -> Option<i64> {
        line.split_whitespace()
            .last()
            .and_then(|value| value.parse().ok())
    }

    fn parse_throw(line: &str) -> Option<usize> {
        line.split_whitespace()
            .last()
            .and_then(|value| value.parse().ok())
    }

    fn parse(block: &str) -> Monkey {
        parse_obj("monkey", block, || {
            let lines: Vec<&str> = block.lines().collect();
            match lines.as_slice() {
                [_, starting_items, operation, test, if_true, if_false] => Some(Monkey {
                    items: Monkey::parse_items(starting_items)?,
                    operation: {
                        let (_, op) = operation.split_once(" = ")?;
                        Operation::parse(op)?
                    },
                    divide_by: Monkey::parse_divide_by(test)?,
                    throw_if_true: Monkey::parse_throw(if_true)?,
                    throw_if_false: Monkey::parse_throw(if_false)?,
                }),
                _ => None,
            }
        })
    }

    fn process_items(&mut self, reduce_worry_level: fn(i64) -> i64) -> Vec<(usize, i64)> {
        let result = self
            .items
            .iter()
            .map(|value| {
                let updated_value = reduce_worry_level(self.operation.evaluate(*value));
                let target_index = if updated_value % self.divide_by == 0 {
                    self.throw_if_true
                } else {
                    self.throw_if_false
                };
                (target_index, updated_value)
            })
            .collect();
        self.items.clear();
        result
    }
}

fn simulate(input: &str, rounds: usize, reduce_worry_level: fn(i64) -> i64, debug: bool) -> usize {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::parse).collect();
    let mut inspections: Vec<usize> = vec![0; monkeys.len()];
    let common_divisor: i64 = monkeys.iter().map(|v| v.divide_by).product();
    for round in 0..rounds {
        if debug {
            println!("Turn {}", round);
        }
        for idx in 0..monkeys.len() {
            let throws: Vec<(usize, i64)> = monkeys[idx].process_items(reduce_worry_level);
            inspections[idx] += throws.len();
            for (idx, value) in throws {
                monkeys[idx].items.push(value % common_divisor);
            }
        }
        if debug {
            for (idx, monkey) in monkeys.iter().enumerate() {
                println!("  - {}: {:?}", idx, monkey);
            }
            println!("Inspections: {:?}", inspections);
            println!("----");
        }
    }
    inspections.sort();
    inspections.reverse();
    inspections[0] * inspections[1]
}

fn part1(input: &str) -> usize {
    simulate(input, 20, |v| v / 3, false)
}

fn part2(input: &str) -> usize {
    simulate(input, 10000, |v| v, false)
}

fn main() {
    aoc_main!(part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 10605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 2713310158);
    }
}
