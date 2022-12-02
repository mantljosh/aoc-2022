use std::fs::read_to_string;

use crate::limit_heap::LimitHeap;
use nom::{
    character::complete::newline,
    character::complete::u32 as parse_u32,
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

fn part_one(elves: &Vec<Elf>) -> u32 {
    elves.iter().map(Elf::calories).max().unwrap_or_default()
}

fn part_two(elves: &Vec<Elf>) -> u32 {
    elves
        .iter()
        .map(Elf::calories)
        .collect::<LimitHeap<_, 3>>()
        .iter()
        .sum()
}

pub fn run() {
    let input = read_to_string("./inputs/day1.txt").unwrap();
    let (_, elves) = parse_input(input.as_str()).unwrap();

    println!("Day one:");
    let calories = part_one(&elves);
    println!("The elf with the most calories has {calories} calories");

    let calories = part_two(&elves);
    println!("The top three elves have {calories} calories");
}

#[cfg(test)]
mod test {
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
        let elves = vec![
            Elf {
                items: vec![100, 200],
            },
            Elf { items: vec![500] },
        ];

        assert_eq!(super::part_one(&elves), 500);
    }

    #[test]
    fn part_two() {
        let elves = vec![
            Elf {
                items: vec![100, 200],
            },
            Elf { items: vec![500] },
            Elf { items: vec![600] },
            Elf { items: vec![700] },
            Elf { items: vec![100] },
        ];

        assert_eq!(super::part_two(&elves), 1800);
    }
}
