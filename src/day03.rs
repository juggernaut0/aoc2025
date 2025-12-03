use aoc::parse_lines_with;

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        solve(&input, 2)
    }

    fn solve_2(&self, input: String) -> String {
        solve(&input, 12)
    }
}

fn solve(input: &str, num_digits: usize) -> String {
    parse_lines_with(input, parse_line)
        .map(|it| solve_bank(&it, num_digits))
        .sum::<u64>()
        .to_string()
}

fn parse_line(line: &str) -> Vec<u64> {
    line.chars()
        .map(|c| c.to_digit(10).unwrap().into())
        .collect()
}

#[allow(clippy::cast_possible_truncation)]
fn solve_bank(bank: &[u64], num_digits: usize) -> u64 {
    if num_digits == 1 {
        return *bank.iter().max().unwrap();
    }

    let len = bank.len();
    // find index of first largest value in bank[0..=len-num_digits]
    let mut first_index = 0;
    let mut first_value = bank[0];
    for (i, &v) in bank[0..=len - num_digits].iter().enumerate().skip(1) {
        if v > first_value {
            first_value = v;
            first_index = i;
        }
    }

    let suffix_value = solve_bank(&bank[first_index + 1..], num_digits - 1);
    first_value * 10u64.pow((num_digits - 1) as u32) + suffix_value
}
