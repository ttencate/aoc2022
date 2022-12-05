fn run(input: &str) -> (String, String) {
    let mut lines = input.lines();
    let mut num_stacks = 0;
    let mut stacks = Vec::new();
    while let Some(line) = lines.next() {
        let line = line.as_bytes();
        num_stacks = num_stacks.max((line.len() + 1) / 4);
        if stacks.len() < num_stacks {
            stacks.resize(num_stacks, Vec::new());
        }
        if line[1] == b'1' {
            break;
        }
        for i in 0..num_stacks {
            let crate_ = line[1 + 4*i];
            if crate_ != b' ' {
                stacks[i].push(crate_);
            }
        }
    }
    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    debug_assert_eq!(lines.next().unwrap(), "");

    let mut part1_stacks = stacks.clone();
    let mut part2_stacks = stacks;
    for line in lines {
        let mut parts = line.split(' ');
        debug_assert_eq!(parts.next().unwrap(), "move");
        let count = parts.next().unwrap().parse::<usize>().unwrap();
        debug_assert_eq!(parts.next().unwrap(), "from");
        let from = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        debug_assert_eq!(parts.next().unwrap(), "to");
        let to = parts.next().unwrap().parse::<usize>().unwrap() - 1;

        for _ in 0..count {
            let crate_ = part1_stacks[from].pop().unwrap();
            part1_stacks[to].push(crate_);
        }

        let idx = part2_stacks[from].len() - count;
        let crates = part2_stacks[from][idx..].to_vec();
        part2_stacks[to].extend(crates);
        part2_stacks[from].truncate(idx);
    }

    let part1 = part1_stacks
        .into_iter()
        .filter_map(|stack| stack.last().copied())
        .collect::<Vec<u8>>();
    let part2 = part2_stacks
        .into_iter()
        .filter_map(|stack| stack.last().copied())
        .collect::<Vec<u8>>();
    unsafe { (String::from_utf8_unchecked(part1), String::from_utf8_unchecked(part2)) }
}

#[test]
fn examples() {
    assert_eq!(run(&aoc::example!(0)), ("CMZ".to_owned(), "MCD".to_owned()));
}

#[test]
fn input() {
    assert_eq!(run(&aoc::input!()), ("VPCDMSLWJ".to_owned(), "TPWCGNCCG".to_owned()));
}

aoc::main!(run);
