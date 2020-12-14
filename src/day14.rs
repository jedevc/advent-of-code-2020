use std::fs;

use std::collections::HashMap;
use std::sync::Arc;

lazy_static! {
    static ref DATA: Vec<(Assign, Mask)> = {
        let lines: Vec<String> = fs::read_to_string("input/day14/program.txt")
            .expect("could not open file")
            .lines()
            .map(str::to_string)
            .collect();

        let mut mask = None;
        let mut parts = vec![];
        for line in lines {
            if line.starts_with("mask") {
                mask = Some(Mask::parse(&line));
            } else if line.starts_with("mem") {
                parts.push((Assign::parse(&line), mask.clone().unwrap()));
            } else {
                panic!("invalid line")
            }
        }

        parts
    };
}

#[derive(Debug, Clone)]
struct Mask {
    mask: Arc<Vec<char>>,
}

#[derive(Debug)]
struct Assign {
    target: u64,
    value: u64,
}

impl Mask {
    fn new(spec: &str) -> Mask {
        Mask {
            mask: Arc::new(spec.chars().collect()),
        }
    }

    fn parse(spec: &str) -> Mask {
        let mask = spec.split(" = ").skip(1).next().unwrap();
        Mask::new(mask)
    }

    fn apply_mask(&self, target: u64) -> u64 {
        let and_mask = self
            .mask
            .iter()
            .map(|&ch| ch == '0')
            .fold(0, |acc, x| acc * 2 + if x { 0 } else { 1 });
        let or_mask = self
            .mask
            .iter()
            .map(|&ch| ch == '1')
            .fold(0, |acc, x| acc * 2 + if x { 1 } else { 0 });
        target & and_mask | or_mask
    }
}

impl Assign {
    fn parse(spec: &str) -> Assign {
        let target = spec
            .split("[")
            .skip(1)
            .next()
            .unwrap()
            .split("]")
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let value = spec.split(" = ").skip(1).next().unwrap().parse().unwrap();
        Assign { target, value }
    }
}

pub fn solve1() {
    let mut mem = HashMap::new();
    for (assign, mask) in DATA.iter() {
        mem.insert(assign.target, mask.apply_mask(assign.value));
    }
    let sum: u64 = mem.iter().map(|(_, v)| *v).sum();
    println!("sum: {}", sum)
}

pub fn solve2() {}

