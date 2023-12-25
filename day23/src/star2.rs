use std::collections::{HashMap, HashSet};

pub fn star2() {
    let grid: Vec<Vec<_>> = include_str!("data.in")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let mut edges = HashMap::new();

    //Build graph   
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != '#' {
                let mut e = HashMap::new();
                if (x < grid[0].len() - 1) && (grid[y][x + 1] != '#') {
                    e.insert((y, x + 1), 1);
                }

                if (y < grid.len() - 1) && (grid[y + 1][x] != '#') {
                    e.insert((y + 1, x), 1);
                }

                if (x > 0) && (grid[y][x - 1] != '#') {
                    e.insert((y, x - 1), 1);
                }

                if (y > 0) && (grid[y - 1][x] != '#') {
                    e.insert((y - 1, x), 1);
                }
                edges.insert((y, x), e);
            }
        }
    }

    //Reduce graph
    for (node, e) in edges.clone(){
        if e.len() != 2{
            let mut paths = HashMap::new();
            for ((py, px), pdist) in e{
                let (path, dist) = find_junction(&edges, HashSet::from([node]), pdist, py, px);
                paths.insert(path, dist);
            }
            edges.insert(node, paths);
        }
    }

    //Find path
    let max = explore(
        &edges,
        HashSet::new(),
        0,
        0,
        1,
        (grid.len() - 1, grid.len() - 2),
    );
    print!("{}\n", max);
}

fn find_junction(
    edges: &HashMap<(usize, usize), HashMap<(usize, usize), usize>>,
    mut visited: HashSet<(usize, usize)>,
    dist: usize,
    y: usize,
    x: usize,
) -> ((usize,usize), usize) {
    visited.insert((y,x));
    
    let paths = edges.get(&(y,x)).unwrap();
    if paths.len() != 2{
        return ((y, x), dist);
    } else {
        for ((dy, dx), ddist) in paths{
            if !visited.contains(&(*dy, *dx)){
                return find_junction(edges, visited.clone(), dist + ddist, *dy, *dx);
            }
        }
    }
    unreachable!();
}

fn explore(
    edges: &HashMap<(usize, usize), HashMap<(usize, usize), usize>>,
    mut visited: HashSet<(usize, usize)>,
    dist: usize,
    y: usize,
    x: usize,
    goal: (usize, usize),
) -> usize {
    let mut max = 0;

    if visited.contains(&(y, x)) {
        return 0;
    } else {
        visited.insert((y, x));
    }

    if (y, x) == goal {
        return dist;
    }
    for ((dy, dx), ddist) in edges.get(&(y, x)).unwrap() {
        max = max.max(explore(edges, visited.clone(), dist + ddist, *dy, *dx, goal));
    }
    return max;
}
