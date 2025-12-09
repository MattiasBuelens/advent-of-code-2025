use crate::util::Vector3D;
use aoc_runner_derive::{aoc, aoc_generator};
use disjoint::DisjointSet;
use itertools::Itertools;

#[aoc_generator(day8)]
fn parse(input: &str) -> Vec<Vector3D> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.splitn(3, ',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            let z = parts.next().unwrap().parse().unwrap();
            Vector3D::new(x, y, z)
        })
        .collect()
}

fn connect(boxes: &[Vector3D], num_connections: usize) -> usize {
    let mut pairs = boxes
        .iter()
        .enumerate()
        .tuple_combinations::<(_, _)>()
        .collect::<Vec<_>>();
    pairs.sort_by_key(|((_, a), (_, b))| a.euclidean_distance_squared(b));
    let mut links = DisjointSet::with_len(boxes.len());
    for ((i, _), (j, _)) in pairs.into_iter().take(num_connections) {
        links.join(i, j);
    }
    let mut sets = links.sets();
    sets.sort_by_key(|set| set.len());
    let three_largest_sets = &sets[(sets.len() - 3)..];
    three_largest_sets.iter().map(|set| set.len()).product()
}

#[aoc(day8, part1)]
fn part1(boxes: &[Vector3D]) -> usize {
    connect(boxes, 1000)
}

#[aoc(day8, part2)]
fn part2(boxes: &[Vector3D]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("../example/2025/day8.txt");

    #[test]
    fn part1_example() {
        assert_eq!(connect(&parse(EXAMPLE), 10), 40);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 40);
    }
}
