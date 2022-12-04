use std::{fs, io::Read};

use anyhow::{Error, Result};
use itertools::Itertools;
use nom::{character::complete, multi::separated_list1, IResult};

fn main() -> Result<()> {
    let input = read_input()?;

    let (took, result) = took::took(|| part_one(input.clone()));
    let (position, fuel_spent) = result?;
    println!("Result part one: {position} - {fuel_spent}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| part_two(input));
    let (position, fuel_spent) = result?;
    println!("Result part two: {position} - {fuel_spent}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: Vec<i32>) -> Result<(i32, i64)> {
    algorithm(input, |num, x| (num - x).abs() as i64)
}

fn part_two(input: Vec<i32>) -> Result<(i32, i64)> {
    algorithm(input, |num, x| calc((num - x).abs() as i64))
}

fn algorithm(input: Vec<i32>, map_fn: fn(i32, i32) -> i64) -> Result<(i32, i64)> {
    let (min, max) = input
        .iter()
        .minmax()
        .into_option()
        .ok_or_else(|| Error::msg("No minimum or maximum found"))?;
    let mut result: Option<(i32, i64)> = None;
    for x in *min..=*max {
        let sum = input.iter().map(|num| map_fn(*num, x)).sum::<i64>();
        match result {
            None => result = Some((x, sum)),
            Some((_, res)) => {
                if res > sum {
                    result = Some((x, sum));
                }
            }
        }
    }
    result.ok_or_else(|| Error::msg("No result found"))
}

fn calc(n: i64) -> i64 {
    match n {
        0 => 0,
        n if n < 0 => panic!("{n} is negative!"),
        _ => (1..=n).sum(),
    }
}

fn parse(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(complete::char(','), complete::i32)(input)
}

fn read_input() -> Result<Vec<i32>> {
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
        let input = read_input()?;

        let (position, result) = part_one(input)?;

        assert_eq!(354, position);
        assert_eq!(349812, result);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = read_input()?;

        let (position, result) = part_two(input)?;

        assert_eq!(488, position);
        assert_eq!(99763899, result);

        Ok(())
    }
}
