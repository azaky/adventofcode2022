use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, stdin};
use std::iter::zip;
use std::vec::Vec;

fn read_lines() -> Vec<String> {
    BufReader::new(stdin())
        .lines()
        .flatten()
        .collect::<Vec<String>>()
}

// k = the number of actors
// m = time
// WARNING: This requires exponential memory with respect to k and the number of nonzero valves.
// The actual memory is 2 * n**k * 2**(nonzero valves) * 4 bytes.
// For part two, it requires around 0.9GB.
fn solve(
    valves: &Vec<(i32, Vec<usize>)>,
    edges: &HashSet<(usize, usize)>,
    k: usize,
    m: usize,
    start: usize,
) -> i32 {
    let n = valves.len();

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

    // dp[t][...positions][state] with flying table on t

    let kk = n.pow(k as u32);
    let kk1 = (n + 1).pow(k as u32);
    let mut dp = vec![vec![vec![-1; num_states]; kk]; 2];

    // To pack k values into a single value, we use base-n.

    fn serialize(v: &Vec<usize>, n: usize) -> usize {
        v.iter()
            .enumerate()
            .map(|(i, value)| value * n.pow(i as u32))
            .sum::<usize>()
    }

    fn deserialize(s: usize, k: usize, n: usize) -> Vec<usize> {
        (0..k)
            .map(|i| (s / n.pow(i as u32)) % n)
            .collect::<Vec<_>>()
    }

    dp[0][serialize(&vec![start; k], n)][0] = 0;

    for real_t in 1..m + 1 {
        let t = real_t % 2;
        let u = (real_t - 1) % 2;

        // Reset values first.
        for v in 0..kk {
            for s in 0..num_states {
                dp[t][v][s] = -1;
            }
        }

        // v is current positions, while q is previous positions.
        // We indicate q == n when current action is open valve (which means the position is unchanged).
        for vv in 0..kk {
            for qq in 0..kk1 {
                let v = deserialize(vv, k, n);
                let q = deserialize(qq, k, n + 1);

                let mut valid = true;
                let mut addum = 0;
                let mut current_state = 0usize;
                let mut p = vec![0usize; n];
                for (i, (qi, vi)) in zip(&q, &v).enumerate() {
                    let qi = *qi;
                    let vi = *vi;
                    if qi != n && !edges.contains(&(qi, vi)) {
                        valid = false;
                        break;
                    }
                    p[i] = if qi == n { vi } else { qi };
                    if qi != n || valves[vi].0 == 0 {
                        continue;
                    }
                    let b = 1usize << nonzero_valves_idx.get(&vi).unwrap();
                    if current_state & b != 0 {
                        valid = false;
                        break;
                    }
                    current_state ^= b;
                    addum += ((m - real_t) as i32) * valves[vi].0;
                }
                if !valid {
                    continue;
                }

                let pp = serialize(&p, n);

                for prev_state in 0..num_states {
                    let x = dp[u][pp][prev_state];
                    if x == -1 || (current_state & prev_state) != 0 {
                        continue;
                    }
                    let new_state = current_state ^ prev_state;
                    dp[t][vv][new_state] = dp[t][vv][new_state].max(x + addum);
                }
            }
        }

        // let max = *dp[t].iter().flatten().max().unwrap();
        // println!("t = {}, max = {}", real_t, max);
    }

    *dp[m % 2].iter().flatten().max().unwrap()
}

fn main() {
    let input = read_lines();

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

    let start = *valves_idx.get("AA").unwrap();

    let ans1 = solve(&valves, &edges, 1, 30, start);
    println!("{}", ans1);

    let ans2 = solve(&valves, &edges, 2, 26, start);
    println!("{}", ans2);
}
