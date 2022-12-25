use std::io::{stdin, BufRead, BufReader};
use std::vec::Vec;

fn read_lines() -> Vec<String> {
    BufReader::new(stdin())
        .lines()
        .flatten()
        .collect::<Vec<String>>()
}

fn snafu_to_decimal(s: &String) -> i64 {
    let mut num = 0;
    for c in s.as_bytes().iter() {
        num = num * 5
            + match c {
                b'2' => 2,
                b'1' => 1,
                b'0' => 0,
                b'-' => -1,
                b'=' => -2,
                _ => unreachable!(),
            };
    }
    num
}

fn decimal_to_snafu(n: i64) -> String {
    let mut s = vec![0u8; 0];
    let mut n = n;
    while n != 0 {
        // pick last digit in -2..2
        let d = ((n + 2) % 5) - 2;
        n -= d;
        assert!(n % 5 == 0);
        n /= 5;
        s.push(match d {
            -2 => b'=',
            -1 => b'-',
            0 => b'0',
            1 => b'1',
            2 => b'2',
            _ => unreachable!(),
        });
    }
    s.reverse();
    String::from_utf8(s).unwrap()
}

fn main() {
    let input = read_lines();

    let sum = input.iter().map(|s| snafu_to_decimal(s)).sum::<i64>();
    println!("{}", sum);
    println!("{}", decimal_to_snafu(sum));
}
