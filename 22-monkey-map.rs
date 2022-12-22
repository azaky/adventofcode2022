use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead, BufReader};
use std::vec::Vec;

fn read_lines() -> Vec<String> {
    BufReader::new(stdin())
        .lines()
        .flatten()
        .collect::<Vec<String>>()
}

#[derive(Debug, Clone, Copy)]
enum Step {
    Forward(usize),
    Turn(u8),
}

const DIR: &'static [(i32, i32)] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let input = read_lines();

    let sep = input.iter().position(|line| line.len() == 0).unwrap();

    let mut steps: Vec<Step> = vec![];
    let mut num = 0;
    for &c in input[sep + 1].as_bytes().iter() {
        if b'0' <= c && c <= b'9' {
            num = 10 * num + (c - b'0') as usize;
        } else if c == b'L' || c == b'R' {
            steps.push(Step::Forward(num));
            num = 0;
            steps.push(Step::Turn(c));
        }
    }
    if num > 0 {
        steps.push(Step::Forward(num));
    }
    println!("steps = {:?}", steps);

    let map = input[0..sep]
        .iter()
        .map(|line| line.as_bytes().iter().map(|&x| x).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let n = sep;
    let m = map.iter().map(|line| line.len()).max().unwrap();

    // calculate (first, last) for each rows and columns
    let row_range = (0..n)
        .map(|r| {
            let first = map[r].iter().position(|&x| x != b' ').unwrap();
            let last = map[r].len() - 1;
            (first as i32, last as i32)
        })
        .collect::<Vec<_>>();
    let col_range = (0..m)
        .map(|c| {
            let first = (0..n)
                .position(|r| map[r].len() > c && map[r][c] != b' ')
                .unwrap();
            let last = (0..n)
                .rev()
                .filter(|&r| map[r].len() > c && map[r][c] != b' ')
                .next()
                .unwrap();
            (first as i32, last as i32)
        })
        .collect::<Vec<_>>();
    println!("row: {:?}", row_range);
    println!("col: {:?}", col_range);

    let mut pos = (0, map[0].iter().position(|&c| c == b'.').unwrap() as i32);
    let mut dir = 0;

    println!("Initial position: {:?}", pos);

    for step in steps.iter() {
        match step {
            Step::Forward(x) => {
                for _ in 0..*x {
                    let mut next = (pos.0 + DIR[dir].0, pos.1 + DIR[dir].1);
                    // Out of bounds / wrap around?
                    if next.0 < 0
                        || next.0 >= (n as i32)
                        || next.1 < 0
                        || next.1 >= (m as i32)
                        || map[next.0 as usize].len() <= next.1 as usize
                        || map[next.0 as usize][next.1 as usize] == b' '
                    {
                        next = match dir {
                            0 => (pos.0, row_range[pos.0 as usize].0),
                            1 => (col_range[pos.1 as usize].0, pos.1),
                            2 => (pos.0, row_range[pos.0 as usize].1),
                            3 => (col_range[pos.1 as usize].1, pos.1),
                            _ => unreachable!(),
                        };
                    }
                    assert!(
                        0 <= next.0
                            && next.0 < (n as i32)
                            && 0 <= next.1
                            && next.1 < (m as i32)
                            && (next.1 as usize) < map[next.0 as usize].len()
                            && map[next.0 as usize][next.1 as usize] != b' '
                    );
                    if map[next.0 as usize][next.1 as usize] == b'.' {
                        pos = next;
                    } else {
                        break;
                    }
                }
            }
            Step::Turn(x) => {
                if *x == b'L' {
                    dir = (dir + 3) % 4;
                } else if *x == b'R' {
                    dir = (dir + 1) % 4;
                }
            }
        }
        println!("After {:?}: {:?}, dir = {}", step, pos, dir);
    }

    let ans1 = 1000 * (pos.0 + 1) + 4 * (pos.1 + 1) + (dir as i32);
    println!("{}", ans1);
}
