use std::io::{BufReader, BufRead};
use std::fs::File;
use std::fs;
use std::io;
use std::time::Instant;

pub fn part1() -> usize {
    let mut data: Vec<usize> = split_reader!(2, ",")
        .map(|w| w.parse::<usize>().unwrap())
        .collect();

    run(&mut data, 12, 2)
}

pub fn part2() -> usize {
    let data: Vec<usize> = split_reader!(2, ",")
        .map(|w| w.parse::<usize>().unwrap())
        .collect();

    let mut result = 0usize;

    for x in 0..=99 {
        for y in 0..=99 {
            if run(&mut data.to_vec(), x, y) == 19690720 {
                result = 100 * x + y;
            }
        }
    }

    result
}

fn run(mut data: &mut Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut ip = 0;

    data[1] = noun;
    data[2] = verb;

    while data[ip] != 99 {
        let result = process_opcode(data[ip], data[ip + 1], data[ip + 2], &data);
        let dp = data[ip + 3];
        data[dp] = result;

        ip += 4;
    }

    data[0]
}


pub fn process_opcode(opcode: usize, f: usize, s:usize, data: &Vec<usize>) -> usize {
    match opcode {
        1 => data[f] + data[s],
        2 => data[f] * data[s],
        _ => unreachable!()
    }
}