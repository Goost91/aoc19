use std::io::{BufReader, BufRead};
use std::fs::File;
use std::fs;
use std::io;
use std::time::Instant;
use std::convert::TryInto;
use crate::intcode;
use crate::intcode::{State, Word};

pub fn part1() -> Word {
    let start = Instant::now();
    let mut data: Vec<Word> = split_reader!(2, ",")
        .map(|w| w.parse::<Word>().unwrap())
        .collect();

    run(&mut data, 12, 2);
    time_since!(start);
    0
}

pub fn part2() -> usize {
    let data: Vec<Word> = split_reader!(2, ",")
        .map(|w| w.parse::<Word>().unwrap())
        .collect();

    let mut result = 0usize;

    for x in 0..100 {
        for y in 0..100 {
            if run(&mut data.to_vec(), x.try_into().unwrap(), y.try_into().unwrap()) == 19690720 {
                result = 100 * x + y;
            }
        }
    }

    result
}

fn run(mut data: &mut Vec<Word>, noun: Word, verb: Word) -> Word {
    data[1] = noun;
    data[2] = verb;
    
    let mut state = State { mem: &mut data.to_vec(), input: &mut vec![], ip: 0, current_opcode: 0 };

    intcode::run(&mut state);

    state.mem[0]
}


