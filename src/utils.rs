use std::{fmt::Debug, fs::read_to_string, str::FromStr};

pub fn read_input<const DAY: u8>() -> String {
    read_to_string(format!("inputs/day_{}.txt", DAY)).unwrap()
}

pub fn parse_lines<'a, F: FromStr>(s: &'a str) -> impl Iterator<Item = F> + 'a
where
    <F as FromStr>::Err: Debug,
{
    s.trim().lines().map(|l| l.parse::<F>().unwrap())
}
