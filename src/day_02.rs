use std::{num::ParseIntError, str::FromStr};

use crate::utils::{parse_lines, read_input};

pub fn main() {
    let s = read_input::<2>();

    println!("{}", part_one(&s));
    println!("{}", part_two(&s));
}

fn part_one(s: &str) -> usize {
    let commands = parse_lines::<Command>(s);

    let mut h = 0;
    let mut depth = 0;

    for Command { dir, x } in commands {
        match dir {
            Dir::Forward => {
                h += x;
            }
            Dir::Up => {
                depth -= x;
            }
            Dir::Down => {
                depth += x;
            }
        }
    }

    h * depth
}

fn part_two(s: &str) -> usize {
    let commands = parse_lines::<Command>(s);

    let mut h = 0;
    let mut aim = 0;
    let mut depth = 0;

    for Command { dir, x } in commands {
        match dir {
            Dir::Forward => {
                h += x;
                depth += aim * x;
            }
            Dir::Up => {
                aim -= x;
            }
            Dir::Down => {
                aim += x;
            }
        }
    }

    h * depth
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Forward,
    Up,
    Down,
}

impl FromStr for Dir {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir = match s {
            "forward" => Dir::Forward,
            "up" => Dir::Up,
            "down" => Dir::Down,
            _ => panic!("unknown dir: {}", s),
        };

        Ok(dir)
    }
}

#[derive(Debug, Clone, Copy)]
struct Command {
    dir: Dir,
    x: usize,
}

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, x) = s.split_once(' ').unwrap();

        Ok(Command {
            dir: dir.parse()?,
            x: x.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2
";

    #[test]
    fn part_one_example() {
        assert_eq!(part_one(EXAMPLE), 150);
    }

    #[test]
    fn part_two_example() {
        assert_eq!(part_two(EXAMPLE), 900);
    }
}
