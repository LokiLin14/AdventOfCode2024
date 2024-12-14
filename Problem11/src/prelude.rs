use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use itertools::Itertools;

pub trait UnwrapOrPanic<T> {
    fn unwrap_or_panic(self) -> T;
}
impl<T, E: Debug> UnwrapOrPanic<T> for Result<T, E> {
    fn unwrap_or_panic(self) -> T {
        self.unwrap_or_else(|e| panic!("{:?}", e))
    }
}

#[derive(Debug, Clone)]
pub struct CharGrid {
    pub rows : usize,
    pub columns : usize,
    chars : Vec<Vec<char>>
}

impl CharGrid {
    pub fn new(width: usize, height: usize, default_char: char) -> Self {
        CharGrid {
            rows: width,
            columns: height,
            chars: vec![vec![default_char; width]; height],
        }
    }

    pub fn from(str : &str) -> CharGrid {
        // transform input to a grid
        let grid : Vec<Vec<char>> = str.split('\n')
            .map(|s| s.chars().collect()).collect();

        // asserts that the grid is rectangle shaped
        let num_rows = grid.len();
        let num_columns = if !grid.is_empty() { grid[0].len() } else { 0 };
        grid.iter().for_each(|row| {assert_eq!(row.len(), num_columns);});

        CharGrid {rows : num_rows, columns : num_columns, chars : grid}
    }

    pub fn in_grid_usize(&self, index : (usize, usize)) -> bool  {
        index.0 < self.rows && index.1 < self.columns
    }

    pub fn in_grid_i32(&self, index : (i32, i32)) -> bool  {
        index.0 >= 0 && index.0 < self.rows as i32 && index.1 >= 0 && index.1 < self.columns as i32
    }

    pub fn get(&self, index : (usize, usize)) -> char {
        if !self.in_grid_usize(index) {
            panic!("Index out of bounds");
        }
        self.chars[index.0][index.1]
    }

    pub fn adjacent_indexes(&self, index : (usize, usize)) -> Vec<(usize, usize)> {
        let mut vec : Vec<(usize, usize)> = Vec::new();
        if index.0 > 0 {
            vec.push((index.0 - 1, index.1));
        }
        if index.1 > 0 {
            vec.push((index.0, index.1 - 1));
        }
        if index.0 + 1 < self.rows {
            vec.push((index.0 + 1, index.1));
        }
        if index.1 + 1 < self.columns {
            vec.push((index.0, index.1 + 1));
        }
        vec
    }

    pub fn area(&self) -> usize {
        self.rows * self.columns
    }

    pub fn index_to_usize(&self, (row_index, col_index) : (usize, usize)) -> usize {
        row_index * self.columns + col_index
    }

    pub fn indexes(&self) -> impl Iterator<Item=(usize, usize)> {
        (0..self.rows).cartesian_product(0..self.columns)
    }
}
pub fn get_input(part: i32, is_example: bool) -> String {
    let filename = format!("part{part}-{}", { if is_example { "example" } else { "test" } });
    get_input_by_path(&Path::new(".").join("data").join(filename))
}

pub fn get_input_by_path(path: &Path) -> String {
    let file = File::open(path).unwrap_or_panic();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap_or_panic();
    contents
}

fn arithmetic_sum(start : i64, length : i64) -> i64 {
    length * (start + start + (length - 1)) / 2
}