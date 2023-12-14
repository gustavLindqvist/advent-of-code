use std::collections::HashMap;

pub fn star2() {
    let grid: Vec<Vec<u8>> = include_bytes!("data.in")
        .split(|c| c == &b'\n')
        .map(|c| c.to_vec())
        .collect::<Vec<_>>();

    let mut found: HashMap<_, usize> = HashMap::new();
    let mut cycle = 0;
    let mut offset = 0;
    let mut t = grid;
    for c in 0..1000 {
        for _ in 0..4 {
            t = transpose(t);
        }
        if found.contains_key(&t) {
            offset = *found.get(&t).unwrap();
            cycle = c - offset;
            break;
        } else {
            found.insert(t.clone(), c);
        }
    }
    for _ in 0..(1000000000 - offset) % cycle - 1{
        for _ in 0..4 {
            t = transpose(t);
        }
    }
    
    //North support beam...
    let mut test: Vec<Vec<_>> = vec![];
    for y in 0..t[0].len() {
        let mut row = vec![];
        for x in (0..t.len()).rev() {
            row.push(t[x][y]);
        }
        test.push(row);
    }
    let sum: usize = test
        .iter()
        .map(|v| {
            v.iter()
                .enumerate()
                .fold(0, |s, (i, c)| s + ((i + 1) * (c == &b'O') as usize))
        })
        .sum();
    println!("{}", sum);
}

fn transpose(grid: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut transpose: Vec<Vec<_>> = vec![];
    for y in 0..grid[0].len() {
        let mut row = vec![];
        for x in (0..grid.len()).rev() {
            row.push(grid[x][y]);
        }

        transpose.push(
            itertools::Itertools::intersperse(
                row.split(|c| c == &b'#').map(|s| {
                    let mut tmp = s.to_vec();
                    tmp.sort();
                    tmp
                }),
                [b'#'].to_vec(),
            )
            .flatten()
            .collect(),
        );
    }
    transpose
}
