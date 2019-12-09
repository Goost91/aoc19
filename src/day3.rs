use std::fs::*;
use std::io::*;
use std::collections::HashSet;
use std::ops::*;
use std::time::Instant;

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

type Instruction = (Direction, i64);
pub type Point = (i64, i64);

pub fn part1() -> i64 {
    let input = line_reader!(3).collect::<Vec<String>>();

    let first_wire = process_wire(input[0].split(",")
                                          .map(parse_word)
                                          .collect::<Vec<Instruction>>());
    let second_wire = process_wire(input[1].split(",")
                                           .map(parse_word)
                                           .collect::<Vec<Instruction>>());

    let mut collisions = first_wire.iter().filter(|t| second_wire.contains(t)).map(manhattan_distance).collect::<Vec<i64>>();
    collisions.sort_unstable();
    collisions[0]
}

fn manhattan_distance(point: &Point) -> i64 {
    (point.0.abs() + point.1.abs())
}

pub fn part2() -> i32 {
    let input = line_reader!(3).collect::<Vec<String>>();

    let first_insns = input[0].split(",")
                              .map(parse_word)
                              .collect::<Vec<Instruction>>();
    let second_insns = input[1].split(",")
                              .map(parse_word)
                              .collect::<Vec<Instruction>>();

    let first_wire = process_wire(first_insns.to_vec());
    let second_wire = process_wire(second_insns.to_vec());

    let mut collisions = first_wire.iter().filter(|t| second_wire.contains(t)).collect::<Vec<&Point>>();
    let mut collision_steps = Vec::new();

    for point in collisions {
        collision_steps.push(steps_until(first_insns.to_vec(), *point)+steps_until(second_insns.to_vec(), *point));

    }

    collision_steps.sort_unstable();
    collision_steps[0]
}

fn parse_word(s: &str) -> Instruction {
    let len = s[1..].parse::<i64>().unwrap();
    match s.chars().nth(0).unwrap() {
        'L' => (Direction::Left, len),
        'R' => (Direction::Right, len),
        'U' => (Direction::Up, len),
        'D' => (Direction::Down, len),
        _ => unreachable!()
    }
}

fn add_point(point: Point, dir: &Instruction) -> Point {
    let mut dx = 0;
    let mut dy = 0;
    match dir.0 {
        Direction::Left => dx = -1,
        Direction::Right => dx = 1,
        Direction::Up => dy = 1,
        Direction::Down => dy = -1
    }
    Point::from((point.0 + dx, point.1 + dy))
}

fn process_wire(input: Vec<Instruction>) -> HashSet<Point> {
    let mut curr_point = Point::from((0, 0));
    let mut result = HashSet::new();

    for p in 0..input.len() {
        for i in 0..input[p].1 {
            curr_point = add_point(curr_point, &input[p]);
            result.insert(curr_point.clone());
        }
    }

    result
}

fn steps_until(input: Vec<Instruction>, point: Point) -> i32 {
    let mut curr_point = Point::from((0, 0));
    let mut steps = 0;

    'outer: for p in 0..input.len() {
        for i in 0..input[p].1 {
            curr_point = add_point(curr_point, &input[p]);
            steps += 1;

            if curr_point == point {
                break 'outer;
            }
        }
    }

    steps
}