fn is_possible_expr(target : i64, array : &[i64]) -> bool {
    if array.len() == 0 {
        return target == 0;
    }
    let i = array.len() - 1;
    let v = array[i];
    if target - array[i] >= 0 && is_possible_expr(target - array[i], &array[0..i]) {
        return true;
    }
    if target % v == 0 && is_possible_expr(target / v, &array[0..i]) {
        return true;
    }
    false
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

        let target : i64;
        let array : Vec<i64>;
        if let Some((tar, rem)) = buffer.split_once(":") {
            target = tar.trim().parse().unwrap();
            array = rem.split_whitespace().map(|s|
                s.parse().unwrap_or_else(|e| { panic!("{}", e)})
            ).collect::<Vec<_>>()
        } else {
            break;
        }

        println!("Target: {:?}", target);
        println!("With numbers: {:?}", array);
        if is_possible_expr(target, array.as_slice()) {
            println!("Target is possible.");
            sum += target;
            count += 1;
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
    use crate::is_possible_expr;

    #[test]
    fn is_possible_stress() {
        let mut rand_gen = StdRng::seed_from_u64(222);

        let num_tests = 10000;
        let max_total = 100000;
        for _ in 0..num_tests {
            let n = rand_gen.random_range(1..50);

            let mut total = 0;
            let mut history = Vec::new();
            for _ in 0..n {
                let v = rand_gen.random_range(1 .. 50);
                if total * n < max_total && rand_gen.random_bool(0.5) {
                    total *= v;
                } else {
                    total += v;
                }
                history.push(v);
            }

            assert!(is_possible_expr(total, history.as_slice()));
        }

    }
}
