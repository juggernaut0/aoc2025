use aoc::parse_lines_with;
use std::iter::repeat_n;

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        solve(parse_lines_with(&input, parse_line))
    }

    fn solve_2(&self, input: String) -> String {
        solve(
            parse_lines_with(&input, parse_line)
                .flat_map(|n| repeat_n(n.signum(), n.unsigned_abs() as usize)),
        )
    }
}

fn parse_line(line: &str) -> i32 {
    line.replace('L', "-")
        .replace('R', "")
        .parse::<i32>()
        .unwrap()
}

fn solve(iter: impl Iterator<Item = i32>) -> String {
    iter.scan(50, |acc, n| {
        *acc += n;
        Some(*acc)
    })
    .filter(|it| it % 100 == 0)
    .count()
    .to_string()
}
