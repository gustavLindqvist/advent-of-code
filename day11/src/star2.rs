use std::collections::HashSet;

pub fn star2() {
    let grid: Vec<Vec<_>> = include_str!("data.in")
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let mut expy = HashSet::new();
    for i in (0..grid.len()).rev() {
        if !grid[i].iter().any(|c| c == &'#') {
            expy.insert(i);
        }
    }
    let mut expx = HashSet::new();

    for x in (0..grid[0].len()).rev() {
        let mut galaxy = false;
        for i in 0..grid.len() {
            galaxy |= grid[i][x] == '#';
        }
        if !galaxy {
            expx.insert(x);
        }
    }

    let mut galaxys: Vec<_> = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '#' {
                galaxys.push((y, x));
            }
        }
    }

    let size = 1000000;
    let mut sum = 0;
    for i in 0..galaxys.len() {
        for j in i + 1..galaxys.len() {

            let yrange = (galaxys[i].0.min(galaxys[j].0))..(galaxys[i].0.max(galaxys[j].0));
            let ydistance: usize = yrange
                .map(|i| if expy.contains(&i) { size } else { 1 })
                .sum();
            let xrange = (galaxys[i].1.min(galaxys[j].1))..(galaxys[i].1.max(galaxys[j].1));
            let xdistance: usize = xrange
                .map(|i| if expx.contains(&i) { size } else { 1 })
                .sum();
            sum += xdistance + ydistance;
        }
    }
    print!("{}\n", sum);
}
