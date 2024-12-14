mod prelude;

use std::collections::BTreeMap;
use itertools::Itertools;
use prelude::*;

fn main() {
    solve_part1(get_input(1, true));
    solve_part1(get_input(1, false));
    solve_part2(get_input(2, true));
    solve_part2(get_input(1, false));
}

// returns a pair (usize, Vec<usize>) meaning (number of groups, group id of each index)
fn group_by_adjacent(grid : &CharGrid) -> (usize, Vec<usize>) {
    let mut groups = 0;
    let mut visited = vec![false; grid.area()];
    let mut ids = vec![0; grid.area()];
    for index in grid.indexes() {
        if visited[grid.usize_of(index)] {
            continue;
        }
        let group_char = grid.get(index);
        let mut stack = vec![index];
        while let Some(current_index) = stack.pop() {
            let current_usize = grid.usize_of(current_index);
            visited[current_usize] = true;
            ids[current_usize] = groups;
            for adj_index in grid.adjacent_indexes(current_index) {
                if grid.get(adj_index) != group_char || visited[grid.usize_of(adj_index)] {
                    continue;
                }
                stack.push(adj_index);
            }
        }
        groups += 1;
    }
    (groups, ids)
}

fn solve_part2(input: String) {
    let grid = CharGrid::from(&input);

    let (num_ids, ids) = group_by_adjacent(&grid);

    // count the corners in every group on every 2x2 window, similar to marching cubes
    let mut corners = vec![0; num_ids];
    for index in grid.indexes() {
        let index_row = index.0 as i32;
        let index_col = index.1 as i32;
        let diagonals = [
            (index_row - 1, index_col - 1),
            (index_row - 1, index_col + 1),
            (index_row + 1, index_col - 1),
            (index_row + 1, index_col + 1)
        ];

        let index_id = ids[grid.usize_of(index)];
        for diagonal_index in diagonals {
            let equals = [
                (index_row, diagonal_index.1),
                (diagonal_index.0, index_col),
                diagonal_index
            ].iter()
                .map(|&(r, c)| {
                    if r >= 0 && r < grid.rows as i32 && c >= 0 && c < grid.columns as i32 {
                        index_id == ids[grid.usize_of((r as usize, c as usize))]
                    } else {
                        false
                    }
                }).collect::<Vec<bool>>();

            // handle different cases similar to how marching cubes handles cases
            if is_corner(equals[0], equals[1], equals[2]) {
                corners[index_id] += 1;
            }
        }
    }

    let mut areas = vec![0; num_ids];
    grid.indexes()
        .map(|index| ids[grid.usize_of(index)])
        .for_each(|id| { areas[id] += 1 });

    let ans = areas.iter().zip(corners.iter())
        .map(|(&area, &perimeter)| { area * perimeter })
        .sum::<usize>();

    println!("Part2 answer is {}.", ans);
}

fn is_corner(adj0_equal : bool, adj1_equal : bool, diag_equal : bool) -> bool {
    if !adj0_equal && !adj1_equal {
        true
    } else if adj0_equal && adj1_equal && !diag_equal {
        true
    } else {
        false
    }
}

fn solve_part1(input: String) {
    let grid = CharGrid::from(&input);

    let (num_ids, ids) = group_by_adjacent(&grid);

    let mut perimeters = vec![0; num_ids];
    for index in grid.indexes() {
        let current_id = ids[grid.usize_of(index)];
        let mut adj_diffs = 4 - grid.adjacent_indexes(index).len();
        for adj_index in grid.adjacent_indexes(index) {
            if current_id != ids[grid.usize_of(adj_index)] {
                adj_diffs += 1;
            }
        }
        perimeters[current_id] += adj_diffs;
    }

    let mut areas = vec![0; num_ids];
    grid.indexes()
        .map(|index| ids[grid.usize_of(index)])
        .for_each(|id| { areas[id] += 1 });

    let ans = areas.iter().zip(perimeters.iter())
        .map(|(&area, &perimeter)| { area * perimeter })
        .sum::<usize>();

    println!("Part1 answer is {}.", ans);
}
