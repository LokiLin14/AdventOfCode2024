mod prelude;

use prelude::*;
use regex::Regex;
use std::mem::swap;

fn main() {
    solve_part1(get_input(1, InputType::Debugging));
    solve_part1(get_input(1, InputType::Example));
    solve_part1(get_input(1, InputType::Test));
    // solve_parts(get_input(2, true));
    solve_part2(get_input(2, InputType::Test));
}

#[derive(Debug, Clone)]
struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    target: (i64, i64),
}

fn parse_button(str: &str, button_name: &str) -> (i64, i64) {
    let button_regex = Regex::new(r"Button ([a-zA-Z]+): X\+([0-9]+), Y\+([0-9]+)").unwrap();

    let capture = button_regex.captures(str).unwrap();

    assert_eq!(&capture[1], button_name);

    let x_amt = capture[2].parse::<i64>().unwrap();
    let y_amt = capture[3].parse::<i64>().unwrap();

    (x_amt, y_amt)
}

fn parse_prize(str: &str) -> (i64, i64) {
    let prize_regex = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();

    let capture = prize_regex.captures(str).unwrap();

    let x_amt = capture[1].parse::<i64>().unwrap();
    let y_amt = capture[2].parse::<i64>().unwrap();

    (x_amt, y_amt)
}

fn parse_claw_machine(str: &str) -> ClawMachine {
    let lines = str.split("\n").collect::<Vec<&str>>();
    assert_eq!(lines.len(), 3);
    let button_a = parse_button(lines[0], "A");
    let button_b = parse_button(lines[1], "B");
    let target = parse_prize(lines[2]);
    ClawMachine { button_a, button_b, target }
}
const BUTTON_COSTS: (i64, i64) = (3, 1);
fn solution_cost(sol: (i64, i64)) -> i64 {
    sol.0 * BUTTON_COSTS.0 + sol.1 * BUTTON_COSTS.1
}
fn solve_claw_machine_by_naive(claw_machine: &ClawMachine) -> Option<(i64, i64)> {
    let max_usages = 100;

    let mut min_cost_solution = None;

    for a_presses in 0..=max_usages {
        for b_presses in 0..=max_usages {
            let x_sum = a_presses * claw_machine.button_a.0
                + b_presses * claw_machine.button_b.0;
            let y_sum = a_presses * claw_machine.button_a.1
                + b_presses * claw_machine.button_b.1;
            if x_sum != claw_machine.target.0 || y_sum != claw_machine.target.1 {
                continue;
            }
            let current_solution = (a_presses, b_presses);
            if let Some(prev_solution) = min_cost_solution {
                let prev_cost = solution_cost(prev_solution);
                let current_cost = solution_cost(current_solution);
                if prev_cost > current_cost {
                    min_cost_solution = Some(current_solution);
                }
            } else {
                min_cost_solution = Some(current_solution);
            }
        }
    }
    min_cost_solution
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        a %= b;
        swap(&mut a, &mut b);
    }
    a
}

fn extended_euclidean(a: i64, b: i64) -> (i64, i64, i64) {
    let mut a = (a, 1, 0);
    let mut b = (b, 0, 1);
    while b.0 != 0 {
        let sub_amt = a.0 / b.0;
        a.0 -= sub_amt * b.0;
        a.1 -= sub_amt * b.1;
        a.2 -= sub_amt * b.2;
        swap(&mut a, &mut b);
    }
    a
}

fn normalized_2vector(vec: (i64, i64)) -> (i64, i64) {
    let div = gcd(vec.0, vec.1);
    (vec.0 / div, vec.1 / div)
}

// to determine if there is some non-negative x y s.t. a * x + b * y = target
// if we don't restrict x and y to be non-negative, then there is a solution iff target is
// divisible by the gcd of x and y
// but this solution might have negative x or y which will make our solution invalid
// to add back the constraint that x and y are non-negative, note that
//


