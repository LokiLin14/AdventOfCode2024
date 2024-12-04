
use std::io;

fn filter_by_difference(arr: &Vec<i32>, min_diff : i32, max_diff : i32, max_drops : usize) -> bool {
    let mut dropped : usize = 0;
    let mut last_appended : Option<i32> = None;
    for &num in arr {
        match last_appended {
            None => {
                last_appended = Some(num);
            }
            Some(last) => {
                if (min_diff..=max_diff).contains(&(num - last)) {
                    last_appended = Some(num);
                } else {
                    dropped += 1;
                    if dropped > max_drops {
                        return false;
                    }
                }
            }
        }
    }
    return true;
}

fn generate_vectors<T>(original: &Vec<T>) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..original.len())
        .map(|i| original.iter()
            .enumerate()
            .filter(|(j, _)| *j != i)  // Exclude the element at index `i`
            .map(|(_, value)| value.clone())  // Clone the remaining elements
            .collect::<Vec<T>>()
        )
        .collect()
}
pub fn check_data(ints: &Vec<i32>, limit : usize) -> bool {
    if(limit == 0) {
        return filter_by_difference(ints, 1, 3, 0) ||
            filter_by_difference(ints, -3, -1, 0);
    } else if limit == 1 {
        return generate_vectors::<i32>(ints).iter().any(|drop_one|
            filter_by_difference(drop_one, 1, 3, 0)
                || filter_by_difference(drop_one, -3, -1, 0)
        );
    } else {
        unimplemented!("No idea how to do this efficiently. ")
    }


}

pub fn read_integers_from_stdin() -> Result<Vec<i32>, String> {
    let mut input = String::new();

    // Read a line from stdin
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| "Failed to read from stdin".to_string())?;

    // Parse the line into a vector of integers
    input
        .trim()
        .split_whitespace()
        .map(|word| {
            word.parse::<i32>()
                .map_err(|_| format!("Failed to parse '{}' as an integer", word))
        })
        .collect()
}