use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map, value, map_res},
    multi::separated_list0,
    sequence::{delimited, tuple, preceded},
    IResult,
};

#[derive(Clone, Copy, Debug)]
enum Expression {
    Old,
    Value(i64),
}

impl Expression {
    fn eval(&self, old: i64) -> i64 {
        match self {
            Expression::Old => old,
            Expression::Value(n) => *n,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add(Expression, Expression),
    Multiply(Expression, Expression),
}

impl Operation {
    fn apply(&self, old: i64) -> i64 {
        match self {
            Operation::Add(a, b) => a.eval(old).checked_add(b.eval(old)).unwrap(),
            Operation::Multiply(a, b) => a.eval(old).checked_mul(b.eval(old)).unwrap(),
        }
    }
}

struct Test {
    divisor: i64,
    pass_dest: usize,
    fail_dest: usize,
}

impl Test {
    fn get_destination(&self, value: i64) -> usize {
        if value % self.divisor == 0 {
            self.pass_dest
        } else {
            self.fail_dest
        }
    }
}

struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test: Test,
}

fn parse_expression(input: &str) -> IResult<&str, Expression> {
    alt((
        value(Expression::Old, tag("old")),
        map(nom::character::complete::i64, |n| Expression::Value(n)),
    ))(input)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let add = map(
        tuple((parse_expression, tag(" + "), parse_expression)),
        |(left, _, right)| Operation::Add(left, right),
    );

    let multiply = map(
        tuple((parse_expression, tag(" * "), parse_expression)),
        |(left, _, right)| Operation::Multiply(left, right),
    );

    alt((add, multiply))(input)
}

fn parse_test(input: &str) -> IResult<&str, Test> {
    let divisor = delimited(
        tag("  Test: divisible by "),
        nom::character::complete::i64,
        newline,
    );

    let pass_dest = map_res(delimited(tag("    If true: throw to monkey "), nom::character::complete::u64, newline), usize::try_from);
    let fail_dest = map_res(delimited(tag("    If false: throw to monkey "), nom::character::complete::u64, newline), usize::try_from);

    map(tuple((divisor, pass_dest, fail_dest)), |(divisor, pass_dest, fail_dest)| Test {
        divisor,
        pass_dest,
        fail_dest
    })(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let start = delimited(tag("Monkey "), digit1, tag(":\n"));

    let items = delimited(
        tag("  Starting items: "),
        separated_list0(tag(", "), nom::character::complete::i64),
        newline,
    );

    let operation = delimited(tag("  Operation: new = "), parse_operation, newline);

    map(preceded(start, tuple((items, operation, parse_test))), |(items, operation, test)| Monkey {
        items,
        operation,
        test
    })(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list0(newline, parse_monkey)(input)
}

pub struct Solution;
impl crate::Solution for Solution {
    const DAY: usize = 11;
    type O1 = u64;
    type O2 = u64;

    fn part_one(input: &str) -> Self::O1 {
        let (_, mut monkeys) = parse(input).unwrap();
        let mut inspection_count = vec![0; monkeys.len()];

        for _ in 0..20 {
            for i in 0..monkeys.len() {
                let items = monkeys[i].items.split_off(0);
                inspection_count[i] += items.len() as u64;
                for item in items {
                    let new_item = monkeys[i].operation.apply(item) / 3;
                    let destination = monkeys[i].test.get_destination(new_item);
                    monkeys[destination].items.push(new_item);
                }
            }
        }

        inspection_count.iter().sorted().rev().take(2).product()
    }

    fn part_two(input: &str) -> Self::O2 {
        let (_, mut monkeys) = parse(input).unwrap();
        let mut inspection_count = vec![0; monkeys.len()];

        let modulus: i64 = monkeys.iter().map(|m| m.test.divisor).product();

        for _ in 0..10000 {
            for i in 0..monkeys.len() {
                let items = monkeys[i].items.split_off(0);
                inspection_count[i] += items.len() as u64;
                for item in items {
                    let new_item = monkeys[i].operation.apply(item) % modulus;
                    let destination = monkeys[i].test.get_destination(new_item);
                    monkeys[destination].items.push(new_item);
                }
            }
        }

        inspection_count.iter().sorted().rev().take(2).product()
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    #[test]
    fn parse_operation() {
        let input = indoc! {"
            old * 19
        "};

        let (_, operation) = super::parse_operation(input).unwrap();
        assert!(matches!(operation, super::Operation::Multiply(super::Expression::Old, super::Expression::Value(19))));
    }

    #[test]
    fn parse_test() {
        let input = indoc! {"
              Test: divisible by 23
                If true: throw to monkey 2
                If false: throw to monkey 3
            x
        "};

        let (_, test) = super::parse_test(input).unwrap();
        assert_eq!(test.divisor, 23);
        assert_eq!(test.pass_dest, 2);
        assert_eq!(test.fail_dest, 3);
    }

    #[test]
    fn parse_monkey() {
        let input = indoc! {"
            Monkey 0:
              Starting items: 79, 98
              Operation: new = old * 19
              Test: divisible by 23
                If true: throw to monkey 2
                If false: throw to monkey 3
        "};

        let (_, monkey) = super::parse_monkey(input).unwrap();

        assert_eq!(monkey.items, [79, 98]);
        assert!(matches!(monkey.operation, super::Operation::Multiply(super::Expression::Old, super::Expression::Value(19))));
        assert_eq!(monkey.test.divisor, 23);
        assert_eq!(monkey.test.pass_dest, 2);
        assert_eq!(monkey.test.fail_dest, 3);
    }
}