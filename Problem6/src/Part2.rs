use std::collections::HashSet;

mod prelude;
use prelude::*;

fn will_loop(grid : &Grid, current : (usize, usize, Direction), obs : (usize, usize)) -> bool {
    let mut oriented: HashSet<(usize, usize, Direction)> = HashSet::new();

    let mut current = grid.get_guard_position();
    loop {
        if oriented.contains(&current) {
            return true;
        }
        oriented.insert(current);

        let next_orientation;
        if let Some(temp) = move_forward(&grid, current) {
            next_orientation = temp;
        } else {
            return false;
        }

        // move forward one step
        if grid.room[next_orientation.0][next_orientation.1] != '#' &&
            (next_orientation.0 != obs.0 || next_orientation.1 != obs.1) {
            current = next_orientation;
        } else {
            current = turn_clockwise(current);
        }
    }
}

fn main() {
    let grid = Grid::read_from_stdin();
    println!("Grid: {:?}", grid);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut obstructions: HashSet<(usize, usize)> = HashSet::new();
    let mut oriented: HashSet<(usize, usize, Direction)> = HashSet::new();

    let mut current = grid.get_guard_position();
    while !oriented.contains(&current) {
        println!("Guard now at: {:?}", current);
        oriented.insert(current);
        visited.insert((current.0, current.1));

        let next_orientation;
        if let Some(temp) = move_forward(&grid, current) {
            next_orientation = temp;
        } else {
            break;
        }

        // move forward one step
        if grid.room[next_orientation.0][next_orientation.1] != '#' {
            if will_loop(&grid, current, (next_orientation.0, next_orientation.1)) {
                obstructions.insert((next_orientation.0, next_orientation.1));
            }
            current = next_orientation;
        } else {
            current = turn_clockwise(current);
        }
    }
    println!("Obstructions: {:?}", obstructions);
    println!("Total obstructions: {}", obstructions.len());
    println!("Visited total: {}", visited.len());
}

fn move_forward(grid: &Grid, (r, c, d): (usize, usize, Direction)) -> Option<(usize, usize, Direction)> {
    return match d {
        Direction::UP => {
            if r == 0 {
                return None;
            }
            Some((r - 1, c, d))
        }
        Direction::DOWN => {
            if r == grid.rows - 1 {
                return None;
            }

            Some((r + 1, c, d))
        }
        Direction::LEFT => {
            if c == 0 {
                return None;
            }
            Some((r, c - 1, d))
        }
        Direction::RIGHT => {
            if c == grid.cols - 1 {
                return None;
            }
            Some((r, c + 1, d))
        }
    }
}

fn turn_clockwise((r, c, d): (usize, usize, Direction)) -> (usize, usize, Direction) {
    (r, c, Direction::turn_clockwise(d))
}