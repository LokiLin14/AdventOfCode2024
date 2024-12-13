mod prelude;

use itertools::Itertools;
use prelude::*;

fn main() {
    solve_part1(get_input(1, true));
    solve_part1(get_input(1, false));
    solve_part2(get_input(1, true));
    solve_part2(get_input(1, false));
}

fn solve_part2(str : String) {
    let grid = CharGrid::from(&str);

    let mut scores = vec![0; grid.area()];

    // set default condition
    grid.indexes()
        .filter(|&index| interpret_char_as_height(&grid, index) == 9)
        .for_each(|index| {
            scores[grid.index_to_usize(index)] = 1;
        });

    // propagate score
    for cur_height in (0..9).rev() {
        grid.indexes()
            .filter(|&index| interpret_char_as_height(&grid, index) == cur_height)
            .for_each(|index| {
                for adj_index in grid.adjacent_indexes(index) {
                    if interpret_char_as_height(&grid, adj_index) == cur_height + 1 {
                        scores[grid.index_to_usize(index)] += scores[grid.index_to_usize(adj_index)];
                    }
                }
            })
    }

    let mut total_score = 0;
    grid.indexes()
        .filter(|&index| interpret_char_as_height(&grid, index) == 0)
        .for_each(|index| {
            let score = scores[grid.index_to_usize(index)];
            // println!("Index {index:?} has score {}.", score);
            total_score += score;
        });
    println!("Part2 score is {}", total_score);
}

fn interpret_char_as_height(grid : &CharGrid, index : (usize, usize)) -> u32 {
    grid.get(index).to_digit(10).unwrap()
}

fn solve_part1(str : String) {
    let grid = CharGrid::from(&str);

    let mut score = 0;
    for index in (0..grid.rows).cartesian_product(0..grid.columns) {
        let current_height = grid.get(index).to_digit(10).unwrap();
        if current_height != 0 {
            continue;
        }
        let trailheads = reachable_trailheads(&grid, index);
        // println!("Trailheads at {index:?}: {:?}", trailheads);
        score += trailheads;
    }
    println!("Part1 score is {}", score);
}

fn reachable_trailheads(grid : &CharGrid, index : (usize, usize)) -> usize {
    let mut visited = vec![false; grid.area()];
    let mut trailheads = 0;
    let mut to_visit = vec![index];
    while !to_visit.is_empty() {
        let cur_grid_index = to_visit.pop().unwrap();
        let cur_visited_index = grid.index_to_usize(cur_grid_index);
        if visited[cur_visited_index] {
            continue;
        }
        visited[cur_visited_index] = true;
        let cur_height = grid.get(cur_grid_index).to_digit(10).unwrap();
        if cur_height == 9 {
            trailheads += 1;
        }
        for adj_index in grid.adjacent_indexes(cur_grid_index) {
            let adj_height = grid.get(adj_index).to_digit(10).unwrap();
            if adj_height == cur_height + 1 {
                to_visit.push(adj_index);
            }
        }
    }
    trailheads
}