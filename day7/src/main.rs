use anyhow::Result;
use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map, multi::separated_list1,
    IResult,
};
use std::{fs, io::Read};

fn main() -> Result<()> {
    let input = read_input()?;

    let (took, (position, fuel_spent)) = took::took(|| part_one(input.clone()));
    println!("Result part one: {} - {}", position, fuel_spent);
    println!("Time spent: {}", took);

    let (took, (position, fuel_spent)) = took::took(|| part_two(input));
    println!("Result part two: {} - {}", position, fuel_spent);
    println!("Time spent: {}", took);

    Ok(())
}

fn part_one(input: Vec<i32>) -> (i32, i64) {
    algorithm(input, |num, x| (num - x).abs() as i64)
}

fn part_two(input: Vec<i32>) -> (i32, i64) {
    algorithm(input, |num, x| calc((num - x).abs() as i64))
}

fn algorithm(input: Vec<i32>, map_fn: fn(i32, i32) -> i64) -> (i32, i64) {
    let (min, max) = input.iter().minmax().into_option().unwrap();
    let mut result: Option<(i32, i64)> = None;
    for x in *min..=*max {
        let sum = input.iter().map(|num| map_fn(*num, x)).sum::<i64>();
        if result == None {
            result = Some((x, sum));
        } else {
            let (_, res) = result.unwrap();
            if res > sum {
                result = Some((x, sum));
            }
        }
    }
    result.unwrap()
}

fn calc(n: i64) -> i64 {
    match n {
        0 => 0,
        n if n < 0 => panic!("{} is negative!", n),
        _ => (1..=n).sum(),
    }
}

fn parse(input: &str) -> IResult<&str, Vec<i32>> {
    map(separated_list1(tag(","), digit1), |numbers| {
        numbers
            .into_iter()
            .map(|num: &str| num.parse().unwrap())
            .collect::<Vec<i32>>()
    })(input)
}

fn read_input() -> Result<Vec<i32>> {
    let mut buf = String::new();
    fs::File::open("src/input.txt")?.read_to_string(&mut buf)?;

    let (_, input) = parse(&buf).ok().unwrap();

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let input = read_input()?;

        let (position, result) = part_one(input);

        assert_eq!(354, position);
        assert_eq!(349812, result);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = read_input()?;

        let (position, result) = part_two(input);

        assert_eq!(488, position);
        assert_eq!(99763899, result);

        Ok(())
    }
}
