#![feature(iterator_fold_self, min_const_generics)]

#[macro_use]
extern crate lazy_static;

extern crate regex;

extern crate clap;
use clap::App;

use std::collections::HashMap;
use std::time::SystemTime;

mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

macro_rules! solvers {
    ($day:ident) => {
        ($day::solve1 as fn(), $day::solve2 as fn())
    };
}

lazy_static! {
    static ref SOLVERS: HashMap<&'static str, (fn(), fn())> = {
        let mut m = HashMap::new();
        m.insert("day1", solvers!(day1));
        m.insert("day2", solvers!(day2));
        m.insert("day3", solvers!(day3));
        m.insert("day4", solvers!(day4));
        m.insert("day5", solvers!(day5));
        m.insert("day6", solvers!(day6));
        m.insert("day7", solvers!(day7));
        m.insert("day8", solvers!(day8));
        m.insert("day9", solvers!(day9));
        m.insert("day10", solvers!(day10));
        m.insert("day11", solvers!(day11));
        m
    };
}

fn main() {
    let matches = App::new("Advent of Code")
        .author("Justin Chadwell <me@jedevc.com>")
        .about("Solvers for Advent of Code 2020")
        .arg_from_usage("<solver> 'solver to invoke'")
        .arg_from_usage("--repeat [repeats] 'number of times to repeat'")
        .get_matches();

    let solver_name = matches.value_of("solver").unwrap();
    let repeats = matches
        .value_of("repeat")
        .map(|x| x.parse::<usize>().unwrap_or(1))
        .unwrap_or(1);

    match SOLVERS.get(solver_name) {
        Some((solver1, solver2)) => {
            println!("----- part 1 -----");
            exec_timed(*solver1, repeats);

            println!();

            println!("----- part 2 -----");
            exec_timed(*solver2, repeats);
        }
        None => {
            eprintln!("No such solver with name '{}'\n", solver_name);
            eprintln!("Available solvers:");
            for key in SOLVERS.keys() {
                eprintln!("- {}", key);
            }
        }
    }
}

fn exec_timed(target: fn(), repeats: usize) {
    let now = SystemTime::now();
    for _ in 0..repeats {
        target();
    }
    let nanos = now.elapsed().unwrap().as_nanos();
    eprintln!(
        "time = {}.{:0>9}",
        nanos / 1_000_000_000,
        nanos % 1_000_000_000
    )
}
