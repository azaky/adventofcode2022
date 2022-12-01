// From https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sum: i64 = 0;
    let mut max: i64 = 0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./01-calorie-counting.input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(value) = line {
                if value != "" {
                    sum += value.parse::<i64>().unwrap();
                } else {
                    if sum > max {
                        max = sum
                    }
                    sum = 0
                }
            }
        }
    }

    if sum > max {
        max = sum
    }

    println!("{}", max)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
