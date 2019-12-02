#[macro_use] mod common;
mod day1;

// dummy file to fool intellij-rust into thinking the separate files are part of the module
fn main() {
    println!("Result of day1.1: {}", day1::part1());
    println!("Result of day1.2: {}", day1::part2());
}