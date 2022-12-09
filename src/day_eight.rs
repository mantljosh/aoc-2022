use std::{ops::Index, collections::HashSet};

fn parse(input: &str) -> Grid<u32> {
    let width = input.lines().next().unwrap().len();
    let cells: Vec<u32> = input.lines().flat_map(|line| line.chars()).map(|c| c.to_digit(10).unwrap()).collect();

    Grid {
        width,
        height: cells.len() / width,
        cells
    }
}

struct Grid<T> {
    height: usize,
    width: usize,
    cells: Vec<T>,
}

impl<'a, T> Grid<T> {
    fn row(&self, row: usize) -> impl DoubleEndedIterator<Item = (usize, usize)> {
        (0..self.width).map(move |col| (row, col))
    }

    fn column(&self, col: usize) -> impl DoubleEndedIterator<Item = (usize, usize)> {
        (0..self.height).map(move |row| (row, col))
    }

    fn rows(&'a self) -> impl Iterator<Item = impl DoubleEndedIterator<Item = (usize, usize)> + 'a> {
        (0..self.height).map(|row| self.row(row))
    }

    fn columns(&'a self) -> impl Iterator<Item = impl DoubleEndedIterator<Item = (usize, usize)> + 'a> {
        (0..self.width).map(|column| self.column(column))
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, column) = index;
        &self.cells[row * self.width + column]
    }
}

fn scan_cells<'a>(grid: &'a Grid<u32>, cells: impl Iterator<Item = (usize, usize)> + 'a) -> impl Iterator<Item = (usize, usize)> + 'a {
    cells.scan(None, |max, curr| {
        if let Some(max) = *max {
            if grid[curr] <= max {
                return Some(None);
            }
        }

        *max = Some(grid[curr]);

        Some(Some(curr))
    }).flatten()
}

pub struct Solution;
impl crate::Solution for Solution {
    const DAY: usize = 8;
    type O1 = usize;
    type O2 = usize;

    fn part_one(input: &str) -> Self::O1 {
        let grid = parse(input);

        let left_to_right = grid.rows().flat_map(|row| scan_cells(&grid, row));
        let right_to_left = grid.rows().flat_map(|row| scan_cells(&grid, row.rev()));
        let top_to_bottom = grid.columns().flat_map(|column| scan_cells(&grid, column));
        let bottom_to_top = grid.columns().flat_map(|column| scan_cells(&grid, column.rev()));

        left_to_right.chain(right_to_left).chain(top_to_bottom).chain(bottom_to_top).collect::<HashSet<_>>().len()
    }

    fn part_two(input: &str) -> Self::O2 {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::Solution;

    const SAMPLE_INPUT: &'static str = indoc! {"
        30373
        25512
        65332
        33549
        35390
    "};

    #[test]
    fn parse() {
        let grid = super::parse(SAMPLE_INPUT);
        assert_eq!(grid.width, 5);
        assert_eq!(grid.height, 5);
        assert_eq!(grid[(0,0)], 3);
        assert_eq!(grid[(1,3)], 1);
    }

    #[test]
    fn part_one() {
        let result = super::Solution::part_one(SAMPLE_INPUT);
        assert_eq!(result, 21);
    }
}