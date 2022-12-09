use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread::sleep;
use std::time::Duration;
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

fn solve(moves: &Vec<(usize, i32)>, len: usize, animate: bool) -> i32 {
    let mut rope = vec![(0, 0); len + 1];

    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    visited.insert(*rope.last().unwrap());

    let mut screen_minx = -15;
    let mut screen_maxx = 15;
    let mut screen_miny = -10;
    let mut screen_maxy = 10;

    for (j, (dir, k)) in moves.iter().cloned().enumerate() {
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

                if animate {
                    let minx = rope.iter().map(|x| x.0).min().unwrap();
                    let maxx = rope.iter().map(|x| x.0).max().unwrap();
                    let miny = rope.iter().map(|x| x.1).min().unwrap();
                    let maxy = rope.iter().map(|x| x.1).max().unwrap();
                    if screen_maxx - maxx < 5 {
                        screen_maxx += 1;
                        screen_minx += 1;
                    }
                    if minx - screen_minx < 5 {
                        screen_maxx -= 1;
                        screen_minx -= 1;
                    }
                    if screen_maxy - maxy < 3 {
                        screen_maxy += 1;
                        screen_miny += 1;
                    }
                    if miny - screen_miny < 3 {
                        screen_maxy -= 1;
                        screen_miny -= 1;
                    }

                    let mut v: HashMap<(i32, i32), char> = HashMap::new();
                    for r in rope[1..].iter().cloned() {
                        v.insert(r, 'x');
                    }
                    v.insert(rope[0], 'O');

                    print!("{}[2J", 27 as char);

                    println!(
                        "Move {:4} / {} : {} {}",
                        j + 1,
                        moves.len(),
                        DIRS.chars().nth(dir).unwrap(),
                        k
                    );
                    println!("Head position    : ({}, {})", rope[0].0, rope[0].1);
                    println!(
                        "Tail position    : ({}, {})",
                        rope.last().unwrap().0,
                        rope.last().unwrap().1
                    );
                    println!();
                    println!(
                        "{}{}{}",
                        screen_minx,
                        " ".repeat(
                            31 - screen_minx.to_string().len() - screen_maxx.to_string().len()
                        ),
                        screen_maxx
                    );
                    // println!("Top left position: ({}, {})", screenminx, screenminy);
                    for y in (screen_miny..screen_maxy + 1).rev() {
                        for x in screen_minx..screen_maxx + 1 {
                            match v.get(&(x, y)) {
                                Some(c) => print!("{}", *c),
                                None => {
                                    if visited.contains(&(x, y)) {
                                        print!(".");
                                    } else {
                                        print!(" ");
                                    }
                                }
                            }
                        }
                        if y == screen_miny || y == screen_maxy {
                            print!(" {}", y);
                        }
                        println!();
                    }
                    sleep(Duration::from_millis(5));
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

    let ans1 = solve(&moves, 1, false);
    let ans2 = solve(&moves, 9, false);

    println!("{}\n{}", ans1, ans2);
}
