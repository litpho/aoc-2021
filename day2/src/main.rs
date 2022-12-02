use anyhow::{Error, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space1,
    character::complete::{digit1, line_ending},
    combinator::{eof, map_res},
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};
use std::{fs, io::Read};

fn main() -> Result<()> {
    let input = read_input()?;

    let (took, (x, depth)) = took::took(|| part_one(&input));
    println!("Result part one: {} / {} = {}", x, depth, x * depth);
    println!("Time spent: {}", took);

    let (took, (x, depth)) = took::took(|| part_two(&input));
    println!("Result part two: {} / {} = {}", x, depth, x * depth);
    println!("Time spent: {}", took);

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
    many1(terminated(parse_line, alt((line_ending, eof))))(input)
}

fn parse_line(input: &str) -> IResult<&str, Instruction> {
    map_res(
        separated_pair(parse_command, space1, digit1),
        |(command, number)| {
            let value = number.parse::<i32>()?;
            let instruction = match command {
                "forward" => Instruction::Forward(value),
                "down" => Instruction::Down(value),
                "up" => Instruction::Up(value),
                _ => unreachable!(),
            };
            Ok::<Instruction, Error>(instruction)
        },
    )(input)
}

fn parse_command(input: &str) -> IResult<&str, &str> {
    alt((tag("forward"), tag("down"), tag("up")))(input)
}

fn read_input() -> Result<Vec<Instruction>> {
    let mut buf = String::new();
    fs::File::open("src/input.txt")?.read_to_string(&mut buf)?;

    let (_, input) = parse(&buf).expect("Parse failure");

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

    #[test]
    fn test_part_one() -> Result<()> {
        let input = read_input()?;

        let (x, depth) = part_one(&input);

        assert_eq!(1967, x);
        assert_eq!(1031, depth);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = read_input()?;

        let (x, depth) = part_two(&input);

        assert_eq!(1967, x);
        assert_eq!(967791, depth);

        Ok(())
    }
}
