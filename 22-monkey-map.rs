use std::collections::vec_deque::VecDeque;
use std::collections::{vec_deque, HashMap, HashSet};
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

fn part1(map: &Vec<Vec<u8>>, steps: &Vec<Step>) -> i32 {
    let n = map.len();
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

    1000 * (pos.0 + 1) + 4 * (pos.1 + 1) + (dir as i32)
}

fn part2(map: &Vec<Vec<u8>>, steps: &Vec<Step>) -> i32 {
    let n = map.len();
    let m = map.iter().map(|line| line.len()).max().unwrap();
    let s = usize::max(m, n) / 4;

    let mut cube_map = vec![vec![0usize; 4]; 4];
    let mut cube_iter = 0usize;
    for i in 0..n {
        for j in 0..map[i].len() {
            if map[i][j] != b' ' && cube_map[i / s][j / s] == 0 {
                cube_iter += 1;
                cube_map[i / s][j / s] = cube_iter;
            }
        }
    }

    // println!("{:?}", cube_map);
    println!("cube_map:");
    for i in 0..4 {
        for j in 0..4 {
            if cube_map[i][j] > 0 {
                print!("{}", cube_map[i][j]);
            } else {
                print!(" ");
            }
        }
        println!();
    }

    // Map cube wrap
    let mut cube_pos = vec![(0usize, 0usize); 7];
    for i in 0..4 {
        for j in 0..4 {
            cube_pos[cube_map[i][j]] = (i, j);
        }
    }

    // cube_wrap[face][dir] = (face, dir)
    let mut cube_wrap = vec![vec![(0usize, 0usize); 4]; 7];
    for f in 1..7 {
        for d in 0..4 {
            // Check for direct
            let r = (cube_pos[f].0 as i32) + DIR[d].0;
            let c = (cube_pos[f].1 as i32) + DIR[d].1;
            if 0 <= r && r < 4 && 0 <= c && c < 4 && cube_map[r as usize][c as usize] != 0 {
                cube_wrap[f][d] = (cube_map[r as usize][c as usize], d);
            } else {
                // BFS? Overkill?
                let initial_pos = match d {
                    0 => (1, 2, 0),
                    1 => (2, 1, 0),
                    2 => (1, 0, 0),
                    3 => (0, 1, 0),
                    _ => unreachable!(),
                };
                let mut q = VecDeque::from([(initial_pos, f)]);
                let mut v = vec![false; 7];
                while !q.is_empty() {
                    let (pos, cf) = q.pop_front().unwrap();
                    if v[cf] {
                        continue;
                    }
                    v[cf] = true;
                    if cf != f && pos.2 == 0 {
                        let td = match pos {
                            (1, 0, 0) => 0,
                            (0, 1, 0) => 1,
                            (1, 2, 0) => 2,
                            (2, 1, 0) => 3,
                            _ => unreachable!(),
                        };
                        cube_wrap[f][d] = (cf, td);
                        break;
                    }
                    for cd in 0..4 {
                        let r = (cube_pos[cf].0 as i32) + DIR[cd].0;
                        let c = (cube_pos[cf].1 as i32) + DIR[cd].1;
                        if !(0 <= r
                            && r < 4
                            && 0 <= c
                            && c < 4
                            && cube_map[r as usize][c as usize] != 0)
                        {
                            continue;
                        }
                        let nf = cube_map[r as usize][c as usize];
                        let npos = match cd {
                            0 => (pos.0, pos.2, 2 - pos.1),
                            1 => (pos.2, pos.1, 2 - pos.0),
                            2 => (pos.0, 2 - pos.2, pos.1),
                            3 => (2 - pos.2, pos.1, pos.0),
                            _ => unreachable!(),
                        };
                        if nf != f {
                            q.push_back((npos, nf));
                        }
                    }
                }
            }

            println!("cube_wrap[{}][{}] = {:?}", f, d, cube_wrap[f][d]);
        }
    }

    let raw_pos = (0i32, map[0].iter().position(|&c| c == b'.').unwrap() as i32);
    let mut dir = 0;
    let mut f = cube_map[(raw_pos.0 as usize) / s][(raw_pos.1 as usize) / s];
    let mut pos = (raw_pos.0 % (s as i32), raw_pos.1 % (s as i32));

    println!("Initial position: {:?}, dir = {}, f = {}", pos, dir, f);

    for step in steps.iter() {
        match step {
            Step::Forward(x) => {
                // println!("Step: {:?}", step);
                for _ in 0..*x {
                    let mut next_pos = (pos.0 + DIR[dir].0, pos.1 + DIR[dir].1);
                    let mut next_f = f;
                    let mut next_dir = dir;
                    // Out of bounds / wrap around?
                    if next_pos.0 < 0
                        || next_pos.0 >= (s as i32)
                        || next_pos.1 < 0
                        || next_pos.1 >= (s as i32)
                    {
                        next_pos = (next_pos.0 + (s as i32), next_pos.1 + (s as i32));
                        while next_pos.0 >= (s as i32) {
                            next_pos.0 -= s as i32
                        }
                        while next_pos.1 >= (s as i32) {
                            next_pos.1 -= s as i32
                        }

                        next_f = cube_wrap[f][dir].0;
                        let nd = cube_wrap[f][dir].1;

                        // just rotate? left?
                        while next_dir != nd {
                            next_dir = (next_dir + 1) % 4;
                            next_pos = (next_pos.1, (s as i32) - 1 - next_pos.0);
                        }
                    }

                    let raw_pos = (
                        ((cube_pos[next_f].0 * s) as i32) + next_pos.0,
                        ((cube_pos[next_f].1 * s) as i32) + next_pos.1,
                    );
                    if map[raw_pos.0 as usize][raw_pos.1 as usize] == b'.' {
                        pos = next_pos;
                        dir = next_dir;
                        f = next_f;
                        // println!("\tmoved to {:?}, dir = {}, f = {}", pos, dir, f);
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

        println!("After {:?}: {:?}, dir = {}, f = {}", step, pos, dir, f);
    }

    let raw_pos = (
        ((cube_pos[f].0 * s) as i32) + pos.0,
        ((cube_pos[f].1 * s) as i32) + pos.1,
    );

    1000 * (raw_pos.0 + 1) + 4 * (raw_pos.1 + 1) + (dir as i32)
}

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

    let ans1 = part1(&map, &steps);
    println!("{}", ans1);

    let ans2 = part2(&map, &steps);
    println!("{}", ans2);
}
