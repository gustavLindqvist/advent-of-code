pub fn star1(){
    let data = include_str!("data.in");
    let symbols: Vec<Vec<bool>> = data.lines().map(|l| l.replace('.', "0").chars().map(|c| !c.is_ascii_digit()).collect()).collect();
    
    let mut sum = 0;
    for (i, l) in data.lines().enumerate(){

        let mut number = String::from("");
        let mut part = false;
        for (j, c) in l.chars().enumerate(){
            if c.is_ascii_digit(){
                number.push(c);
                let rangey = (i.wrapping_sub(1).min(i))..=((i+1).min(symbols.len() - 1));
                let rangex = (j.wrapping_sub(1).min(j))..=((j+1).min(symbols[0].len() - 1));
                for y in rangey{
                    for x in rangex.clone(){
                        part |= symbols[y][x];
                    }
                }
            } else {
                if number.len() != 0{
                    if part{
                        sum += number.parse::<i32>().unwrap();
                    }
                    number = String::from("");
                    part = false;
                }
            }
        }
        if part{
            sum += number.parse::<i32>().unwrap();
        }
    }
    dbg!(sum);
} 