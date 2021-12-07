use crate::utils::{parse_lines, read_input};

pub fn main() {
    let s = read_input::<1>();

    println!("{}", part_one(&s));
    println!("{}", part_two(&s));
}

fn part_one(s: &str) -> u32 {
    let mut nums = parse_lines::<u32>(s);

    let first = nums.next().unwrap();

    nums.fold((first, 0), |(prev, count), x| {
        (x, if x > prev { count + 1 } else { count })
    })
    .1
}

fn part_two(s: &str) -> u32 {
    let mut nums = parse_lines::<u32>(s);

    let mut a = nums.next().unwrap();
    let mut b = nums.next().unwrap();
    let mut c = nums.next().unwrap();

    let mut count = 0;

    for x in nums {
        if x > a {
            count += 1;
        }

        a = b;
        b = c;
        c = x;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn part_one_example() {
        assert_eq!(part_one(EXAMPLE), 7);
    }

    #[test]
    fn part_two_example() {
        assert_eq!(part_two(EXAMPLE), 5);
    }
}
