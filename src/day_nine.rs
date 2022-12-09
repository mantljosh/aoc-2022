use nom::{
    branch::alt,
    character::complete::{char, newline},
    combinator::{iterator, map_res, opt, value},
    sequence::{separated_pair, terminated},
    IResult,
};
use std::{
    collections::HashSet,
    iter::{once, repeat},
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

fn head_positions(directions: impl Iterator<Item = Direction>) -> impl Iterator<Item = (i32, i32)> {
    let positions = directions.scan((0, 0), |(x, y), dir| {
        match dir {
            Direction::Up => *y += 1,
            Direction::Down => *y -= 1,
            Direction::Left => *x -= 1,
            Direction::Right => *x += 1,
        }

        Some((*x, *y))
    });

    once((0i32, 0i32)).chain(positions)
}

fn follow_leader(
    leader_positions: impl Iterator<Item = (i32, i32)>,
) -> impl Iterator<Item = (i32, i32)> {
    leader_positions.scan((0, 0), |tail, head| {
        let delta_x = head.0 - tail.0;
        let delta_y = head.1 - tail.1;

        *tail = match (delta_x.abs() > 1, delta_y.abs() > 1) {
            (true, true) => (
                head.0 - (delta_x / delta_x.abs()),
                head.1 - (delta_y / delta_y.abs()),
            ),
            (true, false) => (head.0 - (delta_x / delta_x.abs()), head.1),
            (false, true) => (head.0, head.1 - (delta_y / delta_y.abs())),
            (false, false) => *tail,
        };

        Some(*tail)
    })
}

pub struct Solution;
impl crate::Solution for Solution {
    const DAY: usize = 9;
    type O1 = usize;
    type O2 = usize;

    fn part_one(input: &str) -> Self::O1 {
        let mut instructions = iterator(input, parse_instruction);
        let directions = instructions.flat_map(|(dir, count)| repeat(dir).take(count));
        follow_leader(head_positions(directions))
            .collect::<HashSet<_>>()
            .len()
    }

    fn part_two(input: &str) -> Self::O2 {
        let mut instructions = iterator(input, parse_instruction);
        let directions = instructions.flat_map(|(dir, count)| repeat(dir).take(count));

        let mut tail: Box<dyn Iterator<Item = (i32, i32)>> = Box::new(head_positions(directions));
        for _ in 0..9 {
            tail = Box::new(follow_leader(tail));
        }

        tail.collect::<HashSet<_>>().len()
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::Solution;

    #[test]
    fn part_one() {
        let input = indoc! {"
            R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2
        "};
        assert_eq!(super::Solution::part_one(input), 13);
    }

    #[test]
    fn part_two() {
        let input = indoc! {"
            R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20
        "};
        assert_eq!(super::Solution::part_two(input), 36);
    }
}
