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

const input_filename: &str = "./08-treetop-tree-house.input.txt";

fn main() {
    let input = read_lines(input_filename);

    let forest: Vec<Vec<u8>> = input.iter().map(|line| line.as_bytes().iter().map(|x| *x).collect::<Vec<_>>()).collect::<Vec<_>>();

    let n = forest.len();
    let m = forest[0].len();

    let mut visible = vec![vec![false; m]; n];
    let mut seen_l = vec![vec![0; m]; n];
    let mut seen_r = vec![vec![0; m]; n];
    let mut seen_u = vec![vec![0; m]; n];
    let mut seen_d = vec![vec![0; m]; n];
    let mut seen = vec![vec![0 as i64; m]; n];

    for i in 0..n {
        // From left.
        {
            visible[i][0] = true;
            let mut max = forest[i][0];
            seen_l[i][0] = 0;
            let mut s = vec![0 as usize];
            for j in 1..m {
                if forest[i][j] > max {
                    max = forest[i][j];
                    visible[i][j] = true;
                }
                while !s.is_empty() && forest[i][*s.last().unwrap()] < forest[i][j] {
                    s.pop();
                }
                if s.is_empty() {
                    seen_l[i][j] = j;
                } else {
                    seen_l[i][j] = j-*s.last().unwrap();
                }
                s.push(j);
            }
        }
        // From right.
        {
            visible[i][m-1] = true;
            let mut max = forest[i][m-1];
            seen_r[i][m-1] = 0;
            let mut s = vec![m-1 as usize];
            for j in (0..m-1).rev() {
                if forest[i][j] > max {
                    max = forest[i][j];
                    visible[i][j] = true;
                }
                while !s.is_empty() && forest[i][*s.last().unwrap()] < forest[i][j] {
                    s.pop();
                }
                if s.is_empty() {
                    seen_r[i][j] = m-1-j;
                } else {
                    seen_r[i][j] = *s.last().unwrap()-j;
                }
                s.push(j);
            }
        }

    }
    for j in 0..m {
        // From top.
        {
            visible[0][j] = true;
            let mut max = forest[0][j];
            seen_u[0][j] = 0;
            let mut s = vec![0 as usize];
            for i in 1..n {
                if forest[i][j] > max {
                    max = forest[i][j];
                    visible[i][j] = true;
                }
                while !s.is_empty() && forest[*s.last().unwrap()][j] < forest[i][j] {
                    s.pop();
                }
                if s.is_empty() {
                    seen_u[i][j] = i;
                } else {
                    seen_u[i][j] = i-*s.last().unwrap();
                }
                s.push(i);
            }
        }
        // From bottom.
        {
            visible[n-1][j] = true;
            let mut max = forest[n-1][j];
            seen_d[n-1][j] = 0;
            let mut s = vec![n-1 as usize];
            for i in (0..n-1).rev() {
                if forest[i][j] > max {
                    max = forest[i][j];
                    visible[i][j] = true;
                }
                while !s.is_empty() && forest[*s.last().unwrap()][j] < forest[i][j] {
                    s.pop();
                }
                if s.is_empty() {
                    seen_d[i][j] = n-1-i;
                } else {
                    seen_d[i][j] = *s.last().unwrap()-i;
                }
                s.push(i);
            }
        }
    }

    for i in (0..n) {
        for j in (0..m) {
            seen[i][j] = (seen_l[i][j] as i64) * (seen_r[i][j] as i64) * (seen_u[i][j] as i64) * (seen_d[i][j] as i64);
        }
    }

    // println!("{:?}", seen_l);
    // println!("{:?}", seen_r);
    // println!("{:?}", seen_u);
    // println!("{:?}", seen_d);

    let ans1 = visible.iter().map(|r| r.iter().map(|v| *v as i32).sum::<i32>()).sum::<i32>();
    let ans2 = *seen.iter().map(|r| r.iter().max().unwrap()).max().unwrap();

    println!("{}\n{}", ans1, ans2);
}

fn read_lines(filename: &str) -> Vec<String> {
    let file = must!(File::open(filename));
    BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>()
}
