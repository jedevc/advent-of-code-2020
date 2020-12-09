use std::fs;

use std::collections::VecDeque;

lazy_static! {
    static ref DATA: Vec<usize> = {
        fs::read_to_string("input/day9/numbers.txt")
            .expect("could not open file")
            .lines()
            .map(str::parse::<usize>)
            .map(Result::unwrap)
            .collect()
    };
}

pub fn solve1() {
    let mut xmas = XMAS::new();
    let mut invalid = 0;
    for n in DATA.iter() {
        if !xmas.feed(*n) {
            invalid = *n;
            break;
        }
    }
    println!("invalid number: {}", invalid);
}

pub fn solve2() {
    let mut xmas = XMAS::new();
    let mut invalid = 0;
    for n in DATA.iter() {
        if !xmas.feed(*n) {
            invalid = *n;
            break;
        }
    }

    let mut weakness = 0;
    for (i, x) in DATA.iter().enumerate() {
        let mut sum = *x;
        for (j, y) in DATA.iter().enumerate().skip(i + 1) {
            sum += y;
            if sum == invalid {
                let range = &DATA[i..=j];
                let min = range.iter().min().unwrap();
                let max = range.iter().max().unwrap();
                weakness = min + max;
                break
            }
        }
    }
    println!("weakness: {}", weakness);
}

struct XMAS {
    queue: VecDeque<usize>,
}

impl XMAS {
    fn new() -> XMAS {
        XMAS { queue: VecDeque::new() }
    }

    fn check(&self, n: usize) -> bool {
        for (i, x) in self.queue.iter().enumerate() {
            for y in self.queue.iter().skip(i + 1) {
                if x + y == n {
                    return true;
                }
            }
        }

        false
    }

    fn feed(&mut self, n: usize) -> bool {
        if self.queue.len() == 25 {
            if self.check(n) == false {
                return false
            }
            self.queue.pop_front();
        }

        self.queue.push_back(n);

        true
    }
}
