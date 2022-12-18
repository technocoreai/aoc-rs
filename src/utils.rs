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

pub type Point2D = [usize; 2];
pub type Point3D = [usize; 3];

#[derive(Debug)]
pub struct Matrix<T, const DIMS: usize> {
    data: Vec<T>,
    dimensions: [usize; DIMS],
}

impl<T: Clone, const DIMS: usize> Matrix<T, DIMS> {
    pub fn new(initial: T, dimensions: [usize; DIMS]) -> Matrix<T, DIMS> {
        Matrix {
            data: std::iter::repeat(initial)
                .take(dimensions.iter().product())
                .collect(),
            dimensions,
        }
    }
}

impl<T, const DIMS: usize> Matrix<T, DIMS> {
    pub fn size(&self, idx: usize) -> usize {
        self.dimensions[idx]
    }

    pub fn in_bounds(&self, coordinates: [usize; DIMS]) -> bool {
        for idx in (0..DIMS).rev() {
            let coord = coordinates[idx];
            if coord > self.dimensions[idx] {
                return false;
            }
        }
        true
    }

    fn offset(&self, coordinates: [usize; DIMS]) -> usize {
        let mut result = 0;
        for idx in (0..DIMS).rev() {
            let coord = coordinates[idx];
            if coord > self.dimensions[idx] {
                panic!("Out of bounds: {coordinates:?}");
            }
            result *= self.dimensions[idx];
            result += coord;
        }
        result
    }

    pub fn neighbours(&self, coordinates: [usize; DIMS]) -> Vec<[usize; DIMS]> {
        let mut result = Vec::with_capacity(DIMS * 2);
        for (i, coord) in coordinates.into_iter().enumerate() {
            if coord > 0 {
                let mut n_down = coordinates;
                n_down[i] -= 1;
                result.push(n_down);
            }
            if coord < self.dimensions[i] - 1 {
                let mut n_up = coordinates;
                n_up[i] += 1;
                result.push(n_up);
            }
        }
        result
    }

    pub fn enumerate(&self) -> MatrixEnumerateIterator<T, DIMS> {
        MatrixEnumerateIterator {
            matrix: self,
            index: 0,
        }
    }
}

impl<T, const DIMS: usize> std::ops::Index<[usize; DIMS]> for Matrix<T, DIMS> {
    type Output = T;

    fn index(&self, coordinates: [usize; DIMS]) -> &Self::Output {
        &self.data[self.offset(coordinates)]
    }
}

impl<T, const DIMS: usize> std::ops::IndexMut<[usize; DIMS]> for Matrix<T, DIMS> {
    fn index_mut(&mut self, coordinates: [usize; DIMS]) -> &mut Self::Output {
        let offset = self.offset(coordinates);
        &mut self.data[offset]
    }
}

impl<T> Matrix<T, 2> {
    pub fn width(&self) -> usize {
        self.size(0)
    }

    pub fn height(&self) -> usize {
        self.size(1)
    }
}

impl<T: Clone> Matrix<T, 2> {
    pub fn empty() -> Matrix<T, 2> {
        Matrix {
            data: Vec::new(),
            dimensions: [0, 0],
        }
    }

    pub fn add_row(&mut self, row: &[T]) {
        if self.dimensions[0] == 0 {
            self.dimensions[0] = row.len()
        } else if self.dimensions[0] != row.len() {
            panic!(
                "Row size mismatch: got {} but expected {}",
                row.len(),
                self.dimensions[0]
            )
        }
        self.data.extend_from_slice(row);
        self.dimensions[1] += 1;
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Matrix<T, 2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let elem = &self[[x, y]];
                elem.fmt(f)?;
            }
            if y < self.height() - 1 {
                writeln!(f)?
            }
        }
        Ok(())
    }
}

pub struct MatrixEnumerateIterator<'a, T, const DIMS: usize> {
    matrix: &'a Matrix<T, DIMS>,
    index: usize,
}

impl<'a, T, const DIMS: usize> Iterator for MatrixEnumerateIterator<'a, T, DIMS> {
    type Item = ([usize; DIMS], &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.matrix.dimensions.iter().product() {
            None
        } else {
            let mut coordinates = [0usize; DIMS];
            (0..DIMS).fold(self.index, |v, idx| {
                coordinates[idx] = v % self.matrix.size(idx);
                v / self.matrix.size(idx)
            });
            let result = Some((coordinates, &self.matrix.data[self.index]));
            self.index += 1;
            result
        }
    }
}
