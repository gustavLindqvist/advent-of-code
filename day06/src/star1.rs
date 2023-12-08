pub fn star1(){
    let product = itertools::Itertools::tuple_windows(include_str!("data.in").split("\n")).map(|(s1, s2)| s1.split_whitespace().zip(s2.split_whitespace()).skip(1).map(|(t1,t2)|(t1.parse().unwrap(), t2.parse().unwrap())).map(
        |(t,d)| (1..(t)).map(move |h| (h*(t-h) > d) as u32).sum::<u32>()
    ).fold(1, |p, i| p*i)).next().unwrap();
    print!("{:?}\n", product);
}