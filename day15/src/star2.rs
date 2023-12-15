use std::collections::HashMap;

use itertools::Itertools;
pub fn star2() {
    let mut boxes: HashMap<u8, Vec<(&[u8], u8)>> = HashMap::new();
    (0..=255).for_each(|i| {
        boxes.insert(i, vec![]);
    });
    for inst in include_bytes!("data.in").split(|c| c == &b',') {
        if inst.contains(&b'-') {
            let (l, _) = inst.split(|c| c == &b'-').collect_tuple().unwrap();
            boxes.insert(
                hash(l),
                boxes
                    .get(&hash(l))
                    .unwrap()
                    .clone()
                    .iter()
                    .filter(|(label, _)| *label != l)
                    .map(|t| *t)
                    .collect(),
            );
        } else {
            let (l, f) = inst.split(|c| c == &b'=').collect_tuple().unwrap();
            let mut vec = boxes.get(&hash(l)).unwrap().clone();
            if vec.iter().filter(|(t0, _)| *t0 == l).count() > 0 {
                boxes.insert(
                    hash(l),
                    boxes
                        .get(&hash(l))
                        .unwrap()
                        .clone()
                        .iter()
                        .map(|(t0, t1)| if *t0 == l { (l, f[0]) } else { (*t0, *t1) })
                        .collect(),
                );
            } else {
                vec.push((l, f[0]));
                boxes.insert(hash(l), vec);
            }
        }
    }
    let sum: usize = boxes
        .iter()
        .map(|(k, v)| {
            (*k as usize + 1)
                * (1..)
                    .zip(v.iter())
                    .map(|(i, (_, f))| (i * (f - 48)) as usize)
                    .sum::<usize>()
        })
        .sum();
    println!("{}\n", sum);
}

fn hash(label: &[u8]) -> u8 {
    label.iter().fold(0, |current, c| ((current + *c) * 17))
}
