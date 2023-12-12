use counter::Counter;

pub fn star1() {
    let lines = include_str!("data.in").lines();
    let mut sum = 0;
    for (l, c) in lines.map(|l| l.split_once(" ").unwrap()) {
        let new_c = c;
        let new_l = [l, ".."].join("");
        
        let comp = new_c.split(",").map(|s| s.parse::<usize>().unwrap());
        let mut strings = Counter::new();
        strings.insert(new_l.clone(), 1);
        for c in comp {
            let size = c + 1;
            let mut new_strings = Counter::new();
            for (s, count) in strings.iter() {
                if s.len() >= size {
                    for i in 0..(s.len() - size) {
                        let mut ok = true;
                        ok &= !s[i..i + c].contains(".");
                        ok &= !s[0..i].contains("#");
                        ok &= !s[i + c..=i + c].contains("#");
                        if ok {
                            let key: String = s.chars().skip(i + c + 1).collect();
                            if new_strings.contains_key(&key){
                                new_strings[&key] += *count;
                            } else {
                                new_strings.insert(key, *count);   
                            }
                        }
                    }
                }
            }
            strings = new_strings;
        }

        let mut tmp: u64 = 0;
        for (s, i) in strings.iter(){
            if !s.contains("#"){
                tmp += i;
            }
        }
        sum += tmp;
    }
    print!("{}\n", sum);
}