use itertools::Itertools;

pub fn star1() {
    let lines = include_str!("data.in").lines();
    let mut sum = 0;
    for (l, c) in lines.map(|l| l.split_once(" ").unwrap()) {

        let comp = c.split(",").map(|s| s.parse::<usize>().unwrap());
        let comp_len = comp.clone().count();
        let mut perms = vec![l.to_string().replace(".", " ")];
        
        for (i, c) in l.chars().enumerate() {
            if c == '?' {
                let mut new_perms = vec![];
                for p in perms {
                    let mut s1 = p.clone();
                    s1.replace_range(i..i + 1, " ");
                    new_perms.push(s1);
                    let mut s2 = p.clone();
                    s2.replace_range(i..i + 1, "#");
                    new_perms.push(s2);
                }
                perms = new_perms;
            }
        }
        let tmp = perms
            .iter()
            .map(|s| s.split_whitespace().collect_vec())
            .filter(|s| s.len() == comp_len)
            .map(|s| {
                s.iter()
                    .map(|s| s.len())
                    .zip(comp.clone())
                    .map(|(t0, t1)| t0 == t1)
                    .fold(true, |b, n| b & n)
            })
            .filter(|b| *b)
            .count();

        sum += tmp;
    }
    print!("{}\n", sum);
}
