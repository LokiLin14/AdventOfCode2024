
mod prelude;
use prelude::*;

fn main() {
    let prints = read_prints_from_stdio();
    println!("Prints: {:?}", prints);

    let mut sum_valid = 0;
    prints.manuals.iter().for_each(|manual| {
        if !prints.is_valid_manual(manual) {
            return;
        }
        println! ("Valid manual: {:?}", manual);
        sum_valid += manual[(manual.len() - 1) / 2];
    });
    println!("Total valid: {}", sum_valid);
}
