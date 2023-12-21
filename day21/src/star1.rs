use std::collections::{HashSet, VecDeque};

pub fn star1() {
    let s = include_str!("data.in");
    let spos = s.chars().position(|c| c == 'S').unwrap();
    let width = s.chars().position(|c| c == '\n').unwrap();
    let start = (spos / (width + 1), spos % (width + 1));
    let grid = s.split('\n').map(str::as_bytes).collect::<Vec<_>>();
    let mut visited = HashSet::new();
    let mut q = VecDeque::new();
    let mut count = 0;
    q.push_back((start, 0));
    loop {
        if q.len() == 0 {
            break;
        }
        let ((y, x), dist) = q.pop_front().unwrap();
        if visited.contains(&(y,x)){
            continue;
        } else {
            visited.insert((y,x));
        }
        if dist > 65{
            break;
        }
        if dist % 2 == 1{
            count += 1;
        }
        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nexty = (y as isize + dy) as usize;
            let nextx = (x as isize + dx) as usize;

            if nexty >= grid.len() || nextx >= grid[0].len() {
                continue;
            }

            let key = (nexty, nextx);
            
            if grid[nexty][nextx] != b'.' {
                continue;
            }

            q.push_back((key, dist + 1));
        }
    }
    print!("{:?}\n", count);
}