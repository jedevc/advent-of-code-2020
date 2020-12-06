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

fn transform<'b, F>(group: &Vec<String>, f: F) -> HashSet<char>
where
    F: for<'a> Fn(&'a HashSet<char>, &'a HashSet<char>) -> Box<dyn Iterator<Item = &'a char> + 'a>,
{
    group
        .iter()
        .map(|answer| HashSet::from_iter(answer.chars()))
        .fold_first(|acc, h| -> HashSet<char> { HashSet::from_iter(f(&acc, &h).cloned()) })
        .unwrap()
}

macro_rules! transformer {
    ($callable:expr, $method:expr) => {
        transform($callable, |a, b| Box::new($method(a, b)))
    }
}

pub fn solve() {
    println!("----- part 1 -----");
    let sum: usize = DATA
        .iter()
        .map(|group| transformer!(group, HashSet::union))
        .map(|h| h.len())
        .sum();
    println!("sum: {}", sum);
    println!();

    println!("----- part 2 -----");
    let sum: usize = DATA
        .iter()
        .map(|group| transformer!(group, HashSet::intersection))
        .map(|h| h.len())
        .sum();
    println!("sum: {}", sum);
    println!();
}
