use core::asserting::Capture;
use regex::{Captures, Regex, RegexSet, RegexSetBuilder};
use std::io::Read;
use std::iter::Product;
use std::result::Iter;

fn greedy_matches(regexes : &[Regex], string: &str) -> Iter<Product<i32, Captures>> {
    let mut captures = Vec::new();
    let mut remaing_string = string;

    struct CaptureWithOffset {
        offset : usize,

        cap : Captures
    }

    loop {
        let ma = regexes[0].captures(remaing_string).unwrap().get(0).unwrap().start();
        regexes.iter().filter_map(|re| {
            re.captures(remaing_string).map(|cap| {
                cap.get(0).unwrap().start()
            })
        });
        // if let Some(match) = regexes
        //     .filter_map(|re| { re.capture(remaing_string) })
        //     .min()
        // {
        //
        // } else {
        //     break;
        // }
    }
}
fn main() {
    let mut string = String::new();
    std::io::stdin().lock().read_to_string(&mut string).unwrap();
    println!("Received string: {}", string);

    let reDo = Regex::new(r"do\(\)").unwrap();
    let reDont = Regex::new(r"don't\(\)").unwrap();
    let reMul = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let reExpr = RegexSet::new([reDo, reDont, reMul]).unwrap();

    let mut mulEnabled = true;
    let sum = reExpr.matches(&string).into_iter().for_each(|m| {
        match m {
            0 => {
                mulEnabled = false;
            }
            1 => {
                mulEnabled = true;
            }
            2 => {

            }
            _ => {}
        }

        if(m == 0) {
            mulEnabled = false;
        }

    });

    let sum = reExpr.match(&string)
        .map(|caps| {
            let a = caps[1].parse::<i32>().unwrap();
            let b = caps[2].parse::<i32>().unwrap();
            println!("Found match: {} = {}", caps.get(0).unwrap().as_str(), a * b);
            a * b
        }).sum::<i32>();
    println!("The total is {}", sum);
}