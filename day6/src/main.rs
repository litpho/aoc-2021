use anyhow::Result;
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map, multi::separated_list1,
    IResult,
};
use std::{collections::HashMap, fs, io::Read, str::FromStr, time::Instant};

fn main() -> Result<()> {
    let start = Instant::now();
    let input = read_input()?;
    let count = part_one(input);
    println!("Part one count: {}", count);

    let lap = Instant::now();
    println!(
        "Time spent: {} microseconds",
        lap.duration_since(start).as_micros()
    );

    let input = read_input()?;
    let count = part_two(input);
    println!("Part two count: {}", count);

    let end = Instant::now();
    println!(
        "Time spent: {} microseconds",
        end.duration_since(lap).as_micros()
    );

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
            let number = *input.get(&fish).or(Some(&0)).unwrap();
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
    map(separated_list1(tag(","), digit1), |numbers| {
        numbers
            .into_iter()
            .map(|num| i8::from_str(num).unwrap())
            .collect::<Vec<i8>>()
    })(input)
}

fn read_input() -> Result<HashMap<i8, i64>> {
    let mut buf = String::new();
    fs::File::open("src/input.txt")?.read_to_string(&mut buf)?;

    let (_, input) = parse(&buf).ok().unwrap();
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
        let input = read_input()?;

        let count = part_one(input);

        assert_eq!(362639, count);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = read_input()?;

        let count = part_two(input);

        assert_eq!(1639854996917, count);

        Ok(())
    }
}
