mod prelude;

use std::collections::BTreeMap;
use itertools::Itertools;
use prelude::*;

fn main() {
    solve_part1(get_input(1, true));
    solve_part1(get_input(1, false));
    solve_part2(get_input(1, true));
    solve_part2(get_input(1, false));
}

fn solve_part1(input : String) {
    let blinks = 25;
    let arr : Vec<i64> = input.split_whitespace().map(|s| s.parse().unwrap_or_panic()).collect();
    let ans = arr.iter().map(|&v| stones(v, blinks)).sum::<i64>();
    println!("Part1 answer is {}.", ans);
}

fn solve_part2(input : String) {
    let blinks = 75;
    let arr : Vec<i64> = input.split_whitespace().map(|s| s.parse().unwrap_or_panic()).collect();
    let ans = arr.iter().map(|&v| stones(v, blinks)).sum::<i64>();
    println!("Part2 answer is {}.", ans);
}

fn stones(inital_number: i64, blinks: i32) -> i64 {
    if blinks == 0 {
        return 1;
    }
    static mut TABLE: BTreeMap<(i64, i32), i64> = BTreeMap::new();
    unsafe {
        if let Some(ans) = TABLE.get(&(inital_number, blinks)) {
            return *ans;
        }
    }
    let new_numbers;
    let str = inital_number.to_string();
    if inital_number == 0 {
        new_numbers = vec![1];
    } else if str.len() % 2 == 0 {
        let (fst, snd) = str.split_at(str.len() / 2);
        new_numbers = vec![fst.parse().unwrap_or_panic(), snd.parse().unwrap_or_panic()];
    } else {
        new_numbers = vec![inital_number * 2024];
    }
    let ans = new_numbers.iter().map(|&x| stones(x, blinks - 1)).sum();
    unsafe { TABLE.insert((inital_number, blinks), ans); }
    ans
}