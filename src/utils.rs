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

#[derive(Debug)]
pub struct Matrix<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Coord(pub usize, pub usize);

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

    pub fn elements(&self) -> impl Iterator<Item = (Coord, &T)> {
        self.data.iter().enumerate().map(|(idx, elem)| {
            let col = idx % self.width;
            let row = idx / self.width;
            (Coord(col, row), elem)
        })
    }

    pub fn elem(&self, col: usize, row: usize) -> &T {
        let index = row * self.width + col;
        &self.data[index]
    }

    pub fn elem_at(&self, coord: &Coord) -> &T {
        let Coord(col, row) = coord;
        self.elem(*col, *row)
    }

    pub fn elem_mut(&mut self, col: usize, row: usize) -> &mut T {
        let index = row * self.width + col;
        &mut self.data[index]
    }

    pub fn elem_mut_at(&mut self, coord: &Coord) -> &mut T {
        let Coord(col, row) = coord;
        self.elem_mut(*col, *row)
    }

    pub fn step(&self, coord: &Coord, delta_x: isize, delta_y: isize) -> Option<Coord> {
        let Coord(col, row) = coord;
        let new_col = step(*col, self.width - 1, delta_x)?;
        let new_row = step(*row, self.height - 1, delta_y)?;
        Some(Coord(new_col, new_row))
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted_elems: Vec<String> =
            self.data.iter().map(|elem| format!("{}", elem)).collect();
        let max_width = formatted_elems
            .iter()
            .map(|element| element.chars().count())
            .max()
            .unwrap_or(0);

        for row in 0..self.height {
            for col in 0..self.width {
                let elem = &formatted_elems[row * self.width + col];
                write!(f, "{:>max_width$}", elem)?;
                if col < self.width - 1 {
                    write!(f, " ")?
                }
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
        let expected_str = vec!["  5   2   3", " 21 122   6"].join("\n");
        assert_eq!(format!("{}", matrix), expected_str);

        assert_eq!([21, 122, 6], matrix.row(1));
        assert_eq!(vec![&2, &122], matrix.column(1));

        let expected_elements: Vec<(Coord, &u32)> = vec![
            (Coord(0, 0), &5),
            (Coord(1, 0), &2),
            (Coord(2, 0), &3),
            (Coord(0, 1), &21),
            (Coord(1, 1), &122),
            (Coord(2, 1), &6),
        ];
        let elements: Vec<(Coord, &u32)> = matrix.elements().collect();
        assert_eq!(elements, expected_elements);
    }

    #[test]
    pub fn test_fill() {
        let matrix: Matrix<char> = Matrix::fill('x', 2, 3);
        let expected_str = vec!["x x", "x x", "x x"].join("\n");
        assert_eq!(format!("{}", matrix), expected_str);
    }
}
