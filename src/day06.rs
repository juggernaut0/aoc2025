use std::borrow::Borrow;

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let lines: Vec<_> = input.lines().collect();
        let nums: Vec<Vec<u64>> = lines[0..lines.len() - 1]
            .iter()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|it| it.parse().unwrap())
                    .collect()
            })
            .collect();
        let ops: Vec<_> = lines.last().unwrap().split_ascii_whitespace().collect();

        let mut sum = 0;
        for i in 0..ops.len() {
            let op = ops[i];
            let col = match op {
                "+" => nums.iter().map(|row| row[i]).sum::<u64>(),
                "*" => nums.iter().map(|row| row[i]).product::<u64>(),
                _ => unreachable!(),
            };
            sum += col;
        }
        sum.to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let lines: Vec<_> = input.lines().collect();

        let lines_chars = lines[0..lines.len() - 1]
            .iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let transpose_lines = transpose(&lines_chars)
            .into_iter()
            .map(|lcs| lcs.into_iter().collect::<String>())
            .collect::<Vec<_>>();

        transpose_lines
            .split(|line| line.trim().is_empty())
            .zip(lines.last().unwrap().split_ascii_whitespace())
            .map(|(nums_str, ops)| {
                let nums = nums_str.iter().map(|s| s.trim().parse::<u64>().unwrap());
                match ops {
                    "+" => nums.sum::<u64>(),
                    "*" => nums.product(),
                    _ => unreachable!(),
                }
            })
            .sum::<u64>()
            .to_string()
    }
}

#[allow(clippy::needless_range_loop)]
fn transpose<V: Borrow<[T]>, T: Copy + Default>(matrix: &[V]) -> Vec<Vec<T>> {
    if matrix.is_empty() {
        return vec![];
    }
    let row_count = matrix.len();
    let col_count = matrix[0].borrow().len();
    let mut transposed = vec![vec![T::default(); row_count]; col_count];
    for r in 0..row_count {
        for c in 0..col_count {
            transposed[c][r] = matrix[r].borrow()[c];
        }
    }
    transposed
}
