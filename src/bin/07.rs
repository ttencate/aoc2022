use std::path::{Path, PathBuf};
use std::collections::BTreeMap;

fn run(input: &str) -> (u64, u64) {
    let mut file_sizes = BTreeMap::new();
    let mut pwd = PathBuf::from("/");
    for line in input.lines() {
        let mut parts = line.split(" ");
        match (parts.next().unwrap(), parts.next().unwrap()) {
            ("$", "cd") => {
                match parts.next().unwrap() {
                    ".." => { pwd.pop(); },
                    "/" => { pwd = PathBuf::from("/"); },
                    subdir => { pwd.push(subdir); },
                }
                debug_assert!(parts.next().is_none());
            },
            ("$", "ls") => {
            },
            ("dir", _) => {
            },
            (size, file_name) => {
                file_sizes.insert(pwd.join(file_name), size.parse::<u64>().unwrap());
            },
        }
        debug_assert!(parts.next().is_none());
    }

    let mut dir_sizes = BTreeMap::new();
    for (file_name, size) in file_sizes {
        for ancestor in file_name.parent().unwrap().ancestors() {
            *dir_sizes.entry(ancestor.to_owned()).or_default() += size;
        }
    }

    let part1 = dir_sizes
        .values()
        .filter(|&&size| size <= 100000)
        .sum();

    let free_space = 70000000 - dir_sizes[Path::new("/")];
    let min_size = 30000000 - free_space;
    let part2 = *dir_sizes
        .values()
        .filter(|&&size| size >= min_size)
        .min()
        .unwrap();

    (part1, part2)
}

#[test]
fn examples() {
    assert_eq!(run(&aoc::example!(1)), (95437, 24933642));
}

#[test]
fn input() {
    assert_eq!(run(&aoc::input!()), (1723892, 8474158));
}

aoc::main!(run);
