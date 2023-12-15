pub fn star1() {
    let sum = include_bytes!("data.in")
        .split(|c| c == &b',')
        .map(|s| s.iter().fold(0, |current, c| (((current + *c as u32) * 17) % 256))).sum::<u32>();
    println!("{}", sum);
}
