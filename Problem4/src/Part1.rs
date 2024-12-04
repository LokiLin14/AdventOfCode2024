use std::io::BufRead;

struct Grid {
    num_rows: usize,
    num_cols: usize,
    chars: Vec<Vec<char>>,
}

impl Grid {
    fn rows(&self) -> Vec<String> {
        self.chars.iter().map(|row| row.into_iter().collect()).collect()
    }

    fn cols(&self) -> Vec<String> {
        self.chars.iter().skip(1).fold(
            self.chars[0].iter().map(|c| c.to_string()).collect(),
            |mut acc, row| {
                acc.iter_mut().zip(row.iter()).for_each(|(a, b)| {
                    a.push(*b);
                });
                acc
            })
    }

    fn get_down_right_diagonal(grid: &Grid, mut r: usize, mut c: usize) -> String {
        let mut str: String = String::new();
        while r < grid.num_rows && c < grid.num_cols {
            str.push(grid.chars[r][c]);
            r += 1;
            c += 1;
        }
        return str;
    }
    fn get_down_left_diagonal(grid: &Grid, mut r: usize, mut c: usize) -> String {
        let mut str: String = String::new();
        while r < grid.num_rows {
            str.push(grid.chars[r][c]);
            r += 1;
            if c > 0 {
                c -= 1;
            } else {
                break;
            }
        }
        return str;
    }
    fn down_right_diagonals(&self) -> Vec<String> {
        /*
            starts ordering:
            * <- <- <-
            v
            v
            v
        */
        let mut strs = Vec::new();
        for c in (0..self.num_cols).rev() {
            strs.push(Grid::get_down_right_diagonal(self, 0, c));
        }
        for r in 1..self.num_rows {
            strs.push(Grid::get_down_right_diagonal(self, r, 0));
        }
        return strs;
    }
    fn down_left_diagonals(&self) -> Vec<String> {
        /*
            starts ordering:
            -> -> ->
                   v
                   v
                   v
        */
        let mut strs = Vec::new();
        for c in 0..self.num_cols {
            strs.push(Grid::get_down_left_diagonal(self, 0, c));
        }
        for r in 1..self.num_rows {
            strs.push(Grid::get_down_left_diagonal(self, r, self.num_cols - 1));
        }
        return strs;
    }
}


fn count_xmas(str: &str) -> usize {
    let pat = "XMAS";
    let rpat = pat.chars().rev().collect::<String>();
    str.matches(pat).count() + str.matches(&rpat).count()
}

fn read_grid() -> Grid {
    let grid = std::io::stdin().lock().lines().map_while(|line| {
        let line = line.unwrap();
        if line.len() == 0 {
            return None;
        }
        return Some(line.chars().collect::<Vec<char>>());
    }).collect::<Vec<Vec<char>>>();
    if !grid.iter().all(|row| row.len() == grid[0].len()) {
        panic!("grid rows have different length!");
    }

    Grid { num_rows: grid.len(), num_cols: grid[0].len(), chars: grid }
}
fn main() {
    let grid = read_grid();


    println!("read {} rows", grid.num_rows);
    println!("read {} cols", grid.num_cols);

    let horiz = grid.rows().iter().map(|row| count_xmas(row)).sum::<usize>();
    println!("Horizontal matches: {}", horiz);
    let drdiags = grid.down_right_diagonals().iter().map(|row| count_xmas(row)).sum::<usize>();
    println!("Down right diagonal matches: {}", drdiags);
    let dldiags = grid.down_left_diagonals().iter().map(|row| count_xmas(row)).sum::<usize>();
    println!("Down right diagonal matches: {}", dldiags);
    let vert = grid.cols().iter().map(|row| count_xmas(row)).sum::<usize>();
    println!("Vertical matches: {}", vert);
    println!("Total matches: {}", horiz + vert + drdiags + dldiags);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_xmas() {
        assert_eq!(count_xmas("XMAS"), 1);
        assert_eq!(count_xmas("SAMX"), 1);
        assert_eq!(count_xmas("SAMXMAS"), 2);
        assert_eq!(count_xmas("XMASAMXMXMASalskdjlkajsd"), 3);
    }

    #[test]
    fn test_grid() {
        let grid = Grid {
            num_rows: 3,
            num_cols: 3,
            chars: vec![
                "abc",
                "def",
                "ghi",
            ].iter().map(|r| r.chars().collect()).collect(),
        };

        assert!(grid.rows().contains(&"abc".to_string()));
        assert!(grid.rows().contains(&"def".to_string()));
        assert!(grid.rows().contains(&"ghi".to_string()));

        assert!(grid.cols().contains(&"adg".to_string()));
        assert!(grid.cols().contains(&"beh".to_string()));
        assert!(grid.cols().contains(&"cfi".to_string()));

        assert!(grid.down_right_diagonals().contains(&"c".to_string()));
        assert!(grid.down_right_diagonals().contains(&"bf".to_string()));
        assert!(grid.down_right_diagonals().contains(&"aei".to_string()));
        assert!(grid.down_right_diagonals().contains(&"dh".to_string()));
        assert!(grid.down_right_diagonals().contains(&"g".to_string()));

        println!("downleft: {:?}", grid.down_left_diagonals());

        assert!(grid.down_left_diagonals().contains(&"a".to_string()));
        assert!(grid.down_left_diagonals().contains(&"bd".to_string()));
        assert!(grid.down_left_diagonals().contains(&"ceg".to_string()));
        assert!(grid.down_left_diagonals().contains(&"fh".to_string()));
        assert!(grid.down_left_diagonals().contains(&"i".to_string()));
    }
}
