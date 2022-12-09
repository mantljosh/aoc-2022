use std::{
    collections::HashSet,
    iter::{once, repeat},
};

use itertools::Itertools;
use nom::branch::alt;

use nom::character::complete::{char, newline};

use nom::{
    combinator::{iterator, map_res, opt, value},
    sequence::{separated_pair, terminated},
    IResult,
};

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_instruction(input: &str) -> IResult<&str, (Direction, usize)> {
    let direction = alt((
        value(Direction::Up, char('U')),
        value(Direction::Down, char('D')),
        value(Direction::Left, char('L')),
        value(Direction::Right, char('R')),
    ));

    terminated(
        separated_pair(
            direction,
            char(' '),
            map_res(nom::character::complete::u32, usize::try_from),
        ),
        opt(newline),
    )(input)
}

pub struct Solution;
impl crate::Solution for Solution {
    const DAY: usize = 9;
    type O1 = usize;
    type O2 = usize;

    fn part_one(input: &str) -> Self::O1 {
        let mut instructions = iterator(input, parse_instruction);
        let directions = instructions.flat_map(|(dir, count)| repeat(dir).take(count));

        let head_positions = directions.scan((0, 0), |(x, y), dir| {
            match dir {
                Direction::Up => *y += 1,
                Direction::Down => *y -= 1,
                Direction::Left => *x -= 1,
                Direction::Right => *x += 1,
            }

            Some((*x, *y))
        });

        let tail_positions = once((0i32, 0i32))
            .chain(head_positions)
            .tuple_windows()
            .scan((0, 0), |tail, (prev_head, curr_head)| {
                if (curr_head.0 - tail.0).abs() > 1 || (curr_head.1 - tail.1).abs() > 1 {
                    *tail = prev_head;
                }

                Some(*tail)
            });

        tail_positions.collect::<HashSet<_>>().len()
    }

    fn part_two(input: &str) -> Self::O2 {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::Solution;

    const SAMPLE_INPUT: &'static str = indoc! {"
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
    "};

    #[test]
    fn part_one() {
        assert_eq!(super::Solution::part_one(SAMPLE_INPUT), 13);
    }
}
