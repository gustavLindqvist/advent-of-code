pub fn star2(){
    // let mut min = u64::MAX;

    //Slow and steady wins the race
    // let (start, lines) = include_str!("data.in").split_once("\n").unwrap();
    // let ruleset:Vec<Vec<Rule>> = lines.split("\n\n").map(|r| r.split_whitespace().filter(|s| s.chars().next().unwrap().is_numeric()).map(|s| s.parse().unwrap()).collect::<Vec<u64>>().windows(3).step_by(3).map(|v| Rule{to:v[0],from:v[1],length:v[2]}).collect()).collect();
    // let seeds:Vec<_> = start.split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect::<Vec<u64>>();
    // for i in (0..seeds.len()).step_by(2){
    //     for mut s in seeds[i]..(seeds[i] + seeds[i+1]){
    //         for rules in &ruleset{
    //             for r in rules{
    //                 if (s >= r.from) & (s < (r.from + r.length)){
    //                     s = s - r.from + r.to;
    //                     break;
    //                 }
    //             }
    //         }
    //         min = min.min(s)
    //     }
    // }

    let min = u64::MAX;
    println!("{:?}", min);
}

// #[derive(Debug, Clone)]
// struct Rule{
//     to: u64,
//     from: u64,
//     length: u64,
// }