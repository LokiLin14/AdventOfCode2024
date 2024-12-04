mod shared;
use shared::{read_integers_from_stdin, check_data};
fn main() {
    let mut lines_read = 0;
    let mut valid_lines = 0;
    loop {
        let ints;
        match read_integers_from_stdin() {
            Ok(integers) => {
                ints = integers;
            },
            Err(err) => {
                println!("{}", err);
                break;
            }
        }
        if ints.len() == 0 {
            println!("Found empty line, exiting; \n");
            break;
        }

        if check_data(&ints, 1) {
            println!("Valid: {:?}", ints);
            valid_lines += 1;
        }
        lines_read += 1;
    }
    println!("Lines read: {}", lines_read);
    println!("Valid lines: {}", valid_lines);
}