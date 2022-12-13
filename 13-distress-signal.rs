use std::cmp::Ordering;
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

const INPUT_FILENAME: &str = "./13-distress-signal.input.txt";

#[derive(Debug, Clone)]
enum Packet {
    List(Vec<Packet>),
    Num(i32),
}

impl Packet {
    fn parse(s: String) -> Packet {
        let mut v: Option<i32> = None;
        let mut stack: VecDeque<Packet> = VecDeque::new();

        for (i, c) in s.bytes().enumerate() {
            if c == b'[' {
                stack.push_back(Packet::List(vec![]));
            } else if c == b']' {
                match stack.back_mut() {
                    Some(last) => {
                        match last {
                            Packet::List(a) => {
                                // Flush last v
                                match v {
                                    Some(x) => a.push(Packet::Num(x)),
                                    None => (),
                                }
                                v = None;
                            }
                            _ => panic!("pos:{}: ] encountered on a non-list", i),
                        }
                    }
                    None => panic!("pos:{}: ] encountered at top level", i),
                };
                let top = stack.pop_back().unwrap();
                // println!("top = {:?}", top);
                match stack.back_mut() {
                    None => {
                        assert!(i == s.len() - 1);
                        stack.push_back(top);
                    }
                    Some(last) => match last {
                        Packet::List(a) => a.push(top),
                        _ => panic!("pos:{}: ] encountered on a non-list parent", i),
                    },
                };
            } else if c == b',' {
                match v {
                    None => (),
                    Some(x) => match stack.back_mut() {
                        None => panic!("pos:{}: , encountered at top level", i),
                        Some(last) => match last {
                            Packet::List(a) => a.push(Packet::Num(x)),
                            _ => panic!("pos:{}: , encountered on a non-list parent", i),
                        },
                    },
                }
                v = None;
            } else {
                let ord = c as i32 - b'0' as i32;
                assert!(
                    0 <= ord && ord <= 9,
                    "pos:{}: {} encountered while expecting 0-9",
                    i,
                    c as char
                );
                let x = match v {
                    None => ord,
                    Some(x) => 10 * x + ord,
                };
                v = Some(x);
            }
        }
        stack.pop_back().unwrap()
    }

    fn cmp(a: &Packet, b: &Packet) -> Ordering {
        match a {
            Packet::Num(xa) => match b {
                Packet::Num(xb) => {
                    if xa == xb {
                        Ordering::Equal
                    } else {
                        xa.cmp(xb)
                    }
                }
                Packet::List(_) => Packet::cmp(&Packet::List(vec![Packet::Num(*xa)]), b),
            },
            Packet::List(xa) => match b {
                Packet::Num(xb) => Packet::cmp(a, &Packet::List(vec![Packet::Num(*xb)])),
                Packet::List(xb) => {
                    let mut ia = xa.iter();
                    let mut ib = xb.iter();

                    loop {
                        let va = ia.next();
                        let vb = ib.next();
                        if va.is_none() != vb.is_none() {
                            if va.is_none() {
                                return Ordering::Less;
                            } else {
                                return Ordering::Greater;
                            }
                        }
                        if va.is_none() {
                            return Ordering::Equal;
                        }
                        match Packet::cmp(va.unwrap(), vb.unwrap()) {
                            Ordering::Equal => (),
                            x => return x,
                        };
                    }
                }
            },
        }
    }
}

fn main() {
    let input = read_lines(INPUT_FILENAME);

    let mut packets = input
        .iter()
        .cloned()
        .filter(|s| !s.is_empty())
        .map(|s| Packet::parse(s))
        .collect::<Vec<_>>();

    // This is stupid, but we need vector of references.
    let mut packets = packets.iter().collect::<Vec<_>>();

    let ans1 = packets
        .chunks(2)
        .map(|p| Packet::cmp(&p[0], &p[1]))
        .enumerate()
        .filter_map(|(i, r)| if r.is_le() { Some(i) } else { None })
        .map(|i| (i + 1) as i32)
        .sum::<i32>();

    let divider1 = "[[2]]";
    let divider2 = "[[6]]";
    let packet1 = Packet::parse(divider1.to_string());
    let packet2 = Packet::parse(divider2.to_string());

    packets.push(&packet1);
    packets.push(&packet2);
    packets.sort_by(|a, b| Packet::cmp(a, b));

    let idx1 = packets
        .iter()
        .position(|p| Packet::cmp(p, &packet1).is_eq())
        .unwrap();
    let idx2 = packets
        .iter()
        .position(|p| Packet::cmp(p, &packet2).is_eq())
        .unwrap();
    let ans2 = (idx1 + 1) * (idx2 + 1);

    println!("{}\n{}", ans1, ans2);
}
