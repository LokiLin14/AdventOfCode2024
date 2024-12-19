mod prelude;

use itertools::Itertools;
use prelude::*;
use regex::Regex;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

fn main() {
    // solve_part1(&get_input(1, InputType::Debugging), (11, 7));
    // solve_part1(&get_input(1, InputType::Example), (11, 7));
    // solve_part1(&get_input(1, InputType::Test), (101, 103));
    // solve_part2(&get_input(1, InputType::Example), (11, 7));
    solve_part2(&get_input(1, InputType::Test), (101, 103));
}

#[derive(Debug, Clone)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl AsRef<Robot> for Robot {
    fn as_ref(&self) -> &Robot {
        &self
    }
}

fn parse_robot(str: &str) -> Robot {
    let robot_regex = Regex::new(r"p=(-*[0-9]+),(-*[0-9]+) v=(-*[0-9]+),(-*[0-9]+)").unwrap();

    let capture = robot_regex.captures(str).unwrap();

    let position = (capture[1].parse::<i32>().unwrap(), capture[2].parse::<i32>().unwrap());
    let velocity = (capture[3].parse::<i32>().unwrap(), capture[4].parse::<i32>().unwrap());

    Robot { position, velocity }
}

fn move_robot(&Robot { position, velocity }: &Robot, seconds: i32, grid_dimensions: (i32, i32)) -> Robot {
    let new_position = (
        (position.0 + velocity.0 * seconds).rem_euclid(grid_dimensions.0),
        (position.1 + velocity.1 * seconds).rem_euclid(grid_dimensions.1)
    );
    Robot { position: new_position, velocity }
}

fn assign_quadrant<T: AsRef<Robot>>(robot: T, grid_dimensions: (i32, i32)) -> Option<usize> {
    let &Robot { position, .. } = robot.as_ref();

    // short circuit if in middle row or column
    if (grid_dimensions.0 % 2 == 1 && position.0 == grid_dimensions.0 / 2)
        || (grid_dimensions.1 % 2 == 1 && position.1 == grid_dimensions.1 / 2) {
        return None;
    }

    // Quadrants
    // 0 1
    // 2 3
    Some(match (position.0 < grid_dimensions.0 / 2, position.1 < grid_dimensions.1 / 2) {
        (true, true) => { 0 }
        (false, true) => { 1 }
        (true, false) => { 2 }
        (false, false) => { 3 }
    })
}

const PART1_SECONDS: i32 = 100;
fn solve_part1(str: &str, grid_dimensions: (i32, i32)) {
    let robots = str.split('\n').map(parse_robot).collect::<Vec<_>>();
    let answer = calculate_safety_score(&robots, grid_dimensions, PART1_SECONDS);
    println!("Answer for part1 is {}", answer);
}

// the higher the better
fn calculate_grid_heuristic(robots: &Vec<Robot>, grid_dimensions: (i32, i32), seconds: i32) -> i32 {
    let moved_robots = robots.iter()
        .map(|r| move_robot(r, seconds, grid_dimensions))
        .collect::<Vec<_>>();

    let bots = moved_robots.iter().map(|robot| robot.position).collect::<HashSet<_>>();

    let mut score = 0;
    for &(row, col) in bots.iter() {
        if [(row + 1, col), (row, col + 1), (row + 1, col + 1)]
            .iter()
            .map(|pos| bots.contains(pos))
            .all(|b| b) {
            score += 1;
        }
    }

    score
}

fn solve_part2(str: &str, grid_dimensions: (i32, i32)) {
    let robots = str.split('\n').map(parse_robot).collect::<Vec<_>>();

    let k = 10;
    let mut k_best_times = BinaryHeap::new();
    for seconds in 0..(grid_dimensions.0 * grid_dimensions.1) {
        let score = calculate_grid_heuristic(&robots, grid_dimensions, seconds);
        k_best_times.push(Reverse((score, seconds)));
        if k_best_times.len() > k {
            k_best_times.pop();
        }
    }

    for Reverse((score, best_time)) in k_best_times.into_sorted_vec() {
        println!("{best_time} has score {score}");
        let grid_at_t = robots.iter()
            .map(|r| move_robot(r, best_time, grid_dimensions))
            .collect();
        print_grid(&grid_at_t, grid_dimensions);

        let moved_robots = robots.iter()
            .map(|r| move_robot(r, best_time, grid_dimensions))
            .collect::<Vec<_>>();
        let robot_quadrants = moved_robots.iter()
            .filter_map(|r| assign_quadrant(r, grid_dimensions)).collect_vec();
        let mut quadrant_counts = robot_quadrants.iter().counts_by(|&f| f);
        for q in 0..=3 {
            quadrant_counts.entry(q).or_insert(0);
        }

        println!("quadrant_counts: {quadrant_counts:?}");
    }
}

fn print_grid(robots: &Vec<Robot>, grid_dimensions: (i32, i32)) {
    let mut grid = vec![vec!['.'; grid_dimensions.1 as usize]; grid_dimensions.0 as usize];
    robots.iter().counts_by(|f| f.position).iter().for_each(|(&(r, c), &cnt)| {
        grid[r as usize][c as usize] = '#'
        // grid[r as usize][c as usize] = if cnt < 10 {
        //     (b'0' + cnt as u8) as char
        // } else {
        //     '#'
        // }
    });
    grid.iter().for_each(|row| { println!("{}", row.iter().collect::<String>()) });
}

fn calculate_safety_score(robots: &Vec<Robot>, grid_dimensions: (i32, i32), seconds: i32) -> i32 {
    let moved_robots = robots.iter()
        .map(|r| move_robot(r, seconds, grid_dimensions))
        .collect::<Vec<_>>();

    let robot_quadrants = moved_robots.iter()
        .filter_map(|r| assign_quadrant(r, grid_dimensions)).collect_vec();

    let mut quadrant_counts = robot_quadrants.iter().counts_by(|&f| f);
    for q in 0..=3 {
        quadrant_counts.entry(q).or_insert(0);
    }

    quadrant_counts.iter().fold(1, |acc, (_, &cnt)| acc * cnt) as i32
}