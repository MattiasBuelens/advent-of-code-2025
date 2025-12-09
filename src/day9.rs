use crate::util::Vector2D;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Vector2D<i64>> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Vector2D::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[Vector2D<i64>]) -> i64 {
    input
        .iter()
        .tuple_combinations::<(_, _)>()
        .map(|(left, right)| {
            let width = (left.x() - right.x() + 1).abs();
            let height = (left.y() - right.y() + 1).abs();
            width * height
        })
        .max()
        .unwrap()
}

#[aoc(day9, part2)]
fn part2(input: &[Vector2D<i64>]) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    static EXAMPLE: &str = include_str!("../example/2025/day9.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 50);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }
}
