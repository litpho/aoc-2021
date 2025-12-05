use anyhow::Result;
use nom::{
    bytes::complete::take_while1, character::complete::line_ending, combinator::map,
    multi::separated_list1, AsChar, IResult, Parser,
};
use std::cmp::Reverse;

const DATA: &[u8] = include_bytes!("input.txt");

fn main() -> Result<()> {
    let input = read_input(DATA)?;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| part_two(&input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &[Vec<u8>]) -> u16 {
    let x_dim = input[0].len();
    let y_dim = input.len();

    x_y_height_iterator(input)
        .filter_map(|(x, y, height)| {
            let heights = build_height_map(input, x, y, x_dim - 1, y_dim - 1);
            if heights.iter().min().unwrap() == &height
                && heights.iter().filter(|h| h == &&height).count() == 1
            {
                Some(1 + *height as u16)
            } else {
                None
            }
        })
        .sum::<u16>()
}

fn part_two(input: &[Vec<u8>]) -> usize {
    let max_x = input[0].len() - 1;
    let max_y = input.len() - 1;

    let mut basins = x_y_height_iterator(input)
        .filter_map(|(x, y, height)| {
            let heights = build_height_map(input, x, y, max_x, max_y);
            if heights.iter().min().unwrap() == &height
                && heights.iter().filter(|h| h == &&height).count() == 1
            {
                Some((x, y, height))
            } else {
                None
            }
        })
        .map(|(x, y, _)| calculate_basin(x, y, max_x, max_y, input))
        .collect::<Vec<usize>>();

    basins.sort_by_key(|o| Reverse(*o));
    basins.iter().take(3).product()
}

fn calculate_basin(x: usize, y: usize, max_x: usize, max_y: usize, input: &[Vec<u8>]) -> usize {
    let mut neighbours = calculate_neighbours(x, y, max_x, max_y);
    let mut count = 0;
    let mut result = 0;
    while count < neighbours.len() {
        let (x, y) = neighbours.get(count).unwrap();
        let height = input.get(*y).unwrap().get(*x).unwrap();
        if height != &9 {
            let mut new_neighbours = calculate_neighbours(*x, *y, max_x, max_y)
                .into_iter()
                .filter(|x_y| !neighbours.contains(x_y))
                .collect::<Vec<(usize, usize)>>();
            neighbours.append(&mut new_neighbours);
            result += 1;
        }
        count += 1;
    }

    result
}

fn x_y_height_iterator(input: &[Vec<u8>]) -> impl Iterator<Item = (usize, usize, &u8)> {
    input.iter().enumerate().flat_map(|(y, row)| {
        row.iter()
            .enumerate()
            .map(move |(x, height)| (x, y, height))
    })
}

fn build_height_map(input: &[Vec<u8>], x: usize, y: usize, max_x: usize, max_y: usize) -> Vec<&u8> {
    let mut height_map: Vec<&u8> = vec![input.get(y).unwrap().get(x).unwrap()];
    calculate_neighbours(x, y, max_x, max_y)
        .iter()
        .for_each(|(x, y)| height_map.push(input.get(*y).unwrap().get(*x).unwrap()));
    height_map
}

fn calculate_neighbours(x: usize, y: usize, max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    let mut neighbours: Vec<(usize, usize)> = Vec::new();
    if y > 0 {
        neighbours.push((x, y - 1));
    }
    if x > 0 {
        neighbours.push((x - 1, y));
    }
    if x < max_x {
        neighbours.push((x + 1, y));
    }
    if y < max_y {
        neighbours.push((x, y + 1));
    }
    neighbours
}

fn parse(input: &[u8]) -> IResult<&[u8], Vec<Vec<u8>>> {
    separated_list1(line_ending, parse_line).parse(input)
}

fn parse_line(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    map(take_while1(|c: u8| c.is_dec_digit()), |line: &[u8]| {
        line.iter().map(|b| b - 48).collect::<Vec<u8>>()
    })
    .parse(input)
}

fn read_input(data: &'static [u8]) -> Result<Vec<Vec<u8>>> {
    let (_, input) = parse(data)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &[u8] = include_bytes!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        let input = read_input(TESTDATA)?;

        let count = part_one(&input);

        assert_eq!(count, 15);

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let input = read_input(DATA)?;

        let count = part_one(&input);

        assert_eq!(count, 541);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = read_input(DATA)?;

        let count = part_two(&input);

        assert_eq!(count, 847504);

        Ok(())
    }
}
