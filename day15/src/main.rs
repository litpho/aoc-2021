use anyhow::Result;
use nom::{
    bytes::complete::take_while1,
    character::complete::line_ending,
    combinator::{map, map_res},
    multi::separated_list1,
    AsChar, IResult, Parser,
};
use pathfinding::prelude::dijkstra;

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_input(DATA));
    println!("Time spent parsing: {}", took);
    let input = result?;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| part_two(&input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(grid: &Grid) -> usize {
    traverse(grid)
}

fn part_two(grid: &Grid) -> usize {
    let grid = grid.multiply_by_5();

    traverse(&grid)
}

fn traverse(grid: &Grid) -> usize {
    let start = (0, 0);
    let goal = (grid.max_x, grid.max_y);

    let (_, cost) = dijkstra(&start, |(x, y)| next_steps(grid, x, y), |p| *p == goal).unwrap();

    (cost - grid.cell(0, 0).2 + grid.cell(grid.max_x, grid.max_y).2) as usize
}

fn next_steps(grid: &Grid, x: &usize, y: &usize) -> Vec<((usize, usize), i32)> {
    let width = grid.max_x;
    let height = grid.max_y;

    let mut next_steps = vec![];

    // up
    calculate_step(&(*x, *y), &mut next_steps, grid, *y > 0, |s| s, |s| s - 1);
    // down
    calculate_step(
        &(*x, *y),
        &mut next_steps,
        grid,
        *y < height,
        |s| s,
        |s| s + 1,
    );
    // left
    calculate_step(&(*x, *y), &mut next_steps, grid, *x > 0, |s| s - 1, |s| s);
    // right
    calculate_step(
        &(*x, *y),
        &mut next_steps,
        grid,
        *x < width,
        |s| s + 1,
        |s| s,
    );

    next_steps
}

fn calculate_step<G, H>(
    coord: &(usize, usize),
    next_steps: &mut Vec<((usize, usize), i32)>,
    grid: &Grid,
    condition: bool,
    x: G,
    y: H,
) where
    G: Fn(usize) -> usize,
    H: Fn(usize) -> usize,
{
    if condition {
        let new_coord = (x(coord.0), y(coord.1));
        next_steps.push((new_coord, grid.cell(coord.0, coord.1).2));
    }
}

struct Grid {
    content: Vec<Vec<i32>>,
    max_x: usize,
    max_y: usize,
}

impl Grid {
    pub fn new(content: Vec<Vec<i32>>) -> Self {
        let max_x = content.first().unwrap().len() - 1;
        let max_y = content.len() - 1;
        Grid {
            content,
            max_x,
            max_y,
        }
    }

    pub fn cell(&self, x: usize, y: usize) -> (usize, usize, i32) {
        (x, y, *self.content.get(y).unwrap().get(x).unwrap())
    }

    pub fn multiply_by_5(&self) -> Self {
        let mut result: Vec<Vec<i32>> = Vec::new();

        for y in 0..5 {
            for row in &self.content {
                let mut vec: Vec<i32> = Vec::new();
                for x in 0..5 {
                    for col in row {
                        let mut val = x + y + col;
                        if val > 9 {
                            val -= 9;
                        }
                        vec.push(val);
                    }
                }
                result.push(vec);
            }
        }

        Grid::new(result)
    }
}

fn parse(input: &[u8]) -> IResult<&[u8], Grid> {
    map(separated_list1(line_ending, parse_line), |content| {
        Grid::new(content)
    })
    .parse(input)
}

fn parse_line(input: &[u8]) -> IResult<&[u8], Vec<i32>> {
    map_res(take_while1(|c: u8| c.is_dec_digit()), |a: &[u8]| {
        a.iter()
            .map(|c| (c - b'0').to_string().parse::<i32>())
            .collect()
    })
    .parse(input)
}

fn parse_input(input: &'static str) -> Result<Grid> {
    let (_, input) = parse(input.as_bytes()).expect("Parse failure");

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() -> Result<()> {
        assert_eq!(part_one(&parse_input(TESTDATA)?), 40);

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(part_one(&parse_input(DATA)?), 503);

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        assert_eq!(part_two(&parse_input(TESTDATA)?), 315);

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(part_two(&parse_input(DATA)?), 2853);

        Ok(())
    }
}
