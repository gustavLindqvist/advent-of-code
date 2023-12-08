pub fn star2() {
    print!(
        "{}\n",
        include_str!("data.in")
            .lines()
            .map(|l| itertools::Itertools::tuples::<(_, _)>(
                l.to_string()
                    .replace(&[',', ';', ':'][..], "")
                    .split(' ')
                    .skip(2)
            )
            .fold((0, 0, 0), |(r, g, b), (i, c)| (
                (r.max(i.parse::<u32>().unwrap() * (c == "red") as u32)),
                (g.max(i.parse::<u32>().unwrap() * (c == "green") as u32)),
                (b.max(i.parse::<u32>().unwrap() * (c == "blue") as u32)),
            )))
            .fold(0, |c, (r, g, b)| c + (r * g * b))
    );
}
