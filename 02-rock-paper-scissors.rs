use std::io::BufRead;

const input_filename: &str = "./02-rock-paper-scissors.input.txt";

fn main() -> Result<(), std::io::Error> {
    let mut ans1: i32 = 0;
    let mut ans2: i32 = 0;

    for line in read_lines(input_filename)? {
        let line = line?;

        let op = (line.bytes().nth(0).unwrap() - b'A') as i32;
        let me = (line.bytes().nth(2).unwrap() - b'X') as i32;

        // Part 1: me == my move
        ans1 += {
            let shape_score = 1 + me;
            let outcome_score = if op == me { 3 }
                else if (me-op-1) % 3 == 0 { 6 }
                else { 0 };

            shape_score + outcome_score
        };

        // Part 2: me == outcome
        ans2 += {
            let outcome_score = me * 3;
            let shape_score = 1 + if me == 1 { op }
                else if me == 0 { (op + 2) % 3 }
                else { (op + 1) % 3 };

            shape_score + outcome_score
        };
    }

    Ok(println!("{}\n{}", ans1, ans2))
}

// From https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>>
where P: AsRef<std::path::Path>, {
    let file = std::fs::File::open(filename)?;
    Ok(std::io::BufReader::new(file).lines())
}
