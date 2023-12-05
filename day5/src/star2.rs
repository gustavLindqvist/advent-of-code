pub fn star2(){
    //Slow and steady wins the race
    let (start, lines) = include_str!("data.in").split_once("\n").unwrap();
    let ruleset:Vec<Vec<Rule>> = lines.split("\n\n").map(|r| r.split_whitespace().filter(|s| s.chars().next().unwrap().is_numeric()).map(|s| s.parse().unwrap()).collect::<Vec<u64>>().windows(3).step_by(3).map(|v| Rule{to:v[0],from:v[1],length:v[2]}).collect()).collect();
    let seeds:Vec<_> = start.split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect::<Vec<u64>>();
    
    let mut min = u64::MAX;
    for i in (0..seeds.len()).step_by(2){
        println!("Halfway?");
        for mut s in seeds[i]..(seeds[i] + seeds[i+1]){
            for rules in &ruleset{
                for r in rules{
                    if (s >= r.from) & (s < (r.from + r.length)){
                        s = s - r.from + r.to;
                        break;
                    }
                }
            }
            min = min.min(s)
        }
    }
    println!("{:?}", min);
}

#[derive(Debug, Clone)]
struct Rule{
    to: u64,
    from: u64,
    length: u64,
}
// use std::collections::HashSet;
// pub fn star2() {
//     let (start, lines) = include_str!("test.in").split_once("\n").unwrap();
//     let ruleset: Vec<Vec<Rule>> = lines
//         .split("\n\n")
//         .map(|r| {
//             r.split_whitespace().filter(|s| s.chars().next().unwrap().is_numeric()).map(|s| s.parse().unwrap()).collect::<Vec<u64>>().windows(3).step_by(3).map(|v| Rule {    to: v[0],    from: v[1],    length: v[2],}).collect()
//         })
//         .collect();
//     let seeds: Vec<_> = start.split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect::<Vec<u64>>().windows(2).step_by(2).map(|v| Range {    from: v[0],    to: v[0] + v[1] - 1,}).collect();

//     let mut ranges = seeds;

//     for rules in ruleset {
//         let mut next_ranges = HashSet::<Range>::new();
//         for s in &ranges {
//             let mut update = false;
//             for r in &rules {
//                 let result = transform(s.clone(), r.clone());
//                 if result.len() > 0{
//                     update = true;
//                     next_ranges.extend(result);
//                     break;
//                 }
//             }
//             if !update{
//                 next_ranges.insert(*s);
//             }
//         }
//         dbg!(ranges.clone());
//         ranges = next_ranges.into_iter().collect();
//     }

//     println!("{:?}", ranges.iter().map(|r| r.from).min().unwrap());
// }

// fn transform(seeds: Range, rule: Rule) -> Vec<Range> {
//     let mut ranges = vec![];

//     if rule.from > seeds.to || rule.from + rule.length < seeds.from {
//         return ranges;
//     } else {
//         if rule.from > seeds.from {
//             ranges.push(Range {
//                 from: seeds.from,
//                 to: rule.from - 1,
//             });
//         }
//         if rule.from + rule.length < seeds.to {
//             ranges.push(Range {
//                 from: rule.from + rule.length + 1,
//                 to: seeds.to,
//             });
//         }

//         ranges.push(Range {
//             from: rule.from.max(seeds.from) - rule.from + rule.to,
//             to: (rule.from + rule.length).min(seeds.to) - rule.from + rule.to,
//         });
//     }

//     ranges
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
// struct Range {
//     from: u64,
//     to: u64,
// }

// #[derive(Debug, Clone)]
// struct Rule {
//     to: u64,
//     from: u64,
//     length: u64,
// }