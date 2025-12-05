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

impl Inventory {
    fn normalize_ranges(&mut self) {
        // Inspired by TimeRanges::Normalize() from Mozilla Firefox
        // https://searchfox.org/firefox-main/rev/39bc83bb8632d54d70542dc5d98c046a317ec99d/dom/media/mediaelement/TimeRanges.cpp#107
        if self.fresh_ranges.len() < 2 {
            return;
        }
        // Sort ranges
        self.fresh_ranges.sort_by(|left, right| {
            left.start()
                .cmp(right.start())
                .then_with(|| left.end().cmp(right.end()))
        });
        // Merge consecutive ranges
        let mut ranges = Vec::with_capacity(self.fresh_ranges.len());
        let mut current_range = self.fresh_ranges[0].clone();
        for range in &self.fresh_ranges[1..] {
            if current_range.start() <= range.start() && range.end() <= current_range.end() {
                // New range is fully contained in current range, skip it.
                continue;
            } else if current_range.end() >= range.start() {
                // Consecutive or overlapping range.
                current_range = (*current_range.start())..=(*range.end());
            } else {
                // Separate range.
                ranges.push(current_range);
                current_range = range.clone();
            }
        }
        ranges.push(current_range);
        self.fresh_ranges = ranges;
    }

    fn count_fresh(&self) -> usize {
        self.fresh_ranges
            .iter()
            .map(|range| range.clone().count())
            .sum()
    }
}

#[aoc(day5, part2)]
fn part2(inventory: &Inventory) -> usize {
    let mut inventory = inventory.clone();
    inventory.normalize_ranges();
    inventory.count_fresh()
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
        assert_eq!(part2(&parse(EXAMPLE)), 14);
    }
}
