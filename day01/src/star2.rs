use std::collections::HashMap;
pub fn star2() {
    let parser = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    let lines = include_str!("data.in").lines().map(|l| l.to_string());
    let mut sum = 0;
    for mut l in lines {
        l.extend("ugly".chars());
        let chars = l.chars().collect::<Vec<char>>();
        let windows = chars.windows(5).collect::<Vec<_>>();
        let mut nums = vec![];
        for w in windows {
            let mut s = String::from("");
            for c in w {
                s.push(*c);
                if parser.contains_key(s.as_str()) {
                    nums.push(parser.get(s.as_str()).unwrap());
                    break;
                }
            }
        }
        sum += *nums.last().unwrap() + nums[0] * 10;
    }
    print!("{}\n", sum);
}
