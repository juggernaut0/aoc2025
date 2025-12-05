use aoc::parse_lines;

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let (ranges, ingredients) = parse_input(&input);
        ingredients
            .into_iter()
            .filter(|it| ranges.iter().any(|(start, end)| it >= start && it <= end))
            .count()
            .to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let (mut ranges, _) = parse_input(&input);
        ranges.sort_by_key(|(s, _)| *s);
        let merged_ranges: Vec<(u64, u64)> =
            ranges.into_iter().fold(vec![], |mut acc, (start, end)| {
                if let Some((_, last_end)) = acc.last_mut() {
                    if start <= *last_end + 1 {
                        *last_end = (*last_end).max(end);
                    } else {
                        acc.push((start, end));
                    }
                } else {
                    acc.push((start, end));
                }
                acc
            });
        merged_ranges
            .into_iter()
            .map(|(s, e)| e - s + 1)
            .sum::<u64>()
            .to_string()
    }
}

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (ranges_str, ingr_str) = input.split_once("\n\n").unwrap();
    let ranges: Vec<(u64, u64)> = ranges_str
        .lines()
        .map(|line| {
            let (start_str, end_str) = line.split_once('-').unwrap();
            let start: u64 = start_str.parse().unwrap();
            let end: u64 = end_str.parse().unwrap();
            (start, end)
        })
        .collect();
    let ingredients: Vec<u64> = parse_lines(ingr_str).collect();
    (ranges, ingredients)
}
