use aoc2022::crt::*;

fn run(input: &str) -> (i64, String) {
    let mut instrs = input
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap());
    let mut crt = Crt::new();
    let mut signal_strength = 0;
    loop {
        let x = crt.x();
        if !crt.tick(&mut instrs) {
            break;
        }
        if (crt.cycle() - 20) % 40 == 0 {
            signal_strength += crt.cycle() * x;
        }
    }
    (signal_strength, crt.screen())
}

#[test]
fn examples() {
    assert_eq!(run(&aoc::example!(1)), (13140, aoc::example!(4)));
}

#[test]
fn input() {
    assert_eq!(run(&aoc::input!()), (17840, "####..##..#.....##..#..#.#....###...##..\n#....#..#.#....#..#.#..#.#....#..#.#..#.\n###..#..#.#....#....#..#.#....#..#.#....\n#....####.#....#.##.#..#.#....###..#.##.\n#....#..#.#....#..#.#..#.#....#....#..#.\n####.#..#.####..###..##..####.#.....###.\n".to_owned()));
}

aoc::main!(run);
