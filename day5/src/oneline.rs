use std::{collections::HashMap, ops::Range};
pub fn oneline(){
    //NAH this is gonna be rough
    let (start, lines) = include_str!("test.in").split_once("\n").unwrap();
    let ruleset:Vec<Vec<Rule>> = lines.split("\n\n").map(|r| r.split_whitespace().filter(|s| s.chars().next().unwrap().is_numeric()).map(|s| s.parse().unwrap()).collect::<Vec<i64>>().windows(3).step_by(3).map(|v| Rule{to:v[0],from:v[1],length:v[2]}).collect()).collect();
    let seeds:Vec<_> = start.split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect::<Vec<i64>>();

    let mut transforms: Vec<HashMap<Range<i64>, i64>>= vec!();
    for t in ruleset{
        let mut tmp = HashMap::new();
        for r in t{
            tmp.insert(r.from..(r.from + r.length), r.to - r.from);
        }
        transforms.push(tmp);
    }
    let trans = transforms.clone();
    let min = seeds.windows(2).step_by(2).fold(i64::MAX, |min, v| min.min((v[0]..(v[0] + v[1])).fold(i64::MAX, |min, seed|(0..trans.len()).fold(seed, |s, i| s + *trans[i].iter().filter(|(k,v)| k.contains(&s)).map(|(_,v)|v).next().unwrap_or_else(||&0)))));

    println!("{:?}", min);
}


#[derive(Debug, Clone)]
struct Rule{
    to: i64,
    from: i64,
    length: i64,
}