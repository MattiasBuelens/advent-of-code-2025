use aoc_runner_derive::{aoc, aoc_generator};

type Bank = Vec<u8>;

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Bank> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn max_joltage(bank: &[u8]) -> u8 {
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
    input.iter().map(|bank| max_joltage(bank) as u64).sum()
}

#[aoc(day3, part2)]
fn part2(input: &[Bank]) -> u64 {
    todo!()
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
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }
}
