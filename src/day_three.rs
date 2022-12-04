use std::{cmp::Ordering, collections::HashSet, fs::read_to_string};

use nom::{
    branch::alt,
    character::{complete::char, complete::newline},
    combinator::value,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}


fn get_duplicate_item(input: &str) -> Option<char> {
    let (left, right) = input.split_at(input.len() / 2);
    let left_items = left.chars().collect::<HashSet<_>>();
    right.chars().filter(|c| left_items.contains(c)).next()
}

fn get_item_priority(input: char) -> Option<u8> {
    match input {
        'a'..='z' => Some(input as u8 - b'a' + 1),
        'A'..='Z' => Some(input as u8 - b'A' + 27),
        _ => None
    }
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|s| get_duplicate_item(s).expect("Rucksack did not contain duplicate a duplicate"))
        .map(|i| get_item_priority(i).expect("Duplicate item does not have a defined priority"))
        .map(u32::from)
        .sum()
}

fn part_two() -> u32 {
    todo!();
}

pub fn run() {
    let input = read_to_string("./inputs/day3.txt").unwrap();

    println!("Day 3:");

    let priority_sum = part_one(input.as_str());
    println!("Part one: {priority_sum}")
}

#[cfg(test)]
mod test {
    #[test]
    fn get_item_priority() {
        assert_eq!(super::get_item_priority('a'), Some(1));
        assert_eq!(super::get_item_priority('b'), Some(2));
        assert_eq!(super::get_item_priority('z'), Some(26));
        assert_eq!(super::get_item_priority('A'), Some(27));
        assert_eq!(super::get_item_priority('B'), Some(28));
        assert_eq!(super::get_item_priority('Z'), Some(52));
        assert_eq!(super::get_item_priority('1'), None);
        assert_eq!(super::get_item_priority('$'), None);
    }

    #[test]
    fn get_duplicate_item() {
        assert_eq!(super::get_duplicate_item("abcdef"), None);
        assert_eq!(super::get_duplicate_item("abcdea"), Some('a'));
    }
}
