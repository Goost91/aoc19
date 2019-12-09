use crate::intcode::{Word, State};
use crate::intcode;
use std::fs::File;
use std::fs;
use std::time::Instant;

pub fn part1() -> Word {
    let s = Instant::now();
    let mut data: Vec<Word> = split_reader!(5, ",")
        .map(|w| w.parse::<Word>().unwrap())
        .collect();

    run(&mut data, 1);

    time_since!(s);
    0
}

pub fn part2() -> Word {
    let s = Instant::now();
    let mut data: Vec<Word> = split_reader!(5, ",")
        .map(|w| w.parse::<Word>().unwrap())
        .collect();

    run(&mut data, 5);
    time_since!(s);
    0
}

fn run(mut data: &mut Vec<Word>, input: Word) -> Word {
    let mut state = State { mem: &mut data.to_vec(), input: &mut vec![input], ip: 0, current_opcode: 0 };
    intcode::run(&mut state);
    0
}


