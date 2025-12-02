use std::iter::once;

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        solve(&input, &once(2))
    }

    fn solve_2(&self, input: String) -> String {
        solve(&input, &(2..))
    }
}

fn solve(input: &str, chunks_counts: &(impl Iterator<Item = usize> + Clone)) -> String {
    let ranges = parse_ranges(input);
    ranges
        .iter()
        .flat_map(|r| r.0..=r.1)
        .filter(|&n| is_invalid(n, chunks_counts.clone()))
        .sum::<u64>()
        .to_string()
}

fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(',')
        .map(|part| {
            let mut bounds = part.split('-').map(|s| s.parse().unwrap());
            let start = bounds.next().unwrap();
            let end = bounds.next().unwrap();
            (start, end)
        })
        .collect()
}

fn is_invalid(n: u64, chunks_counts: impl Iterator<Item = usize>) -> bool {
    let ns: Vec<_> = n.to_string().chars().collect();
    for chunks in chunks_counts {
        if chunks > ns.len() {
            break;
        }
        if is_repeated_chunks(&ns, chunks) {
            return true;
        }
    }
    false
}

fn is_repeated_chunks(id_vec: &[char], chunks: usize) -> bool {
    if !id_vec.len().is_multiple_of(chunks) {
        return false;
    }
    id_vec
        .chunks(id_vec.len() / chunks)
        .reduce(|c1, c2| if c1 == c2 { c1 } else { &[] })
        .is_some_and(|c| !c.is_empty())
}
