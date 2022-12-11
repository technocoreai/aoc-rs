extern crate peg;
use utils::aoc_main;

peg::parser! {
    grammar monkey_parser() for str {
        rule ws() = [' '|'\n']*

        rule i64() -> i64
            = n:$(['0'..='9']+) { ? n.parse::<i64>().or(Err("i64")) }

        rule value() -> Value
            = "old" { Value::OldValue }
            / v:i64() { Value::Const(v) }

        rule operation_expr() -> Operation
            = a:value() ws() "*" ws() b:value() { Operation::Multiply(a, b) }
            / a:value() ws() "+" ws() b:value() { Operation::Add(a, b) }

        rule divisible_by() -> i64
            = "Test: divisible by" ws() v:i64() { v }

        rule item_list() -> Vec<i64>
            = v:(i64() ** ", ") { v }

        rule usize() -> usize
            = v:i64() { ? usize::try_from(v).or(Err("usize")) }

        pub rule monkey() -> Monkey
            = "Monkey" ws() i64() ":" ws()
              "Starting items:" ws() items:item_list() ws()
              "Operation: new =" ws() operation:operation_expr() ws()
              "Test: divisible by" ws() divide_by:i64() ws()
              "If true: throw to monkey" ws() throw_if_true:usize() ws()
              "If false: throw to monkey" ws() throw_if_false:usize() ws() {
                Monkey {
                    items,
                    operation,
                    divide_by,
                    throw_if_true,
                    throw_if_false,
                }
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Const(i64),
    OldValue,
}

impl Value {
    fn evaluate(&self, old_value: i64) -> i64 {
        match self {
            Value::Const(value) => *value,
            Value::OldValue => old_value,
        }
    }
}

#[derive(Debug)]
pub enum Operation {
    Add(Value, Value),
    Multiply(Value, Value),
}

impl Operation {
    fn evaluate(&self, old_value: i64) -> i64 {
        match self {
            Operation::Add(a, b) => a.evaluate(old_value) + b.evaluate(old_value),
            Operation::Multiply(a, b) => a.evaluate(old_value) * b.evaluate(old_value),
        }
    }
}

#[derive(Debug)]
pub struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    divide_by: i64,
    throw_if_true: usize,
    throw_if_false: usize,
}

impl Monkey {
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
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|monkey_declaration| {
            monkey_parser::monkey(monkey_declaration).unwrap_or_else(|err| {
                panic!("Unable to parse monkey: {}\n{}", err, monkey_declaration);
            })
        })
        .collect();
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
