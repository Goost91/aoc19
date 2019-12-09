use crate::common::to_digits;
use std::clone::Clone;
use std::borrow::Borrow;

pub type Word = i128;

#[derive(Debug)]
pub struct State<'a> {
    pub mem: &'a mut Vec<Word>,
    pub input: &'a mut Vec<Word>,
    pub ip: usize,
    pub current_opcode: Word,
}

impl State<'_> {
    fn set_mem(&mut self, addr: usize, value: Word) {
        self.mem[addr] = value
    }

    fn set_opcode(&mut self, opcode: Word) {
        self.current_opcode = opcode;
    }

    fn set_ip(&mut self, ip: usize) {
        self.ip = ip;
    }
}

pub fn opcode_length(opcode: &Word) -> Word {
    match opcode % 100 {
        1 | 2 | 7 | 8 => 4,
        5 | 6 => 3,
        3 | 4 => 2,
        _ => 1
    }
}

pub fn run(state: &mut State) {
    while state.mem[state.ip] != 99 {
        state.set_opcode(state.mem[state.ip]);
        let opcode_len = opcode_length(&state.mem[state.ip]) as usize;
        let params = get_params(opcode_len, state);
        let result = process_opcode(state, params);

        state.ip += opcode_len;
    }
}

pub fn read_value(state: &State, offset: usize) -> Word {
    let mut mode = state.current_opcode / 100;
    for _ in 1..offset {
        mode /= 10;
    }
    let value = state.mem[state.ip + offset];
    if mode % 10 > 0 {
        value
    } else {
        state.mem[value as usize]
    }
}

pub fn get_params(input_length: usize, state: &mut State) -> Vec<Word> {
    let mut results = vec![];
    let do_write = is_write_op(state.mem[state.ip]);

    if do_write {
        for i in 1..input_length - 1 {
            results.push(read_value(state, i));
        }

        results.push(state.mem[state.ip + input_length-1]);
    } else {
        for i in 1..input_length {
            results.push(read_value(state, i));
        }
    }

    results
}

pub fn process_opcode(state: &mut State, params: Vec<Word>) {
    match state.mem[state.ip] % 100 {
        1 => state.set_mem(params[2] as usize, params[0] + params[1]),
        2 => state.set_mem(params[2] as usize, params[0] * params[1]),
        3 => state.set_mem(params[0] as usize, state.input.clone().pop().unwrap()),
        4 => println!("Value is {}", params[0]),
        5 => if params[0] != 0 { state.set_ip((params[1] - 3) as usize) },
        6 => if params[0] == 0 { state.set_ip((params[1] - 3) as usize) },
        7 => state.set_mem(params[2] as usize, if params[0] < params[1] { 1 } else { 0 }),
        8 => state.set_mem(params[2] as usize, if params[0] == params[1] { 1 } else { 0 }),

        x => unreachable!("opcode {} doesn't exist\r\n State: {:?}", x, state)
    }
}

pub fn is_write_op(op: Word) -> bool {
    match op % 100 {
        1 | 2 | 3 | 7 | 8 => true,
        _ => false
    }
}