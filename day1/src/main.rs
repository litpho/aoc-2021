use std::{fs, io::Read};

use anyhow::Result;
use nom::{character::complete, character::complete::line_ending, multi::separated_list1, IResult};

fn main() -> Result<()> {
    let input = parse_input()?;

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
    separated_list1(line_ending, complete::u16)(input)
}

fn parse_input() -> Result<Vec<u16>> {
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
        assert_eq!(1681, part_one(&parse_input()?));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(1704, part_two(&parse_input()?));

        Ok(())
    }
}
