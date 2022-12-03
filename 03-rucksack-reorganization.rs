use std::io::BufRead;
use std::iter::FromIterator;
use std::collections::HashSet;

const input_filename: &str = "./03-rucksack-reorganization.input.txt";

fn get_value(x: u8) -> i32 {
    if b'a' <= x && x <= b'z' {
        (x - b'a' + 1) as i32
    } else {
        (x - b'A' + 27) as i32
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut ans1: i32 = 0;
    let mut ans2: i32 = 0;

    let mut group0: HashSet<i32> = HashSet::new();
    let mut group1: HashSet<i32> = HashSet::new();

    for (line_num, line) in read_lines(input_filename)?.enumerate() {
        let line = line?;

        let n: usize = line.len();

        let values: std::vec::Vec<i32> = line.bytes().into_iter().map(get_value).collect();

        // Part 1
        let chars: HashSet<i32> = values[..n/2].iter().cloned().collect();
        ans1 += values[n/2..].iter().cloned().collect::<HashSet<i32>>().intersection(&chars).next().unwrap();

        // Part 2
        let values_set: HashSet<i32> = values.iter().cloned().collect();
        match line_num % 3 {
            0 => group0 = values_set,
            1 => group1 = values_set.intersection(&group0).copied().collect::<HashSet<i32>>(),
            2 => ans2 += values_set.intersection(&group1).next().unwrap(),
            _ => panic!(),
        };
    };

    Ok(println!("{}\n{}", ans1, ans2))
}

// From https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>>
where
    P: AsRef<std::path::Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(std::io::BufReader::new(file).lines())
}
