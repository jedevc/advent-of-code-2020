use std::fs;

lazy_static! {
    static ref DATA: Vec<Opcode> = {
        fs::read_to_string("input/day8/program.txt")
            .expect("could not open file")
            .lines()
            .map(Opcode::parse)
            .collect()
    };
}

pub fn solve() {
    println!("----- part 1 -----");
    let (_, acc) = emulate(&DATA);
    println!("acc was {}", acc);

    println!("----- part 2 -----");
    for i in 0..DATA.len() {
        let replaced = match DATA[i] {
            Opcode::Nop(n) => Opcode::Jmp(n),
            Opcode::Jmp(n) => Opcode::Nop(n),
            _ => continue,
        };
        let program: Vec<Opcode> = DATA
            .iter()
            .enumerate()
            .map(|(j, op)| if j == i { &replaced } else { op })
            .cloned()
            .collect();

        let (success, acc) = emulate(&program);
        if success {
            println!("acc was {}", acc);
        }
    }

    println!();
}

fn emulate(program: &[Opcode]) -> (bool, isize) {
    let mut emulator = Emulator::new(program);
    let mut trace = vec![false; DATA.len()];
    while !emulator.done() {
        if trace[emulator.pc] {
            return (false, emulator.acc);
        }
        trace[emulator.pc] = true;
        emulator.execute();
    }

    (true, emulator.acc)
}

struct Emulator<'a> {
    program: &'a [Opcode],
    pc: usize,
    acc: isize,
}

impl<'a> Emulator<'a> {
    fn new(program: &'a [Opcode]) -> Emulator {
        Emulator {
            program,
            pc: 0,
            acc: 0,
        }
    }

    fn done(&self) -> bool {
        self.pc >= self.program.len()
    }

    fn execute(&mut self) {
        let opcode = &self.program[self.pc];
        self.pc += 1;

        match opcode {
            Opcode::Nop(_) => (),
            Opcode::Acc(n) => self.acc += n,
            Opcode::Jmp(n) => self.pc = ((self.pc as isize) + n - 1) as usize,
        }
    }
}

#[derive(Clone, Debug)]
enum Opcode {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl Opcode {
    fn parse(line: &str) -> Opcode {
        let parts: Vec<&str> = line.split(" ").collect();
        let n = parts[1].parse().unwrap();
        match parts[0] {
            "nop" => Opcode::Nop(n),
            "acc" => Opcode::Acc(n),
            "jmp" => Opcode::Jmp(n),
            _ => panic!("unrecognized opcode"),
        }
    }
}
