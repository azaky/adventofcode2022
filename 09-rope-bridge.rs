use std::collections::HashSet;
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

const INPUT_FILENAME: &str = "./09-rope-bridge.input.txt";

const DIRS: &str = "UDRL";
const DIR: &'static [(i32, i32)] = &[(0, 1), (0, -1), (1, 0), (-1, 0)];

fn solve(moves: &Vec<(usize, i32)>, len: usize) -> i32 {
    let mut rope = vec![(0, 0); len + 1];

    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    visited.insert(*rope.last().unwrap());

    for (dir, k) in moves.iter().cloned() {
        for _ in 0..k {
            rope[0].0 += DIR[dir].0;
            rope[0].1 += DIR[dir].1;

            for i in 1..len + 1 {
                let dist = i32::max(
                    (rope[i - 1].0 - rope[i].0).abs(),
                    (rope[i - 1].1 - rope[i].1).abs(),
                );
                if dist > 1 {
                    if (rope[i - 1].0 - rope[i].0).abs() == 2 {
                        rope[i].0 = (rope[i - 1].0 + rope[i].0) / 2;
                        rope[i].1 = rope[i - 1].1;
                    } else {
                        rope[i].1 = (rope[i - 1].1 + rope[i].1) / 2;
                        rope[i].0 = rope[i - 1].0;
                    }
                }
            }

            visited.insert(*rope.last().unwrap());
        }
    }

    visited.len() as i32
}

fn main() {
    let input = read_lines(INPUT_FILENAME);

    let moves = input
        .iter()
        .map(|line| {
            let tokens = line.split(' ').collect::<Vec<_>>();
            let dir = tokens[0].chars().nth(0).unwrap();
            let i = DIRS.chars().position(|c| c == dir).unwrap();
            let k = tokens[1].parse::<i32>().unwrap();
            (i, k)
        })
        .collect::<Vec<_>>();

    let ans1 = solve(&moves, 1);
    let ans2 = solve(&moves, 9);

    println!("{}\n{}", ans1, ans2);
}
