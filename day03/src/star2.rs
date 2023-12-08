use std::collections::{HashMap, HashSet};
pub fn star2() {
    let data = include_str!("data.in");

    let mut numbers = HashMap::<(usize, usize), i32>::new();
    for (i, l) in data.lines().enumerate() {
        let mut number = String::from("");
        let mut index = vec![];
        for (j, c) in l.chars().enumerate() {
            if c.is_ascii_digit() {
                index.push(j);
                number.push(c);
            } else {
                if number.len() != 0 {
                    for x in index {
                        numbers.insert((x, i), number.parse().unwrap());
                    }
                    number = String::from("");
                    index = vec![];
                }
            }
        }
        for x in index {
            numbers.insert((x, i), number.parse().unwrap());
        }
    }
    let mut sum = 0;
    let ymax = data.len();
    for (i, l) in data.lines().enumerate() {
        let xmax = l.len();
        for (j, c) in l.chars().enumerate() {
            if c == '*' {
                let mut gears = HashSet::new();
                let rangey = (i.wrapping_sub(1).min(i))..=((i + 1).min(ymax - 1));
                let rangex = (j.wrapping_sub(1).min(j))..=((j + 1).min(xmax - 1));
                for y in rangey {
                    for x in rangex.clone() {
                        if numbers.contains_key(&(x, y)) {
                            gears.insert(numbers.get(&(x, y)).unwrap());
                        }
                    }
                }
                if gears.len() == 2 {
                    sum += gears.iter().fold(1, |s, i| s * (**i));
                }
            }
        }
    }
    print!("{:?}\n", sum);
}
