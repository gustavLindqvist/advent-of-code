use std::collections::HashMap;

pub fn star2() {
    let order = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
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
    let mut score = HashMap::new();
    for p in &pairs {
        let mut count = p.0.chars().collect::<counter::Counter<_>>();
        let j = count.remove(&'J').unwrap_or(0);
        let unique = count.len().max(1);
        let max = count.values().max().unwrap_or(&0) + j;
        score.insert(p.0, (5 + unique - max) as usize);
    }
    pairs.sort_by(|a, b| score.get(b.0).cmp(&score.get(a.0)));
    let score: Vec<u64> = pairs
        .iter()
        .enumerate()
        .map(|(r, t)| (r as u64 + 1) * (t.1.parse::<u64>().unwrap()))
        .collect();
    print!("{:?}\n", score.iter().sum::<u64>());
}
