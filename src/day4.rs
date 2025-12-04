use crate::util::Vector2D;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

type Grid = HashSet<Vector2D>;

#[aoc_generator(day4)]
fn parse(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '@' => Some(Vector2D::new(x as i32, y as i32)),
                '.' => None,
                c => panic!("invalid character: {c}"),
            })
        })
        .collect()
}

fn removable_rolls(grid: &Grid) -> Vec<Vector2D> {
    grid.iter()
        .copied()
        .filter(|&roll| {
            let neighbour_rolls = roll
                .neighbours_diagonal()
                .filter(|pos| grid.contains(pos))
                .count();
            neighbour_rolls < 4
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(grid: &Grid) -> usize {
    removable_rolls(grid).len()
}

#[aoc(day4, part2)]
fn part2(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    let mut total_removed = 0usize;
    loop {
        let removable = removable_rolls(&grid);
        if removable.is_empty() {
            break;
        }
        total_removed += removable.len();
        for roll in removable {
            grid.remove(&roll);
        }
    }
    total_removed
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 43);
    }
}
