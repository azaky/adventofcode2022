use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead, BufReader};
use std::vec::Vec;

fn read_lines() -> Vec<String> {
    BufReader::new(stdin())
        .lines()
        .flatten()
        .collect::<Vec<String>>()
}

const DIR: &'static [(i32, i32)] = &[(-1, 0), (1, 0), (0, -1), (-0, 1)];

fn main() {
    let map = read_lines()
        .iter()
        .map(|line| line.as_bytes().iter().map(|&c| c).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut pos: HashSet<(i32, i32)> = HashSet::new();
    for (i, row) in map.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == b'#' {
                pos.insert((i as i32, j as i32));
            }
        }
    }

    fn get_proposal(c: (i32, i32), pos: &HashSet<(i32, i32)>, dirs: &Vec<usize>) -> (i32, i32) {
        // Check empty
        let mut is_empty = true;
        for r in c.0 - 1..c.0 + 2 {
            for cc in c.1 - 1..c.1 + 2 {
                if c != (r, cc) && pos.contains(&(r, cc)) {
                    is_empty = false;
                    break;
                }
            }
        }
        if is_empty {
            return c;
        }

        for d in dirs.iter().map(|&d| d) {
            let mut blocked = false;
            for x in -1..2 {
                let mut dr = DIR[d].0;
                let mut dc = DIR[d].1;
                if dr == 0 {
                    dr = x;
                } else if dc == 0 {
                    dc = x;
                } else {
                    unreachable!();
                }
                let new_pos = (c.0 + dr, c.1 + dc);
                if pos.contains(&new_pos) {
                    blocked = true;
                    break;
                }
            }
            if !blocked {
                return (c.0 + DIR[d].0, c.1 + DIR[d].1);
            }
        }
        return c;
    }

    fn get_boundaries(pos: &HashSet<(i32, i32)>) -> ((i32, i32), (i32, i32)) {
        let minr = pos.iter().map(|&x| x.0).min().unwrap();
        let maxr = pos.iter().map(|&x| x.0).max().unwrap();
        let minc = pos.iter().map(|&x| x.1).min().unwrap();
        let maxc = pos.iter().map(|&x| x.1).max().unwrap();
        ((minr, maxr), (minc, maxc))
    }

    let mut dirs: Vec<usize> = vec![0, 1, 2, 3];

    let mut ans1 = -1;
    let mut ans2 = -1;

    for round in 1..i32::MAX {
        let mut proposal: HashMap<(i32, i32), i32> = HashMap::new();
        for &c in pos.iter() {
            let next_pos = get_proposal(c, &pos, &dirs);
            let existing = proposal.get(&next_pos);
            match existing {
                Some(t) => proposal.insert(next_pos, *t + 1),
                None => proposal.insert(next_pos, 1),
            };
        }

        let mut new_pos: HashSet<(i32, i32)> = HashSet::new();
        let mut moved = false;
        for &c in pos.iter() {
            let next_pos = get_proposal(c, &pos, &dirs);
            if *proposal.get(&next_pos).unwrap() > 1 {
                new_pos.insert(c);
            } else {
                if next_pos != c {
                    moved = true;
                }
                new_pos.insert(next_pos);
            }
        }

        pos = new_pos;

        dirs.push(dirs[0]);
        dirs.remove(0);

        println!("After round {}", round + 1);
        let ((minr, maxr), (minc, maxc)) = get_boundaries(&pos);
        for r in minr..maxr + 1 {
            for c in minc..maxc + 1 {
                if pos.contains(&(r, c)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();

        if round == 10 {
            ans1 = (maxr - minr + 1) * (maxc - minc + 1) - (pos.len() as i32);
        }

        if !moved {
            ans2 = round;
            break;
        }
    }

    // let ((minr, maxr), (minc, maxc)) = get_boundaries(&pos);
    println!("{}", ans1);
    println!("{}", ans2);
}
