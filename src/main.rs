#![warn(clippy::pedantic)]

use aoc::Solution;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

const SOLUTIONS: [&dyn Solution; 12] = [
    &day01::Solution,
    &day02::Solution,
    &day03::Solution,
    &day04::Solution,
    &day05::Solution,
    &day06::Solution,
    &day07::Solution,
    &day08::Solution,
    &day09::Solution,
    &day10::Solution,
    &day11::Solution,
    &day12::Solution,
];

fn main() {
    aoc::run("2025", SOLUTIONS);
}

#[cfg(test)]
aoc::generate_answer_tests!(SOLUTIONS, 12);
