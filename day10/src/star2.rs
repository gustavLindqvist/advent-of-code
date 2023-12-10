use std::collections::HashSet;

pub fn star2() {
    let grid: Vec<_> = include_str!("data.in").chars().collect();
    let width = grid.iter().position(|c| c == &'\n').unwrap();
    let start = grid.iter().position(|c| c == &'S').unwrap();

    let mut cycle = HashSet::new();

    //dir: Clockwise from north 0,1,2,3
    let (mut pos, mut dir) = (start - 1, 3);
    if ['|', '7', 'F'].contains(&grid[start - width - 1]) {
        (pos, dir) = (start - width - 1, 0);
    } else if ['|', 'L', 'J'].contains(&grid[start + width + 1]) {
        (pos, dir) = (start + width + 1, 2);
    }
    loop {
        cycle.insert(pos);
        match (grid[pos], dir) {
            ('|', 2) => pos += width + 1,
            ('|', 0) => pos -= width + 1,

            ('-', 3) => pos -= 1,
            ('-', 1) => pos += 1,

            ('L', 2) => {
                pos += 1;
                dir = 1;
            }
            ('L', 3) => {
                pos -= width + 1;
                dir = 0;
            }

            ('F', 0) => {
                pos += 1;
                dir = 1;
            }
            ('F', 3) => {
                pos += width + 1;
                dir = 2;
            }

            ('J', 1) => {
                pos -= width + 1;
                dir = 0;
            }
            ('J', 2) => {
                pos -= 1;
                dir = 3;
            }

            ('7', 0) => {
                pos -= 1;
                dir = 3;
            }
            ('7', 1) => {
                pos += width + 1;
                dir = 2;
            }
            
            ('S', _) => break,
            (_, _) => {},
        }
    }

    let mut lastpipe = '|';
    let mut inside = false;
    let mut count = 0;
    for i in 0..grid.len() {
        if grid[i] == '\n' {
            lastpipe = '|';
            inside = false;
        }
        let pipe = cycle.contains(&i);
        if pipe {
            let c = grid[i];
            match c {
                '|' => {
                    lastpipe = '|';
                    inside = !inside;
                }
                'F' => lastpipe = 'F',
                'L' => lastpipe = 'L',
                'J' => {
                    if lastpipe == 'F' {
                        inside = !inside;
                    }
                    lastpipe = 'J';
                }
                '7' => {
                    if lastpipe == 'L' {
                        inside = !inside;
                    }
                    lastpipe = '7';
                }
                _ => {}
            }
        } else if inside {
            count += 1;
        }
    }
    print!("{}\n", count);
}
