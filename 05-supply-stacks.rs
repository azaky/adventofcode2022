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

const input_filename: &str = "./05-supply-stacks.input.txt";

fn main() {
    let input = read_lines(input_filename);

    let split_at = input.iter().position(|line| line.is_empty()).unwrap();

    let n = (input[split_at - 1].len() + 1) / 4;
    let mut stacks: Vec<Vec<u8>> = vec![vec![0; 0]; n as usize];

    // Build stacks
    for line in input[..split_at].iter() {
        for (i, item) in line.as_bytes().chunks(4).enumerate() {
            if item[0] == b'[' {
                stacks[i].push(item[1]);
            }
        }
    }

    for stack in stacks.iter_mut() {
        stack.reverse();
    }
    let mut stacks1 = stacks.clone();
    let mut stacks2 = stacks.clone();

    for line in input[split_at + 1..].iter() {
        let moves = line.split(' ').collect::<Vec<_>>();
        let k = moves[1].parse::<usize>().unwrap();
        let from = moves[3].parse::<usize>().unwrap() - 1;
        let to = moves[5].parse::<usize>().unwrap() - 1;

        // println!("move {} from {} to {}", k, from, to);

        // Part 1
        for _ in 0..k {
            let s = stacks1[from].len() - 1;
            let item = stacks1[from].split_off(s)[0];
            stacks1[to].push(item);
        }
        // println!("{:?}", stacks1);

        // Part 2
        {
            let s = stacks2[from].len() - k;
            let mut items = stacks2[from].split_off(s);
            stacks2[to].append(&mut items);
        };
        // println!("{:?}", stacks2);
    }

    let ans1 = must!(String::from_utf8(
        stacks1.iter().map(|s| s[s.len() - 1]).collect::<Vec<_>>()
    ));
    let ans2 = must!(String::from_utf8(
        stacks2.iter().map(|s| s[s.len() - 1]).collect::<Vec<_>>()
    ));

    println!("{}\n{}", ans1, ans2);
}

fn read_lines(filename: &str) -> Vec<String> {
    let file = must!(File::open(filename));
    BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>()
}
