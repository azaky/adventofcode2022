use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<String>>()
}

const INPUT_FILENAME: &str = "./16-proboscidea-volcanium.input.txt";

fn main() {
    let input = read_lines(INPUT_FILENAME);

    // <(name, flow, tunnels)>
    let valves = input
        .iter()
        .map(|line| {
            let tokens = line.split(' ').collect::<Vec<_>>();
            let name = tokens[1];
            let flow = tokens[4].as_bytes();
            let flow = std::str::from_utf8(flow[5..flow.len() - 1].borrow())
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let tunnels = tokens[9..]
                .iter()
                .map(|x| std::str::from_utf8(x.as_bytes()[..2].borrow()).unwrap())
                .collect::<Vec<_>>();
            (name, flow, tunnels)
        })
        .collect::<Vec<_>>();

    let valves_idx = valves
        .iter()
        .enumerate()
        .map(|(i, v)| (v.0, i))
        .collect::<HashMap<_, _>>();

    // remap valves names by using its index
    // valves[i] = (flow, tunnels)
    let valves = valves
        .iter()
        .map(|(_, flow, tunnels)| {
            (
                *flow,
                tunnels
                    .into_iter()
                    .map(|t| *valves_idx.get(t).unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    // Build edge list.
    let edges = valves
        .iter()
        .enumerate()
        .flat_map(|(i, (_, tunnels))| tunnels.iter().map(move |t| (i, *t)))
        .collect::<HashSet<_>>();

    // For pruning, we use bitmask with considering only nonzero valves.
    let nonzero_valves_idx = valves
        .iter()
        .enumerate()
        .filter(|(_, v)| v.0 > 0)
        .map(|(i, _)| i)
        .enumerate()
        .map(|(i, j)| (j, i))
        .collect::<HashMap<_, _>>();

    let num_states = 1usize << nonzero_valves_idx.len();

    // Start is AA
    let start = *valves_idx.get("AA").unwrap();

    let n = valves.len();

    // Task 1
    // dp[t][v][state] with flying table on t
    let m = 30usize;
    let mut dp = vec![vec![vec![-1; num_states]; n]; 2];

    dp[0][start][0] = 0;

    for real_t in 1..m + 1 {
        let t = real_t % 2;
        let u = (real_t - 1) % 2;

        // Clear first
        for v in 0..n {
            for s in 0..num_states {
                dp[t][v][s] = -1;
            }
        }

        for v in 0..n {
            for q in 0..n + 1 {
                if q != n && !edges.contains(&(q, v)) {
                    continue;
                }

                let p = if q == n { v } else { q };

                let mut addum = 0;
                let mut current_state = 0usize;
                if q == n {
                    if valves[v].0 == 0 {
                        continue;
                    }
                    current_state ^= 1usize << nonzero_valves_idx.get(&v).unwrap();
                    addum += ((m - real_t) as i32) * valves[v].0;
                }

                for prev_state in 0..num_states {
                    let x = dp[u][p][prev_state];
                    if x == -1 || (current_state & prev_state) != 0 {
                        continue;
                    }
                    let new_state = current_state ^ prev_state;
                    dp[t][v][new_state] = dp[t][v][new_state].max(x + addum);
                }
            }
        }

        // let max = *dp[t].iter().flatten().max().unwrap();
        // println!("max at t = {}: {}", real_t, max);
    }

    let ans1 = *dp[m % 2].iter().flatten().max().unwrap();

    println!("{}", ans1);

    // Task 2
    // dp[t][v_me][v_elephant][state] with flying table on t
    // WARNING: This will use A LOT of memory.
    // 2 * 59 * 59 * 2^15 * 4 bytes = 0.9GB
    let m = 26usize;
    let mut dp = vec![vec![vec![vec![-1; num_states]; n]; n]; 2];

    dp[0][start][start][0] = 0;

    for real_t in 1..m + 1 {
        let t = real_t % 2;
        let u = (real_t - 1) % 2;

        // Clear first
        for va in 0..n {
            for ve in 0..n {
                for s in 0..num_states {
                    dp[t][va][ve][s] = -1;
                }
            }
        }

        for va in 0..n {
            for qa in 0..n + 1 {
                if qa != n && !edges.contains(&(qa, va)) {
                    continue;
                }
                for ve in 0..n {
                    for qe in 0..n + 1 {
                        if qe != n && !edges.contains(&(qe, ve)) {
                            continue;
                        }

                        let pa = if qa == n { va } else { qa };
                        let pe = if qe == n { ve } else { qe };

                        let mut addum = 0;
                        let mut current_state = 0usize;
                        if qa == n {
                            if valves[va].0 == 0 {
                                continue;
                            }
                            current_state ^= 1usize << nonzero_valves_idx.get(&va).unwrap();
                            addum += ((m - real_t) as i32) * valves[va].0;
                        }
                        if qe == n {
                            if (qa == n && va == ve) || valves[ve].0 == 0 {
                                continue;
                            }
                            current_state ^= 1usize << nonzero_valves_idx.get(&ve).unwrap();
                            addum += ((m - real_t) as i32) * valves[ve].0;
                        }

                        for prev_state in 0..num_states {
                            let x = dp[u][pa][pe][prev_state];
                            if x == -1 || (current_state & prev_state) != 0 {
                                continue;
                            }
                            let new_state = current_state ^ prev_state;
                            dp[t][va][ve][new_state] = dp[t][va][ve][new_state].max(x + addum);
                        }
                    }
                }
            }
        }

        // let max = *dp[t].iter().flatten().flatten().max().unwrap();
        // println!("max at t = {}: {}", real_t, max);
    }

    let ans2 = *dp[m % 2].iter().flatten().flatten().max().unwrap();

    println!("{}", ans2);
}
