#[derive(Clone, Copy, PartialEq, Eq)]
enum Shape { Rock, Paper, Scissors }
use Shape::*;

impl Shape {
    fn score(&self) -> u64 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn beats(&self) -> Shape {
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
    
    fn beaten_by(&self) -> Shape {
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }
}

impl From<u8> for Shape {
    fn from(c: u8) -> Self {
        match c {
            b'A' | b'X' => Rock,
            b'B' | b'Y' => Paper,
            b'C' | b'Z' => Scissors,
            _ => panic!("unknown shape character {:?}", c),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Outcome { Win, Draw, Lose }
use Outcome::*;

impl Outcome {
    fn score(&self) -> u64 {
        match self {
            Win => 6,
            Draw => 3,
            Lose => 0,
        }
    }
}

impl From<u8> for Outcome {
    fn from(c: u8) -> Self {
        match c {
            b'X' => Lose,
            b'Y' => Draw,
            b'Z' => Win,
            _ => panic!("unknown outcome character {:?}", c),
        }
    }
}

fn part1(abc: u8, xyz: u8) -> u64 {
    let opponent = Shape::from(abc);
    let me = Shape::from(xyz);
    let outcome = if me.beats() == opponent {
        Win
    } else if me.beaten_by() == opponent {
        Lose
    } else {
        debug_assert!(me == opponent);
        Draw
    };
    me.score() + outcome.score()
}

fn part2(abc: u8, xyz: u8) -> u64 {
    let opponent = Shape::from(abc);
    let outcome = Outcome::from(xyz);
    let me = match outcome {
        Win => opponent.beaten_by(),
        Draw => opponent,
        Lose => opponent.beats(),
    };
    me.score() + outcome.score()
}

fn run(input: &str) -> (u64, u64) {
    input
        .lines()
        .map(|line| {
            let line = line.as_bytes();
            let abc = line[0];
            let xyz = line[2];
            (part1(abc, xyz), part2(abc, xyz))
        })
        .reduce(|(a1, a2), (b1, b2)| (a1 + b1, a2 + b2))
        .unwrap_or((0, 0))
}

#[test]
fn examples() {
    assert_eq!(run(&aoc::example!(0)), (15, 12));
}

#[test]
fn input() {
    assert_eq!(run(&aoc::input!()), (15632, 14416));
}

aoc::main!(run);
