pub fn star2(){
    let (l1, l2) = include_str!("data.in").split_once("\n").unwrap();
    let t = l1.split_whitespace().skip(1).collect::<Vec<_>>().join("").parse::<u64>().unwrap();
    let d = l2.split_whitespace().skip(1).collect::<Vec<_>>().join("").parse::<u64>().unwrap();
    let solutions = (1..(t)).map(move |h| (h*(t-h) > d) as u32).sum::<u32>();
    print!("{:?}\n", solutions);
}