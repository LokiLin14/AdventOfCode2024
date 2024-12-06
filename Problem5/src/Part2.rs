mod prelude;
use prelude::*;
fn main() {
    let prints = read_prints_from_stdio();
    println!("Prints: {:?}", prints);

    let mut sum_valid = 0;
    prints.manuals.iter().for_each(|manual| {
        if prints.is_valid_manual(manual) {
            return;
        }
        let fixed_manual = prints.get_unique_order(manual);
        println!("Fixed manual: {:?}", fixed_manual);
        let middle = fixed_manual[(fixed_manual.len() - 1) / 2];
        println!("Middle element: {:?}", middle);
        sum_valid += middle;
    });
    println!("Total valid: {}", sum_valid);
}
