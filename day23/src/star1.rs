use std::collections::HashSet;

pub fn star1() {
    let grid: Vec<Vec<_>> = include_str!("data.in").lines().map(|l| l.chars().collect()).collect();
    let max = explore(&grid,HashSet::new() , 0, 0, 1);
    print!("{}\n", max);
}

fn explore(grid: &Vec<Vec<char>>, mut visited: HashSet<(usize, usize)>,dist: usize, y: usize, x: usize) -> usize {
    let mut max = 0;
    
    //-1?
    if visited.contains(&(y,x)){
        return 0;
    } else {
        visited.insert((y,x));
    }
    if  (y, x) == (grid.len() - 1, grid.len() - 2) {
        return dist;
    }

    if (x < grid[0].len() -1) && (['.', '>'].contains(&grid[y][x + 1])){
        max = max.max(explore(&grid, visited.clone(), dist + 1, y, x + 1));
    }

    if (y < grid.len() -1) && (['.', 'v'].contains(&grid[y + 1][x])){
        max = max.max(explore(&grid, visited.clone(),dist + 1, y + 1, x));
    }
    
    if (x > 0) && (['.', '<'].contains(&grid[y][x - 1])){
        max = max.max(explore(&grid, visited.clone(), dist + 1, y, x - 1));
    }

    if (y > 0) && (['.', '^'].contains(&grid[y - 1][x])){
        max = max.max(explore(&grid, visited.clone(), dist + 1, y - 1, x));
    }

    return max;
}
