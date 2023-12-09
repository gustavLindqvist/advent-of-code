pub fn star1(){
    let seqs: Vec<Vec<i32>> = include_str!("data.in").lines().map(|l| l.split_whitespace().map(|c| c.parse().unwrap()).collect()).collect();
    let mut sum = 0;
    for mut s in seqs{
        while s.iter().any(|i| i != &0){
            sum += s.last().unwrap();
            s = s.windows(2).map(|v| v[1] - v[0]).collect();   
        }
    }
    print!("{:?}\n",sum);
}