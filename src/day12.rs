use aoc::{Grid, parse_lines};
use std::borrow::Cow;
use std::str::FromStr;

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let (shapes, areas) = parse_input(&input);
        let cells_per_shape: Vec<usize> = shapes
            .iter()
            .map(|shape| shape.points_with_item().filter(|(_, c)| **c == '#').count())
            .collect();
        areas
            .iter()
            .filter(|area| {
                let can_fit_no_packing = {
                    let w3 = area.width / 3;
                    let h3 = area.height / 3;
                    let total_needed: usize = area.shape_counts.iter().sum();
                    let area_size = w3 * h3;
                    total_needed <= area_size
                };
                if can_fit_no_packing {
                    return true;
                }
                let cant_fit_optimal_packing = {
                    let total_area = area.width * area.height;
                    let total_needed: usize = area
                        .shape_counts
                        .iter()
                        .enumerate()
                        .map(|(i, &count)| count * cells_per_shape[i])
                        .sum();
                    total_needed > total_area
                };
                if cant_fit_optimal_packing {
                    return false;
                }
                panic!("Area requires packing logic which is not implemented");
            })
            .count()
            .to_string()
    }

    fn solve_2(&self, _input: String) -> String {
        "Merry ~~Christmas~~ 12th!".to_string()
    }
}

fn parse_input(input: &str) -> (Vec<Grid<char>>, Vec<Area>) {
    let sections: Vec<_> = input.split("\n\n").collect();
    let shapes = sections[..sections.len() - 1]
        .iter()
        .map(|shape_str| {
            let (_, rest) = shape_str.trim().split_once('\n').unwrap();
            rest.parse().unwrap()
        })
        .collect();
    let areas = parse_lines(sections[sections.len() - 1]).collect();
    (shapes, areas)
}

struct Area {
    width: usize,
    height: usize,
    shape_counts: Vec<usize>,
}

impl FromStr for Area {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 39x39: 45 41 45 32 31 42

        let (dim_str, counts_str) = s
            .split_once(':')
            .ok_or("missing ':' separating dimensions and counts")?;
        let (width_str, height_str) = dim_str
            .trim()
            .split_once('x')
            .ok_or("missing 'x' in dimensions")?;
        let width = width_str
            .trim()
            .parse()
            .map_err(|e| format!("failed to parse width: {e:?}"))?;
        let height = height_str
            .trim()
            .parse()
            .map_err(|e| format!("failed to parse height: {e:?}"))?;
        let shape_counts = counts_str
            .split_whitespace()
            .map(|count_str| {
                count_str
                    .parse()
                    .map_err(|e| format!("failed to parse shape count: {e:?}"))
            })
            .collect::<Result<_, _>>()?;
        Ok(Area {
            width,
            height,
            shape_counts,
        })
    }
}
