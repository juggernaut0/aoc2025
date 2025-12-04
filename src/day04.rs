use aoc::{Grid, Point};

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let grid: Grid<Tile> = input.parse().unwrap();

        find_accessible(&grid).len().to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let mut grid: Grid<Tile> = input.parse().unwrap();

        let mut count = 0;
        loop {
            let accessible = find_accessible(&grid);
            if accessible.is_empty() {
                break;
            }

            count += accessible.len();

            for p in accessible {
                grid.set(p, Tile::Empty);
            }
        }
        count.to_string()
    }
}

enum Tile {
    Empty,
    Roll,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '@' => Tile::Roll,
            _ => panic!("Invalid tile character: {c}"),
        }
    }
}

fn find_accessible(grid: &Grid<Tile>) -> Vec<Point> {
    grid.points_with_item()
        .filter_map(|(p, t)| {
            Some(p).filter(|_| matches!(t, Tile::Roll)).filter(|p| {
                let num_neighbors = p
                    .adj_diag()
                    .into_iter()
                    .filter_map(|a| grid.get(a))
                    .filter(|it| matches!(it, Tile::Roll))
                    .count();

                num_neighbors < 4
            })
        })
        .collect()
}
