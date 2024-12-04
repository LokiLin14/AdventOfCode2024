use regex::{Regex, RegexSet, RegexSetBuilder};
use std::io::Read;

fn main() {
    let mut string = String::new();
    std::io::stdin().lock().read_to_string(&mut string).unwrap();
    println!("Received string: {}", string);

    let reMul = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let sum = reMul
        .captures_iter(&string)
        .map(|caps| {
            let a = caps[1].parse::<i32>().unwrap();
            let b = caps[2].parse::<i32>().unwrap();
            println!("Found match: {} = {}", caps.get(0).unwrap().as_str(), a * b);
            a * b
        }).sum::<i32>();
    println!("The total is {}", sum);
}