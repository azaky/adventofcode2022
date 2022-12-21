use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead, BufReader};
use std::vec::Vec;

fn read_lines() -> Vec<String> {
    BufReader::new(stdin())
        .lines()
        .flatten()
        .collect::<Vec<String>>()
}

fn resolve_part1<'a>(
    key: &'a str,
    input: &HashMap<&'a str, Vec<&'a str>>,
    cache: &mut HashMap<&'a str, i64>,
) -> i64 {
    let existing = cache.get(key);
    if existing.is_none() {
        let tokens = input.get(key).unwrap();
        let value = if tokens.len() == 1 {
            tokens[0].parse::<i64>().unwrap()
        } else {
            let a = resolve_part1(tokens[0], input, cache);
            let b = resolve_part1(tokens[2], input, cache);
            match tokens[1] {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                _ => panic!("invalid operator {}", tokens[1]),
            }
        };
        cache.insert(key, value);
        value
    } else {
        *existing.unwrap()
    }
}

fn resolve_part2<'a>(
    key: &'a str,
    input: &HashMap<&'a str, Vec<&'a str>>,
    cache: &mut HashMap<&'a str, (f64, f64)>,
) -> (f64, f64) {
    let existing = cache.get(key);
    if existing.is_none() {
        let tokens = input.get(key).unwrap();
        let value = if tokens.len() == 1 {
            if key == "humn" {
                (1f64, 0f64)
            } else {
                (0f64, tokens[0].parse::<f64>().unwrap())
            }
        } else {
            let a = resolve_part2(tokens[0], input, cache);
            let b = resolve_part2(tokens[2], input, cache);
            match tokens[1] {
                "+" => (a.0 + b.0, a.1 + b.1),
                "-" => (a.0 - b.0, a.1 - b.1),
                "*" => {
                    if f64::abs(a.0 * b.0) > f64::EPSILON {
                        panic!("unhandled case: quadratic: {:?} * {:?}", a, b);
                    }
                    (a.0 * b.1 + a.1 * b.0, a.1 * b.1)
                }
                "/" => {
                    if b.0.abs() > f64::EPSILON || b.1.abs() <= f64::EPSILON {
                        panic!("unhandled case: division: {:?} / {:?}", a, b);
                    }
                    (a.0 / b.1, a.1 / b.1)
                }
                _ => panic!("invalid operator {}", tokens[1]),
            }
        };
        cache.insert(key, value);
        value
    } else {
        *existing.unwrap()
    }
}

fn main() {
    let input = read_lines();
    let input = input
        .iter()
        .map(|line| {
            let parts = line.split(": ").collect::<Vec<_>>();
            (parts[0], parts[1].split(" ").collect::<Vec<_>>())
        })
        .collect::<HashMap<_, _>>();

    let ans1 = {
        let mut cache: HashMap<&str, i64> = HashMap::new();
        resolve_part1("root", &input, &mut cache)
    };
    println!("{}", ans1);

    let ans2 = {
        // For part 2, always assume in form A * humn + B
        let mut cache: HashMap<&str, (f64, f64)> = HashMap::new();

        let to_resolve = input.get("root").unwrap();

        let lhs = resolve_part2(to_resolve[0], &input, &mut cache);
        let rhs = resolve_part2(to_resolve[2], &input, &mut cache);

        println!("{:?} == {:?}", lhs, rhs);

        // a.0 * x + a.1 == b.0 * x + b.1
        // x = (a.1 - b.1) / (b.0 - a.0)

        (lhs.1 - rhs.1) / (rhs.0 - lhs.0)
    };
    println!("{}", ans2);
}
