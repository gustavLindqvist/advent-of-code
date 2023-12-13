pub fn star1() {
    let grids: Vec<Vec<Vec<_>>> = include_str!("test.in")
        .split("\n\n")
        .map(|g| g.lines().map(|l| l.chars().collect()).collect())
        .collect();
    let mut sum = 0;
    for g in grids {
        // X
        'x: for x in 1..(g[0].len()) {
            for offset in 0..x.min(g[0].len() - x){
                for y in 0..g.len(){
                    if g[y][x + offset] != g[y][x - 1 - offset]{
                        // println!("X: Not in: {}> <{}", x, x+1);
                        continue 'x;
                    }
                }
            }
            sum += x;
        }
        // Y
        'y: for y in 1..(g.len()) {
            for offset in 0..y.min(g.len() - y){
                for x in 0..g[0].len(){  
                    if g[y + offset][x] != g[y - 1 - offset][x]{
                        // println!("Y: Not in: {}> <{}", y, y+1);
                        continue 'y;
                    }
                }
            }
            sum += y * 100;
        }
    }
    println!("{}", sum);
}
