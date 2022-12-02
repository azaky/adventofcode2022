use std::io::BufRead;

const input_filename: &str = "./01-calorie-counting.input.txt";

fn main() -> Result<(), std::io::Error> {
    let mut calories: std::vec::Vec<i32> = std::vec::Vec::new();
    let mut sum = 0;

    for line in read_lines(input_filename)? {
        let line = line?;

        if line != "" {
            sum += line.parse::<i32>().unwrap();
        } else {
            calories.push(sum);
            sum = 0
        }
    }

    calories.push(sum);

    calories.sort();
    calories.reverse();

    let ans1 = calories[0];
    let ans2: i32 = calories[0 .. 3].iter().sum();

    Ok(println!("{}\n{}", ans1, ans2))
}

// From https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>>
where P: AsRef<std::path::Path>, {
    let file = std::fs::File::open(filename)?;
    Ok(std::io::BufReader::new(file).lines())
}
