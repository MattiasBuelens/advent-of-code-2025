use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::max;

type Bank = Vec<u8>;

fn parse_bank(line: &str) -> Bank {
    line.chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Bank> {
    input.lines().map(parse_bank).collect()
}

fn max_joltage_part1(bank: &[u8]) -> u8 {
    bank.iter()
        .copied()
        .enumerate()
        .flat_map(|(i, battery1)| {
            let battery2 = bank[i + 1..].iter().copied().max()?;
            Some(battery1 * 10 + battery2)
        })
        .max()
        .unwrap()
}

#[aoc(day3, part1)]
fn part1(input: &[Bank]) -> u64 {
    input
        .iter()
        .map(|bank| max_joltage_part1(bank) as u64)
        .sum()
}

fn max_joltage_part2(bank: &[u8], len: usize) -> u64 {
    debug_assert!(len > 2);
    let mut max_joltages: Vec<Vec<u64>> = Vec::with_capacity(len + 1);
    // Ignore len = 0
    max_joltages.push(Vec::new());
    // With len = 1, the maximum joltage is just the battery's value
    max_joltages.push(bank.iter().map(|&x| x as u64).collect());
    let mut shift = 10;
    for len in 2..=len {
        max_joltages.push(vec![0u64; bank.len()]);
        for (i, &battery) in bank.iter().enumerate().rev().skip(len - 1) {
            // Two options:
            // - Take this battery and add the max joltage at i+1 of the previous length
            // - Skip this battery and use the max joltage at i+1 of the current length
            max_joltages[len][i] = max(
                max_joltages[len - 1][i + 1] + (battery as u64 * shift),
                max_joltages[len][i + 1],
            );
        }
        shift *= 10;
    }
    max_joltages[len].iter().copied().max().unwrap()
}

#[aoc(day3, part2)]
fn part2(input: &[Bank]) -> u64 {
    input.iter().map(|bank| max_joltage_part2(bank, 12)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = r"987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 357);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 3121910778619);
    }

    #[test]
    fn test_max_joltage_part2() {
        assert_eq!(
            max_joltage_part2(&parse_bank("987654321111111"), 12),
            987654321111
        );
        assert_eq!(
            max_joltage_part2(&parse_bank("811111111111119"), 12),
            811111111119
        );
        assert_eq!(
            max_joltage_part2(&parse_bank("234234234234278"), 12),
            434234234278
        );
        assert_eq!(
            max_joltage_part2(&parse_bank("818181911112111"), 12),
            888911112111
        );
    }
}
