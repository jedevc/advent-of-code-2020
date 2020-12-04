use std::fs;

use regex::Regex;
use std::collections::HashMap;

type Passport = HashMap<String, String>;

const REQUIRED_FIELDS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

lazy_static! {
    static ref DATA: Vec<Passport> = {
        fs::read_to_string("input/day4/passports.txt")
            .expect("could not open file")
            .split("\n\n")
            .map(|s| s.split_ascii_whitespace())
            .map(parse_passport)
            .collect()
    };
    static ref REGEXES: HashMap<&'static str, Regex> = {
        let mut m = HashMap::new();
        m.insert("byr", Regex::new(r"^(19[2-9][0-9]|200[0-2])$").unwrap());
        m.insert("iyr", Regex::new(r"^20(1[0-9]|20)$").unwrap());
        m.insert("eyr", Regex::new(r"^20(2[0-9]|30)$").unwrap());
        m.insert(
            "hgt",
            Regex::new(r"^((1[5-8][0-9]|19[0-3])cm|(59|6[0-9]|7[0-6])in)$").unwrap(),
        );
        m.insert("hcl", Regex::new(r"^#[0-9a-f]{6}$").unwrap());
        m.insert("ecl", Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap());
        m.insert("pid", Regex::new(r"^[0-9]{9}$").unwrap());
        m
    };
}

pub fn solve() {
    let checked: Vec<&Passport> = DATA.iter().filter(|p| check_passport(&p)).collect();
    let validated: Vec<&Passport> = checked
        .iter()
        .map(|p| *p)
        .filter(|p| validate_passport(&p))
        .collect();

    println!("----- part 1 -----");
    println!("count: {}", checked.len());
    println!();

    println!("----- part 2 -----");
    println!("count: {}", validated.len());
    println!();
}

fn parse_passport<'a>(parts: impl Iterator<Item = &'a str>) -> Passport {
    let mut passport = HashMap::new();
    for part in parts {
        let mut spec = part.split(":");

        let name = spec.next().unwrap().to_string();
        let data = spec.next().unwrap().to_string();

        passport.insert(name, data);
    }

    passport
}

fn check_passport(passport: &Passport) -> bool {
    for field in &REQUIRED_FIELDS {
        if !passport.contains_key(*field) {
            return false;
        }
    }

    true
}

fn validate_passport(passport: &Passport) -> bool {
    for (field, regex) in REGEXES.iter() {
        if !regex.is_match(&passport[*field]) {
            return false;
        }
    }

    true
}
