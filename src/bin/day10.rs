use crate::MicroInstruction::{AddX, Noop};
use utils::{aoc_main, parse_obj, Matrix};

#[derive(Debug)]
enum MicroInstruction {
    Noop,
    AddX(i64),
}

impl MicroInstruction {
    fn parse(line: &str) -> Vec<MicroInstruction> {
        parse_obj("instruction", line, || {
            let tokens: Vec<&str> = line.split(' ').collect();
            match tokens.as_slice() {
                ["noop"] => Some(vec![Noop]),
                ["addx", num] => {
                    let addend = num.parse::<i64>().ok()?;
                    Some(vec![Noop, AddX(addend)])
                }
                _ => None,
            }
        })
    }
}

fn part1(input: &str) -> i64 {
    let mut reg_x: i64 = 1;
    let mut result: i64 = 0;

    for (idx, instruction) in input.lines().flat_map(MicroInstruction::parse).enumerate() {
        let cycle = idx + 1;

        if probe(cycle) {
            result += cycle as i64 * reg_x;
        }

        reg_x = match instruction {
            Noop => reg_x,
            AddX(addend) => reg_x + addend,
        };
    }
    result
}

fn probe(cycle: usize) -> bool {
    if cycle % 20 == 0 {
        (cycle / 20) % 2 == 1
    } else {
        false
    }
}

fn part2(input: &str) -> String {
    let mut reg_x: i64 = 1;
    let mut result = Matrix::fill(' ', 40, 6);

    for (idx, instruction) in input.lines().flat_map(MicroInstruction::parse).enumerate() {
        let column = idx % result.width();
        let row = idx / result.width();
        let lit = reg_x.abs_diff(column as i64) <= 1;

        if lit {
            *result.elem_mut(column, row) = '█';
        }

        reg_x = match instruction {
            Noop => reg_x,
            AddX(addend) => reg_x + (addend as i64),
        };
    }
    format!("\n{}", result)
}

fn main() {
    aoc_main!(part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_probe() {
        let probed: Vec<usize> = (0usize..1000usize).filter(|i| probe(*i)).take(5).collect();
        assert_eq!(probed, vec![20usize, 60usize, 100usize, 140usize, 180usize])
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 13140);
    }

    #[test]
    fn test_part2() {
        let expected = vec![
            "",
            "██  ██  ██  ██  ██  ██  ██  ██  ██  ██  ",
            "███   ███   ███   ███   ███   ███   ███ ",
            "████    ████    ████    ████    ████    ",
            "█████     █████     █████     █████     ",
            "██████      ██████      ██████      ████",
            "███████       ███████       ███████     ",
        ]
        .join("\n");
        assert_eq!(part2(EXAMPLE_INPUT), expected);
    }
}
