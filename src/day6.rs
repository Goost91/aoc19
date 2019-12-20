use petgraph::prelude::*;
use petgraph as pg;
use std::fs::File;
use std::fs;
use std::time::Instant;

use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use crate::node::*;

pub fn part1() -> usize {
    let mut tree = ArenaTree::default();
    read_input(&mut tree);
    tree.arena.to_vec().into_iter().fold(0, |acc, node| acc + tree.depth(node.idx))
}

pub fn part2() -> usize {
    let mut tree = ArenaTree::default();
    read_input(&mut tree);
    tree.distance_between("YOU".into(), "SAN".into())
}

pub fn read_input(tree: &mut ArenaTree<String>) {
    for line in line_reader!(6) {
        let mut parts = line.split(")");
        let parent = tree.node(parts.next().unwrap().to_string());
        let child = tree.node(parts.next().unwrap().to_string());
        tree.arena[parent].children.push(child);
        tree.arena[child].parent = Some(parent);
    }
}