mod prelude;

use itertools::Itertools;
use prelude::*;
use std::collections::BTreeSet;
use std::{fs, iter};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

fn main() {
    // solve_part1(&get_input(1, InputType::Debugging));
    // solve_part1(&get_input(1, InputType::Example));
    // solve_part1_alternate(&get_input(1, InputType::Example));

    // fs::remove_file("./part1.log");
    // fs::remove_file("./part2.log");
    //
    // solve_part1(&get_input(2, InputType::Debugging));
    // solve_part1_alternate(&get_input(2, InputType::Debugging));

    // solve_part1(&get_input(1, InputType::Test));
    // solve_part2(&get_input(2, InputType::Debugging));
    solve_part2(&get_input(1, InputType::Example));
    solve_part2(&get_input(1, InputType::Test));
}

fn move_boxes(walls: &BTreeSet<(usize, usize)>, boxes: &mut BTreeSet<(usize, usize)>, player: &(usize, usize), direction : Directions, grid : &CharGrid) -> bool {
    let mut require_empty = vec![grid.move_by_direction(direction, *player).unwrap()];
    let mut moved = BTreeSet::new();
    let mut boxes_to_move = Vec::new();
    while let Some(current_index) = require_empty.pop() {
        if walls.contains(&current_index) {
            return false;
        }
        let box_to_move = boxes.get(&current_index)
            .or(boxes.get(&grid.move_by_direction(Directions::LEFT, current_index).unwrap()));
        if let Some(&left_index) = box_to_move {
            if moved.contains(&left_index) {
                continue;
            }
            moved.insert(left_index);
            boxes_to_move.push(left_index);
            // println!("Moved {moved:?}");
            let right_index = grid.move_by_direction(Directions::RIGHT, left_index).unwrap();
            if direction == Directions::UP || direction == Directions::DOWN {
                require_empty.push(grid.move_by_direction(direction, left_index).unwrap());
                require_empty.push(grid.move_by_direction(direction, right_index).unwrap());
            } else if direction == Directions::RIGHT {
                require_empty.push(grid.move_by_direction(direction, right_index).unwrap());
            } else if direction == Directions::LEFT {
                require_empty.push(grid.move_by_direction(direction, left_index).unwrap());
            } else {
                panic!("Unhandled direction {:?}", direction);
            }
        }
    }
    for &index in &boxes_to_move {
        boxes.remove(&index);
    }
    for &index in &boxes_to_move {
        boxes.insert(grid.move_by_direction(direction, index).unwrap());
    }
    true
}

fn solve_part2(str: &str) {
    let (map_chars, movement_chars) = str.split_once("\n\n").unwrap();
    let grid_input = CharGrid::from(map_chars);

    let walls = grid_input
        .indexes()
        .filter(|&idx| grid_input.get(idx) == '#')
        .flat_map(|(row, col)| { [(row, col * 2), (row, col * 2 + 1)] })
        .collect::<BTreeSet<_>>();
    let mut boxes = grid_input
        .indexes()
        .filter(|&idx| grid_input.get(idx) == 'O')
        .map(|(row, col)| { (row, col * 2) })
        .collect::<BTreeSet<(usize, usize)>>();
    let mut player_index = grid_input
        .indexes()
        .find(|&idx| grid_input.get(idx) == '@')
        .map(|(row, col)| { (row, col * 2) })
        .unwrap();
    let grid = CharGrid::new(grid_input.columns * 2, grid_input.rows * 2, '.');
    for move_char in movement_chars.chars().filter(|&c| !c.is_whitespace()) {
        let direction = match move_char {
            '>' => { Directions::RIGHT }
            'v' => { Directions::DOWN }
            '<' => { Directions::LEFT }
            '^' => { Directions::UP }
            _ => panic!("Unknown move_char: {}", move_char),
        };

        if move_boxes(&walls, &mut boxes, &player_index, direction, &grid) {
            player_index = grid.move_by_direction(direction, player_index).unwrap();
        }
    }

    let score = boxes.iter().map(box_to_score).sum::<i32>();
    println!("Part1 answer is {}", score);
}

fn print_result_part2(grid: &CharGrid, walls: &BTreeSet<(usize, usize)>, boxes: &BTreeSet<(usize, usize)>, player: &(usize, usize)) {
    let mut out_grid = CharGrid::new(grid.columns, grid.rows / 2, '.');
    walls.iter().for_each(|&wall| {
        out_grid.set(wall, '#');
    });
    boxes.iter().for_each(|&index| {
        out_grid.set(index, '[');
        out_grid.set(grid.move_by_direction(Directions::RIGHT, index).unwrap(), ']');
    });
    out_grid.set(player.clone(), '@');

    let mut file = OpenOptions::new()
        .append(true) // Open in append mode
        .create(true) // Create the file if it doesn't exist
        .open("./part2.log").expect("Could not open part2.log");
    let string = format!("{}\n\n", out_grid);
    file.write(string.as_bytes()).expect("Could not write to file");
    // println!("{}", out_grid);
}

