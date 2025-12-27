use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::count_paths;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Reactor {
    cables: HashMap<String, Vec<String>>,
}

#[aoc_generator(day11)]
fn parse(input: &str) -> Reactor {
    let cables = input
        .lines()
        .map(|s| {
            let (input, s) = s.split_once(": ").unwrap();
            let outputs = s.split(' ').map(str::to_string).collect();
            (input.to_string(), outputs)
        })
        .collect();
    Reactor { cables }
}

#[aoc(day11, part1)]
fn part1(input: &Reactor) -> usize {
    count_paths(
        "you",
        |&label| {
            input
                .cables
                .get(label)
                .iter()
                .flat_map(|next| next.iter().map(String::as_str))
                .collect::<Vec<_>>()
        },
        |&label| label == "out",
    )
}

#[aoc(day11, part2)]
fn part2(input: &Reactor) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("../example/2025/day11.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 5);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }
}
