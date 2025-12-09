use aoc::{Point3D, pairs_without_dups, parse_lines_with};
use std::collections::{HashMap, HashSet};

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let solver = Solver::new(&input);
        let mut circuits = solver.solve_part_1();

        circuits.sort_by_key(|it| -i32::try_from(it.len()).unwrap());
        (circuits[0].len() * circuits[1].len() * circuits[2].len()).to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let solver = Solver::new(&input);
        let (last_p, last_q) = solver.solve_part_2();

        (last_p.0 * last_q.0).to_string()
    }
}

struct Solver {
    points: Vec<Point3D>,
    circuits: Vec<HashSet<Point3D>>,
    points_to_circuit: HashMap<Point3D, usize>,
}

impl Solver {
    fn new(input: &str) -> Self {
        let points: Vec<Point3D> = parse_lines_with(input, parse_point).collect();
        let circuits: Vec<HashSet<Point3D>> = points
            .iter()
            .map(|&p| {
                let mut set = HashSet::new();
                set.insert(p);
                set
            })
            .collect();
        let points_to_circuit: HashMap<Point3D, usize> =
            points.iter().enumerate().map(|(a, b)| (*b, a)).collect();
        Self {
            points,
            circuits,
            points_to_circuit,
        }
    }

    fn point_pairs(&self) -> Vec<(Point3D, Point3D)> {
        let mut point_pairs: Vec<(Point3D, Point3D)> = pairs_without_dups(&self.points)
            .map(|t| (*t.0, *t.1))
            .collect();
        point_pairs.sort_by_key(|(p, q)| p.sq_dist(*q));
        point_pairs
    }

    fn solve_part_1(mut self) -> Vec<HashSet<Point3D>> {
        for (p, q) in &self.point_pairs()[0..1000] {
            self.connect(*p, *q);
        }
        self.circuits
    }

    fn solve_part_2(mut self) -> (Point3D, Point3D) {
        for (p, q) in self.point_pairs() {
            self.connect(p, q);

            if self.circuits.iter().filter(|c| !c.is_empty()).count() == 1 {
                return (p, q);
            }
        }
        panic!("All points not connected");
    }

    fn connect(&mut self, p: Point3D, q: Point3D) {
        let circuit_p = self.points_to_circuit[&p];
        let circuit_q = self.points_to_circuit[&q];
        if circuit_p != circuit_q {
            let set_p = &self.circuits[circuit_p];
            let set_q = &self.circuits[circuit_q];
            let new_set = set_p.union(set_q).copied().collect::<HashSet<_>>();
            for &point in &new_set {
                self.points_to_circuit.insert(point, circuit_p);
            }
            self.circuits[circuit_p] = new_set;
            self.circuits[circuit_q].clear();
        }
    }
}

fn parse_point(line: &str) -> Point3D {
    let mut parts = line.split(',').map(|s| s.trim().parse().unwrap());
    Point3D(
        parts.next().unwrap(),
        parts.next().unwrap(),
        parts.next().unwrap(),
    )
}
