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

const input_filename: &str = "./06-tuning-trouble.input.txt";

fn calc(s: &str, k: usize) -> i32 {
    let n = s.len();
    let mut ans: i32 = n as i32;
    for i in 0..(n - k) {
        let u = String::from(&s[i..i + k]);
        let mut t = u.as_bytes().to_vec().clone();
        t.sort();
        let mut distinct = true;
        for j in 0..(k - 1) {
            if t[j] == t[j + 1] {
                distinct = false;
                break;
            }
        }
        if distinct {
            ans = (i + k) as i32;
            break;
        }
    }
    ans
}

fn main() {
    let input = read_lines(input_filename);
    let s = &input[0];
    let n = s.len();

    let ans1 = calc(s, 4);
    let ans2 = calc(s, 14);

    println!("{}\n{}", ans1, ans2);
}

fn read_lines(filename: &str) -> Vec<String> {
    let file = must!(File::open(filename));
    BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>()
}
