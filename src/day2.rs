use std::fs;

lazy_static! {
    static ref DATA: Vec<Spec> = {
        let pw_file = fs::read_to_string("input/day2/passwords.txt").expect("could not open file");
        let lines = pw_file.lines().filter(|x| x.len() > 0);

        let parts = lines.map(|line| {
            line.split(" ")
                .map(|s| s.strip_suffix(":").unwrap_or(s))
                .flat_map(|s| s.split("-"))
                .collect::<Vec<&str>>()
        });

        parts
            .map(|line| Spec {
                lower: line[0].parse::<usize>().unwrap(),
                upper: line[1].parse::<usize>().unwrap(),
                target: line[2].chars().next().unwrap(),
                password: line[3].to_string(),
            })
            .collect()
    };
}

struct Spec {
    lower: usize,
    upper: usize,
    target: char,
    password: String,
}

pub fn solve1() {
    check_count_policy();
}

pub fn solve2() {
    check_index_policy();
}

pub fn check_count_policy() {
    let mut valid = 0;

    for spec in DATA.iter() {
        let number = spec
            .password
            .chars()
            .filter(|ch| *ch == spec.target)
            .count();

        valid += if number >= spec.lower && number <= spec.upper {
            1
        } else {
            0
        };
    }

    println!("{}", valid);
}

fn check_index_policy() {
    let mut valid = 0;

    for spec in DATA.iter() {
        let lm = spec
            .password
            .chars()
            .nth(spec.lower - 1)
            .map_or(false, |ch| ch == spec.target);
        let um = spec
            .password
            .chars()
            .nth(spec.upper - 1)
            .map_or(false, |ch| ch == spec.target);

        valid += if lm && !um || !lm && um { 1 } else { 0 }
    }

    println!("{}", valid);
}
