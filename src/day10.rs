use aoc_runner_derive::{aoc, aoc_generator};
use bitvec::prelude::*;
use z3::ast::Int;
use z3::{Optimize, SatResult};

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

fn solve_joltages(machine: &Machine) -> u64 {
    let optimizer = Optimize::new();
    // Create a variable for each unknown (number of button presses)
    let presses = (0..machine.buttons.len())
        .map(|_| Int::fresh_const("press"))
        .collect::<Vec<_>>();
    // Require all button presses to be positive
    for press in &presses {
        optimizer.assert(&press.ge(0))
    }
    // Create an equation for each output joltage
    for (i, &expected_joltage) in machine.joltages.iter().enumerate() {
        let mut joltage = Int::from_u64(0);
        for (j, button) in machine.buttons.iter().enumerate() {
            if button[i] {
                joltage += &presses[j];
            }
        }
        optimizer.assert(&joltage.eq(expected_joltage))
    }
    // Find the smallest number of button presses
    let total = presses.iter().sum::<Int>();
    optimizer.minimize(&total);
    let result = optimizer.check(&[]);
    assert_eq!(result, SatResult::Sat);
    let model = optimizer.get_model().unwrap();
    let total_eval = model.eval(&total, true).unwrap();
    total_eval.as_u64().unwrap()
}

#[aoc(day10, part2)]
fn part2(input: &[Machine]) -> u64 {
    input.iter().map(solve_joltages).sum()
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
    fn part2_examples() {
        let input = parse(EXAMPLE);
        assert_eq!(solve_joltages(&input[0]), 10);
        assert_eq!(solve_joltages(&input[1]), 12);
        assert_eq!(solve_joltages(&input[2]), 11);
    }

    #[test]
    fn part2_example() {
        let input = parse(EXAMPLE);
        assert_eq!(part2(&input), 33);
    }
}
