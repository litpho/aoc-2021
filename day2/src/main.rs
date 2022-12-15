use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {}", took);
    let input = result?;

    let (took, (x, depth)) = took::took(|| part_one(&input));
    println!("Result part one: {x} / {depth} = {}", x * depth);
    println!("Time spent: {took}");

    let (took, (x, depth)) = took::took(|| part_two(&input));
    println!("Result part two: {x} / {depth} = {}", x * depth);
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &[Instruction]) -> (i32, i32) {
    let mut x: i32 = 0;
    let mut depth: i32 = 0;

    input.iter().for_each(|instruction| match instruction {
        Instruction::Forward(value) => x += value,
        Instruction::Down(value) => depth += value,
        Instruction::Up(value) => depth -= value,
    });

    (x, depth)
}

fn part_two(input: &[Instruction]) -> (i32, i32) {
    let mut x: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    input.iter().for_each(|instruction| match instruction {
        Instruction::Forward(value) => {
            x += value;
            depth += aim * value
        }
        Instruction::Down(value) => aim += value,
        Instruction::Up(value) => aim -= value,
    });

    (x, depth)
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Instruction> {
    map(
        separated_pair(parse_command, space1, complete::i32),
        |(command, value)| match command {
            "forward" => Instruction::Forward(value),
            "down" => Instruction::Down(value),
            "up" => Instruction::Up(value),
            _ => unreachable!(),
        },
    )(input)
}

fn parse_command(input: &str) -> IResult<&str, &str> {
    alt((tag("forward"), tag("down"), tag("up")))(input)
}

fn parse_input(input: &'static str) -> Result<Vec<Instruction>> {
    let (_, input) = parse(input)?;

    Ok(input)
}

enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        let (x, depth) = part_one(&parse_input(TESTDATA)?);

        assert_eq!(15, x);
        assert_eq!(10, depth);

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let (x, depth) = part_one(&parse_input(DATA)?);

        assert_eq!(1967, x);
        assert_eq!(1031, depth);

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        let (x, depth) = part_two(&parse_input(TESTDATA)?);

        assert_eq!(15, x);
        assert_eq!(60, depth);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let (x, depth) = part_two(&parse_input(DATA)?);

        assert_eq!(1967, x);
        assert_eq!(967791, depth);

        Ok(())
    }
}
