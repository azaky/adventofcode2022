use std::collections::vec_deque::VecDeque;
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

fn read_lines(filename: &str) -> Vec<String> {
    let file = must!(File::open(filename));
    BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>()
}

const INPUT_FILENAME: &str = "./11-monkey-in-the-middle.input.txt";

#[derive(Debug, Clone)]
enum Operand {
    Num(i64),
    Old,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<i64>,
    inspect_count: i64,
    operator: char,
    operand: Operand,
    test_divisor: i64,
    test_true: usize,
    test_false: usize,
}

impl Monkey {
    fn calc(&self, num: i64) -> i64 {
        let operand = match self.operand {
            Operand::Num(s) => s,
            Operand::Old => num,
        };
        match self.operator {
            '*' => num * operand,
            '+' => num + operand,
            _ => panic!("Unknown operator"),
        }
    }

    fn inspect(&mut self) -> Option<i64> {
        match self.items.pop_front() {
            Some(x) => {
                self.inspect_count += 1;
                Some(self.calc(x))
            }
            None => None,
        }
    }

    fn push(&mut self, num: i64) {
        self.items.push_back(num);
    }

    fn get_next_monkey(&self, num: i64) -> usize {
        if num % self.test_divisor == 0 {
            self.test_true
        } else {
            self.test_false
        }
    }
}

fn do_rounds(monkeys: &Vec<Monkey>, rounds: usize, relief: i64) -> i64 {
    let mut monkeys = monkeys.clone();

    let modulo = monkeys.iter().map(|m| m.test_divisor).product::<i64>();

    for _round in 0..rounds {
        for i in 0..monkeys.len() {
            loop {
                match monkeys[i].inspect() {
                    Some(x) => {
                        // TODO: x * modinverse(relief, modulo)
                        let x = x / relief;
                        let x = x % modulo;
                        let next = monkeys[i].get_next_monkey(x);
                        monkeys[next].push(x);
                        // println!("Monkey {} inspects {} and throws it to monkey {}", i, x, next);
                    }
                    None => break,
                }
            }
        }
        // println!("Round {}", round);
        // for i in 0..monkeys.len() {
        //     println!("\tMonkey {}: {:?}", i, monkeys[i].items);
        // }
    }

    let mut inspects = monkeys.iter().map(|m| m.inspect_count).collect::<Vec<_>>();
    inspects.sort_by(|a, b| b.cmp(a));

    (inspects[0] as i64) * (inspects[1] as i64)
}

fn main() {
    let input = read_lines(INPUT_FILENAME);

    let mut monkeys = input
        .chunks(7)
        .map(|c| {
            let tokens = c
                .iter()
                .map(|l| l.trim().split(' ').collect::<Vec<_>>())
                .collect::<Vec<_>>();
            Monkey {
                items: tokens[1][2..]
                    .iter()
                    .map(|s| s.trim_matches(',').parse::<i64>().unwrap())
                    .collect::<VecDeque<_>>(),
                inspect_count: 0,
                operator: tokens[2][4].chars().nth(0).unwrap(),
                operand: match tokens[2][5].parse::<i64>() {
                    Ok(num) => Operand::Num(num),
                    _ => Operand::Old,
                },
                test_divisor: tokens[3].last().unwrap().parse::<i64>().unwrap(),
                test_true: tokens[4].last().unwrap().parse::<usize>().unwrap(),
                test_false: tokens[5].last().unwrap().parse::<usize>().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let ans1 = do_rounds(&monkeys, 20, 3);
    let ans2 = do_rounds(&monkeys, 10000, 1);

    println!("{}\n{}", ans1, ans2);
}
