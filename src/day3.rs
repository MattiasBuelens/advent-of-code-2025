use aoc_runner_derive::{aoc, aoc_generator};

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

fn max_joltage_part2(bank: &[u8], num_batteries: usize) -> u64 {
    let mut joltage = 0;
    let mut start_idx = 0usize;
    for num_battery in (0..num_batteries).rev() {
        let (idx, battery) = bank[..(bank.len() - num_battery)]
            .iter()
            .copied()
            .enumerate()
            .skip(start_idx)
            .max_by(|(i1, b1), (i2, b2)| b1.cmp(&b2).then_with(|| i2.cmp(i1)))
            .unwrap();
        joltage += (battery as u64) * 10u64.pow(num_battery as u32);
        start_idx = idx + 1;
    }
    joltage
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
