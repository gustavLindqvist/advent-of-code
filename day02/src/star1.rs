pub fn star1() {
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
            .fold(true, |t, (i, c)| t
                & (((i.parse::<i32>().unwrap() < 13) & (c == "red"))
                    | ((i.parse::<i32>().unwrap() < 14) & (c == "green"))
                    | ((i.parse::<i32>().unwrap() < 15) & (c == "blue")))))
            .enumerate()
            .fold(0, |c, (i, b)| c + (i + 1) * (b as usize))
    );
}
