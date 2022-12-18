use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead, BufReader};
use std::time::{Duration, Instant};
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

fn bruteforce_hashset(shapes: &Vec<Vec<(i32, i32)>>, s: &Vec<i32>, t: i64) -> i64 {
    let mut si = 0;
    let mut h = 0;
    let mut grid: HashSet<(i32, i32)> = HashSet::new();

    fn hit(v: &Vec<(i32, i32)>, grid: &HashSet<(i32, i32)>, bounds: (i32, i32)) -> bool {
        v.iter()
            .any(|p| p.0 < bounds.0 || p.0 > bounds.1 || p.1 < 1 || grid.contains(p))
    }

    let t = t as usize;
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
        for p in rock.iter() {
            grid.insert(*p);
            if p.1 > h {
                h = p.1;
            }
        }

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
        if (ti + 1) % 100000 == 0 {
            println!("at {}: h = {}", ti + 1, h);
        }
    }

    h as i64
}

fn bruteforce_bitmask(shapes: &Vec<Vec<(i32, i32)>>, s: &Vec<i32>, t: i64) -> i64 {
    let start = Instant::now();

    // Change data type
    let shapes = shapes
        .iter()
        .map(|s| {
            s.iter()
                .map(|&(x, y)| (x as i32, y as usize))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Use bitmask for optimization
    let shape_bitmask = shapes
        .iter()
        .map(|s| {
            s.iter()
                .map(|&c| (1 << c.0) << ((c.1 * 8) as i32))
                .sum::<i32>()
        })
        .collect::<Vec<_>>();

    let ns = s.len();
    let nshapes = shapes.len();

    let mut si = 0;
    let mut h = 0 as usize;

    let buffer_size = (1 << 8) as usize;
    let buffer_size_mask = buffer_size - 1;
    let mut buffer = vec![0i32; buffer_size];
    buffer[0] = 0x7f;

    let mut ti = 0;
    for real_ti in 0..t {
        let rock = shape_bitmask[ti];

        // push 4 times then start falling.
        let mut d = (2, h + 1);
        for _ in 0..4 {
            let dx = s[si];
            si += 1;
            if si == ns {
                si = 0;
            }

            if dx == -1 {
                if (rock << d.0) & 0x01010101 != 0 {
                    continue;
                }
            } else {
                if (rock << d.0) & 0x40404040 != 0 {
                    continue;
                }
            }

            d.0 += dx;
        }

        let mut fall = false;
        loop {
            fall = !fall;

            if fall {
                // fall
                let base = buffer[(d.1 - 1) & buffer_size_mask]
                    ^ (buffer[d.1 & buffer_size_mask] << 8)
                    ^ (buffer[(d.1 + 1) & buffer_size_mask] << 16)
                    ^ (buffer[(d.1 + 2) & buffer_size_mask] << 24);
                let mask = rock << d.0;
                if base & mask != 0 {
                    break;
                }
                d.1 -= 1;
            } else {
                // jet
                let dx = s[si];
                si += 1;
                if si == ns {
                    si = 0;
                }

                let base = buffer[d.1 & buffer_size_mask]
                    ^ (buffer[(d.1 + 1) & buffer_size_mask] << 8)
                    ^ (buffer[(d.1 + 2) & buffer_size_mask] << 16)
                    ^ (buffer[(d.1 + 3) & buffer_size_mask] << 24);
                let mut mask = rock << d.0;

                if dx == -1 {
                    if mask & 0x1010101 != 0 {
                        continue;
                    }
                    mask >>= 1;
                } else {
                    if mask & 0x40404040 != 0 {
                        continue;
                    }
                    mask <<= 1;
                }
                if base & mask != 0 {
                    continue;
                }

                d.0 += dx;
                assert!(0 <= d.0 && d.0 <= 6);
            }
        }

        if d.1 + (buffer_size / 2) < h {
            panic!("buffer overflow. T = {}, h = {}, dy = {}", real_ti, h, d.1);
        }

        let mut rock = rock;
        for i in 0..4 {
            let y = d.1 + i;
            let mask = (rock & 0xff) << d.0;
            rock >>= 8;

            if mask == 0 {
                break;
            }

            while h < y {
                buffer[(h + 4) & buffer_size_mask] = 0;
                h += 1;
            }

            let y = y & buffer_size_mask;

            assert!(buffer[y] & mask == 0);
            buffer[y] ^= mask;
        }

        ti += 1;
        if ti == nshapes {
            ti = 0;
        }

        // println!("T = {}", real_ti);
        // for y in (1..h + 1).rev() {
        //     for x in 0..7 {
        //         if buffer[y & buffer_size_mask] & (1 << x) == 0 {
        //             print!(".");
        //         } else {
        //             print!("#");
        //         }
        //     }
        //     println!();
        // }
        // println!("\n\n");

        if (real_ti + 1) % 100_000_000 == 0 {
            println!(
                "at {}: h = {}. elapsed: {}s",
                real_ti + 1,
                h,
                start.elapsed().as_secs_f32()
            );
        }
    }

    h as i64
}

fn main() {
    let shapes = vec![
        // ####
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        // .#.
        // ###
        // .#.
        // Center removed
        vec![(0, 1), (1, 0), (1, 2), (2, 1)],
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

    let t = 2022;
    let ans1 = bruteforce_hashset(&shapes, &s, t as i64);
    println!("{}", ans1);

    let t = 1_000_000_000_000i64;
    let ans2 = bruteforce_bitmask(&shapes, &s, t);
    println!("{}", ans2);
}
