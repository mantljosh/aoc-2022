use std::{
    cmp::{max, Reverse},
    collections::BinaryHeap,
    fs::read_to_string,
};

use nom::{
    character::complete::newline,
    character::complete::u32 as parse_u32,
    combinator::{map, opt},
    multi::{fold_many1, many0},
    sequence::terminated,
    IResult
};

fn elf(input: &str) -> IResult<&str, u32> {
    fold_many1(
        terminated(parse_u32, opt(newline)),
        || 0u32,
        |acc, i| acc + i,
    )(input)
}

fn max_calories(input: &str) -> IResult<&str, u32> {
    fold_many1(
        terminated(elf, many0(newline)),
        || 0u32,
        |acc, i| max(acc, i),
    )(input)
}

fn top_three_calories(input: &str) -> IResult<&str, u32> {
    map(
        fold_many1(
            terminated(elf, many0(newline)),
            BinaryHeap::<Reverse<u32>>::new,
            |mut acc, i| {
                if acc.len() < 3 {
                    acc.push(Reverse(i));
                } else if let Some(&min) = acc.peek() {
                    if min.0 < i {
                        acc.pop();
                        acc.push(Reverse(i))
                    }
                }
                acc
            },
        ),
        |heap| heap.iter().map(|i| i.0).sum(),
    )(input)
}

fn main() {
    let input_file = read_to_string("./inputs/day1.txt").unwrap();

    let (_, calories) = max_calories(input_file.as_str()).unwrap();
    println!("The elf with the most calories has {calories} calories");

    let (_, calories) = top_three_calories(input_file.as_str()).unwrap();
    println!("The top three elves have {calories} calories");
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    #[test]
    fn parse_elf() {
        let input = indoc! {"
            100
            200
            300
        "};

        let (_, value) = super::elf(input).unwrap();
        assert_eq!(value, 600);
    }

    #[test]
    fn max_calories() {
        let input = indoc! {"
            100
            200

            300
            400
        "};

        let (_, value) = super::max_calories(input).unwrap();
        assert_eq!(value, 700);
    }

    #[test]
    fn top_three_calories() {
        let input = indoc! {"
            100
            200

            300
            400

            50

            75
        "};

        let (_, value) = super::top_three_calories(input).unwrap();
        assert_eq!(value, 1075);
    }
}
