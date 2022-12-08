type Tree = u8;

fn mark_visible_trees(grid: &[Tree], start: usize, stop: usize, stride: i64, visible: &mut [bool]) {
    visible[start] = true;
    let mut highest = grid[start];
    let mut i: usize = (start as i64 + stride).try_into().unwrap();
    while i != stop {
        let curr = grid[i];
        if curr > highest {
            highest = curr;
            visible[i] = true;
        }
        i = (i as i64 + stride).try_into().unwrap();
    }
}

fn scenic_score(grid: &[Tree], start: usize, stop: usize, stride: i64) -> usize {
    if start == stop {
        return 0;
    }
    let threshold = grid[start];
    let mut count = 0;
    let mut i: usize = (start as i64 + stride).try_into().unwrap();
    loop {
        count += 1;
        if i == stop || grid[i] >= threshold {
            return count;
        }
        i = (i as i64 + stride).try_into().unwrap();
    }
}

fn run(input: &str) -> (usize, usize) {
    let grid = input.as_bytes();
    let cols = grid.split(|&c| c == b'\n').next().unwrap().len();
    let rows = grid.len() / cols - 1;
    let row_stride = cols + 1;
    debug_assert_eq!(cols * (rows + 1), grid.len());

    let left = -1_i64;
    let right = 1_i64;
    let up = -(row_stride as i64);
    let down = row_stride as i64;

    let mut visible = vec![false; grid.len()];
    for row in 0..rows {
        let start = row * row_stride;
        let stop = start + cols - 1;
        mark_visible_trees(grid, start, stop, right, &mut visible);
        mark_visible_trees(grid, stop, start, left, &mut visible);
    }
    for col in 0..cols {
        let start = col;
        let stop = (rows - 1) * row_stride + col;
        mark_visible_trees(grid, start, stop, down, &mut visible);
        mark_visible_trees(grid, stop, start, up, &mut visible);
    }
    let part1 = visible.into_iter().filter(|&v| v).count();

    let mut scenic_scores = vec![0; grid.len()];
    for row in 0..rows {
        for col in 0..cols {
            let start = row * row_stride + col;
            scenic_scores[start] =
                scenic_score(grid, start, col, up) *
                scenic_score(grid, start, (rows - 1) * row_stride + col, down) *
                scenic_score(grid, start, row * row_stride, left) *
                scenic_score(grid, start, row * row_stride + cols - 1, right);
        }
    }
    let part2 = scenic_scores.into_iter().max().unwrap();

    (part1, part2)
}

#[test]
fn examples() {
    assert_eq!(run(&aoc::example!(0)), (21, 8));
}

#[test]
fn input() {
    assert_eq!(run(&aoc::input!()), (1782, 474606));
}

aoc::main!(run);
