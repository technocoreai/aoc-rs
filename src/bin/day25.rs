use utils::aoc_main;

fn snafu_to_decimal(number: &str) -> usize {
    let result: isize = number
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            let value: isize = match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '-' => -1,
                '=' => -2,
                other => panic!("Invalid character: {}", other),
            };
            5isize.pow(i as u32) * value
        })
        .sum();
    result as usize
}

fn decimal_to_snafu(number: usize) -> String {
    let mut digits = Vec::new();
    let mut remaining = number;
    while remaining > 0 {
        let mut div = remaining / 5;
        let rem = remaining % 5;
        let (digit, wrap) = match rem {
            0 | 1 | 2 => (char::from_digit(rem as u32, 10).unwrap(), false),
            3 => ('=', true),
            4 => ('-', true),
            _ => unimplemented!(),
        };
        digits.push(digit);
        if wrap {
            div += 1;
        }
        remaining = div;
    }
    digits.iter().rev().collect()
}

fn part1(input: &str) -> String {
    let total = input.lines().map(snafu_to_decimal).sum();
    decimal_to_snafu(total)
}

fn main() {
    aoc_main!(part1);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn test_convert() {
        for i in 0..10000 {
            assert_eq!(snafu_to_decimal(decimal_to_snafu(i).as_str()), i);
        }
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), "2=-1=0");
    }
}
