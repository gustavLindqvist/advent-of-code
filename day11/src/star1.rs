pub fn star1() {
    let mut grid: Vec<Vec<_>> = include_str!("data.in")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    for i in (0..grid.len()).rev() {
        if !grid[i].iter().any(|c| c == &'#') {
            grid.insert(i + 1, vec![','; grid[0].len()]);
        }
    }
    for x in (0..grid[0].len()).rev() {
        let mut galaxy = false;
        for i in 0..grid.len() {
            galaxy |= grid[i][x] == '#';
        }
        if !galaxy {
            for i in 0..grid.len() {
                grid[i].insert(x, ',');
            }
        }
    }
    
    let mut galaxys: Vec<(i32, i32)> = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '#' {
                galaxys.push((y as i32, x as i32));
            }
        }
    }

    let mut sum = 0;
    for i in 0..galaxys.len() {
        for j in i + 1..galaxys.len() {
            let distance =
                i32::abs(galaxys[i].0 - galaxys[j].0) + i32::abs(galaxys[i].1 - galaxys[j].1);
            sum += distance;
        }
    }
    print!("{}\n", sum);
}
