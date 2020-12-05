use std::fs;

lazy_static! {
    static ref DATA: Vec<BoardingPass> = {
        fs::read_to_string("input/day5/boarding-passes.txt")
            .expect("could not open file")
            .lines()
            .map(|s| BoardingPass::parse(s))
            .collect()
    };
}

pub fn solve() {
    println!("----- part 1 -----");
    let max_id = DATA.iter().map(|bp| bp.id()).max().unwrap();
    println!("max id: {}", max_id);
    println!();

    println!("----- part 2 -----");
    let mut seating_map = [false; 8 * 128];
    for bp in DATA.iter() {
        seating_map[bp.id() as usize] = true;
    }

    let start = 8;
    let end = 127 * 8;
    for i in start..end {
        if !seating_map[i] && seating_map[i - 1] && seating_map[i + 1] {
            println!("seat id: {}", i);
            break;
        }
    }
    println!();
}

#[derive(Debug)]
struct BoardingPass {
    row: u8,
    column: u8,
}

impl BoardingPass {
    fn parse(spec: &str) -> BoardingPass {
        let row = spec[0..7]
            .chars()
            .fold(0, |acc, ch| 2 * acc + parse_letter(ch));
        let column = spec[7..10]
            .chars()
            .fold(0, |acc, ch| 2 * acc + parse_letter(ch));

        BoardingPass { row, column }
    }

    fn id(&self) -> u16 {
        (self.row as u16) * 8 + (self.column as u16)
    }
}

fn parse_letter(letter: char) -> u8 {
    match letter {
        'F' => 0,
        'B' => 1,
        'L' => 0,
        'R' => 1,
        _ => panic!("invalid character"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let bp = BoardingPass::parse("FBFBBFFRLR");
        assert_eq!(bp.row, 44);
        assert_eq!(bp.column, 5);
    }

    #[test]
    fn test_id() {
        let bp = BoardingPass { row: 44, column: 5 };
        assert_eq!(bp.id(), 357);
    }
}
