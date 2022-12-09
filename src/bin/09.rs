use std::collections::HashSet;

use glam::IVec2;
use itertools::Itertools;

struct Rope<const N: usize> {
    knots: [IVec2; N],
    visited: HashSet<IVec2>,
}

impl<const N: usize> Rope<N> {
    fn new() -> Self {
        Self {
            knots: [IVec2::ZERO; N],
            visited: std::iter::once(IVec2::ZERO).collect(),
        }
    }

    fn step_head(&mut self, head_step: IVec2) {
        self.knots[0] += head_step;
        for i in 1..N {
            let diff = self.knots[i] - self.knots[i - 1];
            if diff.abs().max_element() > 1 {
                self.knots[i] -= diff.clamp(IVec2::NEG_ONE, IVec2::ONE);
            }
        }
        self.visited.insert(self.knots[N - 1]);
    }

    fn num_visited(&self) -> usize {
        self.visited.len()
    }
}

fn run(input: &str) -> (usize, usize) {
    let mut rope1 = Rope::<2>::new();
    let mut rope2 = Rope::<10>::new();
    for line in input.lines() {
        let (step_char, num_steps) = line.split(' ').collect_tuple().unwrap();
        let step = match step_char.as_bytes()[0] {
            b'L' => IVec2::new(-1, 0),
            b'R' => IVec2::new(1, 0),
            b'U' => IVec2::new(0, -1),
            b'D' => IVec2::new(0, 1),
            _ => panic!(),
        };
        for _ in 0..num_steps.parse::<usize>().unwrap() {
            rope1.step_head(step);
            rope2.step_head(step);
        }
    }
    (rope1.num_visited(), rope2.num_visited())
}

#[test]
fn examples() {
    assert_eq!(run(&aoc::example!(3)), (13, 1));
    assert_eq!(run(&aoc::example!(7)).1, 36);
}

#[test]
fn input() {
    assert_eq!(run(&aoc::input!()), (6269, 2557));
}

aoc::main!(run);
