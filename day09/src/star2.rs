pub fn star2(){
    let seqs: Vec<Vec<i32>> = include_str!("data.in").lines().map(|l| l.split_whitespace().map(|c| c.parse().unwrap()).collect()).collect();
    let mut sum = 0;
    for mut s in seqs{
        let mut first: Vec<_> = vec!();
        while s.iter().any(|i| i != &0){
            first.push(*s.first().unwrap());
            s = s.windows(2).map(|v| v[1] - v[0]).collect();   
        }
        sum += first.iter().rev().fold(0, |s, i| i - s);
    }
    print!("{:?}\n",sum);
}