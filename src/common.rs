use std::fs::File;
use std::io::BufReader;
use std::error::Error;

#[macro_export]
macro_rules! input_reader {
    ($x:expr) => {
    BufReader::new(File::open(format!("input/day{}", $x)).unwrap())
        .lines()
        .map(Result::unwrap)
    }
}

pub fn read_input(day: u8) -> BufReader<File> {
    BufReader::new(File::open(format!("input/day{}", day)).unwrap())
}

