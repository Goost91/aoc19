#![allow(unused_variables)]
#![allow(unused_imports)]

use std::thread;

#[macro_use] mod common;
mod day1;
mod day2;
mod day3;

// dummy file to fool intellij-rust into thinking the separate files are part of the module
fn main() {/*
    println!("Result of day1.1: {}", day1::part1());
    println!("Result of day1.2: {}", day1::part2());

    println!("Result of day2.1: {}", day2::part1());
    println!("Result of day2.2: {}", day2::part2());*/

    println!("Result of day2.1: {}", day3::part1());
    println!("Result of day2.2: {}", day3::part2());
}