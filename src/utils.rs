#[macro_export]
macro_rules! aoc_main {
    ($p1:ident) => {
        let filename = format!("input/{}.txt", module_path!().replace("day", ""));
        let contents = std::fs::read_to_string(filename).unwrap();
        let input = contents.as_str().trim_end();

        let solution1 = $p1(input);
        println!("Part 1: {solution1}");
    };

    ($p1:ident, $p2:ident) => {
        let filename = format!("input/{}.txt", module_path!().replace("day", ""));
        let contents = std::fs::read_to_string(filename).unwrap();
        let input = contents.as_str().trim_end();

        let solution1 = $p1(input);
        println!("Part 1: {solution1}");
        let solution2 = $p2(input);
        println!("Part 2: {solution2}");
    };
}

pub fn parse<T, F: FnOnce() -> Option<T>>(error_msg: &str, parse_fn: F) -> T {
    parse_fn().unwrap_or_else(|| panic!("{}", error_msg))
}

pub fn parse_obj<T, F: FnOnce() -> Option<T>>(kind: &str, value: &str, parse_fn: F) -> T {
    let error_msg = format!("Invalid {}: {}", kind, value);
    parse(error_msg.as_str(), parse_fn)
}

pub fn parse_peg<T>(
    input: &str,
    parse_fn: fn(&str) -> Result<T, peg::error::ParseError<peg::str::LineCol>>,
) -> T {
    match parse_fn(input) {
        Ok(result) => result,
        Err(error) => {
            let mut context_msg = String::new();
            let context_start = error.location.line.saturating_sub(3);

            for (idx, line) in input
                .lines()
                .enumerate()
                .skip(context_start)
                .take(error.location.line + 3 - context_start)
            {
                context_msg.push_str(format!(" | {}\n", line).as_str());
                if idx == error.location.line - 1 {
                    context_msg
                        .push_str(format!(" | {:>1$}\n", "^", error.location.column).as_str());
                }
            }
            panic!("Error while parsing: expected {}\n\n{}", error, context_msg)
        }
    }
}

#[derive(Debug)]
pub struct Matrix<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

fn step(pos: usize, max: usize, delta: isize) -> Option<usize> {
    match delta.cmp(&0) {
        std::cmp::Ordering::Equal => Some(pos),
        std::cmp::Ordering::Less => pos.checked_sub(delta.wrapping_abs() as usize),
        std::cmp::Ordering::Greater => pos.checked_add(delta as usize).filter(|v| *v <= max),
    }
}

impl<T: Clone> Matrix<T> {
    pub fn empty() -> Matrix<T> {
        Matrix {
            data: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn fill(elem: T, width: usize, height: usize) -> Matrix<T> {
        Matrix {
            data: std::iter::repeat(elem).take(width * height).collect(),
            width,
            height,
        }
    }

    pub fn add_row(&mut self, row: &[T]) {
        if self.width == 0 {
            self.width = row.len()
        } else if self.width != row.len() {
            panic!(
                "Row size mismatch: got {} but expected {}",
                row.len(),
                self.width
            )
        }
        self.data.extend_from_slice(row);
        self.height += 1;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn row(&self, idx: usize) -> &[T] {
        &self.data[self.width * idx..self.width * (idx + 1)]
    }

    pub fn column(&self, idx: usize) -> Vec<&T> {
        (0..self.height)
            .map(|i| &self.data[idx + i * (self.width)])
            .collect()
    }

    pub fn elements(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.data.iter().enumerate().map(|(idx, elem)| {
            let x = idx % self.width;
            let y = idx / self.width;
            (x, y, elem)
        })
    }

    pub fn elem(&self, x: usize, y: usize) -> &T {
        let index = y * self.width + x;
        &self.data[index]
    }

    pub fn elem_at(&self, coords: (usize, usize)) -> &T {
        let (x, y) = coords;
        self.elem(x, y)
    }

    pub fn elem_mut(&mut self, x: usize, y: usize) -> &mut T {
        let index = y * self.width + x;
        &mut self.data[index]
    }

    pub fn elem_mut_at(&mut self, coords: (usize, usize)) -> &mut T {
        let (x, y) = coords;
        self.elem_mut(x, y)
    }

    pub fn update(&mut self, x: usize, y: usize, value: T) {
        *self.elem_mut(x, y) = value
    }

    pub fn update_at(&mut self, coords: (usize, usize), value: T) {
        *self.elem_mut_at(coords) = value
    }

    pub fn neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        if x >= self.width || y >= self.height {
            panic!("Out of bounds: ({x}, {y})")
        }

        let mut result = Vec::with_capacity(4);
        if x > 0 {
            result.push((x - 1, y));
        }
        if y > 0 {
            result.push((x, y - 1));
        }
        if x < self.width - 1 {
            result.push((x + 1, y));
        }
        if y < self.height - 1 {
            result.push((x, y + 1));
        }
        result
    }

    pub fn step(
        &self,
        x: usize,
        y: usize,
        delta_x: isize,
        delta_y: isize,
    ) -> Option<(usize, usize)> {
        let new_col = step(x, self.width - 1, delta_x)?;
        let new_row = step(y, self.height - 1, delta_y)?;
        Some((new_col, new_row))
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let elem = &self.data[row * self.width + col];
                elem.fmt(f)?;
            }
            if row < self.height - 1 {
                writeln!(f)?
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_matrix() {
        let mut matrix: Matrix<u32> = Matrix::empty();
        matrix.add_row(&[1, 2, 3]);
        assert_eq!(matrix.width, 3);
        assert_eq!(matrix.height, 1);
        assert_eq!(matrix.elem(1, 0), &2);

        matrix.add_row(&[21, 122, 6]);
        assert_eq!(matrix.width, 3);
        assert_eq!(matrix.height, 2);
        assert_eq!(matrix.elem(1, 1), &122);
        *matrix.elem_mut(0, 0) = 5;
        let expected_str = vec!["   5   2   3", "  21 122   6"].join("\n");
        assert_eq!(format!("{:>4}", matrix), expected_str);

        assert_eq!([21, 122, 6], matrix.row(1));
        assert_eq!(vec![&2, &122], matrix.column(1));

        let expected_elements: Vec<(usize, usize, &u32)> = vec![
            (0, 0, &5),
            (1, 0, &2),
            (2, 0, &3),
            (0, 1, &21),
            (1, 1, &122),
            (2, 1, &6),
        ];
        let elements: Vec<(usize, usize, &u32)> = matrix.elements().collect();
        assert_eq!(elements, expected_elements);
    }

    #[test]
    pub fn test_matrix_fill() {
        let matrix: Matrix<char> = Matrix::fill('x', 2, 3);
        let expected_str = vec!["xx", "xx", "xx"].join("\n");
        assert_eq!(format!("{}", matrix), expected_str);
    }

    #[test]
    pub fn test_matrix_neighbours() {
        let matrix: Matrix<i32> = Matrix::fill(1, 3, 3);
        assert_eq!(vec![(1, 0), (0, 1)], matrix.neighbours(0, 0));
        assert_eq!(
            vec![(0, 1), (1, 0), (2, 1), (1, 2)],
            matrix.neighbours(1, 1)
        );
        assert_eq!(vec![(1, 2), (2, 1)], matrix.neighbours(2, 2));
    }
}
