use std::fs;

lazy_static! {
    static ref DATA: Vec<usize> = {
        let mut data: Vec<usize> = fs::read_to_string("input/day10/voltages.txt")
            .expect("could not open file")
            .lines()
            .map(str::parse::<usize>)
            .map(Result::unwrap)
            .collect();
        data.push(0);
        data.sort();
        data.push(data.last().unwrap() + 3);
        data
    };
}

pub fn solve1() {
    let mut counts = [0; 3];
    let pairs = DATA.iter().zip(DATA.iter().skip(1));
    for (v, w) in pairs {
        counts[w - v - 1] += 1;
    }
    println!("solution: {}", counts[0] * counts[2]);
}

pub fn solve2() {
    let mut counts: Vec<u128> = vec![0; DATA.len()];
    counts[0] = 1;

    for (i, v) in DATA.iter().enumerate().skip(1) {
        for j in 1..=3 {
            if i < j {
                break;
            }

            let w = DATA[i - j];
            let diff = v - w;
            if diff >= 1 && diff <= 3 {
                counts[i] += counts[i - j];
            }
        }
    }
    println!("solution: {}", counts.last().unwrap());
}
