use itertools::Itertools;

struct Set(u128);

impl Set {
    fn priority(&self) -> u64 {
        let c = self.0.trailing_zeros() as u8;
        debug_assert!(self.0 == 1 << c);
        match c {
            b'a'..=b'z' => 1 + c - b'a',
            b'A'..=b'Z' => 27 + c - b'A',
            _ => panic!(),
        }.into()
    }
}

impl From<&[u8]> for Set {
    fn from(chars: &[u8]) -> Self {
        let mut bits = 0;
        for &c in chars {
            debug_assert!(c < u128::BITS as u8);
            bits |= 1 << c;
        }
        Set(bits)
    }
}

impl std::ops::BitAnd for Set {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}

fn run(input: &str) -> (u64, u64) {
    let part1 = input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let (left, right) = bytes.split_at(bytes.len() / 2);
            (Set::from(left) & Set::from(right)).priority()
        })
        .sum();
    let part2 = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|lines| {
            lines
                .map(|line| Set::from(line.as_bytes()))
                .reduce(|a, b| a & b)
                .unwrap()
                .priority()
        })
        .sum();
    (part1, part2)
}

#[test]
fn examples() {
    assert_eq!(run(&aoc::example!(0)), (157, 70));
}

#[test]
fn input() {
    assert_eq!(run(&aoc::input!()), (7850, 2581));
}

aoc::main!(run);
