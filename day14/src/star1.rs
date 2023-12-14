pub fn star1() {
    let grid: Vec<Vec<u8>> = include_bytes!("data.in")
        .split(|c| c == &b'\n')
        .map(|c| c.to_vec())
        .collect::<Vec<_>>();
    //North to east
    let mut transpose: Vec<Vec<_>> = vec![];
    for y in 0..grid[0].len() {
        let mut row = vec![];
        for x in (0..grid.len()).rev(){
            row.push(grid[x][y]);
        }
        transpose.push(row);
    }
    let sum: usize = transpose.iter_mut().map(|v| {
        itertools::Itertools::intersperse(v.split(|c| c == &b'#')
            .map(|s| {
                let mut tmp = s.to_vec();
                tmp.sort();
                tmp
            }), [b'#'].to_vec())
            .flatten()
            .enumerate()
            .fold(0, |s, (i, c)| s + ((i + 1) * (c == b'O') as usize))
    }).sum();
    println!("{}", sum);
}
