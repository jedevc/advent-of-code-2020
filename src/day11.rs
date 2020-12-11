use std::fs;

lazy_static! {
    static ref DATA: Grid = {
        let cells = fs::read_to_string("input/day11/seats.txt")
            .expect("could not open file")
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| match ch {
                        '.' => Cell::Floor,
                        'L' => Cell::EmptySeat,
                        '#' => Cell::OccupiedSeat,
                        _ => panic!("unknown character"),
                    })
                    .collect()
            })
            .collect();
        Grid::new(cells)
    };
}

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}

pub fn solve1() {
    let matcher = |grid: &Grid, i: usize, j: usize| {
        match grid.cells[i][j] {
            Cell::Floor => (),
            Cell::EmptySeat => {
                if grid.neighbors(i, j) == 0 {
                    return Some(Cell::OccupiedSeat);
                }
            }
            Cell::OccupiedSeat => {
                if grid.neighbors(i, j) >= 4 {
                    return Some(Cell::EmptySeat);
                }
            }
        }

        None
    };
    let mut grid = DATA.next(matcher).unwrap();
    while let Some(grid_next) = grid.next(matcher) {
        grid = grid_next;
    }
    println!("count: {}", grid.count(Cell::OccupiedSeat));
}

pub fn solve2() {
    let matcher = |grid: &Grid, i: usize, j: usize| {
        match grid.cells[i][j] {
            Cell::Floor => (),
            Cell::EmptySeat => {
                if grid.visible(i, j) == 0 {
                    return Some(Cell::OccupiedSeat);
                }
            }
            Cell::OccupiedSeat => {
                if grid.visible(i, j) >= 5 {
                    return Some(Cell::EmptySeat);
                }
            }
        }

        None
    };
    let mut grid = DATA.next(matcher).unwrap();
    while let Some(grid_next) = grid.next(matcher) {
        grid = grid_next;
    }
    println!("count: {}", grid.count(Cell::OccupiedSeat));
}

impl Grid {
    fn new(cells: Vec<Vec<Cell>>) -> Grid {
        let height = cells.len();
        let width = cells[0].len();
        if !cells.iter().skip(1).all(|c| c.len() == width) {
            panic!("bad grid!");
        }

        Grid {
            cells,
            height,
            width,
        }
    }

    fn count(&self, cell: Cell) -> usize {
        self.cells
            .iter()
            .map(|line| line.iter().filter(|c| **c == cell).count())
            .sum()
    }

    fn next(&self, rule: fn(&Grid, usize, usize) -> Option<Cell>) -> Option<Grid> {
        let mut cells = vec![vec![Cell::Floor; self.width]; self.height];
        let mut modified = false;

        for row in 0..self.height {
            for col in 0..self.width {
                if let Some(cell) = rule(self, row, col) {
                    cells[row][col] = cell;
                    modified = true;
                } else {
                    cells[row][col] = self.cells[row][col];
                }
            }
        }

        if !modified {
            return None;
        }
        Some(Grid {
            cells,
            width: self.width,
            height: self.height,
        })
    }

    fn get(&self, row: isize, col: isize) -> Option<Cell> {
        if row < 0 || row >= self.height as isize {
            None
        } else if col < 0 || col >= self.width as isize {
            None
        } else {
            Some(self.cells[row as usize][col as usize])
        }
    }

    fn neighbors(&self, row: usize, col: usize) -> usize {
        let mut count = 0;

        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                match self.get(row as isize + i, col as isize + j) {
                    Some(Cell::OccupiedSeat) => count += 1,
                    _ => (),
                }
            }
        }

        count
    }

    fn visible(&self, row: usize, col: usize) -> usize {
        let mut count = 0;

        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                for k in 1.. {
                    match self.get(row as isize + i * k, col as isize + j * k) {
                        None => break,
                        Some(Cell::Floor) => continue,
                        Some(Cell::EmptySeat) => break,
                        Some(Cell::OccupiedSeat) => {
                            count += 1;
                            break;
                        }
                    }
                }
            }
        }

        count
    }
}
