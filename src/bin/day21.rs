use std::collections::BTreeMap;
use std::fmt::{Display, Formatter, Write};
use utils::{aoc_main, parse_peg};

peg::parser! {
    grammar monkey_math() for str {
        rule integer() -> i64
            = n:$(['0'..='9']+) { ? n.parse::<i64>().or(Err("i64")) }

        rule monkey_id() -> String
            = v:$(['a'..='z']*<4,4>) { v.to_string() }

        rule operation() -> Operation
            = ['+'] { Operation::Add }
            / ['-'] { Operation::Subtract }
            / ['*'] { Operation::Multiply }
            / ['/'] { Operation::Divide }

        rule expression() -> Expression
            = v:integer() { Expression::Integer(v) }
            / m1:monkey_id() " " op:operation() " " m2:monkey_id() { Expression::BinOp(op, m1, m2) }

        rule monkey() -> (String, Expression)
            = m:monkey_id() ": " expr:expression() { (m, expr) }

        pub rule input() -> Vec<(String, Expression)>
            = v:(monkey() ** "\n") { v }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    fn evaluate(&self, v1: i64, v2: i64) -> i64 {
        match self {
            Operation::Add => v1 + v2,
            Operation::Subtract => v1 - v2,
            Operation::Multiply => v1 * v2,
            Operation::Divide => v1 / v2,
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Operation::Add => '+',
            Operation::Subtract => '-',
            Operation::Multiply => '*',
            Operation::Divide => '/',
        };
        f.write_char(char)
    }
}

#[derive(Debug)]
pub enum Expression {
    Integer(i64),
    BinOp(Operation, String, String),
}

#[derive(Debug)]
pub enum SymExpression {
    Integer(i64),
    Human,
    BinOp(Operation, Box<SymExpression>, Box<SymExpression>),
}

impl Display for SymExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SymExpression::Integer(i) => write!(f, "{i}"),
            SymExpression::Human => f.write_str("humn"),
            SymExpression::BinOp(op, e1, e2) => write!(f, "({e1} {op} {e2})"),
        }
    }
}

fn parse_input(input: &str) -> BTreeMap<String, Expression> {
    parse_peg(input, monkey_math::input).into_iter().collect()
}

fn evaluate(monkeys: &BTreeMap<String, Expression>, node: &String) -> i64 {
    match monkeys.get(node) {
        Some(Expression::Integer(v)) => *v,
        Some(Expression::BinOp(op, m1, m2)) => {
            let v1 = evaluate(monkeys, m1);
            let v2 = evaluate(monkeys, m2);
            op.evaluate(v1, v2)
        }
        None => panic!("Monkey not found: {}", node),
    }
}

fn build_sym_expression(monkeys: &BTreeMap<String, Expression>, node: &String) -> SymExpression {
    if node.as_str() == "humn" {
        SymExpression::Human
    } else {
        match monkeys.get(node) {
            Some(Expression::Integer(v)) => SymExpression::Integer(*v),
            Some(Expression::BinOp(op, n1, n2)) => {
                let e1 = build_sym_expression(monkeys, n1);
                let e2 = build_sym_expression(monkeys, n2);
                match (e1, e2) {
                    (SymExpression::Integer(v1), SymExpression::Integer(v2)) => {
                        SymExpression::Integer(op.evaluate(v1, v2))
                    }
                    (a, b) => SymExpression::BinOp(*op, Box::new(a), Box::new(b)),
                }
            }
            None => panic!("Monkey not found: {}", node),
        }
    }
}

fn part1(input: &str) -> i64 {
    let input = parse_input(input);
    evaluate(&input, &"root".to_string())
}

fn solve(solved: SymExpression, other: i64) -> i64 {
    match solved {
        SymExpression::Human => other,
        SymExpression::BinOp(op, e1, e2) => match (op, *e1, *e2) {
            (Operation::Add, expression, SymExpression::Integer(value))
            | (Operation::Add, SymExpression::Integer(value), expression) => {
                solve(expression, other - value)
            }
            (Operation::Subtract, expression, SymExpression::Integer(value)) => {
                solve(expression, other + value)
            }
            (Operation::Subtract, SymExpression::Integer(value), expression) => {
                solve(expression, value - other)
            }
            (Operation::Multiply, expression, SymExpression::Integer(value))
            | (Operation::Multiply, SymExpression::Integer(value), expression) => {
                solve(expression, other / value)
            }
            (Operation::Divide, expression, SymExpression::Integer(value)) => {
                solve(expression, other * value)
            }
            (Operation::Divide, SymExpression::Integer(value), expression) => {
                solve(expression, value / other)
            }
            (op, e1, e2) => panic!("Cannot solve: {} {} {}", e1, op, e2),
        },
        _ => panic!("Cannot solve for {solved}"),
    }
}

fn part2(input: &str) -> i64 {
    let input = parse_input(input);
    if let Some(Expression::BinOp(_, m1, m2)) = input.get(&"root".to_string()) {
        let e1 = build_sym_expression(&input, m1);
        let e2 = build_sym_expression(&input, m2);
        println!("{e1} == {e2}");
        match (e1, e2) {
            (expr, SymExpression::Integer(v)) => solve(expr, v),
            (SymExpression::Integer(v), expr) => solve(expr, v),
            _ => panic!("Too complicated to solve"),
        }
    } else {
        panic!("Root node not found");
    }
}

fn main() {
    aoc_main!(part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 152);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 301);
    }
}
