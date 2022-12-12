use std::collections::VecDeque;

const VISITED: u8 = b'#';
const MIN: u8 = b'a';
const MAX: u8 = b'z';
const START: u8 = b'S';
const END: u8 = b'E';

fn run(input: &str) -> (usize, usize) {
    let mut grid = input.as_bytes().to_vec();
    let col_stride = grid.iter().position(|&h| h == b'\n').unwrap() + 1;
    let start = grid.iter().position(|&h| h == START).unwrap();
    let end = grid.iter().position(|&h| h == END).unwrap();
    grid[start] = MIN;
    grid[end] = MAX;

    let mut part2 = usize::MAX;
    let mut queue = VecDeque::new();
    queue.push_back((end, 0));
    while let Some((curr, steps)) = queue.pop_front() {
        let height = grid[curr];
        if height == VISITED {
            continue;
        }
        grid[curr] = VISITED;

        if height == MIN {
            part2 = part2.min(steps);
        }
        if curr == start {
            debug_assert!(part2 <= steps);
            return (steps, part2);
        }

        for step in [1, 0_usize.wrapping_sub(1), col_stride, 0_usize.wrapping_sub(col_stride)] {
            let neigh = curr.wrapping_add(step);
            if let Some(neigh_height) = grid.get(neigh).copied() {
                if (MIN..=MAX).contains(&neigh_height) { // Checks for b'\n' and VISITED
                    if height <= neigh_height + 1 {
                        queue.push_back((neigh, steps + 1));
                    }
                }
            }
        }
    }
    panic!();
}

#[test]
fn examples() {
    assert_eq!(run(&aoc::example!(0)), (31, 29));
}

#[test]
fn input() {
    assert_eq!(run(&aoc::input!()), (528, 522));
}

aoc::main!(run);
