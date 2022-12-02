use std::{fs::read_to_string, cmp::Ordering};

use nom::{
    character::{complete::newline, complete::char },
    character::complete::u32 as parse_u32,
    combinator::{map, value},
    multi::{count, separated_list1},
    IResult, branch::alt, bytes::complete::tag, sequence::separated_pair,
};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Ord for Choice {
    fn cmp(&self, other: &Self) -> Ordering {
        let is_winner = match self {
            Self::Rock => *other == Self::Scissors,
            Self::Paper => *other == Self::Rock,
            Self::Scissors => *other == Self::Paper,
        };

        if is_winner {
            Ordering::Greater
        } else if self == other {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}

impl PartialOrd for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn score_round(round: &(Choice, Choice)) -> u32 {
    let (opponents_choice, my_choice) = round;

    let choice_score = match my_choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };

    let result_score = match my_choice.cmp(opponents_choice) {
        Ordering::Less => 0,
        Ordering::Equal => 3,
        Ordering::Greater => 6,
    };

    choice_score + result_score
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Choice, Choice)>> {
    let opponent_choice = alt((
        value(Choice::Rock, char('A')),
        value(Choice::Paper, char('B')),
        value(Choice::Scissors, char('C')),
    ));

    let my_choice = alt((
        value(Choice::Rock, char('X')),
        value(Choice::Paper, char('Y')),
        value(Choice::Scissors, char('Z')),
    ));

    separated_list1(newline,
    separated_pair(opponent_choice, char(' '), my_choice))(input)
}

fn part_one(rounds: &Vec<(Choice, Choice)>) -> u32 {
    rounds.iter().map(score_round).sum()
}

fn part_two(elves: &Vec<(Choice, Choice)>) -> u32 {
    todo!()
}

pub fn run() {
    let input = read_to_string("./inputs/day2.txt").unwrap();
    let (_, rounds) = parse_input(input.as_str()).unwrap();

    println!("Day 2:");
    let total_score = part_one(&rounds);
    println!("Part one: {total_score}");
}

#[cfg(test)]
mod test {
    use super::Choice;
    use indoc::indoc;

    #[test]
    fn parse_input() {
        let input = indoc! {"
            A Y
            B X
            C Z
        "};

        let (_, rounds) = super::parse_input(input).unwrap();
        assert_eq!(
            rounds,
            vec![
                (Choice::Rock, Choice::Paper),
                (Choice::Paper, Choice::Rock),
                (Choice::Scissors, Choice::Scissors),
            ]
        );
    }

    #[test]
    fn score_round() {
        assert_eq!(super::score_round(&(Choice::Rock, Choice::Paper)), 8);
        assert_eq!(super::score_round(&(Choice::Paper, Choice::Rock)), 1);
        assert_eq!(super::score_round(&(Choice::Scissors, Choice::Scissors)), 6);
    }

    #[test]
    fn part_one() {
    }

    #[test]
    fn part_two() {
    }
}
