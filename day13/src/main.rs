use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    character::complete::{alpha1, line_ending},
    combinator::map,
    multi::count,
    multi::separated_list1,
    sequence::preceded,
    sequence::separated_pair,
    IResult,
};
use std::{cmp::Ordering, collections::HashSet, fs, io::Read, str::FromStr};

fn main() -> Result<()> {
    let (grid, instructions) = read_input()?;

    let (took, result) = took::took(|| part_one(grid.clone(), instructions.clone()));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| part_two(grid, instructions));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(grid: Grid, instructions: Vec<Instruction>) -> usize {
    let grid = grid.fold(instructions.get(0).unwrap());

    grid.dots.len()
}

fn part_two(grid: Grid, instructions: Vec<Instruction>) -> usize {
    let grid = instructions
        .iter()
        .fold(grid, |grid, instruction| grid.fold(instruction));

    grid.visualize();

    grid.dots.len()
}

#[derive(Clone)]
struct Grid {
    dots: HashSet<(i16, i16)>,
}

impl Grid {
    pub fn new(dots: Vec<(i16, i16)>) -> Self {
        let dots = dots.into_iter().collect::<HashSet<(i16, i16)>>();
        Grid { dots }
    }

    pub fn fold(&self, instruction: &Instruction) -> Self {
        let new_dots = self
            .dots
            .iter()
            .map(|(x, y)| instruction.fold(*x, *y))
            .collect::<HashSet<(i16, i16)>>();
        Grid { dots: new_dots }
    }

    pub fn visualize(&self) {
        let mut output = self.dots.iter().collect::<Vec<&(i16, i16)>>();
        output.sort_by(|this, other| {
            let ordering = this.1.cmp(&other.1);
            if ordering == Ordering::Equal {
                this.0.cmp(&other.0)
            } else {
                ordering
            }
        });

        let mut last: (i16, i16) = (0, 0);
        for (x, y) in output {
            if last.1 < *y {
                (last.1..*y).for_each(|_| println!());
                last = (0, *y);
            }
            if last.0 < *x {
                (last.0..*x - 1).for_each(|_| print!(" "));
            }
            print!("X");
            last = (*x, *y);
        }
        println!();
    }
}

#[derive(Clone, Debug)]
enum Direction {
    X,
    Y,
}

#[derive(Clone, Debug)]
struct Instruction {
    direction: Direction,
    location: i16,
}

impl Instruction {
    pub fn fold(&self, mut x: i16, mut y: i16) -> (i16, i16) {
        match self.direction {
            Direction::X => {
                if x > self.location {
                    x = self.location - (x - self.location)
                }
            }
            Direction::Y => {
                if y > self.location {
                    y = self.location - (y - self.location)
                }
            }
        }
        (x, y)
    }
}

fn parse(input: &str) -> IResult<&str, (Grid, Vec<Instruction>)> {
    separated_pair(parse_dots, count(line_ending, 2), parse_instructions)(input)
}

fn parse_dots(input: &str) -> IResult<&str, Grid> {
    map(separated_list1(line_ending, parse_dot_line), |dots| {
        Grid::new(dots)
    })(input)
}

fn parse_dot_line(input: &str) -> IResult<&str, (i16, i16)> {
    map(separated_pair(digit1, tag(","), digit1), |(a, b)| {
        (i16::from_str(a).unwrap(), i16::from_str(b).unwrap())
    })(input)
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, parse_instruction_line)(input)
}

fn parse_instruction_line(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(tag("fold along "), separated_pair(alpha1, tag("="), digit1)),
        |(a, b)| {
            let direction = match a {
                "x" => Direction::X,
                "y" => Direction::Y,
                _ => panic!("Direction {a} was not readable"),
            };
            let location = b.parse::<i16>().unwrap();
            Instruction {
                direction,
                location,
            }
        },
    )(input)
}

fn read_input() -> Result<(Grid, Vec<Instruction>)> {
    let mut buf = String::new();
    fs::File::open("src/input.txt")?.read_to_string(&mut buf)?;

    let (_, input) = parse(&buf).expect("Parse failure");

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let (grid, instructions) = read_input()?;

        let count = part_one(grid, instructions);

        assert_eq!(814, count);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let (grid, instructions) = read_input()?;

        let count = part_two(grid, instructions);

        // PZEHRAER
        assert_eq!(108, count);

        Ok(())
    }
}
