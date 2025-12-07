use aoc::{Counter, Grid, Point};
use std::collections::HashSet;

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let map: Grid<Tile> = input.parse().unwrap();
        let start_p = map
            .points_with_item()
            .find(|(_, t)| matches!(t, Tile::Start))
            .unwrap()
            .0;
        let mut split_count = 0;
        let mut beams = HashSet::new();
        beams.insert(start_p.0);
        let mut y = start_p.1;
        loop {
            y += 1;
            if y >= map.height() {
                break;
            }
            let mut new_beams = HashSet::new();
            for x in beams {
                match &map[Point(x, y)] {
                    Tile::Empty => {
                        new_beams.insert(x);
                    }
                    Tile::Splitter => {
                        split_count += 1;
                        new_beams.insert(x - 1);
                        new_beams.insert(x + 1);
                    }
                    Tile::Start => panic!("Beam hit start tile again!"),
                }
            }
            beams = new_beams;
        }
        split_count.to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let map: Grid<Tile> = input.parse().unwrap();
        let start_p = map
            .points_with_item()
            .find_map(|(p, t)| Some(p).filter(|_| matches!(t, Tile::Start)))
            .unwrap();
        let mut beams = Counter::new();
        beams.count(start_p.0);
        let mut y = start_p.1;
        loop {
            y += 1;
            if y >= map.height() {
                break;
            }
            let mut new_beams = Counter::new();
            for (x, n) in beams {
                match &map[Point(x, y)] {
                    Tile::Empty => {
                        new_beams.count_n(x, n);
                    }
                    Tile::Splitter => {
                        new_beams.count_n(x - 1, n);
                        new_beams.count_n(x + 1, n);
                    }
                    Tile::Start => panic!("Beam hit start tile again!"),
                }
            }
            beams = new_beams;
        }
        beams.total().to_string()
    }
}

enum Tile {
    Empty,
    Start,
    Splitter,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            'S' => Tile::Start,
            '^' => Tile::Splitter,
            _ => panic!("Unknown tile character: {c}"),
        }
    }
}
