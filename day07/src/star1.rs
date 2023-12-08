use std::collections::HashMap;

pub fn star1() {
    let order = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ]
    .iter()
    .enumerate()
    .map(|t| (t.1, t.0))
    .collect::<HashMap<_, _>>();
    let mut pairs = include_str!("data.in")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .collect::<Vec<_>>();
    for i in (0..=4).rev() {
        pairs.sort_by(|a, b| {
            order
                .get(&b.0.chars().collect::<Vec<char>>()[i])
                .unwrap()
                .cmp(&order.get(&a.0.chars().collect::<Vec<char>>()[i]).unwrap())
        });
    }
    pairs.sort_by(|a, b| score(b.0).cmp(&score(a.0)));
    let score: Vec<u64> = pairs
        .iter()
        .enumerate()
        .map(|(r, t)| (r as u64 + 1) * (t.1.parse::<u64>().unwrap()))
        .collect();
    print!("{:?}\n", score.iter().sum::<u64>());
}

fn score(cards: &str) -> u64 {
    let count = cards.chars().collect::<counter::Counter<_>>();
    let unique = count.len();
    let max = count.values().max().unwrap();
    return (5 + unique - max) as u64;
}
