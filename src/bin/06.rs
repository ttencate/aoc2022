#[derive(Default)]
struct Counts {
    counts: [usize; 26],
    unique: usize,
}

impl Counts {
    fn inc(&mut self, c: u8) {
        let idx = Self::idx(c);
        if self.counts[idx] == 0 {
            self.unique += 1;
        }
        self.counts[idx] += 1;
    }

    fn dec(&mut self, c: u8) {
        let idx = Self::idx(c);
        debug_assert!(self.counts[idx] > 0);
        self.counts[idx] -= 1;
        if self.counts[idx] == 0 {
            self.unique -= 1;
        }
    }

    fn idx(c: u8) -> usize {
        (c - b'a') as usize
    }
}

fn marker(input: &[u8], marker_len: usize) -> usize {
    let mut counts = Counts::default();
    for i in 0..marker_len {
        counts.inc(input[i]);
    }
    for i in marker_len..input.len() {
        if counts.unique == marker_len {
            return i;
        }
        counts.inc(input[i]);
        counts.dec(input[i - marker_len]);
    }
    if counts.unique == marker_len {
        return input.len();
    }
    panic!();
}

fn run(input: &str) -> (usize, usize) {
    let input = input.as_bytes();
    (marker(input, 4), marker(input, 14))
}

#[test]
fn examples() {
    assert_eq!(run(&aoc::example!(0)), (7, 19));
    assert_eq!(run("bvwbjplbgvbhsrlpgdmjqwftvncz"), (5, 23));
    assert_eq!(run("nppdvjthqldpwncqszvftbrmjlhg"), (6, 23));
    assert_eq!(run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), (10, 29));
    assert_eq!(run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), (11, 26));
}

#[test]
fn input() {
    assert_eq!(run(&aoc::input!()), (1042, 2980));
}

aoc::main!(run);
