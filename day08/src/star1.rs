use std::collections::HashMap;

use itertools::Itertools;

pub fn star1(){
    let (ins, map) = include_str!("data.in").split_once("\n\n").unwrap();
    let pairs = map.lines().map(|l| l.chars().filter(|c| !['=', ',', '(', ')'].contains(c)).collect::<String>()).collect::<Vec<_>>();
    let mut path = HashMap::new();
    pairs.iter().for_each(|s| s.split_whitespace().tuple_windows().for_each(|(t0, t1,t2)|{path.insert(t0, (t1, t2));}));
    
    let mut steps = 0;
    let mut node = "AAA";
    let mut inst = ins.chars().cycle();
    while node != "ZZZ"{
        steps += 1;
        if inst.next().unwrap() == 'L'{
            node = path.get(node).unwrap().0;
        } else {
            node = path.get(node).unwrap().1;
        }
    }
    print!("{:?}\n",steps);
}