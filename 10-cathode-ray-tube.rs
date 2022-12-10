use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

macro_rules! must {
    ($s:expr) => {
        match ($s) {
            Ok(v) => v,
            Err(error) => panic!("must: Error {:?}", error),
        }
    };
}

fn read_lines(filename: &str) -> Vec<String> {
    let file = must!(File::open(filename));
    BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>()
}

const INPUT_FILENAME: &str = "./10-cathode-ray-tube.input.txt";

fn main() {
    let input = read_lines(INPUT_FILENAME);

    let mut x = 1;
    let mut cycle = 0;
    let cycles = HashMap::from([("noop", 1), ("addx", 2)]);

    let mut ans1 = 0;
    let mut ans2 = "".to_string();

    for line in input.iter() {
        let tokens = line.split(' ').collect::<Vec<_>>();
        for _ in 0..cycles.get(tokens[0]).unwrap().clone() {
            cycle += 1;

            // Part 1
            if cycle % 40 == 20 {
                ans1 += cycle * x;
            }

            // Part 2
            if i32::abs((cycle - 1) % 40 - x) <= 1 {
                ans2.push('#');
            } else {
                ans2.push(' ');
            }
            if (cycle % 40) == 0 {
                ans2.push('\n');
            }
        }
        if tokens[0] == "addx" {
            x += tokens[1].parse::<i32>().unwrap();
        }
    }

    println!("{}\n\n{}", ans1, ans2);
}
