use std::io::Read;

#[derive(Debug)]
pub struct Grid {
    pub rows : usize,
    pub cols : usize,
    pub room : Vec<Vec<char>>
}
#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

use Direction::*;
impl Direction {
    pub fn turn_clockwise(direction: Direction) -> Direction {
        match direction {
            UP => {RIGHT}
            DOWN => {LEFT}
            LEFT => {UP}
            RIGHT => {DOWN}
        }
    }
}


impl Grid {
    pub fn read_from_stdin() -> Grid {
        let mut filechars = Default::default();
        std::io::stdin().lock().read_to_string(&mut filechars).expect("Can't read from stdin");

        let room : Vec<Vec<char>> = filechars.trim()
            .split('\n')
            .map(|line| line.trim().chars().collect())
            .collect();

        if room.is_empty() {
            panic!("Room is empty!")
        }
        if !room.iter().all(|line| line.len() == room[0].len()) {
            panic!("Rows do not have same length!");
        }

        Grid { rows : room.len(), cols : room[0].len(), room }
    }

    pub fn get_guard_position(&self) -> (usize, usize, Direction) {
        for r in 0..self.cols {
            for c in 0..self.rows {
                if self.room[r][c] == '^' {
                    return (r, c, Direction::UP);
                }
            }
        }
        panic!("No guard found!")
    }
}

