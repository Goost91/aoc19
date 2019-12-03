use std::io::{BufReader, BufRead};
use std::fs::File;
use std::fs;
use std::io;
use std::time::Instant;
use std::convert::TryInto;

pub fn part1() -> u128 {
    let mut data: Vec<u128> = split_reader!(2, ",")
        .map(|w| w.parse::<u128>().unwrap())
        .collect();

    run(&mut data, 12, 2)
}

pub fn part2() -> usize {
    let data: Vec<u128> = split_reader!(2, ",")
        .map(|w| w.parse::<u128>().unwrap())
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

fn run(mut data: &mut Vec<u128>, noun: u128, verb: u128) -> u128 {
    let mut ip = 0;

    data[1] = noun;
    data[2] = verb;

    while data[ip] != 99 {
        let result = process_opcode(data[ip], data[ip + 1], data[ip + 2], &data);
        let dp: usize = data[ip + 3] as usize;
        data[dp] = result;

        ip += 4;
    }

    data[0]
}


fn process_opcode(opcode: u128, f: u128, s: u128, data: &Vec<u128>) -> u128 {
    match opcode {
        1 => data[f as usize] + data[s as usize],
        2 => data[f as usize] * data[s as usize],
        _ => unreachable!()
    }
}