fn solve_claw_machine_fast(claw_machine: &ClawMachine) -> Option<(i64, i64)> {
    // special case the fact that we have the same vector
    let norm_a = normalized_2vector(claw_machine.button_a);
    let norm_b = normalized_2vector(claw_machine.button_b);
    if norm_a == norm_b {
        let norm_target = normalized_2vector(claw_machine.target);
        if norm_a != norm_target {
            return None;
        }
        // handle the case that they are all facing the same direction
        let a = claw_machine.button_a.0 / norm_a.0;
        let b = claw_machine.button_b.0 / norm_a.0;
        let target = claw_machine.target.0 / norm_a.0;
        // determine if there is some x y s.t. a * x + b * y = target
        let min_mod = extended_euclidean(a, b);
        if target % min_mod.0 != 0 {
            return None;
        }
        // find the smallest cost solution and try to force
        // the solution to have positive x and y values
        let amt = target / min_mod.0;
        let partial_solution = (amt * min_mod.1, amt * min_mod.2);
        assert!(BUTTON_COSTS.0 >= BUTTON_COSTS.1); // try to minimize the x value
        let min_shift_amt = (b / min_mod.0, -a / min_mod.0);
        let shift_amt;
        if partial_solution.0 >= 0 {
            shift_amt = partial_solution.0 / min_shift_amt.0;
        } else {
            shift_amt = (0 - partial_solution.0 + (min_shift_amt.0 - 1)) / min_shift_amt.0;
        }
        let solution = (partial_solution.0 - shift_amt * min_shift_amt.0,
                        partial_solution.1 - shift_amt * min_shift_amt.1);
        assert!(solution.0 >= 0);
        if solution.1 <= 0 {
            return None;
        }
        return Some(solution);
    }

    // solve with the 2d matrix inverse
    // [ d -b ]
    // [ -c a ]
    // over
    // ad - bc

    // our matrix is
    // [ a b ]   [ button_a.0 button_b.0 ]
    // [ c d ] = [ button_a.1 button_b.1 ]
    let a = claw_machine.button_a.0;
    let b = claw_machine.button_b.0;
    let c = claw_machine.button_a.1;
    let d = claw_machine.button_b.1;

    // multiply inverse with
    // [ target.0 ]
    // [ target.1 ]

    let a_presses = claw_machine.target.0 * d - claw_machine.target.1 * b;
    let b_presses = -claw_machine.target.0 * c + claw_machine.target.1 * a;

    let div = a * d - b * c;
    if a_presses % div != 0 || b_presses % div != 0 {
        return None;
    }
    Some((a_presses / div, b_presses / div))
}

fn solve_part1(str: String) {
    let claw_machines = str.split("\n\n")
        .map(parse_claw_machine).collect::<Vec<ClawMachine>>();

    let total_cost = claw_machines.iter().filter_map(solve_claw_machine_by_naive).map(solution_cost).sum::<i64>();

    println!("Claw machines:\n {claw_machines:?}");
    println!("Total cost: {total_cost}");
}

fn solve_part2(str: String) {
    let claw_machines = str.split("\n\n")
        .map(parse_claw_machine).map(|cm| ClawMachine {
        button_a : cm.button_a,
        button_b : cm.button_b,
        target : (cm.target.0 + 10000000000000, cm.target.1 + 10000000000000)
    }).collect::<Vec<ClawMachine>>();

    let total_cost = claw_machines.iter()
        .filter_map(solve_claw_machine_fast).map(solution_cost).sum::<i64>();

    println!("Claw machines:\n {claw_machines:?}");
    println!("Total cost: {total_cost}");
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn claw_machines_solved_by_both(str : String) {
        let claw_machines = str.split("\n\n")
            .map(parse_claw_machine).collect::<Vec<ClawMachine>>();
        assert_eq!(claw_machines.iter().map(solve_claw_machine_by_naive).collect::<Vec<_>>()
                   , claw_machines.iter().map(solve_claw_machine_fast).collect::<Vec<_>>());
    }

    #[test]
    fn part_1_is_solved_by_both() {
        claw_machines_solved_by_both(get_input(1, InputType::Debugging));
        claw_machines_solved_by_both(get_input(1, InputType::Example));
        claw_machines_solved_by_both(get_input(1, InputType::Test));
    }
}