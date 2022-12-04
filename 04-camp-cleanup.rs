use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::vec::Vec;

const input_filename: &str = "./04-camp-cleanup.input.txt";

fn main() -> Result<(), std::io::Error> {
    let values = read_lines(input_filename)?
        .iter()
        .map(|s| {
            (*s).split(',')
                .map(|t| {
                    t.split('-')
                        .map(|u| u.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>()
        })
        .collect::<Vec<Vec<Vec<i32>>>>();

    let ans1 = values
        .iter()
        .map(|v| ((v[0][0] - v[1][0]) * (v[0][1] - v[1][1]) <= 0) as i32)
        .sum::<i32>();

    let ans2 = values
        .iter()
        .map(|v| !(v[1][1] < v[0][0] || v[0][1] < v[1][0]) as i32)
        .sum::<i32>();

    Ok(println!("{}\n{}", ans1, ans2))
}

// From https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> std::io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>())
}
