use std::collections::vec_deque::VecDeque;
use std::collections::HashSet;
use std::io::{stdin, BufRead, BufReader};
use std::vec::Vec;

fn read_lines() -> Vec<String> {
    BufReader::new(stdin())
        .lines()
        .flatten()
        .collect::<Vec<String>>()
}

const DIR: &'static [(i32, i32)] = &[(-1, 0), (1, 0), (0, -1), (-0, 1), (0, 0)];

fn modulo(a: i32, m: i32) -> usize {
    (((a % m) + m) % m) as usize
}

fn bfs(map: &Vec<Vec<u8>>, initial: ((i32, i32), i32), target: (i32, i32)) -> i32 {
    let n = map.len() as i32;
    let m = map[0].len() as i32;

    let mut q = VecDeque::from([initial]);
    let mut v = HashSet::from([initial]);

    while !q.is_empty() {
        let (pos, t) = q.pop_front().unwrap();

        let next = DIR
            .iter()
            .map(|&d| (pos.0 + d.0, pos.1 + d.1))
            .collect::<Vec<_>>();

        for &(r, c) in next.iter() {
            if (r, c) == target {
                return t + 1;
            }
            if !(r < 0
                || r >= n
                || c < 0
                || c >= m
                || v.contains(&((r, c), t + 1))
                || map[r as usize][c as usize] == b'#'
                || map[r as usize][modulo(c - 1 + (t + 1), m - 2) + 1] == b'<'
                || map[r as usize][modulo(c - 1 - (t + 1), m - 2) + 1] == b'>'
                || map[modulo(r - 1 + (t + 1), n - 2) + 1][c as usize] == b'^'
                || map[modulo(r - 1 - (t + 1), n - 2) + 1][c as usize] == b'v')
            {
                q.push_back(((r, c), t + 1));
                v.insert(((r, c), t + 1));
            }
        }
    }

    unreachable!()
}

fn main() {
    let map = read_lines()
        .iter()
        .map(|s| s.as_bytes().iter().map(|&c| c).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let n = map.len() as i32;
    let m = map[0].len() as i32;

    let ans1 = bfs(&map, ((0, 1), 0), (n - 1, m - 2));
    println!("{}", ans1);

    let ans2 = bfs(
        &map,
        ((0, 1), bfs(&map, ((n - 1, m - 1), ans1), (0, 1))),
        (n - 1, m - 2),
    );
    println!("{}", ans2);
}
