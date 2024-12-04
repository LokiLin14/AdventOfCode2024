use regex::Regex;
use std::io::Read;

fn main() {
    let mut string = String::new();
    std::io::stdin().lock().read_to_string(&mut string).unwrap();
    println!("Received string: {}", string);

    let re_do = Regex::new(r"^do\(\)").unwrap();
    let re_dont = Regex::new(r"^don't\(\)").unwrap();
    let re_mul = Regex::new(r"^mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut sum = 0;
    let mut mul_enabled = true;
    let mut str_remaining : &str = &string;
    while !str_remaining.is_empty() {
        if let Some(ma) = re_do.find(&str_remaining) {
            println!("Parsed: {}", ma.as_str());
            str_remaining = str_remaining.get(ma.len()..).unwrap();
            mul_enabled = true;
            continue;
        }
        if let Some(ma) = re_dont.find(&str_remaining) {
            println!("Parsed: {}", ma.as_str());
            str_remaining = str_remaining.get(ma.len()..).unwrap();
            mul_enabled = false;
            continue;
        }
        if let Some(ma) = re_mul.captures(&str_remaining) {
            str_remaining = str_remaining.get(ma.len()..).unwrap();
            let add = if mul_enabled {
                ma[1].parse::<i32>().unwrap() * ma[2].parse::<i32>().unwrap()
            } else {
                0
            };
            println!("Parsed: {} = {}", ma[0].to_string().as_str(), add);
            sum += add;
            continue;
        }
        str_remaining = str_remaining.chars().next().map(|c| &str_remaining[c.len_utf8()..]).unwrap();
    }

    println!("The total is {}", sum);
}