use anyhow::Result;
use nom::{
    character::complete::{digit1, line_ending},
    combinator::map,
    multi::separated_list1,
    IResult,
};
use std::{fs, io::Read, time::Instant};

fn main() -> Result<()> {
    let input = read_input()?;

    let start = Instant::now();
    let count = part_one(&input);
    println!("Count part one: {}", count);

    let lap = Instant::now();
    println!(
        "Time spent: {} microseconds",
        lap.duration_since(start).as_micros()
    );

    let count = part_two(&input);
    println!("Count part two: {}", count);

    let end = Instant::now();
    println!(
        "Time spent: {} microseconds",
        end.duration_since(lap).as_micros()
    );

    Ok(())
}

fn part_one(input: &[u16]) -> usize {
    input.windows(2).filter(|s| s[1] > s[0]).count()
}

fn part_two(input: &[u16]) -> usize {
    input
        .windows(4)
        .filter(|s| {
            let first = s[0] + s[1] + s[2];
            let second = s[1] + s[2] + s[3];
            second > first
        })
        .count()
}

fn parse(input: &str) -> IResult<&str, Vec<u16>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, u16> {
    map(digit1, |num: &str| num.parse::<u16>().unwrap())(input)
}

fn read_input() -> Result<Vec<u16>> {
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

        let count = part_one(&input);

        assert_eq!(1681, count);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = read_input()?;

        let count = part_two(&input);

        assert_eq!(1704, count);

        Ok(())
    }
}
