use std::fs;

lazy_static! {
    static ref DATA: Vec<(Instruction, i32)> = {
        fs::read_to_string("input/day12/instructions.txt")
            .expect("could not open file")
            .lines()
            .map(|line| {
                let chars: Vec<char> = line.chars().collect();
                let instruction = match chars[0] {
                    'N' => Instruction::North,
                    'S' => Instruction::South,
                    'E' => Instruction::East,
                    'W' => Instruction::West,
                    'L' => Instruction::Left,
                    'R' => Instruction::Right,
                    'F' => Instruction::Forward,
                    _ => panic!("invalid character"),
                };
                let amount = chars[1..].iter().collect::<String>().parse().unwrap();
                (instruction, amount)
            })
            .collect()
    };
}

enum Instruction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

pub fn solve1() {
    let (mut x, mut y): (i32, i32) = (0, 0);
    let mut direction: i32 = 0;

    for (instruction, amount) in DATA.iter() {
        match instruction {
            Instruction::North => y -= *amount,
            Instruction::South => y += *amount,
            Instruction::East => x += *amount,
            Instruction::West => x -= *amount,
            Instruction::Left => direction -= *amount,
            Instruction::Right => direction += *amount,
            Instruction::Forward => match direction % 360 {
                -270 => y += *amount,
                -180 => x -= *amount,
                -90 => y -= *amount,
                0 => x += *amount,
                90 => y += *amount,
                180 => x -= *amount,
                270 => y -= *amount,
                _ => panic!("invalid direction"),
            },
        }
    }
    println!("x: {}, y: {}", x, y);
    println!("manhattan distance: {}", x.abs() + y.abs());
}

pub fn solve2() {
    let (mut x, mut y): (i32, i32) = (0, 0);
    let (mut wx, mut wy): (i32, i32) = (10, -1);

    for (instruction, amount) in DATA.iter() {
        match instruction {
            Instruction::North => wy -= *amount,
            Instruction::South => wy += *amount,
            Instruction::East => wx += *amount,
            Instruction::West => wx -= *amount,
            Instruction::Left => {
                (wx, wy) = rotate(wx, wy, 360 - *amount);
            }
            Instruction::Right => {
                (wx, wy) = rotate(wx, wy, *amount);
            }
            Instruction::Forward => {
                y += wy * *amount;
                x += wx * *amount;
            }
        }
    }
    println!("x: {}, y: {}", x, y);
    println!("manhattan distance: {}", x.abs() + y.abs());
}

fn rotate(x: i32, y: i32, amount: i32) -> (i32, i32) {
    match amount {
        0 => (x, y),
        90 => (-y, x),
        180 => (-x, -y),
        270 => (y, -x),
        _ => panic!("invalid rotation"),
    }
}
