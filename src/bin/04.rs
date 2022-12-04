use std::cmp::{max, min};
use std::ops::RangeInclusive;

use itertools::Itertools;

fn parse_range(s: &str) -> RangeInclusive<u64> {
    let (start, end) = s
        .split('-')
        .map(|x| x.parse().unwrap())
        .collect_tuple()
        .unwrap();
    start..=end
}

fn contains(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> bool {
    a.contains(&b.start()) && a.contains(&b.end())
}

fn overlaps(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> bool {
    min(a.end(), b.end()) >= max(a.start(), b.start())
}

fn run(input: &str) -> (u64, u64) {
    input
        .lines()
        .map(|line| {
            let (a, b) = line
                .split(',')
                .map(parse_range)
                .collect_tuple()
                .unwrap();
            let contains = (contains(&a, &b) || contains(&b, &a)) as u64;
            let overlaps = overlaps(&a, &b) as u64;
            (contains, overlaps)
        })
        .fold((0, 0), |(a1, o1), (a2, o2)| (a1 + a2, o1 + o2))
}

#[test]
fn examples() {
    assert_eq!(run(&aoc::example!(0)), (2, 4));
}

#[test]
fn input() {
    assert_eq!(run(&aoc::input!()), (453, 919));
}

aoc::main!(run);
