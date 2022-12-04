use std::{cmp::Reverse, collections::HashMap, fs, io::Read};

use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space1},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use once_cell::sync::Lazy;

static BYTEMAP: Lazy<HashMap<char, u8>> = Lazy::new(|| {
    HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 4),
        ('d', 8),
        ('e', 16),
        ('f', 32),
        ('g', 64),
    ])
});

fn main() -> Result<()> {
    let mut input = read_input()?;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| part_two(&mut input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &[(Groups, Groups)]) -> usize {
    input
        .iter()
        .flat_map(|(_, o)| &o.groups)
        .filter(|o| o.len() == 2 || o.len() == 3 || o.len() == 4 || o.len() == 7)
        .count()
}

fn part_two(input: &mut [(Groups, Groups)]) -> i32 {
    input
        .iter_mut()
        .map(|(s, o)| calculate_output_value(s, o))
        .sum::<i32>()
}

fn calculate_output_value(signals: &mut Groups, outputs: &mut Groups) -> i32 {
    signals.sort_groups();

    let code_mapping = determine_code_mapping(signals);

    outputs.sort_groups();

    let mut buf = String::new();
    for output in &outputs.groups {
        buf.push_str(
            code_mapping
                .get(output.group.as_str())
                .unwrap()
                .to_string()
                .as_str(),
        );
    }

    buf.parse::<i32>().unwrap()
}

fn determine_code_mapping(signals: &mut Groups) -> HashMap<&str, u8> {
    let mut code_map: HashMap<&str, u8> = HashMap::new();
    let mut length_map: HashMap<u8, GroupAsBits> = signals
        .groups
        .iter()
        .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
        .map(|signal| (signal.len() as u8, signal.as_bits()))
        .collect();
    // sort groups by reverse length so digit 9 comes before length 5
    signals.groups.sort_by_key(|s| Reverse(s.len()));
    for signal in &signals.groups {
        let digit = match signal.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            5 => {
                let signal_as_bits = signal.as_bits();
                if signal_as_bits.contains(length_map.get(&2).unwrap()) {
                    3
                } else if length_map.get(&6).unwrap().contains(&signal_as_bits) {
                    5
                } else {
                    2
                }
            }
            6 => {
                let signal_as_bits = signal.as_bits();
                if signal_as_bits.contains(length_map.get(&4).unwrap()) {
                    length_map.insert(6, signal_as_bits);
                    9
                } else if signal_as_bits.contains(length_map.get(&2).unwrap()) {
                    0
                } else {
                    6
                }
            }
            7 => 8,
            _ => unreachable!(),
        };
        code_map.insert(&signal.group, digit);
    }

    code_map
}

struct Groups {
    groups: Vec<Group>,
}

impl Groups {
    pub fn new(groups: Vec<Group>) -> Self {
        Groups { groups }
    }

    pub fn sort_groups(&mut self) {
        self.groups.iter_mut().for_each(|group| group.sort());
    }
}

struct GroupAsBits(u8);

impl GroupAsBits {
    fn contains(&self, other: &Self) -> bool {
        self.0 & other.0 == other.0
    }
}

struct Group {
    group: String,
}

impl Group {
    pub fn new(group: &str) -> Self {
        let group = group.to_string();
        Group { group }
    }

    pub fn len(&self) -> usize {
        self.group.len()
    }

    pub fn as_bits(&self) -> GroupAsBits {
        GroupAsBits(self.group.chars().map(|c| BYTEMAP.get(&c).unwrap()).sum())
    }

    pub fn sort(&mut self) {
        let mut vec = self.group.chars().collect::<Vec<char>>();
        vec.sort_unstable();
        self.group = vec.into_iter().collect::<String>()
    }
}

fn parse(input: &str) -> IResult<&str, Vec<(Groups, Groups)>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, (Groups, Groups)> {
    separated_pair(parse_groups, tag(" | "), parse_groups)(input)
}

fn parse_groups(input: &str) -> IResult<&str, Groups> {
    map(separated_list1(space1, parse_group), Groups::new)(input)
}

fn parse_group(input: &str) -> IResult<&str, Group> {
    map(alpha1, Group::new)(input)
}

fn read_input() -> Result<Vec<(Groups, Groups)>> {
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

        assert_eq!(473, count);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let mut input = read_input()?;

        let count = part_two(&mut input);

        assert_eq!(1097568, count);

        Ok(())
    }
}
