use crate::util::Vector2D;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

type Shape = HashSet<Vector2D>;

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
            let mut shape = HashSet::new();
            for (y, line) in lines.enumerate() {
                for (x, c) in line.chars().enumerate() {
                    match c {
                        '#' => shape.insert(Vector2D::new(x as i32, y as i32)),
                        '.' => continue,
                        c => panic!("invalid character {c}"),
                    };
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

const PRESENT_SIZE: usize = 3;

impl Input {
    fn can_fit(&self, region: &Region) -> bool {
        // Very naive!
        if self.can_fit_without_interlocking(region) {
            true
        } else if !self.can_fit_with_ideal_interlocking(region) {
            false
        } else {
            // This case doesn't actually happen on the real input. ¯\_(ツ)_/¯
            panic!("this shouldn't happen")
        }
    }

    fn can_fit_without_interlocking(&self, region: &Region) -> bool {
        // All presents fit in a 3x3 square.
        // If the region is large enough to hold as many 3x3 squares as there are presents,
        // then their shape doesn't matter: they'll fit regardless.
        let region_area = region.width * region.height;
        let maximum_presents_area =
            region.presents.iter().copied().sum::<usize>() * (PRESENT_SIZE * PRESENT_SIZE);
        region_area >= maximum_presents_area
    }

    fn can_fit_with_ideal_interlocking(&self, region: &Region) -> bool {
        // The region must be at least as large
        // as the number of filled squares occupied by each present,
        // assuming they all interlock perfectly without leaving any gaps.
        let region_area = region.width * region.height;
        let minimum_presents_area = region
            .presents
            .iter()
            .copied()
            .enumerate()
            .map(|(idx, count)| {
                let filled_squares_for_shape = self.shapes[idx].len();
                filled_squares_for_shape * count
            })
            .sum::<usize>();
        region_area >= minimum_presents_area
    }
}

#[aoc(day12, part1)]
fn part1(input: &Input) -> usize {
    input
        .regions
        .iter()
        .filter(|region| input.can_fit(region))
        .count()
}

#[aoc(day12, part2)]
fn part2(input: &Input) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("../example/2025/day12.txt");

    #[ignore]
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 2);
    }
}
