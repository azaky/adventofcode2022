use std::collections::HashMap;
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

const input_filename: &str = "./07-no-space-left-on-device.input.txt";

fn main() {
    let input = read_lines(input_filename);

    let mut current_path = vec!["".to_string(); 0];
    let mut file_size: HashMap<String, i32> = HashMap::new();
    let mut children: HashMap<String, Vec<String>> = HashMap::new();

    for line in input {
        let tokens = line
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        if tokens[0] == "$" {
            // Command
            let cmd = tokens[1].clone();
            if cmd == "cd" {
                if tokens[2] == "/" {
                    current_path.clear();
                } else if tokens[2] == ".." {
                    current_path.pop();
                } else {
                    current_path.push(tokens[2].clone());
                }
            } else if cmd == "ls" {
                // println!("ls {}", current_path.join("/"));
            }
        } else {
            // We're in ls
            let p = current_path.join("/").to_string();
            if tokens[0] == "dir" {
                match children.get_mut(&p) {
                    Some(c) => {
                        c.push(tokens[1].clone());
                    }
                    None => {
                        children.insert(p, vec![tokens[1].clone(); 1]);
                    }
                }
            } else {
                let size = tokens[0].parse::<i32>().unwrap()
                    + match file_size.get(&p) {
                        Some(s) => *s,
                        None => 0,
                    };
                file_size.insert(p, size);
            }
        }
    }

    fn dfs(
        p: String,
        children: &HashMap<String, Vec<String>>,
        file_size: &mut HashMap<String, i32>,
    ) -> i32 {
        let total_size = match children.get(&p) {
            Some(c) => c
                .iter()
                .map(|v| {
                    let dir = if p == "" {
                        v.clone()
                    } else {
                        [p.clone(), v.clone()].join("/").to_string()
                    };
                    dfs(dir, children, file_size)
                })
                .sum::<i32>(),
            None => 0,
        } + match file_size.get(&p) {
            Some(x) => *x,
            None => 0,
        };
        file_size.insert(p, total_size);
        total_size
    }

    dfs("".to_string(), &children, &mut file_size);

    // for (c, s) in file_size.iter() {
    //     println!("size {}: {}", c, s);
    // }

    let total_size = *(file_size.get("").unwrap());

    let ans1 = file_size.values().filter(|s| **s <= 100000).sum::<i32>();
    let ans2 = *(file_size
        .values()
        .filter(|s| **s >= total_size - 40_000_000)
        .min()
        .unwrap());

    println!("{}\n{}", ans1, ans2);
}

fn read_lines(filename: &str) -> Vec<String> {
    let file = must!(File::open(filename));
    BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>()
}
