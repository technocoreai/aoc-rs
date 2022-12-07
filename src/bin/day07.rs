use crate::ParsedLine::*;
use std::collections::HashMap;
use utils::{aoc_main, parse_obj};

#[derive(Debug, PartialEq)]
struct File {
    size: usize,
}

#[derive(Debug, PartialEq)]
struct Directory {
    subdirectories: HashMap<String, Directory>,
    files: HashMap<String, File>,
    total_size: usize,
}

impl Directory {
    fn subdirectory_mut(&mut self, name: &str) -> &mut Directory {
        self.subdirectories
            .get_mut(name)
            .unwrap_or_else(|| panic!("Missing subdirectory: {}", name))
    }

    fn pretty_print(&self, name: &str, level: usize) {
        println!(
            "{}- {} (dir, size={})",
            " ".repeat(level),
            name,
            self.total_size
        );
        for (dirname, dir) in &self.subdirectories {
            dir.pretty_print(dirname, level + 1);
        }
        for (filename, file) in &self.files {
            println!(
                "{}- {} (file, size={})",
                " ".repeat(level + 1),
                filename,
                file.size
            );
        }
    }

    fn all_directories(&self) -> Vec<&Directory> {
        let mut result = vec![self];
        for subdir in self.subdirectories.values() {
            result.extend(subdir.all_directories().iter());
        }
        result
    }
}

#[derive(Debug, PartialEq)]
enum ParsedLine {
    ChangeDirCommand(String),
    LsCommand,
    DirEntry(String),
    FileEntry(String, usize),
}

fn parse_line(line: &str) -> ParsedLine {
    parse_obj("input line", line, || {
        let tokens: Vec<&str> = line.split(' ').collect();
        match tokens.as_slice() {
            ["$", "cd", dirname] => Some(ChangeDirCommand(dirname.to_string())),
            ["$", "ls"] => Some(LsCommand),
            ["dir", dirname] => Some(DirEntry(dirname.to_string())),
            [size, filename] => {
                let parsed_size = size.parse::<usize>().ok()?;
                Some(FileEntry(filename.to_string(), parsed_size))
            }
            _ => None,
        }
    })
}

fn parse_input(input: &str) -> Directory {
    let mut root = Directory {
        subdirectories: HashMap::new(),
        files: HashMap::new(),
        total_size: 0,
    };
    let mut current_path: Vec<String> = Vec::new();

    for line in input.split('\n') {
        match parse_line(line) {
            ChangeDirCommand(ref path) if path == "/" => {
                current_path.clear();
            }
            LsCommand => (),
            DirEntry(dirname) => {
                let mut current_dir = &mut root;
                for elem in &current_path {
                    current_dir = current_dir.subdirectory_mut(elem);
                }
                current_dir.subdirectories.insert(
                    dirname,
                    Directory {
                        subdirectories: HashMap::new(),
                        files: HashMap::new(),
                        total_size: 0,
                    },
                );
            }
            FileEntry(filename, size) => {
                let mut current_dir = &mut root;
                for elem in &current_path {
                    current_dir.total_size += size;
                    current_dir = current_dir.subdirectory_mut(elem);
                }
                current_dir.total_size += size;
                current_dir.files.insert(filename, File { size });
            }
            ChangeDirCommand(ref path) if path == ".." => {
                current_path.pop();
            }
            ChangeDirCommand(path) => {
                current_path.push(path);
            }
        }
    }

    root.pretty_print("/", 0);
    root
}

fn part1(input: &str) -> usize {
    let root = parse_input(input);
    root.all_directories()
        .iter()
        .map(|d| d.total_size)
        .filter(|s| s < &100000)
        .sum()
}

fn part2(input: &str) -> usize {
    let disk_size: usize = 70000000;
    let required_space: usize = 30000000;

    let root = parse_input(input);
    let free_space = disk_size - root.total_size;

    root.all_directories()
        .iter()
        .map(|d| d.total_size)
        .filter(|size| free_space + size >= required_space)
        .min()
        .unwrap()
}

fn main() {
    aoc_main!(part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("$ cd foo"),
            ChangeDirCommand(String::from("foo"))
        );
        assert_eq!(parse_line("$ ls"), LsCommand);
        assert_eq!(parse_line("dir foo"), DirEntry(String::from("foo")));
        assert_eq!(parse_line("1234 foo"), FileEntry(String::from("foo"), 1234));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 24933642);
    }
}
