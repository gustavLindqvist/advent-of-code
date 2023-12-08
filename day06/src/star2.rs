use itertools::Itertools;

pub fn star2() {
    let (t, d) = include_str!("data.in")
        .split("\n")
        .map(|l| {
            l.split_whitespace()
                .skip(1)
                .collect::<Vec<_>>()
                .join("")
                .parse::<u64>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();
    print!(
        "{:?}\n",
        (1..(t)).map(move |h| (h * (t - h) > d) as u32).sum::<u32>()
    );
}
