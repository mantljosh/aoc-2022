use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, newline, not_line_ending},
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
enum Filesystem<'a> {
    File {
        name: &'a str,
        size: u32,
    },

    Directory {
        name: &'a str,
        children: Vec<Filesystem<'a>>,
        size: u32,
    },
}

fn parse_file(input: &str) -> IResult<&str, Filesystem> {
    map(
        terminated(
            separated_pair(nom::character::complete::u32, char(' '), not_line_ending),
            newline,
        ),
        |(size, name)| Filesystem::File { name, size },
    )(input)
}

fn subdir_listing(input: &str) -> IResult<&str, &str> {
    delimited(tag("dir"), not_line_ending, line_ending)(input)
}

fn parse_directory(input: &str) -> IResult<&str, Filesystem> {
    let name = delimited(tag("$ cd "), not_line_ending, tag("\n$ ls\n"));

    let children = many0(delimited(
        many0(subdir_listing),
        alt((parse_file, parse_directory)),
        many0(subdir_listing),
    ));
    let go_back = opt(tag("$ cd ..\n"));

    map(
        tuple((name, terminated(children, go_back))),
        |(name, children)| {
            let size = children.iter().map(Filesystem::size).sum();
            Filesystem::Directory {
                name,
                children,
                size,
            }
        },
    )(input)
}

impl<'a> Filesystem<'a> {
    fn size(&self) -> u32 {
        match self {
            Filesystem::File { size, .. } => *size,
            Filesystem::Directory { size, .. } => *size,
        }
    }

    fn children(&self) -> impl Iterator<Item = &Filesystem> {
        let children = match self {
            Filesystem::Directory { children, .. } => Some(children),
            _ => None,
        };

        children.into_iter().flatten()
    }

    fn iter(&'a self) -> Box<dyn Iterator<Item = &Filesystem> + 'a> {
        Box::new(std::iter::once(self).chain(self.children().flat_map(Self::iter)))
    }

    fn is_directory(&self) -> bool {
        matches!(self, Filesystem::Directory { .. })
    }
}

pub struct Solution;
impl crate::Solution for Solution {
    const DAY: usize = 7;
    type O1 = u32;
    type O2 = u32;

    fn part_one(input: &str) -> Self::O1 {
        let (_, filesystem) = parse_directory(input).unwrap();
        filesystem
            .iter()
            .filter(|x| x.is_directory())
            .map(Filesystem::size)
            .filter(|size| *size <= 100_000)
            .sum()
    }

    fn part_two(input: &str) -> Self::O2 {
        let (_, filesystem) = parse_directory(input).unwrap();
        const TOTAL_SPACE: u32 = 70_000_000;
        const UPDATE_SIZE: u32 = 30_000_000;
        let used_space = filesystem.size();

        filesystem
            .iter()
            .filter(|x| x.is_directory())
            .map(Filesystem::size)
            .filter(|size| (TOTAL_SPACE - used_space + size) >= UPDATE_SIZE)
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::Filesystem;
    use crate::Solution;
    use indoc::indoc;

    #[test]
    fn parse_file() {
        let input = "123 foo.txt";
        let (_, file) = super::parse_file(input).unwrap();
        assert_eq!(
            file,
            super::Filesystem::File {
                name: "foo.txt",
                size: 123
            }
        );
    }

    #[test]
    fn parse_directory() {
        let input = indoc! {"
            $ cd foo
            $ ls
            dir a
            100 bar.txt
            $ cd a
            $ ls
            200 baz.txt
            $ cd ..
            300 quux.txt
        "};

        let (rest, directory) = super::parse_directory(input).unwrap();
        println!("{rest}");

        assert_eq!(
            directory,
            Filesystem::Directory {
                name: "foo",
                size: 600,
                children: vec![
                    Filesystem::File {
                        name: "bar.txt",
                        size: 100
                    },
                    Filesystem::Directory {
                        name: "a",
                        size: 200,
                        children: vec![Filesystem::File {
                            name: "baz.txt",
                            size: 200
                        }]
                    },
                    Filesystem::File {
                        name: "quux.txt",
                        size: 300
                    }
                ]
            }
        );
    }

    const SAMPLE_INPUT: &'static str = indoc! {"
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
    "};

    #[test]
    fn part_one() {
        let result = super::Solution::part_one(SAMPLE_INPUT);
        assert_eq!(result, 95437);
    }

    #[test]
    fn part_two() {
        let result = super::Solution::part_two(SAMPLE_INPUT);
        assert_eq!(result, 24933642);
    }
}
