use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{all_consuming, map};
use nom::character::complete::{digit1, multispace0};
use nom::error::Error;
use nom::multi::{many0, separated_list0};
use nom::sequence::{delimited, preceded, tuple};

type Item = u64;

fn index(input: &str) -> IResult<&str, usize> {
    map(digit1, |s: &str| s.parse::<usize>().unwrap())(input)
}

fn item(input: &str) -> IResult<&str, Item> {
    map(digit1, |s: &str| s.parse::<Item>().unwrap())(input)
}

#[derive(Clone, Debug)]
enum Operator {
    Add,
    Mul,
}

impl Operator {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(tag("+"), |_| Operator::Add),
            map(tag("*"), |_| Operator::Mul),
        ))(input)
    }
}

#[derive(Clone, Debug)]
enum Operand {
    Old,
    Value(Item),
}

impl Operand {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(tag("old"), |_| Operand::Old),
            map(item, |v| Operand::Value(v)),
        ))(input)
    }

    fn eval(&self, old: Item) -> Item {
        match self {
            Operand::Old => old,
            Operand::Value(v) => *v,
        }
    }
}

#[derive(Clone, Debug)]
struct Operation {
    lhs: Operand,
    operator: Operator,
    rhs: Operand,
}

impl Operation {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((Operand::parse, multispace0, Operator::parse, multispace0, Operand::parse)),
            |(lhs, _, operator, _, rhs)| Self { lhs, operator, rhs })
            (input)
    }

    fn eval(&self, old: Item) -> Item {
        let lhs = self.lhs.eval(old);
        let rhs = self.rhs.eval(old);
        match self.operator {
            Operator::Add => lhs + rhs,
            Operator::Mul => lhs * rhs,
        }
    }
}

#[derive(Clone, Debug)]
struct Test {
    divisible_by: Item,
}

impl Test {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            preceded(tuple((tag("divisible by"), multispace0)), item),
            |divisible_by| Test { divisible_by })
            (input)
    }

    fn eval(&self, item: Item) -> bool {
        item % self.divisible_by == 0
    }
}

#[derive(Clone, Debug)]
struct Action {
    throw_to_monkey: usize,
}

impl Action {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            preceded(tuple((tag("throw to monkey"), multispace0)), index),
            |throw_to_monkey| Action { throw_to_monkey })
            (input)
    }

    fn apply(&self, item: Item, monkeys: &mut [Monkey]) {
        monkeys[self.throw_to_monkey].items.push(item);
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    idx: usize,
    items: Vec<Item>,
    operation: Operation,
    test: Test,
    true_action: Action,
    false_action: Action,
    activity: usize,
}

impl Monkey {
    fn parse(input: &str) -> IResult<&str, Self> {
        let s0 = multispace0;
        map(
            tuple((
                delimited(
                    tuple((s0, tag("Monkey"), s0)),
                    index,
                    tuple((s0, tag(":"), s0))),
                delimited(
                    tuple((tag("Starting items:"), s0)),
                    separated_list0(tuple((s0, tag(","), s0)), item),
                    s0),
                delimited(
                    tuple((tag("Operation:"), s0, tag("new"), s0, tag("="), s0)),
                    Operation::parse,
                    s0),
                delimited(
                    tuple((s0, tag("Test:"), s0)),
                    Test::parse,
                    s0),
                delimited(
                    tuple((s0, tag("If true:"), s0)),
                    Action::parse,
                    s0),
                delimited(
                    tuple((s0, tag("If false:"), s0)),
                    Action::parse,
                    s0),
            )),
            |(idx, items, operation, test, true_action, false_action)| {
                assert!(true_action.throw_to_monkey != idx);
                assert!(false_action.throw_to_monkey != idx);
                Self { idx, items, operation, test, true_action, false_action, activity: 0 }
            })
            (input)
    }

    fn parse_all(input: &str) -> Result<Vec<Self>, nom::Err<Error<&str>>> {
        Ok(all_consuming(many0(Self::parse))(input)?.1)
    }

    fn inspect(&mut self, item: Item) -> Item {
        self.activity += 1;
        self.operation.eval(item)
    }

    fn action(&self, item: Item) -> &Action {
        if self.test.eval(item) {
            &self.true_action
        } else {
            &self.false_action
        }
    }
}

fn monkey_business(mut monkeys: Vec<Monkey>, relief: impl Fn(Item) -> Item, rounds: usize) -> usize {
    let product = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible_by)
        .fold(1, |a, b| a * b);
    for _round in 0..rounds {
        for i in 0..monkeys.len() {
            let items = std::mem::take(&mut monkeys[i].items);
            for item in items {
                let item = relief(monkeys[i].inspect(item)) % product;
                let action = monkeys[i].action(item).clone();
                action.apply(item, &mut monkeys);
            }
        }
    }
    monkeys.sort_by_key(|monkey| monkey.activity);
    monkeys.iter().rev().take(2).map(|monkey| monkey.activity).reduce(|a, b| a * b).unwrap()
}

fn run(input: &str) -> (usize, usize) {
    let monkeys = Monkey::parse_all(input).unwrap();
    assert!(monkeys.iter().enumerate().all(|(i, monkey)| monkey.idx == i));
    (
        monkey_business(monkeys.clone(), |item| item / 3, 20),
        monkey_business(monkeys, |item| item, 10000),
    )
}

#[test]
fn examples() {
    assert_eq!(run(&aoc::example!(0)), (10605, 2713310158));
}

#[test]
fn input() {
    assert_eq!(run(&aoc::input!()), (117624, 16792940265));
}

aoc::main!(run);
