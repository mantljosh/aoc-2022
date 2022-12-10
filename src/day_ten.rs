use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::{iterator, map, opt, value},
    sequence::{preceded, terminated},
    IResult,
};

#[derive(Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let noop = value(Instruction::Noop, tag("noop"));
    let add = map(preceded(tag("addx "), nom::character::complete::i32), |x| {
        Instruction::Addx(x)
    });
    terminated(alt((noop, add)), opt(newline))(input)
}

fn register_values(instructions: impl Iterator<Item = Instruction>) -> impl Iterator<Item = i32> {
    instructions
        .flat_map(|instruction| match instruction {
            Instruction::Noop => vec![0],
            Instruction::Addx(x) => vec![0, x],
        })
        .scan(1, |x, instruction| {
            let current = *x;
            *x += instruction;
            Some(current)
        })
}

pub struct Solution;
impl crate::Solution for Solution {
    const DAY: usize = 10;
    type O1 = i32;
    type O2 = i32;

    fn part_one(input: &str) -> Self::O1 {
        let mut instructions = iterator(input, parse_instruction);
        let signals = register_values(&mut instructions)
            .enumerate()
            .map(|(i, x)| (i as i32 + 1) * x);

        signals.skip(19).step_by(40).take(6).sum()
    }

    fn part_two(input: &str) -> Self::O2 {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use super::Instruction::*;

    #[test]
    fn register_values() {
        let input = [Noop, Addx(3), Addx(-5), Noop];
        let values = super::register_values(input.into_iter()).collect_vec();
        assert_eq!(values, [1, 1, 1, 4, 4, -1]);
    }
}
