#[macro_use]
extern crate lazy_static;

extern crate clap;
use clap::App;

mod day1;

use std::collections::HashMap;

lazy_static! {
    static ref SOLVERS: HashMap<&'static str, fn()> = {
        let mut m = HashMap::new();
        m.insert("day1", day1::solve as fn());
        m
    };
}

fn main() {
    let matches = App::new("Advent of Code")
        .author("Justin Chadwell <me@jedevc.com>")
        .about("Solvers for Advent of Code 2020")
        .arg_from_usage("<solver> 'solver to invoke'")
        .get_matches();

    let solver_name = matches.value_of("solver").unwrap();

    match SOLVERS.get(solver_name) {
        Some(solver) => solver(),
        None => {
            println!("No such solver with name '{}'\n", solver_name);
            println!("Available solvers:");
            for key in SOLVERS.keys() {
                println!("- {}", key);
            }
        }
    }
}
