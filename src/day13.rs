use std::fs;

lazy_static! {
    static ref DATA: (usize, Vec<Option<usize>>) = {
        let lines: Vec<String> = fs::read_to_string("input/day13/timetable.txt")
            .expect("could not open file")
            .lines()
            .map(str::to_string)
            .collect();

        let n = lines[0].parse().unwrap();
        let ns = lines[1].split(",").map(|n| n.parse().ok()).collect();
        (n, ns)
    };
}

pub fn solve1() {
    let earliest = DATA.0;
    let ids: Vec<usize> = DATA.1.iter().filter_map(|id| *id).collect();
    let (id, wait) = ids
        .iter()
        .map(|id| (*id, (earliest / id + 1) * id - earliest))
        .min_by_key(|(_, s)| *s)
        .unwrap();

    println!("{} x {} = {}", id, wait, id * wait);
}

pub fn solve2() {
    let ids: Vec<(usize, usize)> = DATA
        .1
        .iter()
        .enumerate()
        .filter_map(|(i, oid)| oid.map(|id| (i, id)))
        .collect();
    let result = ids.into_iter().fold_first(satisfy).unwrap();
    println!("result: {}", result.0);
}

fn satisfy((a1, b1): (usize, usize), (a2, b2): (usize, usize)) -> (usize, usize) {
    let mut n = a1;
    loop {
        if (n + a2) % b2 == 0 {
            return (n, b1 * b2);
        }
        n += b1;
    }
}

