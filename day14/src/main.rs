use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::map,
    multi::count,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::{collections::HashMap, fs, io::Read};

fn main() -> Result<()> {
    let (template, rules) = read_input()?;

    let (took, result) = took::took(|| part_one(template.clone(), &rules));
    println!("Result part one: {}", result);
    println!("Time spent: {}", took);

    let (took, result) = took::took(|| part_two(template, &rules));
    println!("Result part two: {}", result);
    println!("Time spent: {}", took);

    Ok(())
}

fn part_one(template: String, rules: &HashMap<(char, char), char>) -> i64 {
    execute(template, rules, 10)
}

fn part_two(template: String, rules: &HashMap<(char, char), char>) -> i64 {
    execute(template, rules, 40)
}

fn execute(template: String, rules: &HashMap<(char, char), char>, amount: usize) -> i64 {
    let combinations = determine_possible_combinations(rules);
    let input = initialize_input(&template);
    let result = (0..amount).fold(input, |s, _| step(&s, &combinations));

    let (least, most) = least_most(&template, &result);

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

fn parse(input: &str) -> IResult<&str, (String, HashMap<(char, char), char>)> {
    separated_pair(parse_template, count(line_ending, 2), parse_insertion_rules)(input)
}

fn parse_template(input: &str) -> IResult<&str, String> {
    map(alpha1, |s: &str| s.to_string())(input)
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
    )(input)
}

fn parse_insertion_rule(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alpha1, tag(" -> "), alpha1)(input)
}

fn read_input() -> Result<(String, HashMap<(char, char), char>)> {
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
        let (template, rules) = read_input()?;

        let count = part_one(template, &rules);

        assert_eq!(2010, count);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let (template, rules) = read_input()?;

        let count = part_two(template, &rules);

        assert_eq!(2437698971143, count);

        Ok(())
    }
}
