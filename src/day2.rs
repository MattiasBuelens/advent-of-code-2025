use aoc_runner_derive::{aoc, aoc_generator};

struct Range {
    start: u64,
    end: u64,
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Range> {
    input
        .split(',')
        .map(|s| {
            let (start, end) = s.split_once('-').unwrap();
            Range {
                start: start.parse().unwrap(),
                end: end.parse().unwrap(),
            }
        })
        .collect()
}

fn to_digits(mut n: u64) -> Vec<u8> {
    let mut digits = Vec::new();
    while n > 0 {
        digits.push((n % 10) as u8);
        n /= 10;
    }
    // No need to reverse: invalid IDs will still be invalid when reversed.
    digits
}

fn is_invalid(id: u64) -> bool {
    let digits = to_digits(id);
    if !digits.len().is_multiple_of(2) {
        return false;
    }
    let half_len = digits.len() / 2;
    digits[0..half_len] == digits[half_len..]
}

#[aoc(day2, part1)]
fn part1(input: &[Range]) -> u64 {
    input
        .iter()
        .flat_map(|range| range.start..=range.end)
        .filter(|&id| is_invalid(id))
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &[Range]) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 1227775554);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 0);
    }

    #[test]
    fn test_is_invalid() {
        assert!(is_invalid(55));
        assert!(is_invalid(6464));
        assert!(is_invalid(123123));

        assert!(!is_invalid(12));
        assert!(!is_invalid(56));
    }
}
