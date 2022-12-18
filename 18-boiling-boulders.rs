use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead, BufReader};
use std::vec::Vec;

fn read_lines() -> Vec<String> {
    BufReader::new(stdin())
        .lines()
        .flatten()
        .collect::<Vec<String>>()
}

fn surface(cubes: &HashSet<(i32, i32, i32)>) -> i32 {
    let mut area = 0;
    for cube in cubes.iter() {
        area += 6;
        if cubes.contains(&(cube.0 - 1, cube.1, cube.2)) {
            area -= 1;
        }
        if cubes.contains(&(cube.0 + 1, cube.1, cube.2)) {
            area -= 1;
        }
        if cubes.contains(&(cube.0, cube.1 - 1, cube.2)) {
            area -= 1;
        }
        if cubes.contains(&(cube.0, cube.1 + 1, cube.2)) {
            area -= 1;
        }
        if cubes.contains(&(cube.0, cube.1, cube.2 - 1)) {
            area -= 1;
        }
        if cubes.contains(&(cube.0, cube.1, cube.2 + 1)) {
            area -= 1;
        }
    }
    return area;
}

fn main() {
    let mut cubes = read_lines()
        .iter()
        .map(|c| {
            let nums = c
                .split(",")
                .map(|i| i.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            (nums[0], nums[1], nums[2])
        })
        .collect::<HashSet<_>>();

    let ans1 = surface(&cubes);
    println!("{}", ans1);

    let minx = cubes.iter().map(|&(x, _, _)| x).min().unwrap() - 1;
    let maxx = cubes.iter().map(|&(x, _, _)| x).max().unwrap() + 1;
    let miny = cubes.iter().map(|&(_, y, _)| y).min().unwrap() - 1;
    let maxy = cubes.iter().map(|&(_, y, _)| y).max().unwrap() + 1;
    let minz = cubes.iter().map(|&(_, _, z)| z).min().unwrap() - 1;
    let maxz = cubes.iter().map(|&(_, _, z)| z).max().unwrap() + 1;

    // println!(
    //     "bounds: ({}, {}), y: ({}, {}), z: ({}, {})",
    //     minx, maxx, miny, maxy, minz, maxz
    // );

    let mut v: HashSet<(i32, i32, i32)> = HashSet::new();

    fn dfs(
        c: (i32, i32, i32),
        min: &(i32, i32, i32),
        max: &(i32, i32, i32),
        v: &mut HashSet<(i32, i32, i32)>,
        b: &HashSet<(i32, i32, i32)>,
    ) {
        if c.0 < min.0 || c.1 < min.1 || c.2 < min.2 || c.0 > max.0 || c.1 > max.1 || c.2 > max.2 {
            return;
        }
        if v.contains(&c) || b.contains(&c) {
            return;
        }
        v.insert(c);
        dfs((c.0 - 1, c.1, c.2), min, max, v, b);
        dfs((c.0 + 1, c.1, c.2), min, max, v, b);
        dfs((c.0, c.1 - 1, c.2), min, max, v, b);
        dfs((c.0, c.1 + 1, c.2), min, max, v, b);
        dfs((c.0, c.1, c.2 - 1), min, max, v, b);
        dfs((c.0, c.1, c.2 + 1), min, max, v, b);
    }

    dfs(
        (minx, miny, minz),
        &(minx, miny, minz),
        &(maxx, maxy, maxz),
        &mut v,
        &cubes,
    );
    for x in minx..maxx + 1 {
        for y in miny..maxy + 1 {
            for z in minz..maxz + 1 {
                let c = (x, y, z);
                if !v.contains(&c) && !cubes.contains(&c) {
                    cubes.insert(c);
                }
            }
        }
    }

    let ans2 = surface(&cubes);
    println!("{}", ans2);
}
