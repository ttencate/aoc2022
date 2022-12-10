use std::fmt::Write;
use std::str::FromStr;

const ROWS: usize = 6;
const COLS: usize = 40;

pub struct Crt {
    x: i64,
    cycle: i64,
    instr: Option<(i64, Instruction)>,
    screen: [[u8; COLS]; ROWS],
}

impl Crt {
    pub fn new() -> Self {
        Self {
            x: 1,
            cycle: 0,
            instr: None,
            screen: [[b'.'; COLS]; ROWS],
        }
    }

    pub fn need_instr(&self) -> bool {
        self.instr.is_none()
    }

    pub fn tick(&mut self, instrs: &mut impl Iterator<Item = Instruction>) -> bool {
        self.draw_pixel();
        if self.instr.is_none() {
            if let Some(instr) = instrs.next() {
                self.instr = Some((self.cycle + instr.duration(), instr));
            } else {
                return false;
            }
        }
        self.cycle += 1;
        if self.cycle >= self.instr.as_ref().unwrap().0 {
            let instr = self.instr.take().unwrap().1;
            self.complete(instr);
        }
        true
    }

    fn draw_pixel(&mut self) {
        let i = self.cycle as usize;
        let row = i / COLS;
        let col = i % COLS;
        if row >= ROWS {
            // Nowhere does it say how it would wrap around.
            return;
        }
        self.screen[row][col] =
            if (self.x - col as i64).abs() <= 1 { b'#' } else { b'.' };
    }

    fn complete(&mut self, instr: Instruction) {
        use Instruction::*;
        match instr {
            Noop => {},
            Addx(a) => { self.x += a; },
        }
    }

    pub fn cycle(&self) -> i64 {
        self.cycle
    }

    pub fn x(&self) -> i64 {
        self.x
    }

    pub fn screen(&self) -> String {
        let mut out = String::new();
        for row in &self.screen {
            write!(out, "{}\n", std::str::from_utf8(row).unwrap()).unwrap();
        }
        out
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Noop,
    Addx(i64),
}

impl Instruction {
    fn duration(&self) -> i64 {
        use Instruction::*;
        match self {
            Noop => 1,
            Addx(_) => 2,
        }
    }
}

#[derive(Debug)]
pub struct ParseError(String);

macro_rules! parse_error {
    ($($arg:tt)*) => {
        ParseError(format!($($arg)*))
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Instruction::*;
        let mut parts = s.split(' ');
        let mnemonic = parts.next().ok_or_else(|| parse_error!("empty line"))?;
        Ok(match mnemonic {
            "noop" => Noop,
            "addx" => Addx(parse_i64(parts.next())?),
            _ => return Err(parse_error!("unknown instruction {:?}", mnemonic)),
        })
    }
}

fn parse_i64(s: Option<&str>) -> Result<i64, ParseError> {
    match s {
        Some(s) => s.parse::<i64>()
            .map_err(|err| parse_error!("invalid integer {:?}: {}", s, err)),
        None => Err(parse_error!("expected integer, found end of line")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration() {
        let mut instrs = "noop\naddx 3\naddx -5"
            .lines()
            .map(|line| line.parse::<Instruction>().unwrap());
        let mut crt = Crt::new();
        assert_eq!(crt.cycle(), 0);
        assert_eq!(crt.x(), 1);
        crt.tick(&mut instrs);
        assert_eq!(crt.cycle(), 1);
        assert_eq!(crt.x(), 1);
        crt.tick(&mut instrs);
        assert_eq!(crt.cycle(), 2);
        assert_eq!(crt.x(), 1);
        crt.tick(&mut instrs);
        assert_eq!(crt.cycle(), 3);
        assert_eq!(crt.x(), 4);
        crt.tick(&mut instrs);
        assert_eq!(crt.cycle(), 4);
        assert_eq!(crt.x(), 4);
        crt.tick(&mut instrs);
        assert_eq!(crt.cycle(), 5);
        assert_eq!(crt.x(), -1);
    }
}
