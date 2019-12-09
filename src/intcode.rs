use crate::common::to_digits;
use std::clone::Clone;
use std::borrow::Borrow;

pub type Word = i128;

#[derive(Debug)]
pub struct State<'a> {
    pub mem: &'a mut Vec<Word>,
    pub input: &'a mut Vec<Word>,
    pub ip: usize,
    pub current_opcode: u16,
}

impl State<'_> {
    fn set_mem(&mut self, addr: usize, value: Word) {
        self.mem[addr] = value
    }

    fn set_opcode(&mut self, opcode: u16) {
        self.current_opcode = opcode;
    }

    fn set_ip(&mut self, ip: usize) {
        self.ip = ip;
    }
}

pub fn opcode_length(opcode: &u16) -> Word {
    match opcode % 100 {
        1 | 2 | 7 | 8 => 4,
        5 | 6 => 3,
        3 | 4 => 2,
        _ => 1
    }
}

pub fn run(state: &mut State) {
    state.set_opcode(state.mem[state.ip] as u16);
    while state.current_opcode != 99 {
        let opcode_len = opcode_length(&state.current_opcode) as usize;
        let result = process_opcode(state);

        state.ip += opcode_len;
        state.set_opcode(state.mem[state.ip] as u16);
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

pub fn read_imm(state: &State, offset: usize) -> usize {
    state.mem[state.ip + offset as usize] as usize
}

pub fn process_opcode(state: &mut State) {
    match state.current_opcode % 100 {
        1 => state.set_mem(read_imm(state, 3), read_value(state, 1) + read_value(state, 2)),
        2 => state.set_mem(read_imm(state, 3), read_value(state, 1) * read_value(state, 2)),

        3 => state.set_mem(read_imm(state, 1), state.input.clone().pop().unwrap()),
        4 => println!("Value is {}", read_value(state, 1)),
        5 => if read_value(state, 1) != 0 { state.set_ip((read_value(state, 2) - 3) as usize) },
        6 => if read_value(state, 1) == 0 { state.set_ip((read_value(state, 2) - 3) as usize) },
        7 => state.set_mem(read_imm(state, 3), if read_value(state, 1) < read_value(state, 2) { 1 } else { 0 }),
        8 => state.set_mem(read_imm(state, 3), if read_value(state, 1) == read_value(state, 2) { 1 } else { 0 }),

        x => unreachable!("opcode {} doesn't exist\r\n State: {:?}", x, state)
    }
}

pub fn is_write_op(op: Word) -> bool {
    match op % 100 {
        1 | 2 | 3 | 7 | 8 => true,
        _ => false
    }
}