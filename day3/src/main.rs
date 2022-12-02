use anyhow::{Error, Result};
use nom::{
    branch::alt,
    character::complete::{self, line_ending},
    multi::{count, separated_list1},
    IResult,
};
use std::{cmp::Ordering, collections::HashMap, fs, io::Read};

fn main() -> Result<()> {
    let input = read_input()?;

    let (took, result) = took::took(|| part_one(&input));
    let (gamma, epsilon) = result?;
    println!("Gamma: {gamma}");
    println!("Epsilon: {epsilon}");
    println!("Result part one: {}", gamma * epsilon);
    println!("Time spent: {took}");

    let (took, result) = took::took(|| part_two(input));
    let (oxygen, co2) = result?;
    println!("Oxygen: {oxygen}");
    println!("CO2: {co2}");
    println!("Result part two: {}", oxygen * co2);
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &[Vec<char>]) -> Result<(isize, isize)> {
    let gamma = get_row_as_decimal(input, get_most_common)?;
    let epsilon = get_row_as_decimal(input, get_least_common)?;

    Ok((gamma, epsilon))
}

fn part_two(input: Vec<Vec<char>>) -> Result<(isize, isize)> {
    let oxygen = get_single_row(input.clone(), get_most_common)?;
    let co2 = get_single_row(input, get_least_common)?;

    Ok((oxygen, co2))
}

fn get_row_as_decimal(
    input: &[Vec<char>],
    function: fn(&[Vec<char>], usize) -> Result<char>,
) -> Result<isize> {
    let output = (0..input[0].len())
        .into_iter()
        .map(|x| function(input, x))
        .try_fold(String::new(), |acc, c| {
            Ok::<String, Error>(acc + c?.to_string().as_str())
        })?;
    isize::from_str_radix(&output, 2).map_err(Error::from)
}

// TODO ownership of input
fn get_single_row(
    input: Vec<Vec<char>>,
    function: fn(&[Vec<char>], usize) -> Result<char>,
) -> Result<isize> {
    let mut input = input;
    for column in 0..input[0].len() {
        let filter_value = function(&input, column)?;
        input.retain(|x| x[column] == filter_value);
        if input.len() == 1 {
            let input = input[0]
                .iter()
                .fold(String::new(), |acc, c| acc + &c.to_string());
            return isize::from_str_radix(&input, 2).map_err(Error::from);
        }
    }

    unreachable!()
}

fn get_least_common(input: &[Vec<char>], column: usize) -> Result<char> {
    get_common(input, column, '0', |a, b| b.1.cmp(a.1))
}

fn get_most_common(input: &[Vec<char>], column: usize) -> Result<char> {
    get_common(input, column, '1', |a, b| a.1.cmp(b.1))
}

fn get_common(
    input: &[Vec<char>],
    column: usize,
    default: char,
    max_fn: fn(&(&char, &usize), &(&char, &usize)) -> Ordering,
) -> Result<char> {
    let map = get_row_values(input, column);
    if map.get(&'0').unwrap_or(&0) == map.get(&'1').unwrap_or(&0) {
        return Ok(default);
    }

    map.iter()
        .max_by(max_fn)
        .ok_or_else(|| Error::msg("No maximum found"))
        .map(|(c, _)| *c)
}

fn get_row_values(input: &[Vec<char>], column: usize) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for c in input.iter().map(|row| row[column]) {
        *map.entry(c).or_insert(0) += 1
    }

    map
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<char>> {
    count(parse_binary_digit, 12)(input)
}

fn parse_binary_digit(input: &str) -> IResult<&str, char> {
    alt((complete::char('0'), complete::char('1')))(input)
}

fn read_input() -> Result<Vec<Vec<char>>> {
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

        let (gamma, epsilon) = part_one(&input)?;

        assert_eq!(1491, gamma);
        assert_eq!(2604, epsilon);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = read_input()?;

        let (oxygen, co2) = part_two(input)?;

        assert_eq!(1305, oxygen);
        assert_eq!(2594, co2);

        Ok(())
    }
}
