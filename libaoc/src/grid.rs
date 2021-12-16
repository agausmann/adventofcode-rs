use std::ops::{Index, IndexMut, Mul};

pub struct Grid<T> {
    width: usize,
    values: Vec<T>,
}

impl<T> Grid<T> {
    pub fn line_chars_mapped<L, S, F>(lines: L, mut mapper: F) -> Self
    where
        L: IntoIterator<Item = S>,
        S: AsRef<str>,
        F: FnMut(char) -> T,
    {
        let mut width = None;
        let values: Vec<T> = lines
            .into_iter()
            .flat_map(|line| {
                let values: Vec<T> = line.as_ref().chars().map(&mut mapper).collect();
                if let Some(current_width) = width {
                    assert_eq!(current_width, values.len());
                } else {
                    width = Some(values.len());
                }
                values
            })
            .collect();
        let width = width.unwrap_or(0);

        assert_eq!(values.len() % width, 0);

        Self { width, values }
    }

    pub fn line_split_mapped<L, S, F>(lines: L, delim: &str, mut mapper: F) -> Self
    where
        L: IntoIterator<Item = S>,
        S: AsRef<str>,
        F: FnMut(&str) -> T,
    {
        let mut width = None;
        let values: Vec<T> = lines
            .into_iter()
            .flat_map(|line| {
                let values: Vec<T> = line.as_ref().split(delim).map(&mut mapper).collect();
                if let Some(current_width) = width {
                    assert_eq!(current_width, values.len());
                } else {
                    width = Some(values.len());
                }
                values
            })
            .collect();
        let width = width.unwrap_or(0);

        assert_eq!(values.len() % width, 0);

        Self { width, values }
    }

    pub fn cols(&self) -> usize {
        self.width
    }

    pub fn rows(&self) -> usize {
        self.values.len() / self.width
    }

    pub fn dimensions(&self) -> GridDimensions {
        GridDimensions {
            rows: self.rows(),
            cols: self.cols(),
        }
    }

    pub fn up(&self, c: Coordinate) -> Option<Coordinate> {
        self.dimensions().up(c)
    }

    pub fn down(&self, c: Coordinate) -> Option<Coordinate> {
        self.dimensions().down(c)
    }

    pub fn left(&self, c: Coordinate) -> Option<Coordinate> {
        self.dimensions().left(c)
    }

    pub fn right(&self, c: Coordinate) -> Option<Coordinate> {
        self.dimensions().right(c)
    }

    pub fn contains(&self, c: Coordinate) -> bool {
        self.dimensions().contains(c)
    }

    pub fn get(&self, c: Coordinate) -> Option<&T> {
        if self.contains(c) {
            Some(&self.values[c.row * self.width + c.col])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, c: Coordinate) -> Option<&mut T> {
        if self.contains(c) {
            Some(&mut self.values[c.row * self.width + c.col])
        } else {
            None
        }
    }

    pub fn neighbors_4(&self, c: Coordinate) -> impl Iterator<Item = Coordinate> {
        self.dimensions().neighbors_4(c)
    }

    pub fn neighbors_8(&self, c: Coordinate) -> impl Iterator<Item = Coordinate> {
        self.dimensions().neighbors_8(c)
    }
}

impl<T> Index<Coordinate> for Grid<T> {
    type Output = T;

    fn index(&self, index: Coordinate) -> &Self::Output {
        self.get(index).expect("index out of bounds")
    }
}

impl<T> IndexMut<Coordinate> for Grid<T> {
    fn index_mut(&mut self, index: Coordinate) -> &mut Self::Output {
        self.get_mut(index).expect("index out of bounds")
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridDimensions {
    pub rows: usize,
    pub cols: usize,
}

impl GridDimensions {
    pub fn up(&self, mut c: Coordinate) -> Option<Coordinate> {
        if c.row > 0 {
            c.row -= 1;
            Some(c)
        } else {
            None
        }
    }

    pub fn down(&self, mut c: Coordinate) -> Option<Coordinate> {
        if c.row < self.rows - 1 {
            c.row += 1;
            Some(c)
        } else {
            None
        }
    }

    pub fn left(&self, mut c: Coordinate) -> Option<Coordinate> {
        if c.col > 0 {
            c.col -= 1;
            Some(c)
        } else {
            None
        }
    }

    pub fn right(&self, mut c: Coordinate) -> Option<Coordinate> {
        if c.col < self.cols - 1 {
            c.col += 1;
            Some(c)
        } else {
            None
        }
    }

    pub fn contains(&self, c: Coordinate) -> bool {
        c.row < self.rows && c.col < self.cols
    }

    pub fn neighbors_4(&self, c: Coordinate) -> impl Iterator<Item = Coordinate> {
        [self.up(c), self.right(c), self.down(c), self.left(c)]
            .into_iter()
            .flatten()
    }

    pub fn neighbors_8(&self, c: Coordinate) -> impl Iterator<Item = Coordinate> {
        [
            self.up(c),
            self.up(c).and_then(|d| self.right(d)),
            self.right(c),
            self.right(c).and_then(|d| self.down(d)),
            self.down(c),
            self.down(c).and_then(|d| self.left(d)),
            self.left(c),
            self.left(c).and_then(|d| self.up(d)),
        ]
        .into_iter()
        .flatten()
    }
}

impl Mul<usize> for GridDimensions {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            rows: self.rows * rhs,
            cols: self.cols * rhs,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coordinate {
    pub row: usize,
    pub col: usize,
}

impl Coordinate {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn taxicab_to(self, other: Self) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}

impl From<(usize, usize)> for Coordinate {
    fn from(tup: (usize, usize)) -> Self {
        Self {
            row: tup.0,
            col: tup.1,
        }
    }
}
