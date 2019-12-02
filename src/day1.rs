use std::io::{BufReader, BufRead};
use std::fs::File;
use std::io;
use crate::common::read_input;

pub fn part1() -> f64 {
    run(fuel_required)
}

pub fn part2() -> f64 {
    run(|v| all_fuel_required(v, 0f64))
}

pub fn run<F>(calculate: F) -> f64
    where
        F: Fn(f64) -> f64 {
    input_reader!(1)
        .map(|line| line.parse::<f64>().unwrap())
        .map(calculate)
        .sum()
}

fn fuel_required(weight: f64) -> f64 {
    ((weight / 3f64).floor() - 2f64)
}

pub fn all_fuel_required(mass: f64, total: f64) -> f64 {
    match mass {
        m if m <= 0f64 => total,
        m => {
            let req = fuel_required(m);

            all_fuel_required(req, total + req.max(0f64))
        }
    }
}