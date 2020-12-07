#![feature(iterator_fold_self)]

#[macro_use]
extern crate lazy_static;

extern crate regex;

extern crate clap;
use clap::App;

use std::collections::HashMap;
use std::time::SystemTime;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

lazy_static! {
    static ref SOLVERS: HashMap<&'static str, fn()> = {
        let mut m = HashMap::new();
        m.insert("day1", day1::solve as fn());
        m.insert("day2", day2::solve as fn());
        m.insert("day3", day3::solve as fn());
        m.insert("day4", day4::solve as fn());
        m.insert("day5", day5::solve as fn());
        m.insert("day6", day6::solve as fn());
        m.insert("day7", day7::solve as fn());
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
        .map(|x| x.parse::<i32>().unwrap_or(1))
        .unwrap_or(1);

    match SOLVERS.get(solver_name) {
        Some(solver) => {
            let now = SystemTime::now();
            for _ in 0..repeats {
                solver();
            }
            let nanos = now.elapsed().unwrap().as_nanos();
            eprintln!(
                "time = {}.{:0>9}",
                nanos / 1_000_000_000,
                nanos % 1_000_000_000
            )
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
