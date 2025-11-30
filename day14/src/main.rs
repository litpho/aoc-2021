use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::map,
    multi::{count, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};
use std::{collections::HashMap, fs, io::Read};

fn main() -> Result<()> {
    let input = read_input()?;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| part_two(&input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &Input) -> i64 {
    execute(input, 10)
}

fn part_two(input: &Input) -> i64 {
    execute(input, 40)
}

fn execute(input: &Input, amount: usize) -> i64 {
    let combinations = determine_possible_combinations(&input.rules);
    let initialized_input = initialize_input(&input.template);
    let result = (0..amount).fold(initialized_input, |s, _| step(&s, &combinations));

    let (least, most) = least_most(&input.template, &result);

    most - least
}

fn determine_possible_combinations(
    rules: &HashMap<(char, char), char>,
) -> HashMap<(char, char), Vec<(char, char)>> {
    rules
        .iter()
        .map(|(k, v)| (*k, vec![(k.0, *v), (*v, k.1)]))
        .collect::<HashMap<(char, char), Vec<(char, char)>>>()
}

fn initialize_input(template: &str) -> HashMap<(char, char), i64> {
    let mut input: HashMap<(char, char), i64> = HashMap::new();
    template
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .map(|a| (*a.first().unwrap(), *a.get(1).unwrap()))
        .for_each(|c| {
            *input.entry(c).or_insert(0) += 1;
        });
    input
}

fn step(
    input: &HashMap<(char, char), i64>,
    combinations: &HashMap<(char, char), Vec<(char, char)>>,
) -> HashMap<(char, char), i64> {
    let mut map = HashMap::new();
    input
        .iter()
        .filter(|(_, value)| value > &&0)
        .for_each(|(key, value)| {
            combinations.get(key).unwrap().iter().for_each(|c| {
                *map.entry(*c).or_insert(0) += value;
            })
        });

    map
}

fn least_most(template: &str, input: &HashMap<(char, char), i64>) -> (i64, i64) {
    let mut map: HashMap<char, i64> = HashMap::new();
    // count the first char of the template
    map.insert(*template.chars().collect::<Vec<char>>().first().unwrap(), 1);

    // count right-hand chars
    input
        .iter()
        .for_each(|((_, b), c)| *map.entry(*b).or_insert(0) += c);

    // sort the map values and return the least and most
    let mut result = map.values().copied().collect::<Vec<i64>>();
    result.sort_unstable();

    (*result.first().unwrap(), *result.last().unwrap())
}

struct Input {
    template: String,
    rules: HashMap<(char, char), char>,
}

fn parse(input: &str) -> IResult<&str, Input> {
    map(
        separated_pair(parse_template, count(line_ending, 2), parse_insertion_rules),
        |(template, rules)| Input { template, rules },
    )
    .parse(input)
}

fn parse_template(input: &str) -> IResult<&str, String> {
    map(alpha1, |s: &str| s.to_string()).parse(input)
}

fn parse_insertion_rules(input: &str) -> IResult<&str, HashMap<(char, char), char>> {
    map(
        separated_list1(line_ending, parse_insertion_rule),
        |rules| {
            rules
                .iter()
                .map(|(from, to)| {
                    let from = from.chars().collect::<Vec<char>>();
                    let key = (*from.first().unwrap(), *from.get(1).unwrap());
                    let value = *to.chars().collect::<Vec<char>>().first().unwrap();
                    (key, value)
                })
                .collect::<HashMap<(char, char), char>>()
        },
    )
    .parse(input)
}

fn parse_insertion_rule(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alpha1, tag(" -> "), alpha1).parse(input)
}

fn read_input() -> Result<Input> {
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

        let count = part_one(&input);

        assert_eq!(2010, count);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = read_input()?;

        let count = part_two(&input);

        assert_eq!(2437698971143, count);

        Ok(())
    }
}
