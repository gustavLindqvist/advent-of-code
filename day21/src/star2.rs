use std::collections::{HashSet, VecDeque};

pub fn star2() {
    let steps = 26501365;
    let mut points = vec!();
    let s = include_str!("data.in");
        let spos = s.chars().position(|c| c == 'S').unwrap();
        let width = s.chars().position(|c| c == '\n').unwrap();
        let start = ((spos / (width + 1)) as isize, (spos % (width + 1)) as isize);
        let grid = s.split('\n').map(str::as_bytes).collect::<Vec<_>>();
        
    for n in 0..=2 {
        let mut visited = HashSet::new();
        let mut q = VecDeque::new();
        let mut count = 0;

        
        q.push_back((start, 0));
        loop {
            if q.len() == 0 {
                break;
            }
            let ((y, x), dist) = q.pop_front().unwrap();
            if visited.contains(&(y, x)) {
                continue;
            } else {
                visited.insert((y, x));
            }
            if dist > (width/2 + width * n) {
                break;
            }
            // odd/even grids
            if dist % 2 == (n + 1)%2 {
                count += 1;
            }
            for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let nexty = y + dy;
                let nextx = x + dx;

                let gridy = ((y + 50 * width as isize) + dy) as usize % width;
                let gridx = ((x + 50 * width as isize) + dx) as usize % width;

                let key = (nexty, nextx);

                if grid[gridy][gridx] == b'#' {
                    continue;
                }

                q.push_back((key, dist + 1));
            }
        }
        points.push(count);
    }
    print!("{:?}\n", fitquad(steps/width, points[0], points[1], points[2]));
}

//shameless copy pasted quadratic fit function
fn fitquad(n:usize, a:usize, b:usize, c:usize) -> usize{
    a+n*(b-a) +n*(n-1)/2*((c-b)-(b-a))
}
