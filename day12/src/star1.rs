use counter::Counter;

pub fn star1() {
    let lines = include_str!("data.in").lines();
    let mut sum = 0;
    for (l, c) in lines.map(|l| l.split_once(" ").unwrap()) {
        // let new_c = [c; 5].join(",");
        // let new_l = [[l; 5].join("?"), "..".to_string()].join("");
        let new_c = c;
        let new_l = [l, ".."].join("");
        
        let comp = new_c.split(",").map(|s| s.parse::<usize>().unwrap());
        let mut strings:Counter<usize, u64> = Counter::new();
        let string = new_l;
        strings.insert(0,1);
        for c in comp {
            let size = c + 1;
            let mut new_strings = Counter::new();
            for (index, count) in strings.iter() {
                if string.len() - index >= size {
                    for offset in 0..(string.len() - index - size) {
                        let i = index + offset;
                        let mut ok = true;
                        ok &= !string[i..i + c].contains(".");
                        ok &= !string[*index..i].contains("#");
                        ok &= !string[i + c..=i + c].contains("#");
                        if ok {
                            let key = i + c + 1;
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
            if !string[*s..string.len()].contains("#"){
                tmp += i;
            }
        }
        sum += tmp;
    }
    print!("{}\n", sum);
}