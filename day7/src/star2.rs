use std::collections::HashMap;

pub fn star2(){
    let order = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'].iter().enumerate().map(|t| (t.1, t.0)).collect::<HashMap<_,_>>();
    let mut pairs = include_str!("data.in").lines().map(|l| l.split_once(" ").unwrap()).collect::<Vec<_>>();
    for i in (0..=4).rev(){
        pairs.sort_by(|a,b| order.get(&b.0.chars().collect::<Vec<char>>()[i]).unwrap().cmp(&order.get(&a.0.chars().collect::<Vec<char>>()[i]).unwrap()));
    }
    pairs.sort_by(|a,b| score(b.0).cmp(&score(a.0)));
    let score: Vec<u64> = pairs.iter().enumerate().map(|(r, t)| (r as u64 + 1)*(t.1.parse::<u64>().unwrap())).collect();
    print!("{:?}\n", score.iter().sum::<u64>());
}

fn score (cards: &str) -> u64{
    let mut min = u64::MAX;
    for j in ["A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2"].iter(){
        let test = cards.replace('J', j);
        let count = test.chars().collect::<counter::Counter<_>>();
        let unique = count.len();
        let max = count.values().max().unwrap();
        min = min.min((5 + unique - max) as u64);
    }
    return min;
}