use crate::util::Vector3D;
use aoc_runner_derive::{aoc, aoc_generator};
use disjoint::DisjointSet;
use itertools::Itertools;
use std::cmp::Reverse;

#[aoc_generator(day8)]
fn parse(input: &str) -> Vec<Vector3D<i64>> {
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

fn get_pairs(
    boxes: &[Vector3D<i64>],
) -> impl Iterator<Item = ((usize, Vector3D<i64>), (usize, Vector3D<i64>))> + '_ {
    boxes
        .iter()
        .copied()
        .enumerate()
        .tuple_combinations::<(_, _)>()
        .sorted_by_key(|((_, a), (_, b))| a.euclidean_distance_squared(b))
}

fn connect(boxes: &[Vector3D<i64>], num_connections: usize) -> usize {
    let mut links = DisjointSet::with_len(boxes.len());
    let pairs = get_pairs(boxes).take(num_connections);
    for ((i, _), (j, _)) in pairs {
        links.join(i, j);
    }
    let mut sets = links.sets();
    sets.sort_by_key(|set| Reverse(set.len()));
    let three_largest_sets = &sets[0..3];
    three_largest_sets.iter().map(|set| set.len()).product()
}

#[aoc(day8, part1)]
fn part1(boxes: &[Vector3D<i64>]) -> usize {
    connect(boxes, 1000)
}

fn connect_until_single(boxes: &[Vector3D<i64>]) -> (Vector3D<i64>, Vector3D<i64>) {
    let mut links = DisjointSet::with_len(boxes.len());
    let pairs = get_pairs(boxes);
    for ((i, box1), (j, box2)) in pairs {
        links.join(i, j);
        if (0..boxes.len()).map(|k| links.root_of(k)).all_equal() {
            return (box1, box2);
        }
    }
    panic!("no single circuit after all pairs are connected");
}

#[aoc(day8, part2)]
fn part2(boxes: &[Vector3D<i64>]) -> i64 {
    let (box1, box2) = connect_until_single(boxes);
    box1.x() * box2.x()
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
        assert_eq!(
            connect_until_single(&parse(EXAMPLE)),
            (Vector3D::new(216, 146, 977), Vector3D::new(117, 168, 530))
        );
    }
}
