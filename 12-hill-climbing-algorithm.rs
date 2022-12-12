use std::collections::vec_deque::VecDeque;
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

const INPUT_FILENAME: &str = "./12-hill-climbing-algorithm.input.txt";

const DIR: &'static [(i32, i32)] = &[(0, 1), (0, -1), (1, 0), (-1, 0)];

fn main() {
    let input = read_lines(INPUT_FILENAME);

    let mut s = (0, 0);
    let mut e = (0, 0);

    let grid = input
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(j, c)| match c {
                    b'S' => {
                        s = (i, j);
                        b'a'
                    }
                    b'E' => {
                        e = (i, j);
                        b'z'
                    }
                    x => *x,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let n = grid.len();
    let m = grid[0].len();

    let mut dist = vec![vec![i32::MAX; m]; n];
    let mut v = vec![vec![false; m]; n];
    let mut q = VecDeque::from([e]);

    dist[e.0][e.1] = 0;

    while !q.is_empty() {
        let f = q.pop_front().unwrap();
        if v[f.0][f.1] {
            continue;
        }
        v[f.0][f.1] = true;
        for dir in DIR.iter() {
            let x = (f.0 as i32 + dir.0, f.1 as i32 + dir.1);
            if x.0 < 0 || x.0 >= n as i32 || x.1 < 0 || x.1 >= m as i32 {
                continue;
            }
            let x = (x.0 as usize, x.1 as usize);
            if (grid[x.0][x.1] as i32) - (grid[f.0][f.1] as i32) < -1 {
                continue;
            }
            let d = dist[f.0][f.1] + 1;
            if d < dist[x.0][x.1] {
                dist[x.0][x.1] = d;
                q.push_back(x);
            }
        }
    }

    let ans1 = dist[s.0][s.1];
    let ans2 = grid
        .iter()
        .enumerate()
        .flat_map(|(i, r)| r.iter().enumerate().map(move |(j, c)| (i, j, *c)))
        .filter(|(i, j, c)| *c == b'a')
        .map(|(i, j, c)| dist[i][j])
        .min()
        .unwrap();

    println!("{}\n{}", ans1, ans2);
}
