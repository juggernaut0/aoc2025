use std::collections::HashSet;
use std::iter::once;
use std::num::IntErrorKind;

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
        .flat_map(|r| generate_invalid(r.0, r.1, chunks_counts.clone()))
        .sum::<u64>()
        .to_string()
}

fn generate_invalid(
    start: u64,
    end: u64,
    chunks_counts: impl Iterator<Item = usize>,
) -> HashSet<u64> {
    let mut invalids = HashSet::new();
    let start_chars = start.to_string().chars().collect::<Vec<_>>();
    let end_chars = end.to_string().chars().collect::<Vec<_>>();

    for chunks in chunks_counts {
        if chunks > end_chars.len() {
            break;
        }
        log::info!("checking range {start}-{end} for invalids with {chunks} chunks");
        let mut chunk = start_chars[0..(start_chars.len() / chunks)].to_vec();
        if chunk.is_empty() {
            chunk.push('1');
        }
        loop {
            let chunk_len = chunk.len();
            let candidate = match chunk
                .iter()
                .cycle()
                .take(chunks * chunk_len)
                .collect::<String>()
                .parse::<u64>()
            {
                Ok(n) => n,
                Err(e) if matches!(e.kind(), IntErrorKind::PosOverflow) => break,
                Err(e) => panic!("Unexpected parse error for candidate ID: {e:?}"),
            };

            if candidate > end {
                break;
            }

            log::debug!("Checking candidate invalid ID: {candidate}");

            increment_chunk(&mut chunk);

            if candidate < start {
                continue;
            }
            invalids.insert(candidate);
            log::debug!("Found invalid ID: {candidate}");
        }
    }
    invalids
}

fn increment_chunk(chunk: &mut Vec<char>) {
    for i in (0..chunk.len()).rev() {
        if chunk[i] == '9' {
            chunk[i] = '0';
        } else {
            chunk[i] = ((chunk[i] as u8) + 1) as char;
            return;
        }
    }
    chunk.insert(0, '1');
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
