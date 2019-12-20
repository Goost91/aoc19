use petgraph::prelude::*;
use petgraph as pg;
use std::fs::File;
use std::fs;
use std::time::Instant;

use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use crate::node::*;

pub fn part1() -> usize {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut tree = ArenaTree::default();

    read_input(&mut tree);
    let mut result = 0;

    for x in tree.arena.to_vec() {
        result += tree.depth(x.idx)
    }

    result
}

pub fn part2() -> usize {
    let mut tree = ArenaTree::default();

    read_input(&mut tree);
    find_path(&mut tree)
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

pub fn find_path(tree: &mut ArenaTree<String>) -> usize {
    tree.distance_between("YOU".into(), "SAN".into())
}