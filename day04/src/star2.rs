pub fn star2(){
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
        sum += count(i, &cards);
    }
    print!("{}\n", sum);
}

pub fn count(index: usize, cards: &Vec<usize>) -> usize{
    let i = cards[index];
    if i == 0{
        return 1;
    } else {
        let sum = 1 + ((index + 1)..=((index + i).min(cards.len() - 1))).map(|j| count(j, cards)).sum::<usize>();
        return sum;
    }
}