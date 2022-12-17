use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead, BufReader};
use std::vec::Vec;

fn read_lines() -> Vec<String> {
    BufReader::new(stdin())
        .lines()
        .flatten()
        .collect::<Vec<String>>()
}

fn mov(v: &Vec<(i32, i32)>, d: (i32, i32)) -> Vec<(i32, i32)> {
    v.iter().map(|c| (c.0 + d.0, c.1 + d.1)).collect::<Vec<_>>()
}

fn hit(v: &Vec<(i32, i32)>, grid: &HashSet<(i32, i32)>, bounds: (i32, i32)) -> bool {
    v.iter()
        .any(|p| p.0 < bounds.0 || p.0 > bounds.1 || p.1 < 1 || grid.contains(p))
}

fn main() {
    let shapes = vec![
        // ####
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        // .#.
        // ###
        // .#.
        vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        // ..#
        // ..#
        // ###
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        // #
        // #
        // #
        // #
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        // ##
        // ##
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ];

    let input = read_lines();
    let s = input[0]
        .as_bytes()
        .iter()
        .map(|c| if *c == b'<' { -1 } else { 1 })
        .collect::<Vec<_>>();
    let mut si = 0;

    let mut h = 0;
    let mut grid: HashSet<(i32, i32)> = HashSet::new();

    let mut dh = vec![0; 0];

    let t = 2022usize;
    // let t = 10000000;
    // let t = 10;
    for ti in 0..t {
        let mut rock = mov(&shapes[ti % shapes.len()], (2, h + 4));

        for i in 0..1000000 {
            let next = if i % 2 == 0 {
                // jet
                let d = (s[si], 0);
                si = (si + 1) % s.len();
                mov(&rock, d)
            } else {
                // fall
                mov(&rock, (0, -1))
            };
            if hit(&next, &grid, (0, 6)) {
                if i % 2 == 1 {
                    break;
                }
            } else {
                rock = next;
            }
        }
        let prevh = h;
        for p in rock.iter() {
            grid.insert(*p);
            if p.1 > h {
                h = p.1;
            }
        }
        dh.push(h - prevh);

        // println!("T = {}", ti);
        // for y in (1..h + 1).rev() {
        //     for x in 0..7 {
        //         if grid.contains(&(x, y)) {
        //             print!("#");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }
        // println!("\n\n");
        // if (ti+1) % (5*s.len()) == 0 {
        if (ti+1) % 100000 == 0 {
            println!("at {}: h = {}", ti+1, h);
        }
    }

    // Cycle finding?
    // only consider length multiple of 5 * s.len();
    // {
    //     let base = 5 * s.len();
    //     for k in 1..100000 {
    //         let len = k * base;
    //         let mut cycle = true;
    //         println!("check cycle len = {}", len);
    //         if len * 2 > t {
    //             break;
    //         }
    //         for i in len..t {
    //             if dh[i] != dh[i-len] {
    //                 cycle = false;
    //                 break;
    //             }
    //         }
    //         if cycle {
    //             println!("cycle found at len = {}", cycle);
    //         }
    //     }
    // }

    println!("{}", h);
}
