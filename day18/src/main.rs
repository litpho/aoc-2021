use anyhow::Result;
use nom::{
    branch::alt,
    character::complete::{self, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult, Parser,
};
use std::{
    cell::RefCell,
    fmt::{Display, Formatter},
    fs,
    io::Read,
};

fn main() -> Result<()> {
    let input = read_input()?;

    let (took, result) = took::took(|| part_one(input.clone()));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| part_two(input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: Vec<Vec<Number>>) -> i32 {
    let grid = input.into_iter().fold(Grid::new(), |grid, v| grid.add(&v));

    let output = grid
        .content
        .borrow()
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("Output: {output}");

    grid.magnitude()
}

fn part_two(input: Vec<Vec<Number>>) -> i32 {
    let mut result = 0;
    for (i, a) in input.iter().enumerate() {
        for (_, b) in input.iter().enumerate().filter(|(j, _)| i != *j) {
            let mut grid = Grid::new();
            grid = grid.add(a);
            grid = grid.add(b);
            let magnitude = grid.magnitude();
            if magnitude > result {
                result = magnitude;
            }
        }
    }

    result
}

struct Grid {
    content: RefCell<Vec<Number>>,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            content: RefCell::new(vec![]),
        }
    }

    pub fn add(self, other: &[Number]) -> Self {
        if self.content.borrow().is_empty() {
            self.content.replace(other.to_owned());
        } else {
            let mut content = self.content.borrow().to_owned();
            let mut other = other.to_owned();
            content.iter_mut().for_each(|pair| pair.depth += 1);
            other.iter_mut().for_each(|pair| pair.depth += 1);
            content.append(&mut other);

            self.content.replace(content);

            while self.explode() || self.split() {}
        }

        self
    }

    pub fn explode(&self) -> bool {
        let mut result: Vec<Number> = Vec::new();
        let input = self.content.borrow().clone();
        for (i, window) in input.iter().as_slice().windows(2).enumerate() {
            let first = window.first().unwrap();
            let second = window.get(1).unwrap();
            if first.depth >= 4 && second.depth >= 4 && first.depth == second.depth {
                if i > 0 {
                    if i > 1 {
                        result.append(&mut input[..i - 1].to_vec());
                    }
                    let prev = &input[i - 1];
                    result.push(Number::new(prev.value + first.value, prev.depth));
                }
                result.push(Number::new(0, first.depth - 1));
                if i < input.len() - 2 {
                    let next = &input[i + 2];
                    result.push(Number::new(next.value + second.value, next.depth));
                }
                if input.len() > 3 && i < input.len() - 3 {
                    result.append(&mut input[i + 3..].to_vec());
                }

                self.content.replace(result);

                return true;
            }
        }

        false
    }

    pub fn split(&self) -> bool {
        let mut result: Vec<Number> = Vec::new();
        let input = self.content.borrow().to_owned();
        match input.iter().enumerate().find(|(_, n)| n.value >= 10) {
            Some((i, item)) => {
                if i > 0 {
                    result.append(&mut input[..i].to_vec());
                }
                let first = item.value / 2;
                result.push(Number::new(first, item.depth + 1));
                result.push(Number::new(item.value - first, item.depth + 1));
                if i < input.len() {
                    result.append(&mut input[i + 1..].to_vec());
                }

                self.content.replace(result);

                true
            }
            None => false,
        }
    }

    pub fn magnitude(&self) -> i32 {
        while self.derive_magnitude() {}

        self.content.borrow().first().unwrap().value as i32
    }

    fn derive_magnitude(&self) -> bool {
        let mut result: Vec<Number> = Vec::new();
        let input = self.content.borrow().clone();
        for (i, window) in input.as_slice().windows(2).enumerate() {
            let first = window.first().unwrap();
            let second = window.get(1).unwrap();
            if first.depth == second.depth {
                result.append(&mut input.as_slice()[..i].to_vec());
                result.push(Number::new(
                    3 * first.value + 2 * second.value,
                    first.depth - 1,
                ));
                if input.len() > i + 2 {
                    result.append(&mut input.as_slice()[i + 2..].to_vec());
                }

                self.content.replace(result);

                return true;
            }
        }

        false
    }
}

#[derive(Clone, Debug)]
struct Number {
    value: i16,
    depth: isize,
}

impl Number {
    pub fn new(value: i16, depth: isize) -> Self {
        Number { value, depth }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.value, self.depth)
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Number>>> {
    separated_list1(line_ending, parse_tuple(0)).parse(input)
}

fn parse_tuple(depth: isize) -> impl Fn(&str) -> IResult<&str, Vec<Number>> {
    move |input: &str| {
        map(
            delimited(
                complete::char('['),
                separated_pair(parse_value(depth), complete::char(','), parse_value(depth)),
                complete::char(']'),
            ),
            |(x, y)| [x, y].concat(),
        )
        .parse(input)
    }
}

fn parse_value(depth: isize) -> impl Fn(&str) -> IResult<&str, Vec<Number>> {
    move |input: &str| {
        alt((
            parse_tuple(depth + 1),
            map(complete::i16, |value| vec![Number::new(value, depth)]),
        ))
        .parse(input)
    }
}

fn read_input() -> Result<Vec<Vec<Number>>> {
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

        let result = part_one(input);

        assert_eq!(4176, result);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = read_input()?;

        let result = part_two(input);

        assert_eq!(4633, result);

        Ok(())
    }
}
