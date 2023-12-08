pub fn star1(){
    let lines = include_str!("data.in").lines()
    .map(|l| l.split(' ').filter(|s| *s != "").skip(2).collect::<Vec<_>>());
    let mut cards = vec!();
    for mut l in lines{
        l.sort();
        
        let i = l.windows(2).fold(0, |c, i| c + (i[0] == i[1]) as usize);
        cards.push(i);
    }
    let mut sum = 0;
    for i in 0..cards.len(){
        sum += u32::pow(2, cards[i] as u32) / 2;
    }
    print!("{}\n", sum);
}