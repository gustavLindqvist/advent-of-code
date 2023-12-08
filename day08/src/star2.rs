use itertools::Itertools;
use std::collections::HashMap;

pub fn star2() {
    let (ins, map) = include_str!("data.in").split_once("\n\n").unwrap();
    let pairs = map
        .lines()
        .map(|l| {
            l.chars()
                .filter(|c| !['=', ',', '(', ')'].contains(c))
                .collect::<String>()
        })
        .collect::<Vec<_>>();
    let mut path = HashMap::new();
    pairs.iter().for_each(|s| {
        s.split_whitespace()
            .tuple_windows()
            .for_each(|(t0, t1, t2)| {
                path.insert(t0, (t1, t2));
            })
    });

    let nodes = path.keys().filter(|s| s.ends_with('A')).collect_vec();
    let mut steps = vec![];
    for n in nodes {
        let mut node = *n;
        let mut step = 0;
        let mut inst = ins.chars().cycle();
        while !node.ends_with('Z') {
            step += 1;
            if inst.next().unwrap() == 'L' {
                node = path.get(node).unwrap().0;
            } else {
                node = path.get(node).unwrap().1;
            }
        }
        steps.push(step);
    }
    print!("{:?}\n", (steps.iter().fold(1, |f, n| lcm(f, *n))));
}

use std::cmp::{max, min};

fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
