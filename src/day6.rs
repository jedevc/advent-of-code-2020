use std::fs;

use std::collections::HashSet;
use std::iter::FromIterator;

lazy_static! {
    static ref DATA: Vec<Vec<String>> = {
        fs::read_to_string("input/day6/customs.txt")
            .expect("could not open file")
            .split("\n\n")
            .map(|g| g.lines().map(str::to_string).collect())
            .collect()
    };
}

pub fn solve() {
    println!("----- part 1 -----");
    let sum: usize = DATA
        .iter()
        .map(|group| {
            group
                .iter()
                .map(|answer| HashSet::from_iter(answer.chars()))
                .fold_first(|acc, h| -> HashSet<char> {
                    HashSet::from_iter(acc.union(&h).cloned())
                }).unwrap()
        })
        .map(|h| h.len())
        .sum();
    println!("sum: {}", sum);
    println!();

    println!("----- part 2 -----");
    let sum: usize = DATA
        .iter()
        .map(|group| {
            group
                .iter()
                .map(|answer| HashSet::from_iter(answer.chars()))
                .fold_first(|acc, h| -> HashSet<char> {
                    HashSet::from_iter(acc.intersection(&h).cloned())
                }).unwrap()
        })
        .map(|h| h.len())
        .sum();
    println!("sum: {}", sum);
    println!();
}
