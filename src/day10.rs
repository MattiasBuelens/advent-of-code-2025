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

fn configure(machine: &Machine) -> BitVec {
    solve_recurse(
        machine,
        &bitvec!(0; machine.lights.len()),
        BitVec::with_capacity(machine.buttons.len()),
    )
    .expect("no solution")
}

fn solve_recurse(machine: &Machine, lights: &BitVec, presses: BitVec) -> Option<BitVec> {
    if presses.len() == machine.buttons.len() {
        return if &machine.lights == lights {
            Some(presses)
        } else {
            None
        };
    }
    let mut presses_off = presses.clone();
    presses_off.push(false);
    let mut presses_on = presses.clone();
    presses_on.push(true);
    let mut lights_on = lights.clone();
    lights_on ^= &machine.buttons[presses.len()];
    let solution_off = solve_recurse(machine, lights, presses_off);
    let solution_on = solve_recurse(machine, &lights_on, presses_on);
    match (solution_off, solution_on) {
        (Some(solution_off), Some(solution_on))
            if solution_off.count_ones() <= solution_on.count_ones() =>
        {
            Some(solution_off)
        }
        (_, Some(solution_on)) => Some(solution_on),
        (Some(solution_off), None) => Some(solution_off),
        (None, None) => None,
    }
}

#[aoc(day10, part1)]
fn part1(input: &[Machine]) -> usize {
    input
        .iter()
        .map(|machine| configure(machine).count_ones())
        .sum()
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
        let input = parse(EXAMPLE);
        assert_eq!(configure(&input[0]), bitvec!(0, 0, 0, 0, 1, 1));
        assert_eq!(configure(&input[1]), bitvec!(0, 0, 1, 1, 1));
        assert_eq!(configure(&input[2]), bitvec!(0, 1, 1, 0));
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }
}
