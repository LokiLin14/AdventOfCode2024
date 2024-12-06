use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

#[derive(Debug)]
pub struct Prints {
    // maps i32 to the numbers that must appear after it
    comparisons: HashMap<i32, Vec<i32>>,
    pub manuals: Vec<Vec<i32>>,
}

pub fn read_prints_from_stdio() -> Prints {
    let mut comparisons : HashMap<i32, Vec<i32>> = HashMap::new();
    let comparison_regex = Regex::new(r"(\d+)\|(\d+)").unwrap();
    loop {
        let mut buffer = String::new();
        std::io::stdin().lock().read_line(&mut buffer).unwrap();

        if buffer.trim().is_empty() {
            break;
        }

        let caps = comparison_regex.captures(&buffer).unwrap_or_else(||
            panic!("Regex failed on \"{}\"", buffer)
        );
        let a = caps[1].parse().unwrap();
        let b = caps[2].parse().unwrap();
        if !comparisons.contains_key(&a) {
            comparisons.insert(a, vec![]);
        }
        comparisons.get_mut(&a).unwrap().push(b);
    }

    let mut manuals: Vec<Vec<i32>> = Vec::new();
    loop {
        let mut buffer = String::new();
        std::io::stdin().lock().read_line(&mut buffer).unwrap();

        if buffer.trim().is_empty() {
            break;
        }

        let manual = buffer.split(',').map(|s| s.trim().parse::<i32>().unwrap()).collect();
        manuals.push(manual);
    }
    Prints { comparisons, manuals }
}

impl Prints {
    pub fn is_valid_manual(&self, manual : &Vec<i32>) -> bool {
        manual.iter().fold(Some(HashSet::new()), |maybe_seen, x| {
            match maybe_seen {
                None => None,
                Some(mut seen) => {
                    if self.comparisons.get(x).map_or(false, |v| v.iter().any(|must_be_after| {
                        seen.contains(must_be_after)
                    })) {
                        None
                    } else {
                        seen.insert(x);
                        Some(seen)
                    }
                }
            }
        }).is_some()
    }

    pub fn get_unique_order(&self, manual : &Vec<i32>) -> Vec<i32> {
        let manual_set : HashSet<i32> = HashSet::from_iter(manual.iter().copied());

        let mut counted_manual: Vec<(usize, i32)> = manual.iter().map(|x| {
            self.comparisons.get(x).map_or(0, |v| v.iter().filter(|must_be_before|
                manual_set.contains(must_be_before)
            ).count())
        }).zip(manual.iter().copied()).collect();

        counted_manual.sort_by(|a, b| a.0.cmp(&b.0).reverse());

        counted_manual.iter().map(|&(_, v) : &(usize, i32)| v).collect()

        // we should also assert that this ordering is unique
    }
}