fn solve_part1_alternate(str: &str) {
    let (map_chars, movement_chars) = str.split_once("\n\n").unwrap();
    let grid_input = CharGrid::from(map_chars);

    let walls = grid_input
        .indexes()
        .filter(|&idx| grid_input.get(idx) == '#')
        .flat_map(|(row, col)| { [(row, col * 2), (row, col * 2 + 1)] })
        .collect::<BTreeSet<_>>();
    let mut boxes = grid_input
        .indexes()
        .filter(|&idx| grid_input.get(idx) == 'O')
        .map(|(row, col)| { (row, col * 2) })
        .collect::<BTreeSet<(usize, usize)>>();
    let mut player_index = grid_input
        .indexes()
        .find(|&idx| grid_input.get(idx) == '@')
        .map(|(row, col)| { (row, col * 2) })
        .unwrap();
    let grid = CharGrid::new(grid_input.columns * 2, grid_input.rows * 2, '.');
    let movement_chars_expanded = movement_chars.chars().filter(|&c| !c.is_whitespace()).flat_map(|move_char| {
        if move_char == '<' {
            vec!['<', '<']
        } else if move_char == '>' {
            vec!['>', '>', '>', '<']
        } else {
            vec![move_char]
        }
    }).collect::<String>();

    for move_char in movement_chars_expanded.chars() {
        let direction = match move_char {
            '>' => { Directions::RIGHT }
            'v' => { Directions::DOWN }
            '<' => { Directions::LEFT }
            '^' => { Directions::UP }
            _ => panic!("Unknown move_char: {}", move_char),
        };

        if move_boxes(&walls, &mut boxes, &player_index, direction, &grid) {
            player_index = grid.move_by_direction(direction, player_index).unwrap();
        }

        // println!("{}", move_char);
        print_result_part2(&grid, &walls, &boxes, &player_index);
        // println!();
    }

    let score = boxes.iter().map(box_to_score_alternate).sum::<i32>();
    println!("Part1 answer is {}", score);
}

fn solve_part1(str: &str) {
    let (map_chars, movement_chars) = str.split_once("\n\n").unwrap();
    let grid = CharGrid::from(map_chars);

    let walls = grid
        .indexes()
        .filter(|&idx| grid.get(idx) == '#')
        .collect::<BTreeSet<_>>();
    let mut boxes = grid
        .indexes()
        .filter(|&idx| grid.get(idx) == 'O')
        .collect::<BTreeSet<(usize, usize)>>();
    let mut player_index = grid
        .indexes()
        .find(|&idx| grid.get(idx) == '@')
        .unwrap();


    // print_result(&grid, &walls, &boxes, &player_index);
    // println!();
    for move_char in movement_chars.chars().filter(|&c| !c.is_whitespace()) {
        let direction = match move_char {
            '>' => { Directions::RIGHT }
            'v' => { Directions::DOWN }
            '<' => { Directions::LEFT }
            '^' => { Directions::UP }
            _ => panic!("Unknown move_char: {}", move_char),
        };

        let starting_index = grid.move_by_direction(direction, player_index);
        let mut cast = iter::from_fn({
            let mut iter_index = starting_index;
            let grid = &grid; // Capture `grid` by reference inside the block
            move || {
                let current_index = iter_index;
                iter_index = current_index.and_then(|idx| grid.move_by_direction(direction, idx));
                current_index
            }
        });

        let ending_index = cast
            .find(|idx| !boxes.contains(idx))
            .and_then(|idx| { if walls.contains(&idx) { None } else { Some(idx) } });

        if let Some(ending_idx) = ending_index {
            let starting_idx = starting_index.unwrap();

            if starting_idx != ending_idx {
                boxes.remove(&starting_idx);
                boxes.insert(ending_idx);
            }
            player_index = starting_idx;
        }

        // println!("{}", move_char);
        print_result(&grid, &walls, &boxes, &player_index);
        // println!();
    }

    let score = boxes.iter().map(box_to_score).sum::<i32>();
    println!("Part1 answer is {}", score);
}


fn print_result(grid: &CharGrid, walls: &BTreeSet<(usize, usize)>, boxes: &BTreeSet<(usize, usize)>, player: &(usize, usize)) {
    let mut out_grid = CharGrid::new(grid.columns, grid.rows, '.');
    walls.iter().for_each(|&wall| { out_grid.set(wall, '#') });
    boxes.iter().for_each(|&index| { out_grid.set(index, 'O') });
    out_grid.set(player.clone(), '@');
    let mut file = OpenOptions::new()
        .append(true) // Open in append mode
        .create(true) // Create the file if it doesn't exist
        .open("./part1.log").expect("Could not open part1.log");
    let string = format!("{}\n\n", out_grid);
    file.write(string.as_bytes()).expect("Could not write to file");
    // println!("{}", out_grid);
}


fn box_to_score(index: &(usize, usize)) -> i32 {
    (index.0 * 100 + index.1) as i32
}

fn box_to_score_alternate(index: &(usize, usize)) -> i32 {
    (index.0 * 100 + index.1 / 2) as i32
}

