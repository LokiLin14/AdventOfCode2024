use std::io::BufRead;
use itertools::iproduct;

struct Grid {
    num_rows: usize,
    num_cols: usize,
    chars: Vec<Vec<char>>,
}

impl Grid {}


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

    let mut matches_count = 0;
    for r in 1..=grid.num_rows - 2 {
        for c in 1..=grid.num_cols - 2 {
            if grid.chars[r][c] != 'A' {
                continue;
            }
            let r = r as isize;
            let c = c as isize;
            let mut xmas_count = 0;
            for (dr, dc) in iproduct!([-1, 1], [-1, 1]) {
                let first = grid.chars[(r + dr) as usize][(c + dc) as usize];
                let third = grid.chars[(r - dr) as usize][(c - dc) as usize];
                if first == 'M' && third == 'S' {
                    xmas_count += 1;
                }
            }

            println!("got {} xmas at row {} col {}", xmas_count, r, c);
            if xmas_count == 2 {
                matches_count += 1;
            }
        }
    }
    println!("total matches: {}", matches_count);
}
