mod prelude;

use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use prelude::*;
use crate::AntinodeModel::{PART1, PART2};

fn main() {
    let problem = get_input_by_problem(1, false);
    part1(&problem);
    part2(&problem);
}

fn part1(str : &str) {
    let grid = CharGrid::from(str);
    let antinodes = calculate_distinct_antinodes(&grid, PART1);

    println!("There are {} antinodes by part1's model.", antinodes.len());
}

fn part2(str : &str) {
    let grid = CharGrid::from(str);
    let antinodes = calculate_distinct_antinodes(&grid, PART2);
    println!("There are {} antinodes by part2's model.", antinodes.len());
}

fn pretty_format_antinodes(rows : usize, cols : usize, antinodes : &HashSet<(usize, usize)>) -> String {
    let mut grid = vec![vec!['.'; cols]; cols];
    for &(row_index, col_index) in antinodes {
        grid[row_index][col_index] = '#';
    }
    grid.iter().map(|row| row.iter().collect::<String>()).join("\n")
}

enum AntinodeModel {
    PART1,
    PART2,
}
fn calculate_distinct_antinodes(grid : &CharGrid, model : AntinodeModel) -> HashSet<(usize, usize)> {
    // Organizes positions of each tower
    let mut towers : HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for index  in (0..grid.rows).cartesian_product(0..grid.columns) {
        let current_char = grid.get(index);
        if current_char != '.' {
            towers.entry(current_char).or_insert(Vec::new()).push(index);
        }
    }

    // Loops through all anti nodes
    let mut antinodes = HashSet::new();
    for (_, indexes) in towers {
        for i in 0..indexes.len() {
            for j in i + 1..indexes.len() {
                let a = (indexes[i].0 as i32, indexes[i].1 as i32);
                let b = (indexes[j].0 as i32, indexes[j].1 as i32);
                match model {
                    AntinodeModel::PART1 => {
                        let antinode0 = (a.0 + (a.0 - b.0), a.1 + (a.1 - b.1));
                        if grid.in_grid_i32(antinode0) {
                            antinodes.insert((antinode0.0 as usize, antinode0.1 as usize));
                        }
                        let antinode1 = (b.0 + (b.0 - a.0), b.1 + (b.1 - a.1));
                        if grid.in_grid_i32(antinode1) {
                            antinodes.insert((antinode1.0 as usize, antinode1.1 as usize));
                        }
                    }
                    AntinodeModel::PART2 => {
                        let mut antinode0 = (a.0, a.1);
                        while grid.in_grid_i32(antinode0) {
                            antinodes.insert((antinode0.0 as usize, antinode0.1 as usize));
                            antinode0.0 = antinode0.0 + (a.0 - b.0);
                            antinode0.1 = antinode0.1 + (a.1 - b.1);
                        }
                        let mut antinode1 = (a.0, a.1);
                        while grid.in_grid_i32(antinode1) {
                            antinodes.insert((antinode1.0 as usize, antinode1.1 as usize));
                            antinode1.0 = antinode1.0 + (b.0 - a.0);
                            antinode1.1 = antinode1.1 + (b.1 - a.1);
                        }
                    }
                }
            }
        }
    }

    antinodes
}
