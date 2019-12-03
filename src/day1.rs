use std::io::{BufReader, BufRead};
use std::fs::File;
use std::io;
use crate::common::read_input;

pub fn part1() -> i64 {
    run(fuel_required)
}

pub fn part2() -> i64 {
    run(all_fuel_required)
}

fn run<F>(calculate: F) -> i64
    where
        F: Fn(i64) -> i64 {
    parsed_line_reader!(1, i64)
        .map(calculate)
        .sum()
}

fn fuel_required(mass: i64) -> i64 {
    ((mass as f64 / 3.0).floor() - 2.0) as i64
}

fn all_fuel_required(mass: i64) -> i64 {
    fn iter(mass: i64, total: i64) -> i64 {
        match mass {
            m if m <= 0 => total,
            m => {
                let req = fuel_required(m);

                iter(req, total + req.max(0))
            }
        }
    }

    iter(mass, 0)
}