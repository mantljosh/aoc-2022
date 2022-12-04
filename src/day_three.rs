use itertools::Itertools;
use std::{collections::HashSet, hash::Hash};

fn get_duplicate_item(input: &str) -> Option<char> {
    let (left, right) = input.split_at(input.len() / 2);
    find_common_items([left.chars(), right.chars()])
        .into_iter()
        .next()
}

fn get_item_priority(input: char) -> Option<u8> {
    match input {
        'a'..='z' => Some(input as u8 - b'a' + 1),
        'A'..='Z' => Some(input as u8 - b'A' + 27),
        _ => None,
    }
}

fn find_common_items<I, T>(bags: impl IntoIterator<Item = I>) -> HashSet<T>
where
    HashSet<T>: FromIterator<T>,
    I: IntoIterator<Item = T>,
    T: Eq + Hash + Clone,
{
    bags.into_iter()
        .map(FromIterator::from_iter)
        .reduce(|acc: HashSet<T>, bag| acc.intersection(&bag).cloned().collect())
        .unwrap_or_default()
}

pub struct Solution;
impl crate::Solution for Solution {
    const DAY: usize = 3;
    type O1 = u32;
    type O2 = u32;

    fn part_one(input: &str) -> Self::O1 {
        input
            .lines()
            .map(|s| get_duplicate_item(s).expect("Rucksack did not contain duplicate a duplicate"))
            .map(|i| get_item_priority(i).expect("Duplicate item does not have a defined priority"))
            .map(u32::from)
            .sum()
    }

    fn part_two(input: &str) -> Self::O2 {
        input
            .lines()
            .map(|line| line.chars().collect::<HashSet<char>>())
            .chunks(3)
            .into_iter()
            .map(find_common_items)
            .map(|common_items| common_items.into_iter().next().expect("No common items"))
            .map(|i| get_item_priority(i).expect("Duplicate item does not have a defined priority"))
            .map(u32::from)
            .sum()
    }
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

    #[test]
    fn find_common_items() {
        let groups = vec![vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]];

        let common_items = super::find_common_items(groups);
        assert_eq!(common_items.len(), 1);
        assert!(common_items.contains(&3));
    }
}
