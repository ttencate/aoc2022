struct Grid<'a> {
    num_rows: usize,
    num_cols: usize,
    rows: Vec<&'a [u8]>,
}

impl<'a> Grid<'a> {
    fn new(rows: Vec<&'a [u8]>) -> Self {
        let num_rows = rows.len();
        let num_cols = rows.iter().map(|row| row.len()).max().unwrap();
        Self {
            num_rows,
            num_cols,
            rows,
        }
    }

    fn get(&self, coord: Coord) -> u8 {
        self.rows[coord.row as usize]
            .get(coord.col as usize)
            .copied()
            .unwrap_or(b' ')
    }

    fn wrap(&self, coord: Coord) -> Coord {
        Coord {
            row: coord.row.rem_euclid(self.num_rows as i64),
            col: coord.col.rem_euclid(self.num_cols as i64),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Coord {
    row: i64,
    col: i64,
}

impl std::ops::Add for Coord {
    type Output = Coord;
    fn add(self, rhs: Coord) -> Coord {
        Coord {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

fn run(input: &str) -> usize {
    let mut lines = input
        .as_bytes()
        .split(|&c| c == b'\n')
        .collect::<Vec<&[u8]>>();
    let instructions = loop {
        let line = lines.pop().unwrap();
        if !line.is_empty() {
            break line;
        }
    };
    let grid = Grid::new(lines);
    let mut pos = Coord {
        row: 0,
        col: grid.rows[0].into_iter().position(|&c| c != b' ').unwrap() as i64,
    };
    let mut facing = 0;

    let mut idx = 0;
    while idx < instructions.len() {
        let c = instructions[idx];
        if c.is_ascii_digit() {
            let start = idx;
            while idx < instructions.len() && instructions[idx].is_ascii_digit() {
                idx += 1;
            }
            let steps = std::str::from_utf8(&instructions[start..idx]).unwrap().parse::<usize>().unwrap();
            for _ in 0..steps {
                let step = match facing {
                    0 => Coord { row: 0, col: 1 },
                    1 => Coord { row: 1, col: 0 },
                    2 => Coord { row: 0, col: -1 },
                    3 => Coord { row: -1, col: 0 },
                    _ => unreachable!(),
                };
                let mut next = grid.wrap(pos + step);
                while grid.get(next) == b' ' {
                    next = grid.wrap(next + step);
                }
                if grid.get(next) == b'#' {
                    break;
                }
                assert!(grid.get(next) == b'.');
                pos = next;
            }
        } else if c == b'L' {
            facing = (facing + 3) % 4;
            idx += 1;
        } else if c == b'R' {
            facing = (facing + 1) % 4;
            idx += 1;
        } else if !c.is_ascii_whitespace() {
            panic!();
        }
    }
    
    1000 * (pos.row + 1) as usize + 4 * (pos.col + 1) as usize + facing
}

#[test]
fn examples() {
    assert_eq!(run(&aoc::example!(0)), 6032);
}

#[test]
fn input() {
    assert_eq!(run(&aoc::input!()), 0);
}

aoc::main!(run);
