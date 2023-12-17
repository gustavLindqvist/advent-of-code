use std::collections::{BinaryHeap, HashMap};

pub fn star1() {
    let grid = include_str!("data.in")
        .split('\n')
        .map(str::as_bytes)
        .collect::<Vec<_>>();
    let mut dists = HashMap::new();
    let mut q = BinaryHeap::from_iter([(0, (0, 0, (0, 0)))]);
    let mut cost;
    let mut y;
    let mut x;
    let mut dir;

    loop {
        (cost, (y, x, dir)) = q.pop().unwrap();

        if (y, x) == (grid.len() - 1, grid[0].len() - 1) {
            break;
        }

        if dists.contains_key(&(y, x, dir)){
            if *dists.get(&(y, x, dir)).unwrap() < -cost{
                continue;
            }
        }

        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if dir == (dy, dx) || dir == (-dy, -dx) {
                continue;
            }
            let mut next_cost = -cost;
            for dist in 1..=3 {
                let nexty = (y as isize + dy * dist) as usize;
                let nextx = (x as isize + dx * dist) as usize;
                if nexty >= grid.len() || nextx >= grid[0].len() {
                    continue;
                }
                //char to int...
                next_cost += (grid[nexty][nextx] - b'0') as i64;
                
                let key = (nexty, nextx, (dy, dx));
                if next_cost < *dists.get(&key).unwrap_or(&i64::MAX) {
                    dists.insert(key, next_cost);
                    q.push((-next_cost, key));
                }
            }
        }
    }
    println!("{:?}", -cost);
}
