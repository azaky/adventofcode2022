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

const INPUT_FILENAME: &str = "./14-regolith-reservoir.input.txt";

fn main() {
    let input = read_lines(INPUT_FILENAME);

    let paths = input
        .iter()
        .map(|line| {
            line.split(" -> ")
                .map(|s| {
                    let nums = s
                        .split(',')
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect::<Vec<_>>();
                    (nums[0] - 500, nums[1])
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let maxy = paths
        .iter()
        .flat_map(|p| p.iter().map(|c| c.1))
        .max()
        .unwrap();

    let mut grid: HashSet<(i32, i32)> = HashSet::new();

    for path in paths {
        for i in 1..path.len() {
            let mut a = path[i - 1];
            let b = path[i];
            loop {
                grid.insert(a);
                a.0 += (b.0 - a.0).signum();
                a.1 += (b.1 - a.1).signum();
                if a == b {
                    break;
                }
            }
            grid.insert(b);
        }
    }

    // println!("maxy = {}", maxy);

    fn fall(c: (i32, i32), rocks: &HashSet<(i32, i32)>, maxy: i32) -> (i32, i32) {
        let mut c = c;
        let mut changed = true;
        while changed && c.1 <= maxy {
            changed = false;
            for dx in [0, -1, 1] {
                let b = (c.0 + dx, c.1 + 1);
                if !rocks.contains(&b) {
                    c = b;
                    changed = true;
                    break;
                }
            }
        }
        c
    }

    let ans1 = {
        let mut count = 0;
        let mut grid = grid.clone();

        loop {
            let c = fall((0, 0), &grid, maxy);
            if c.1 >= maxy {
                break;
            }
            grid.insert(c);
            count += 1;
        }

        count
    };

    let ans2 = {
        let mut count = 0;
        let mut grid = grid.clone();

        loop {
            let c = fall((0, 0), &grid, maxy);
            grid.insert(c);
            count += 1;
            if c == (0, 0) {
                break;
            }
        }

        count
    };

    println!("{}\n{}", ans1, ans2);
}
