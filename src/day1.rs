use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Rotation {
    L(i32),
    R(i32),
}

impl FromStr for Rotation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, amount) = s.split_at(1);
        Ok(match dir {
            "L" => Rotation::L(amount.parse().unwrap()),
            "R" => Rotation::R(amount.parse().unwrap()),
            _ => panic!("Unrecognized rotation"),
        })
    }
}

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<Rotation> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[derive(Debug, Clone)]
struct Dial(i32);

impl Dial {
    fn is_zero(&self) -> bool {
        self.0 == 0
    }

    fn rotate(&mut self, rotation: Rotation) {
        self.0 = match rotation {
            Rotation::L(amount) => self.0 - amount,
            Rotation::R(amount) => self.0 + amount,
        } % 100;
    }
}

#[aoc(day1, part1)]
fn part1(input: &[Rotation]) -> usize {
    let mut dial = Dial(50);
    let mut zero_times = 0usize;
    for &rotation in input {
        dial.rotate(rotation);
        if dial.is_zero() {
            zero_times += 1;
        }
    }
    zero_times
}

#[aoc(day1, part2)]
fn part2(input: &[Rotation]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }
}
