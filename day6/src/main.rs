use anyhow::Result;
use nom::{character::complete, multi::separated_list1, IResult, Parser};
use std::collections::HashMap;

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let input = read_input(DATA)?;

    let (took, result) = took::took(|| part_one(input.clone()));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| part_two(input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: HashMap<i8, i64>) -> i64 {
    algorithm(input, 80)
}

fn part_two(input: HashMap<i8, i64>) -> i64 {
    algorithm(input, 256)
}

fn algorithm(mut input: HashMap<i8, i64>, days: i16) -> i64 {
    for _ in 0..days {
        let mut new_borns = 0;
        let mut reset_fish = 0;
        for fish in 0..=8 {
            let number = *input.get(&fish).unwrap_or(&0);
            if fish == 0 {
                new_borns = number;
                reset_fish = number;
            } else {
                input.insert(fish - 1, number);
            }
        }
        input.entry(6).and_modify(|x| *x += reset_fish);
        input.insert(8, new_borns);
    }

    input.values().sum()
}

fn parse(input: &str) -> IResult<&str, Vec<i8>> {
    separated_list1(complete::char(','), complete::i8).parse(input)
}

fn read_input(data: &str) -> Result<HashMap<i8, i64>> {
    let (_, input) = parse(data).expect("Parse failure");
    let mut map: HashMap<i8, i64> = HashMap::new();
    for fish in input {
        *map.entry(fish).or_insert(0) += 1;
    }

    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<()> {
        let input = read_input(DATA)?;

        let count = part_one(input);

        assert_eq!(count, 362639);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = read_input(DATA)?;

        let count = part_two(input);

        assert_eq!(count, 1639854996917);

        Ok(())
    }
}
