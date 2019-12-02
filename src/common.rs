use std::fs::File;
use std::io::BufReader;
use std::error::Error;

pub fn read_input(day: u8) -> BufReader<File> {
    BufReader::new(File::open(format!("input/day{}", 1)).unwrap())
}

