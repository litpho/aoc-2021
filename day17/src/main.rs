use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete,
    combinator::{map, opt},
    sequence::{pair, separated_pair},
    IResult, Parser,
};
use std::{fs, io::Read};

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

fn part_one(input: &TargetArea) -> i32 {
    get_hit_velocities_and_heights(input)
        .into_iter()
        .max_by_key(|(_, _, height)| *height)
        .unwrap()
        .2
}

fn part_two(input: &TargetArea) -> usize {
    get_hit_velocities_and_heights(input).len()
}

fn get_hit_velocities_and_heights(input: &TargetArea) -> Vec<(i32, i32, i32)> {
    (-500..500)
        .flat_map(|x| (-500..500).map(move |y| (x, y)))
        .filter_map(|(x, y)| input.height_and_then_hit(x, y))
        .collect::<Vec<(i32, i32, i32)>>()
}

#[derive(Debug)]
struct TargetArea {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl TargetArea {
    pub fn new(x_range: (i32, i32), y_range: (i32, i32)) -> Self {
        let min_x = x_range.0;
        let max_x = x_range.1;
        let min_y = y_range.0;
        let max_y = y_range.1;

        TargetArea {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    pub fn height_and_then_hit(
        &self,
        starting_x_vel: i32,
        starting_y_vel: i32,
    ) -> Option<(i32, i32, i32)> {
        let mut x = 0;
        let mut y = 0;
        let mut x_vel = starting_x_vel;
        let mut y_vel = starting_y_vel;
        let mut highest_y = 0;
        loop {
            x += x_vel;
            y += y_vel;
            if y > highest_y {
                highest_y = y;
            }

            if self.min_x <= x && x <= self.max_x && self.min_y <= y && y <= self.max_y {
                return Some((starting_x_vel, starting_y_vel, highest_y));
            }

            x_vel += match x_vel {
                vel if vel > 0 => -1,
                vel if vel < 0 => 1,
                _ => 0,
            };
            y_vel -= 1;

            if x > self.max_x || y < self.min_y {
                return None;
            }
        }
    }
}

fn parse(input: &str) -> IResult<&str, TargetArea> {
    map(
        (
            tag("target area: x="),
            parse_range,
            tag(", y="),
            parse_range,
        ),
        |(_, x_range, _, y_range)| TargetArea::new(x_range, y_range),
    )
    .parse(input)
}

fn parse_range(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(parse_number, tag(".."), parse_number).parse(input)
}

fn parse_number(input: &str) -> IResult<&str, i32> {
    map(
        pair(opt(complete::char('-')), complete::i32),
        |(negative, number)| {
            let multiplier = if negative.is_some() { -1 } else { 1 };
            multiplier * number
        },
    )
    .parse(input)
}

fn read_input() -> Result<TargetArea> {
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

        let result = part_one(&input);

        assert_eq!(6903, result);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = read_input()?;

        let result = part_two(&input);

        assert_eq!(2351, result);

        Ok(())
    }
}
