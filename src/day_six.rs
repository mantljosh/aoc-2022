use heapless::Deque;
use itertools::Itertools;

fn find_unique_sequence<const N: usize>(input: &str) -> Option<usize> {
    let mut unique_chars = 0;
    let mut queue: Deque<char, N> = Deque::new();

    for (index, char) in input.chars().enumerate() {
        if queue.is_full() {
            let oldest_char = queue.pop_front().unwrap();
            if !queue.iter().contains(&oldest_char) {
                unique_chars -= 1;
            }
        }

        if !queue.iter().contains(&char) {
            unique_chars += 1;
        }
        queue.push_back(char).unwrap();

        if unique_chars == N {
            return Some(index);
        }
    }

    None
}

pub struct Solution;
impl crate::Solution for Solution {
    const DAY: usize = 6;
    type O1 = usize;
    type O2 = usize;

    fn part_one(input: &str) -> Self::O1 {
        find_unique_sequence::<4>(input).unwrap() + 1
    }

    fn part_two(input: &str) -> Self::O2 {
        find_unique_sequence::<14>(input).unwrap() + 1
    }
}
