use anyhow::Result;
use nom::{
    character::complete::{self, line_ending},
    multi::separated_list1,
    IResult, Parser,
};

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {}", took);
    let input = result?;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| part_two(&input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &[u16]) -> usize {
    input.windows(2).filter(|s| s[1] > s[0]).count()
}

fn part_two(input: &[u16]) -> usize {
    input
        .windows(4)
        .filter(|s| {
            let first = s[0] + s[1] + s[2];
            let second = s[1] + s[2] + s[3];
            second > first
        })
        .count()
}

fn parse(input: &str) -> IResult<&str, Vec<u16>> {
    separated_list1(line_ending, complete::u16).parse(input)
}

fn parse_input(input: &'static str) -> Result<Vec<u16>> {
    let (_, input) = parse(input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(part_one(&parse_input(TESTDATA)?), 7);

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(part_one(&parse_input(DATA)?), 1681);

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(part_two(&parse_input(TESTDATA)?), 5);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(part_two(&parse_input(DATA)?), 1704);

        Ok(())
    }
}
