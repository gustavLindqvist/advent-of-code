pub fn star1(){
    let (start, lines) = include_str!("data.in").split_once("\n").unwrap();
    let ruleset:Vec<Vec<Rule>> = lines.split("\n\n").map(|r| r.split_whitespace().filter(|s| s.chars().next().unwrap().is_numeric()).map(|s| s.parse().unwrap()).collect::<Vec<u64>>().windows(3).step_by(3).map(|v| Rule{to:v[0],from:v[1],length:v[2]}).collect()).collect();
    let mut seeds = start.split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect::<Vec<u64>>();
    
    for rules in ruleset{
        for si in 0..seeds.len(){
            for r in rules.clone(){
                let s = seeds[si];
                if (s >= r.from) & (s < (r.from + r.length)){
                    seeds[si] = s - r.from + r.to;
                    break;
                }
            }
        }
    }
    println!("{:?}", seeds.iter().min().unwrap());
}

#[derive(Debug, Clone)]
struct Rule{
    to: u64,
    from: u64,
    length: u64,
}