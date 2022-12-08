use std::{collections::HashMap, fmt::Display, fs::read_to_string, time::Instant};

use clap::Parser;
use itertools::Itertools;

mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_five;
mod day_six;
mod day_seven;

mod limit_heap;

trait SolutionRunner {
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
    fn day(&self) -> usize;
}

trait Solution {
    const DAY: usize;
    type O1: Display;
    type O2: Display;

    fn part_one(input: &str) -> Self::O1;

    fn part_two(input: &str) -> Self::O2;
}

impl<T: Solution> SolutionRunner for T {
    fn part_one(&self, input: &str) -> String {
        T::part_one(input).to_string()
    }

    fn part_two(&self, input: &str) -> String {
        T::part_two(input).to_string()
    }

    fn day(&self) -> usize {
        T::DAY
    }
}

fn create_solution_map<const N: usize>(
    solutions: [&dyn SolutionRunner; N],
) -> HashMap<usize, &dyn SolutionRunner> {
    solutions.into_iter().map(|s| (s.day(), s)).collect()
}

#[derive(Parser, Debug)]
#[command()]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    day: Option<usize>,
}

fn main() {
    let solutions = create_solution_map([
        &day_one::Solution,
        &day_two::Solution,
        &day_three::Solution,
        &day_four::Solution,
        &day_five::Solution,
        &day_six::Solution,
        &day_seven::Solution,
    ]);

    let args = Args::parse();

    let days_to_run = match args.day {
        Some(day) => vec![day],
        None => solutions.keys().copied().sorted().collect(),
    };

    for day in days_to_run {
        let solution = solutions
            .get(&day)
            .expect(&format!("No solution for day {day}"));

        let input = read_to_string(format!("./inputs/day{day}.txt"))
            .expect(&format!("No input file for day {day}"));
        let input = input.as_str();

        println!("Day {day}");
        let start = Instant::now();
        let answer = solution.part_one(input);
        let duration = start.elapsed().as_micros();
        println!("  Part one: {answer} ({duration}us)");

        let start = Instant::now();
        let answer = solution.part_two(input);
        let duration = start.elapsed().as_micros();
        println!("  Part two: {answer} ({duration}us)");
        println!()
    }
}
