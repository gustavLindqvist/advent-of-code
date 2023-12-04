pub fn oneline(){print!("{:?}\n",include_str!("data.in").lines()
    .map(|l| l.split(' ').filter(|s| *s != "").skip(2).collect::<counter::Counter<_>>())
    .map(|c| c.values().filter(|i| **i == 2).count())
    .fold(((0,0),vec![1; 256]), |((p1, p2), v), i| ((p1 + (u32::pow(2, i as u32) / 2), p2 + v[0]), v.iter().skip(1).zip(std::iter::repeat(v[0]).take(i).chain(std::iter::repeat(0))).map(|(i,j)|i+j).collect::<Vec<_>>())).0);
}