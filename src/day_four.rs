use std::ops::RangeInclusive;

use nom::{
    character::complete::{char, newline, u32 as parse_u32},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

type Assignment = RangeInclusive<u32>;
type Pair = (Assignment, Assignment);

fn parse_assignment(input: &str) -> IResult<&str, Assignment> {
    map(separated_pair(parse_u32, char('-'), parse_u32), |(a, b)| {
        RangeInclusive::new(a, b)
    })(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list1(
        newline,
        separated_pair(parse_assignment, char(','), parse_assignment),
    )(input)
}

fn fully_overlaps(a: &Assignment, b: &Assignment) -> bool {
    a.contains(b.start()) && a.contains(b.end()) || b.contains(a.start()) && b.contains(a.end())
}

pub struct Solution;
impl crate::Solution for Solution {
    const DAY: usize = 4;
    type O1 = usize;
    type O2 = u32;

    fn part_one(input: &str) -> Self::O1 {
        let (_, pairs) = parse(input).expect("Failed to parse assignment pairs");

        pairs.iter().filter(|(a, b)| fully_overlaps(a, b)).count()
    }

    fn part_two(input: &str) -> Self::O2 {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use std::ops::RangeInclusive;

    use indoc::indoc;

    #[test]
    fn parse_assignment() {
        let (_, assignment) = super::parse_assignment("5-20").unwrap();
        assert_eq!(assignment, 5..=20);
    }

    #[test]
    fn parse() {
        let input = indoc! {"
            1-3,2-7
            3-10,4-22
        "};

        let (_, pairs) = super::parse(input).unwrap();
        assert_eq!(pairs, vec![(1..=3, 2..=7), (3..=10, 4..=22),]);
    }

    #[test]
    fn fully_overlaps() {
        assert_eq!(
            super::fully_overlaps(&RangeInclusive::new(2, 8), &RangeInclusive::new(3, 7)),
            true
        );
        assert_eq!(
            super::fully_overlaps(&RangeInclusive::new(2, 8), &RangeInclusive::new(3, 8)),
            true
        );
        assert_eq!(
            super::fully_overlaps(&RangeInclusive::new(2, 8), &RangeInclusive::new(3, 9)),
            false
        );
        assert_eq!(
            super::fully_overlaps(&RangeInclusive::new(2, 8), &RangeInclusive::new(1, 7)),
            false
        );
        assert_eq!(
            super::fully_overlaps(&RangeInclusive::new(1, 2), &RangeInclusive::new(4, 5)),
            false
        );
    }
}
