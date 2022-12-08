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

const input_filename: &str = "./08-treetop-tree-house.input.txt";

fn sweep(heights: &Vec<u8>) -> (Vec<bool>, Vec<i64>) {
    let n = heights.len();
    let mut visible = vec![false; n];
    let mut seen = vec![0 as i64; n];
    let mut max = heights[0];
    let mut s = vec![0 as usize];
    visible[0] = true;
    for i in 1..n {
        if heights[i] > max {
            visible[i] = true;
            max = heights[i];
        }
        while !s.is_empty() && heights[*s.last().unwrap()] < heights[i] {
            s.pop();
        }
        seen[i] = (i - match s.last() {
            Some(x) => *x,
            None => 0,
        }) as i64;
        s.push(i);
    }
    (visible, seen)
}

fn main() {
    let input = read_lines(input_filename);

    let forest: Vec<Vec<u8>> = input
        .iter()
        .map(|line| line.as_bytes().iter().map(|x| *x).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let n = forest.len();
    let m = forest[0].len();

    let mut visible = vec![vec![false; m]; n];
    let mut seen = vec![vec![1 as i64; m]; n];

    for i in 0..n {
        // From left.
        let (v, s) = sweep(&forest[i]);
        for j in 0..m {
            visible[i][j] |= v[j];
            seen[i][j] *= s[j];
        }
        // From right.
        let (v, s) = sweep(&forest[i].iter().cloned().rev().collect::<Vec<_>>());
        for j in 0..m {
            visible[i][j] |= v[m - 1 - j];
            seen[i][j] *= s[m - 1 - j];
        }
    }
    for j in 0..m {
        // From top.
        let (v, s) = sweep(&(0..n).map(|i| forest[i][j]).collect::<Vec<_>>());
        for i in 0..n {
            visible[i][j] |= v[i];
            seen[i][j] *= s[i];
        }
        // From bottom.
        let (v, s) = sweep(&(0..n).rev().map(|i| forest[i][j]).collect::<Vec<_>>());
        for i in 0..n {
            visible[i][j] |= v[n - 1 - i];
            seen[i][j] *= s[n - 1 - i];
        }
    }

    let ans1 = visible
        .iter()
        .map(|r| r.iter().map(|v| *v as i32).sum::<i32>())
        .sum::<i32>();
    let ans2 = *seen.iter().map(|r| r.iter().max().unwrap()).max().unwrap();

    println!("{}\n{}", ans1, ans2);
}
