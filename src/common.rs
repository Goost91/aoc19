use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use std::fs;
use std::time::*;


#[macro_export]
macro_rules! line_reader {
    ($x:expr) => {
    BufReader::new(File::open(format!("input/day{}", $x)).unwrap())
        .lines()
        .map(Result::unwrap)
    }
}

#[macro_export]
macro_rules! parsed_line_reader {
    ($x:expr, $y:ty) => {
    BufReader::new(File::open(format!("input/day{}", $x)).unwrap())
        .lines()
        .map(Result::unwrap)
        .map(|line| line.parse::<$y>().unwrap())
    }
}

#[macro_export]
macro_rules! split_reader {
    ($x:expr, $y:expr) => {
        fs::read_to_string(format!("input/day{}", $x)).unwrap().split($y)
    }
}


pub fn read_input(day: u8) -> BufReader<File> {
    BufReader::new(File::open(format!("input/day{}", day)).unwrap())
}

