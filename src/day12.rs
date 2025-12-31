use crate::util::Vector2D;
use aoc_runner_derive::{aoc, aoc_generator};

type Shape = Vec<Vector2D>;

#[derive(Debug, Clone)]
struct Region {
    width: usize,
    height: usize,
    presents: Vec<usize>,
}

#[derive(Debug, Clone)]
struct Input {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Input {
    let blocks = input.split("\n\n").collect::<Vec<_>>();
    let (regions, shapes) = blocks.split_last().unwrap();
    let shapes = shapes
        .iter()
        .map(|shape| {
            let mut lines = shape.lines();
            let _name = lines.next().unwrap();
            let mut shape = vec![];
            for (y, line) in lines.enumerate() {
                for (x, c) in line.chars().enumerate() {
                    match c {
                        '#' => shape.push(Vector2D::new(x as i32, y as i32)),
                        '.' => continue,
                        c => panic!("invalid character {c}"),
                    }
                }
            }
            shape
        })
        .collect();
    let regions = regions
        .lines()
        .map(|line| {
            let (size, presents) = line.split_once(": ").unwrap();
            let (width, height) = size.split_once('x').unwrap();
            Region {
                width: width.parse().unwrap(),
                height: height.parse().unwrap(),
                presents: presents.split(' ').map(|s| s.parse().unwrap()).collect(),
            }
        })
        .collect();
    Input { shapes, regions }
}

#[aoc(day12, part1)]
fn part1(input: &Input) -> usize {
    todo!()
}

#[aoc(day12, part2)]
fn part2(input: &Input) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("../example/2025/day12.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }
}
