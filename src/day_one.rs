use crate::limit_heap::LimitHeap;
use nom::{
    character::complete::{newline, u32 as parse_u32},
    combinator::map,
    multi::{count, separated_list1},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
struct Elf {
    items: Vec<u32>,
}

impl Elf {
    fn calories(&self) -> u32 {
        self.items.iter().sum()
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Elf>> {
    let elf = map(separated_list1(newline, parse_u32), |items| Elf {
        items: items,
    });

    separated_list1(count(newline, 2), elf)(input)
}

pub struct Solution;
impl crate::Solution for Solution {
    const DAY: usize = 1;
    type O1 = u32;
    type O2 = u32;

    fn part_one(input: &str) -> Self::O1 {
        let (_, elves) = parse_input(input).unwrap();
        elves
            .iter()
            .map(Elf::calories)
            .max()
            .expect("Input contained no elves")
    }

    fn part_two(input: &str) -> Self::O2 {
        let (_, elves) = parse_input(input).unwrap();
        elves
            .iter()
            .map(Elf::calories)
            .collect::<LimitHeap<_, 3>>()
            .iter()
            .sum()
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    use super::Elf;
    use indoc::indoc;

    #[test]
    fn parse_input() {
        let input = indoc! {"
            100
            200

            300

            400
            500
            600
        "};

        let (_, elves) = super::parse_input(input).unwrap();
        assert_eq!(
            elves,
            vec![
                Elf {
                    items: vec![100, 200]
                },
                Elf { items: vec![300] },
                Elf {
                    items: vec![400, 500, 600]
                },
            ]
        );
    }

    #[test]
    fn part_one() {
        let input = indoc! {"
            100
            200

            500
        "};

        assert_eq!(super::Solution::part_one(input), 500);
    }

    #[test]
    fn part_two() {
        let input = indoc! {"
            100
            200

            500

            600

            700

            100
        "};

        assert_eq!(super::Solution::part_two(input), 1800);
    }
}
