fn pop_last_digits(mut target: i64, mut value: i64) -> Option<i64> {
    if value == 0 {
        return if target % 10 == 0 {
            Some(target / 10)
        } else {
            None
        }
    }

    loop {
        if value == 0 {
            break;
        }
        if target % 10 != value % 10 {
            return None;
        }
        target /= 10;
        value /= 10;
    }
    Some(target)
}

fn calc(str : &str) -> i64 {
    let strings = str.split_whitespace().collect::<Vec<_>>();

    let mut num : i64 = 0;
    let mut i = 1;
    while i < strings.len() {
        if strings[i] == "+" {
            num += strings[i + 1].parse::<i64>().unwrap();
        } else if strings[i] == "*" {
            num *= strings[i + 1].parse::<i64>().unwrap();
        } else if strings[i] == "||" {
            let v = strings[i + 1].parse::<i64>().unwrap();
            num = format!("{num}{v}").parse::<i64>().unwrap();
        } else {
            assert!(false);
        }
        i += 2;
    }
    num
}

fn is_possible_expr(target: i64, array: &[i64]) -> Option<String> {
    if array.len() == 0 {
        return if target == 0 {
            Some(String::from("0"))
        } else {
            None
        }
    }
    let i = array.len() - 1;
    let v = array[i];
    if target >= array[i] {
        if let Some(mut sol) = is_possible_expr(target - array[i], &array[0..i]) {
            let str = format!(" + {}", v);
            sol.push_str(str.as_str());
            return Some(sol);
        }
    }
    if let Some(rem) = pop_last_digits(target, v) {
        if let Some(mut sol) = is_possible_expr(rem, &array[0..i]) {
            let str = format!(" || {}", v);
            sol.push_str(str.as_str());
            return Some(sol);
        }
    }
    if target % v == 0 {
        if let Some(mut sol) = is_possible_expr(target / v, &array[0..i]) {
            let str = format!(" * {}", v);
            sol.push_str(str.as_str());
            return Some(sol);
        }
    }
    return None;
}

fn main() {
    let mut sum = 0;
    let mut count = 0;
    let mut total_problems = 0;
    loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        if buffer.trim().is_empty() {
            break;
        }
        total_problems += 1;

        let target: i64;
        let array: Vec<i64>;
        if let Some((tar, rem)) = buffer.split_once(":") {
            target = tar.trim().parse().unwrap();
            array = rem.split_whitespace().map(|s|
                s.parse().unwrap_or_else(|e| { panic!("{}", e) })
            ).collect::<Vec<_>>()
        } else {
            break;
        }

        println!("Target: {:?}", target);
        println!("With numbers: {:?}", array);
        if let Some(sol) = is_possible_expr(target, array.as_slice()) {
            println!("Target is possible by {}", sol);
            sum += target;
            count += 1;
            assert_eq!(calc(sol.as_str()), target);
        } else {
            println!("Target is impossible.");
        }
        println!();
    }
    println!("Solved {} / {}", count, total_problems);
    println!("Sum is {}", sum);
}

#[cfg(test)]
mod tests {
    use rand::prelude::StdRng;
    use rand::{Rng, SeedableRng};
    use crate::{is_possible_expr, pop_last_digits};

    // #[test]
    // fn is_possible_stress() {
    //     let mut rand_gen = StdRng::seed_from_u64(222);
    //
    //     let num_tests = 1000;
    //     let max_total = 100000;
    //     for _ in 0..num_tests {
    //         let n = rand_gen.random_range(1..50);
    //
    //         let mut total = 0;
    //         let mut history = Vec::new();
    //         for _ in 0..n {
    //             let v = rand_gen.random_range(1 .. 50);
    //             if total * n < max_total && rand_gen.random_bool(0.5) {
    //                 total *= v;
    //             } else {
    //                 total += v;
    //             }
    //             history.push(v);
    //         }
    //
    //         assert!(is_possible_expr(total, history.as_slice()));
    //     }
    // }

    #[test]
    fn concat_stress() {
        for a in 0..1000 {
            for b in 0..1000 {
                let ab = format!("{a}{b}").parse::<i64>().unwrap();
                if(pop_last_digits(ab, b) != Some(a)) {
                    println!("{} {}", a, b);
                }
                assert_eq!(pop_last_digits(ab, b), Some(a));
            }
        }

    }
}
