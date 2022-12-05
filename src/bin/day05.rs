use std::collections::VecDeque;
use std::fmt::Debug;
use utils::{aoc_main, parse_obj};

#[derive(Debug, PartialEq)]
struct MoveCommand {
    from: usize,
    to: usize,
    count: usize,
}

impl MoveCommand {
    fn move_one_by_one(&self, stacks: &mut CrateStacks) {
        for _ in 0..self.count {
            let from_stack = &mut stacks[self.from];
            let moved = from_stack
                .pop_front()
                .unwrap_or_else(|| panic!("Empty stack at {}", self.from));

            let to_stack = &mut stacks[self.to];
            to_stack.push_front(moved);
        }
    }

    fn move_batch(&self, stacks: &mut CrateStacks) {
        let from_stack = &mut stacks[self.from];
        let moved: Vec<char> = from_stack.drain(0..self.count).collect();

        let to_stack = &mut stacks[self.to];
        for c in moved.iter().rev() {
            to_stack.push_front(*c);
        }
    }
}

impl From<&str> for MoveCommand {
    fn from(s: &str) -> Self {
        let tokens: Vec<&str> = s.split(' ').collect();
        parse_obj("move command", s, || match tokens[..] {
            ["move", count, "from", from, "to", to] => Some(MoveCommand {
                from: from.parse::<usize>().ok()? - 1,
                to: to.parse::<usize>().ok()? - 1,
                count: count.parse::<usize>().ok()?,
            }),
            _ => None,
        })
    }
}

type CrateStacks = Vec<VecDeque<char>>;

#[derive(Debug)]
struct Input {
    crates: CrateStacks,
    commands: Vec<MoveCommand>,
}

fn parse_layout_line(line: &str) -> Vec<(usize, char)> {
    line.chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .enumerate()
        .flat_map(|(idx, chunk)| match chunk {
            ['[', crate_id, ']', ' '] => Some((idx, *crate_id)),
            ['[', crate_id, ']'] => Some((idx, *crate_id)),
            _ => None,
        })
        .collect()
}

fn parse_crate_layout(from: &str) -> CrateStacks {
    let layout_lines: Vec<Vec<(usize, char)>> = from.split('\n').map(parse_layout_line).collect();
    let expected_size = 1 + layout_lines
        .iter()
        .flat_map(|line| line.iter().map(|(stack_num, _)| stack_num))
        .max()
        .unwrap_or_else(|| panic!("Empty crate layout"));

    let mut result: Vec<VecDeque<char>> = std::iter::repeat(VecDeque::new())
        .take(expected_size)
        .collect();
    for items in layout_lines {
        for (stack_num, char) in items {
            let stack = &mut result[stack_num];
            stack.push_back(char);
        }
    }
    result
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let (layout, commands) = s
            .split_once("\n\n")
            .unwrap_or_else(|| panic!("Unable to split the input"));

        Input {
            crates: parse_crate_layout(layout),
            commands: commands.split('\n').map(MoveCommand::from).collect(),
        }
    }
}

fn print_stacks(stacks: &CrateStacks) {
    for (idx, stack) in stacks.iter().enumerate() {
        println!("{:>2}: {:?}", idx, stack)
    }
    println!("---")
}

fn solve(input: &str, handler: fn(&MoveCommand, &mut CrateStacks)) -> String {
    let Input {
        mut crates,
        commands,
    } = Input::from(input);
    for cmd in commands {
        println!("Running {:?}", cmd);
        print_stacks(&crates);
        handler(&cmd, &mut crates);
    }
    let topmost: Vec<String> = crates
        .into_iter()
        .flat_map(|stack| stack.get(0).map(char::to_string))
        .collect();
    topmost.join("")
}

fn part1(input: &str) -> String {
    solve(input, MoveCommand::move_one_by_one)
}

fn part2(input: &str) -> String {
    solve(input, MoveCommand::move_batch)
}

fn main() {
    aoc_main!(part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_parse_move_command() {
        assert_eq!(
            MoveCommand::from("move 3 from 1 to 2"),
            MoveCommand {
                from: 1,
                to: 2,
                count: 3,
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), "CMZ");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), "MCD");
    }
}
