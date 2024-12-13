use std::collections::HashSet;

mod prelude;
use prelude::*;

fn main() {
    let grid = Grid::read_from_stdin();
    println!("Grid: {:?}", grid);

    let mut visited : HashSet<(usize, usize)> = HashSet::new();
    let mut oriented: HashSet<(usize, usize, Direction)> = HashSet::new();
    let mut current = grid.get_guard_position();
    while !oriented.contains(&current) {
        println!("Guard now at: {:?}", current);

        oriented.insert(current);
        visited.insert((current.0, current.1));

        let r = current.0;
        let c = current.1;
        let d = current.2;

        match current.2 {
            Direction::UP => {
                if r == 0 {
                    break;
                }
                if grid.room[r-1][c] == '#' {
                    current = (r, c, Direction::RIGHT);
                } else {
                    current = (r-1, c, d);
                }
            }
            Direction::DOWN => {
                if r == grid.rows - 1 {
                    break;
                }
                if grid.room[r + 1][c] == '#' {
                    current = (r, c, Direction::LEFT);
                } else {
                    current = (r + 1, c, d);
                }
            }
            Direction::LEFT => {
                if c == 0 {
                    break;
                }
                if grid.room[r][c-1] == '#' {
                   current = (r, c, Direction::UP);
                } else {
                    current = (r, c-1, d);
                }
            }
            Direction::RIGHT => {
                if c == grid.cols - 1 {
                    break;
                }
                if grid.room[r][c+1] == '#' {
                    current = (r, c, Direction::DOWN);
                } else {
                    current = (r, c+1, d);
                }
            }
        }
    }
    println!("Visited total: {}", visited.len());
}
