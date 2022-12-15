use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>()
}

const INPUT_FILENAME: &str = "./15-beacon-exclusion-zone.input.txt";

// Sensor at x=1638847, y=3775370
fn parse_coordinates(s: &str) -> (i32, i32) {
    let s = s.split("at x=").last().unwrap();
    let c = s
        .split(", y=")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    (c[0], c[1])
}

// Sensor at x=1638847, y=3775370: closest beacon is at x=2498385, y=3565515
fn parse_line(s: &str) -> ((i32, i32), (i32, i32)) {
    let s = s.split(":").collect::<Vec<_>>();
    (parse_coordinates(s[0]), parse_coordinates(s[1]))
}

// Get intersection of the square with line y=y and bounded by range_x.
// Returns the number of intersection cells and a hole, if any.
fn calc(
    positions: &Vec<((i32, i32), (i32, i32))>,
    y: i32,
    range_x: (i32, i32),
) -> (i32, Option<i32>) {
    // Get boundaries on y=y
    let mut boundaries = positions
        .iter()
        .map(|(S, B)| {
            let d = (S.0 - B.0).abs() + (S.1 - B.1).abs() - (S.1 - y).abs();
            if d < 0 {
                None
            } else {
                Some((S.0 - d, S.0 + d))
            }
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    boundaries.sort();

    // Line-sweep
    let mut total = 0;
    let mut hole = None;
    let mut lastx = i32::MIN;
    for b in boundaries.iter() {
        if b.0 <= lastx {
            if b.1 > lastx {
                let right = b.1.min(range_x.1);
                total += right - lastx;
                lastx = right;
            }
        } else {
            if b.1 >= range_x.0 && b.0 <= range_x.1 {
                let left = b.0.max(range_x.0);
                let right = b.1.min(range_x.1);
                total += right - left + 1;
                if lastx == i32::MIN && left > range_x.0 {
                    hole = Some(range_x.0);
                } else if lastx >= range_x.0 && left > lastx + 1 {
                    hole = Some(lastx + 1);
                }
                lastx = right;
            }
        }
        // println!("sweep: after {:?}: lastx = {}, ans = {}", b, lastx, total);
    }

    let mut used_x: HashSet<i32> = HashSet::new();
    for (S, B) in positions.iter() {
        if S.1 == y {
            total -= used_x.insert(S.0) as i32;
        }
        if B.1 == y {
            total -= used_x.insert(B.0) as i32;
        }
    }

    (total, hole)
}

fn main() {
    let input = read_lines(INPUT_FILENAME);

    let positions = input
        .iter()
        .map(|s| parse_line(s.as_str()))
        .collect::<Vec<_>>();

    let limit = if INPUT_FILENAME.ends_with(".input.txt") {
        4_000_000
    } else {
        20
    };

    let ans1 = calc(&positions, limit / 2, (i32::MIN, i32::MAX)).0;

    let ans2 = {
        let hole = (0..limit)
            .map(|y| (calc(&positions, y, (0, limit)).1, y))
            .filter(|(x, y)| x.is_some())
            .map(|(x, y)| (x.unwrap(), y))
            .next()
            .unwrap();

        println!("Found hole: {:?}", hole);

        (hole.0 as i64) * 4_000_000i64 + (hole.1 as i64)
    };

    println!("{}\n{}", ans1, ans2);
}
