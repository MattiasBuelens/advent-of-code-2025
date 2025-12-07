use crate::util::Vector2D;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Either;
use pathfinding::directed::count_paths::count_paths;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Manifold {
    height: i32,
    start: Vector2D,
    splitters: HashSet<Vector2D>,
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Manifold {
    let height = input.lines().count() as i32;
    let mut start = None::<Vector2D>;
    let mut splitters = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start = Some(Vector2D::new(x as i32, y as i32));
                }
                '^' => {
                    splitters.insert(Vector2D::new(x as i32, y as i32));
                }
                '.' => continue,
                c => panic!("invalid character {c}"),
            }
        }
    }
    Manifold {
        height,
        start: start.unwrap(),
        splitters,
    }
}

#[aoc(day7, part1)]
fn part1(manifold: &Manifold) -> usize {
    let mut num_splits = 0;
    let mut beams = HashSet::<i32>::new();
    let mut y = manifold.start.y();
    beams.insert(manifold.start.x());
    while y < manifold.height {
        let mut next_beams = HashSet::<i32>::new();
        for beam in beams {
            let pos = Vector2D::new(beam, y);
            if manifold.splitters.contains(&pos) {
                num_splits += 1;
                next_beams.insert(pos.x() - 1);
                next_beams.insert(pos.x() + 1);
            } else {
                next_beams.insert(pos.x());
            }
        }
        beams = next_beams;
        y += 1;
    }
    num_splits
}

#[aoc(day7, part2)]
fn part2(manifold: &Manifold) -> usize {
    count_paths(
        manifold.start,
        |&pos| {
            if manifold.splitters.contains(&pos) {
                Either::Left([pos + Vector2D::new(-1, 0), pos + Vector2D::new(1, 0)].into_iter())
            } else {
                Either::Right([pos + Vector2D::new(0, 1)].into_iter())
            }
        },
        |pos| pos.y() == manifold.height,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("../example/2025/day7.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 21);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 40);
    }
}
