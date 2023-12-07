// pub fn oneline(){
//     [false, true].iter().map(|b| include_str!("data.in").split("\n").map(|l| l.split_whitespace().skip(1).map(|s| s.parse::<f64>().unwrap()).collect::<Vec<_>>()));
// }

pub fn oneline(){
    let (l1, l2) = include_str!("data.in").split_once("\n").unwrap();
    let t = l1.split_whitespace().skip(1).collect::<Vec<_>>().join("").parse::<f64>().unwrap();
    let d = l2.split_whitespace().skip(1).collect::<Vec<_>>().join("").parse::<f64>().unwrap();
    let solutions = f64::sqrt(f64::powi(t/2.0,2) - d).floor() * 2.0 + 1.0;
    print!("{:?}\n", solutions as u64);
}