use std::fs;

lazy_static! {
    static ref DATA: Vec<Vec<bool>> = {
        fs::read_to_string("input/day3/map.txt")
            .expect("could not open file")
            .lines()
            .map(|line| line.chars().map(|ch| ch == '#').collect())
            .collect()
    };
}

pub fn solve1() {
    println!("trees: {}", slope(1, 3));
}

pub fn solve2() {
    let product = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
        .iter()
        .map(|(y, x)| slope(*y, *x))
        .fold(1, |a, b| a * b);
    println!("product: {}", product);
}

fn slope(y: usize, x: usize) -> usize {
    let mut count = 0;

    let (mut j, mut i) = (0, 0);
    while j < DATA.len() {
        let tree = DATA[j][i % DATA[j].len()];
        if tree {
            count += 1;
        }

        j += y;
        i += x;
    }

    count
}
