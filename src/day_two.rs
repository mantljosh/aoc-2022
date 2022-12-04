use std::{cmp::Ordering};

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

impl Choice {
    fn wins_to(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    fn loses_to(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }
}

impl Ord for Choice {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.wins_to() == *other {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

fn score_round(round: (Choice, Choice)) -> u32 {
    let (opponents_choice, my_choice) = round;

    let choice_score = match my_choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };

    let result_score = match my_choice.cmp(&opponents_choice) {
        Ordering::Less => 0,
        Ordering::Equal => 3,
        Ordering::Greater => 6,
    };

    choice_score + result_score
}

fn apply_strategy(round: (Choice, Outcome)) -> (Choice, Choice) {
    let (opponent_choice, desired_outcome) = round;

    let my_choice = match desired_outcome {
        Outcome::Win => opponent_choice.loses_to(),
        Outcome::Lose => opponent_choice.wins_to(),
        Outcome::Draw => opponent_choice.clone(),
    };

    (opponent_choice, my_choice)
}

fn parse_rounds(input: &str) -> IResult<&str, Vec<(Choice, Choice)>> {
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

    separated_list1(
        newline,
        separated_pair(opponent_choice, char(' '), my_choice),
    )(input)
}

fn parse_desired_outcomes(input: &str) -> IResult<&str, Vec<(Choice, Outcome)>> {
    let opponent_choice = alt((
        value(Choice::Rock, char('A')),
        value(Choice::Paper, char('B')),
        value(Choice::Scissors, char('C')),
    ));

    let desired_outcome = alt((
        value(Outcome::Lose, char('X')),
        value(Outcome::Draw, char('Y')),
        value(Outcome::Win, char('Z')),
    ));

    separated_list1(
        newline,
        separated_pair(opponent_choice, char(' '), desired_outcome),
    )(input)
}

pub struct Solution;
impl crate::Solution for Solution {
    const DAY: usize = 2;
    type O1 = u32;
    type O2 = u32;

    fn part_one(input: &str) -> Self::O1 {
        let (_, rounds) = parse_rounds(input).unwrap();
        rounds.iter().copied().map(score_round).sum()
    }

    fn part_two(input: &str) -> Self::O2 {
        let (_, rounds) = parse_desired_outcomes(input).unwrap();
        rounds
            .iter()
            .copied()
            .map(apply_strategy)
            .map(score_round)
            .sum()
    }
}
#[cfg(test)]
mod test {
    use super::{Choice, Outcome};
    use indoc::indoc;

    #[test]
    fn parse_rounds() {
        let input = indoc! {"
            A Y
            B X
            C Z
        "};

        let (_, rounds) = super::parse_rounds(input).unwrap();
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
    fn parse_desired_outcomes() {
        let input = indoc! {"
            A Y
            B X
            C Z
        "};

        let (_, rounds) = super::parse_desired_outcomes(input).unwrap();
        assert_eq!(
            rounds,
            vec![
                (Choice::Rock, Outcome::Draw),
                (Choice::Paper, Outcome::Lose),
                (Choice::Scissors, Outcome::Win),
            ]
        );
    }

    #[test]
    fn score_round() {
        assert_eq!(super::score_round((Choice::Rock, Choice::Paper)), 8);
        assert_eq!(super::score_round((Choice::Paper, Choice::Rock)), 1);
        assert_eq!(super::score_round((Choice::Scissors, Choice::Scissors)), 6);
    }

    #[test]
    fn apply_strategy() {
        assert_eq!(
            super::apply_strategy((Choice::Rock, Outcome::Win)),
            (Choice::Rock, Choice::Paper)
        );
        assert_eq!(
            super::apply_strategy((Choice::Paper, Outcome::Draw)),
            (Choice::Paper, Choice::Paper)
        );
        assert_eq!(
            super::apply_strategy((Choice::Scissors, Outcome::Lose)),
            (Choice::Scissors, Choice::Paper)
        );
    }
}
