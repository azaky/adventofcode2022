use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead, BufReader};
use std::vec::Vec;

fn read_lines() -> Vec<String> {
    BufReader::new(stdin())
        .lines()
        .flatten()
        .collect::<Vec<String>>()
}

fn shuffle(v: &Vec<(usize, i64)>) -> Vec<(usize, i64)> {
    let mut v = v.clone();

    let n = v.len();
    let modulus = (n - 1) as i64;

    for k in 0..n {
        let (pos, &elem) = v
            .iter()
            .enumerate()
            .filter(|(pos, (i, _))| *i == k)
            .next()
            .unwrap();

        let target = ((((pos as i64) + elem.1) % modulus) + modulus) % modulus;
        let target = target as usize;

        let mut new_v = vec![(0usize, 0i64); 0];

        for &e in v.iter() {
            if new_v.len() == target {
                new_v.push(elem);
            }
            if e.0 != k {
                new_v.push(e);
            }
        }

        assert!(new_v.len() == n);
        v = new_v;

        // println!("{:?}", v);
    }

    v
}

fn sum_at(v: &Vec<(usize, i64)>, pos: &Vec<usize>) -> i64 {
    let pos0 = v.iter().position(|x| x.1 == 0).unwrap();
    pos.iter()
        .map(|&x| {
            println!("at {}: {}", x, v[(pos0 + x) % v.len()].1);
            v[(pos0 + x) % v.len()].1
        })
        .sum::<i64>()
}

fn main() {
    let v = read_lines()
        .iter()
        .enumerate()
        .map(|(i, x)| (i, x.parse::<i64>().unwrap()))
        .collect::<Vec<_>>();

    let ans1 = {
        let v = shuffle(&v);

        sum_at(&v, &vec![1000, 2000, 3000])
    };

    let ans2 = {
        let mut v = v
            .iter()
            .map(|&(i, num)| (i, num * 811589153))
            .collect::<Vec<_>>();

        // println!("initial: {:?}", v);
        for i in 0..10 {
            v = shuffle(&v);
            // println!("after {}: {:?}", i+1, v);
        }

        sum_at(&v, &vec![1000, 2000, 3000])
    };

    println!("{}", ans1);
    println!("{}", ans2);
}
