use aoc_runner_derive::{aoc, aoc_generator};
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
struct Inventory {
    fresh_ranges: Vec<RangeInclusive<u64>>,
    ingredients: Vec<u64>,
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Inventory {
    let (fresh_ranges, ingredients) = input.split_once("\n\n").unwrap();
    let fresh_ranges = fresh_ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .collect();
    let ingredients = ingredients
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    Inventory {
        fresh_ranges,
        ingredients,
    }
}

impl Inventory {
    fn is_fresh(&self, ingredient: u64) -> bool {
        self.fresh_ranges
            .iter()
            .any(|range| range.contains(&ingredient))
    }
}

#[aoc(day5, part1)]
fn part1(inventory: &Inventory) -> usize {
    inventory
        .ingredients
        .iter()
        .filter(|&&ingredient| inventory.is_fresh(ingredient))
        .count()
}

#[aoc(day5, part2)]
fn part2(inventory: &Inventory) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }
}
