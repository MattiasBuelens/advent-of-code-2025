use aoc_runner_derive::{aoc, aoc_generator};
use bitvec::prelude::*;

#[derive(Debug, Clone)]
struct Machine {
    lights: BitVec,
    buttons: Vec<BitVec>,
    joltages: Vec<u64>,
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|s| {
            let (lights, s) = s.split_once(' ').unwrap();
            let (buttons, joltages) = s.rsplit_once(' ').unwrap();
            let lights: BitVec = lights
                .strip_prefix('[')
                .unwrap()
                .strip_suffix(']')
                .unwrap()
                .chars()
                .map(|c| c == '#')
                .collect();
            let buttons: Vec<BitVec> = buttons
                .split(' ')
                .map(|button| {
                    let mut button_mask = bitvec!(0; lights.len());
                    button
                        .strip_prefix('(')
                        .unwrap()
                        .strip_suffix(')')
                        .unwrap()
                        .split(',')
                        .map(|n| n.parse::<usize>().unwrap())
                        .for_each(|light_index| button_mask.set(light_index, true));
                    button_mask
                })
                .collect();
            let joltages: Vec<u64> = joltages
                .strip_prefix('{')
                .unwrap()
                .strip_suffix('}')
                .unwrap()
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect();
            Machine {
                lights,
                buttons,
                joltages,
            }
        })
        .collect()
}

#[aoc(day10, part1)]
fn part1(input: &[Machine]) -> usize {
    todo!()
}

#[aoc(day10, part2)]
fn part2(input: &[Machine]) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = include_str!("../example/2025/day10.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 0);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }
}
