use aoc::{Point, pairs_without_dups, parse_lines};

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let points: Vec<Point<i64>> = parse_lines(&input).collect();
        pairs_without_dups(&points)
            .map(|(p, q)| ((p.0 - q.0).abs() + 1) * ((p.1 - q.1).abs() + 1))
            .max()
            .unwrap()
            .to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let points: Vec<Point<i64>> = parse_lines(&input).collect();
        let mut edges: Vec<_> = points.windows(2).map(|ps| (ps[0], ps[1])).collect();
        edges.push((points[points.len() - 1], points[0]));

        // a rectangle is interior if all edges are fully to one side of it

        pairs_without_dups(&points)
            .filter(|(p, q)| {
                let min_x = p.0.min(q.0);
                let max_x = p.0.max(q.0);
                let min_y = p.1.min(q.1);
                let max_y = p.1.max(q.1);
                edges.iter().all(|&(edge_start, edge_end)| {
                    if edge_start.0 <= min_x && edge_end.0 <= min_x {
                        // left
                        true
                    } else if edge_start.0 >= max_x && edge_end.0 >= max_x {
                        // right
                        true
                    } else if edge_start.1 <= min_y && edge_end.1 <= min_y {
                        // below
                        true
                    } else if edge_start.1 >= max_y && edge_end.1 >= max_y {
                        // above
                        true
                    } else {
                        false
                    }
                })
            })
            .map(|(p, q)| ((q.0 - p.0).abs() + 1) * ((q.1 - p.1).abs() + 1))
            .max()
            .unwrap()
            .to_string()
    }
}
