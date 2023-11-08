use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::preceded,
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
    input
        .iter()
        .fold((0, 0), |(x, depth), instruction| match instruction {
            Instruction::Forward(value) => (x + value, depth),
            Instruction::Down(value) => (x, depth + value),
            Instruction::Up(value) => (x, depth - value),
        })
}

fn part_two(input: &[Instruction]) -> (i32, i32) {
    let (x, depth, _) =
        input.iter().fold(
            (0, 0, 0),
            |(x, depth, aim), instruction| match instruction {
                Instruction::Forward(value) => (x + value, depth + aim * value, aim),
                Instruction::Down(value) => (x, depth, aim + value),
                Instruction::Up(value) => (x, depth, aim - value),
            },
        );

    (x, depth)
}

enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Instruction> {
    alt((parse_forward, parse_down, parse_up))(input)
}

fn parse_forward(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("forward "), complete::i32), |value| {
        Instruction::Forward(value)
    })(input)
}

fn parse_down(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("down "), complete::i32), |value| {
        Instruction::Down(value)
    })(input)
}

fn parse_up(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("up "), complete::i32), |value| {
        Instruction::Up(value)
    })(input)
}

fn parse_input(input: &'static str) -> Result<Vec<Instruction>> {
    let (_, input) = parse(input)?;

    Ok(input)
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
