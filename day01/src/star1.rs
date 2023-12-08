pub fn star1() {
    let sum: u32 = include_str!("data.in").lines()
    .map(|s|
         s.chars().filter(|c| c.is_digit(10)).collect()
        ).map(|s: String| 
            (s.chars().next().unwrap().to_digit(10).unwrap())*10 + 
            (s.chars().last().unwrap().to_digit(10).unwrap()))
            .sum();
    print!("{}\n", sum);
}