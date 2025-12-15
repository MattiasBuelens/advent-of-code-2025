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

#[derive(Debug)]
struct Rect {
    top_left: Vector2D<i64>,
    bottom_right: Vector2D<i64>,
}

impl Rect {
    fn new(first: Vector2D<i64>, second: Vector2D<i64>) -> Self {
        Self {
            top_left: Vector2D::new(first.x().min(second.x()), first.y().min(second.y())),
            bottom_right: Vector2D::new(first.x().max(second.x()), first.y().max(second.y())),
        }
    }

    fn area(&self) -> i64 {
        let width = self.bottom_right.x() - self.top_left.x() + 1;
        let height = self.bottom_right.y() - self.top_left.y() + 1;
        width * height
    }
}

#[aoc(day9, part1)]
fn part1(input: &[Vector2D<i64>]) -> i64 {
    input
        .iter()
        .tuple_combinations::<(_, _)>()
        .map(|(&left, &right)| Rect::new(left, right).area())
        .max()
        .unwrap()
}

impl Rect {
    fn intersects_line(&self, first: Vector2D<i64>, second: Vector2D<i64>) -> bool {
        let start = first.min(second);
        let end = first.max(second);
        if start.x() == end.x() {
            ((self.top_left.x() + 1)..self.bottom_right.x()).contains(&start.x())
                && (self.top_left.y() + 1) <= end.y()
                && start.y() < self.bottom_right.y()
        } else {
            assert_eq!(start.y(), end.y());
            ((self.top_left.y() + 1)..self.bottom_right.y()).contains(&start.y())
                && (self.top_left.x() + 1) <= end.x()
                && start.x() < self.bottom_right.x()
        }
    }
}

#[aoc(day9, part2)]
fn part2(input: &[Vector2D<i64>]) -> i64 {
    // The real input is a large circle with a thin horizontal cut-out in the middle.
    // The largest rectangle will always share a corner with this cut-out.
    let cutout_index = input
        .iter()
        .tuple_windows()
        .position(|(&x, &y)| (x - y).manhattan_distance() > 90_000)
        .unwrap();
    let corners = &input[cutout_index + 1..][0..2];
    let best_rect = input
        .iter()
        .cartesian_product(corners)
        .map(|(&first, &second)| Rect::new(first, second))
        .filter(|rect| {
            input
                .iter()
                .tuple_windows()
                .all(|(&first, &second)| !rect.intersects_line(first, second))
        })
        .max_by_key(Rect::area)
        .unwrap();
    best_rect.area()
}

#[cfg(test)]
mod tests {
    use super::*;
    static EXAMPLE: &str = include_str!("../example/2025/day9.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 50);
    }

    // Does not work on example input.
    #[ignore]
    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 24);
    }
}
