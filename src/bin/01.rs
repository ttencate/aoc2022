fn run(input: &str) -> (u64, u64) {
    let top3 = input
        .split("\n\n")
        .map(|elf| {
            elf
                .lines()
                .map(|line| line.parse::<u64>().unwrap())
                .sum()
        })
        .fold([0; 3], |mut top3, calories| {
            if calories >= top3[0] {
                top3[0] = calories;
                top3.sort();
            }
            top3
        });
    (*top3.last().unwrap(), top3.iter().sum())
}

#[test]
fn examples() {
    assert_eq!(run(&aoc::example!(0)), (24000, 45000));
}

#[test]
fn input() {
    assert_eq!(run(&aoc::input!()), (66616, 199172));
}

aoc::main!(run);
