use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
enum Op {
    #[default]
    Add,
    Multiply,
}

#[derive(Debug, Default, Clone)]
struct Problem {
    operands: Vec<u64>,
    op: Op,
}

fn parse_part1(input: &str) -> Vec<Problem> {
    let mut problems = Vec::new();
    for line in input.lines() {
        for (i, s) in line.split_ascii_whitespace().enumerate() {
            if problems.get(i).is_none() {
                problems.push(Problem::default());
            }
            let problem = problems.get_mut(i).unwrap();
            match s {
                "+" => problem.op = Op::Add,
                "*" => problem.op = Op::Multiply,
                s => problem.operands.push(s.parse().unwrap()),
            }
        }
    }
    problems
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.op {
            Op::Add => self.operands.iter().sum(),
            Op::Multiply => self.operands.iter().product(),
        }
    }
}

#[aoc(day6, part1)]
fn part1(input: &str) -> u64 {
    let problems = parse_part1(input);
    problems.iter().map(|p| p.solve()).sum()
}

fn parse_part2(input: &str) -> Vec<Problem> {
    todo!()
}

#[aoc(day6, part2)]
fn part2(input: &str) -> u64 {
    let problems = parse_part2(input);
    problems.iter().map(|p| p.solve()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = r"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 4277556);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 3263827);
    }
}
