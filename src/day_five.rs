use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, newline},
    combinator::{map, value},
    multi::{count, fold_many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

fn parse_stacks(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let item = map(delimited(char('['), anychar, char(']')), |c| Some(c));
    let gap = value(None, count(anychar, 3));
    let row = terminated(separated_list1(char(' '), alt((item, gap))), newline);

    let columns = fold_many1(row, Vec::new, |mut acc: Vec<Vec<char>>, row| {
        if acc.len() < row.len() {
            acc.resize_with(row.len(), Default::default);
        }

        for (column, item) in row.into_iter().enumerate() {
            if let Some(item) = item {
                acc[column].push(item)
            }
        }
        acc
    });

    let mut reversed_columns = nom::combinator::map(columns, |mut columns| {
        for column in columns.iter_mut() {
            column.reverse()
        }
        columns
    });

    reversed_columns(input)
}

fn parse_procedure(input: &str) -> IResult<&str, Vec<(u8, u8, u8)>> {
    let quantity = preceded(tag("move "), nom::character::complete::u8);
    let from = delimited(
        char(' '),
        preceded(tag("from "), nom::character::complete::u8),
        char(' '),
    );
    let to = preceded(tag("to "), nom::character::complete::u8);

    separated_list1(newline, tuple((quantity, from, to)))(input)
}

fn parse(input: &str) -> IResult<&str, (Vec<Vec<char>>, Vec<(u8, u8, u8)>)> {
    separated_pair(parse_stacks, newline, parse_procedure)(input)
}

pub struct Solution;
impl crate::Solution for Solution {
    const DAY: usize = 5;
    type O1 = String;
    type O2 = String;

    fn part_one(input: &str) -> Self::O1 {
        let (_, (mut stacks, procedure)) = parse(input).unwrap();

        for (quantity, from, to) in procedure {
            for _ in 0..quantity {
                let moved_item = stacks[(from - 1) as usize]
                    .pop()
                    .expect("Procedure attempted to move nonexisting item");
                stacks[(to - 1) as usize].push(moved_item);
            }
        }

        stacks
            .iter()
            .map(|column| column.last().unwrap_or(&' '))
            .collect()
    }

    fn part_two(input: &str) -> Self::O2 {
        let (_, (mut stacks, procedure)) = parse(input).unwrap();

        for (quantity, from, to) in procedure {
            let from_stack = &mut stacks[(from - 1) as usize];
            let mut moved_items = from_stack.split_off(from_stack.len() - (quantity as usize));
            stacks[(to - 1) as usize].append(&mut moved_items);
        }

        stacks
            .iter()
            .map(|column| column.last().unwrap_or(&' '))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    #[test]
    fn parse_stacks() {
        let input = indoc! {"
                [B]    
            [A] [C] [D]
             1   2   3 
        "};

        let (_, stacks) = super::parse_stacks(input).unwrap();
        assert_eq!(stacks.len(), 3);
        assert_eq!(stacks[0], vec!['A']);
        assert_eq!(stacks[1], vec!['C', 'B']);
        assert_eq!(stacks[2], vec!['D']);
    }

    #[test]
    fn parse_procedure() {
        let input = indoc! {"
            move 2 from 1 to 9
            move 10 from 3 to 22
        "};

        let (_, procedure) = super::parse_procedure(input).unwrap();
        assert_eq!(procedure.len(), 2);
        assert_eq!(procedure[0], (2, 1, 9));
        assert_eq!(procedure[1], (10, 3, 22));
    }

    #[test]
    fn parse() {
        let input = indoc! {"
                [B]    
            [A] [C] [D]
             1   2   3 

            move 2 from 1 to 9
            move 10 from 3 to 22
        "};

        let (_, (stacks, procedure)) = super::parse(input).unwrap();

        assert_eq!(stacks.len(), 3);
        assert_eq!(stacks[0], vec!['A']);
        assert_eq!(stacks[1], vec!['C', 'B']);
        assert_eq!(stacks[2], vec!['D']);

        assert_eq!(procedure.len(), 2);
        assert_eq!(procedure[0], (2, 1, 9));
        assert_eq!(procedure[1], (10, 3, 22));
    }
}
