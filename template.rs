use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, stdin};
use std::vec::Vec;

fn read_lines() -> Vec<String> {
    BufReader::new(stdin())
        .lines()
        .flatten()
        .collect::<Vec<String>>()
}

fn main() {
    let input = read_lines();

    
}
