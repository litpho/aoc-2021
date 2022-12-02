use anyhow::{Error, Result};
use nom::{
    character::complete::line_ending, character::complete::one_of, combinator::map, multi::many1,
    multi::separated_list1, IResult,
};
use std::{fs, io::Read};

fn main() -> Result<()> {
    let input = read_input()?;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {}", result);
    println!("Time spent: {}", took);

    let (took, result) = took::took(|| part_two(&input));
    println!("Result part two: {}", result?);
    println!("Time spent: {}", took);

    Ok(())
}

fn part_one(input: &[Vec<Token>]) -> i32 {
    input
        .iter()
        .filter_map(|s| check_line(s))
        .map(|c| c.points1())
        .sum::<i32>()
}

fn part_two(input: &[Vec<Token>]) -> Result<i64> {
    let mut results = input
        .iter()
        .filter_map(|s| line_to_score(s))
        .collect::<Vec<i64>>();

    results.sort_unstable();

    results
        .get(results.len() / 2)
        .cloned()
        .ok_or_else(|| Error::msg("Result didn't exist"))
}

fn check_line(line: &[Token]) -> Option<Token> {
    let mut stack: Vec<Token> = Vec::new();
    get_failing_token(line, &mut stack)
}

fn line_to_score(line: &[Token]) -> Option<i64> {
    let mut stack: Vec<Token> = Vec::new();
    if get_failing_token(line, &mut stack).is_some() {
        return None;
    };

    let score = stack
        .into_iter()
        .map(|c| c.opposite())
        .map(|c| c.points2())
        .rfold(0, |acc, c| acc * 5 + c);

    Some(score)
}

fn get_failing_token(line: &[Token], stack: &mut Vec<Token>) -> Option<Token> {
    for t in line {
        match t {
            t if t.is_open() => stack.push(*t),
            _ => {
                let pop = stack.remove(stack.len() - 1);
                if t != &pop.opposite() {
                    return Some(*t);
                }
            }
        }
    }

    None
}

#[derive(Clone, Copy, PartialEq)]
struct Token(char);

impl Token {
    pub fn new(c: char) -> Self {
        match c {
            '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' => Token(c),
            _ => panic!("Character '{}' is not supported", c),
        }
    }

    pub fn opposite(&self) -> Self {
        match self.0 {
            '(' => Token(')'),
            ')' => Token('('),
            '[' => Token(']'),
            ']' => Token('['),
            '{' => Token('}'),
            '}' => Token('{'),
            '<' => Token('>'),
            '>' => Token('<'),
            _ => unreachable!(),
        }
    }

    pub fn is_open(&self) -> bool {
        ['(', '[', '{', '<'].contains(&self.0)
    }

    pub fn points1(&self) -> i32 {
        match self.0 {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        }
    }

    pub fn points2(&self) -> i64 {
        match self.0 {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => unreachable!(),
        }
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Token>>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Token>> {
    map(many1(one_of("[]{}()<>")), |v: Vec<char>| {
        v.iter().map(|c| Token::new(*c)).collect()
    })(input)
}

fn read_input() -> Result<Vec<Vec<Token>>> {
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

        assert_eq!(339477, count);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let mut input = read_input()?;

        let count = part_two(&mut input)?;

        assert_eq!(3049320156, count);

        Ok(())
    }
}
