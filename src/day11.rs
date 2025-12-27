use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Either;
use pathfinding::prelude::count_paths;
use std::collections::HashMap;
use std::iter::empty;

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

impl Reactor {
    fn connections(&self, label: &str) -> impl Iterator<Item = &str> + '_ {
        if let Some(next) = self.cables.get(label) {
            Either::Left(next.iter().map(String::as_str))
        } else {
            Either::Right(empty())
        }
    }
}

#[aoc(day11, part1)]
fn part1(input: &Reactor) -> usize {
    count_paths(
        "you",
        |&label| input.connections(label),
        |&label| label == "out",
    )
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct State<'a> {
    label: &'a str,
    dac: bool,
    fft: bool,
}

#[aoc(day11, part2)]
fn part2(input: &Reactor) -> usize {
    count_paths(
        State {
            label: "svr",
            dac: false,
            fft: false,
        },
        |state| {
            let state = state.clone();
            input.connections(state.label).map(move |next_label| State {
                label: next_label,
                dac: next_label == "dac" || state.dac,
                fft: next_label == "fft" || state.fft,
            })
        },
        |state| state.label == "out" && state.dac && state.fft,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE1: &str = include_str!("../example/2025/day11p1.txt");
    static EXAMPLE2: &str = include_str!("../example/2025/day11p2.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE1)), 5);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE2)), 2);
    }
}
