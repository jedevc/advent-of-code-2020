use std::fs;
use std::cmp::Ordering;

lazy_static! {
    static ref REPORT: ExpenseReport = ExpenseReport::new();
}

pub fn solve() {
    println!("----- part 1 -----");
    REPORT.repair();
    println!();

    println!("----- part 2 -----");
    REPORT.repair3();
    println!();
}

struct ExpenseReport {
    nums: Vec<i32>
}

impl ExpenseReport {
    fn new() -> ExpenseReport {
        let mut nums: Vec<i32> =
            fs::read_to_string("input/day1/expense-report.txt").expect("could not open file")
            .split("\n")
            .filter_map(|line| line.parse::<i32>().ok())
            .collect();
        nums.sort();

        ExpenseReport{nums}
    }

    fn repair(&self) {
        if let Some((i, j)) = find_pair(&self.nums, 2020) {
            let solution = self.nums[i] * self.nums[j];
            println!("{} + {} = {}", self.nums[i], self.nums[j], 2020);
            println!("{} x {} = {}", self.nums[i], self.nums[j], solution);
        } else {
            println!("Failure :(");
        }
    }

    fn repair3(&self) {
        for k in 0..self.nums.len() - 2 {
            let numslice = &self.nums[k + 1..];
            if let Some((i, j)) = find_pair(numslice, 2020 - self.nums[k]) {
                let solution = self.nums[k] * numslice[i] * numslice[j];
                println!("{} + {} + {} = {}", self.nums[k], numslice[i], numslice[j], 2020);
                println!("{} x {} x {} = {}", self.nums[k], numslice[i], numslice[j], solution);
                return;
            }
        }

        println!("Failure :(");
    }
}

fn find_pair(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let (mut i, mut j) = (0, nums.len() - 1);
    while i < j {
        let sum = nums[i] + nums[j];
        match sum.cmp(&target) {
            Ordering::Equal => {
                return Some((i, j));
            },
            Ordering::Less => i += 1,
            Ordering::Greater => j -= 1,
        }
    }

    None
}
